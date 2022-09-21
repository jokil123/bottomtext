use std::fmt::Debug;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures_util::{Future, FutureExt};

pub struct Cache<T, E: Debug> {
    cache_data: Arc<T>,
    write: fn(&T) -> Box<dyn Future<Output = Result<(), E>> + Unpin>,
    read: fn() -> Box<dyn Future<Output = Result<T, E>> + Unpin>,
    max_read_writes: Option<u32>,
    read_writes: u32,
    max_interval: Option<Duration>,
    last_invalidation: Instant,
}

impl<T, E: Debug> Cache<T, E> {
    pub fn get_cache(&self) -> Arc<T> {
        let cache = self.cache_data.clone();
        self.validate();
        cache
    }

    pub fn set_cache(&mut self, data: T) {
        self.cache_data = Arc::new(data);
        self.validate();
    }

    pub fn merge_cache(&mut self, reducer: fn(cache: Arc<T>, new: Arc<T>) -> T) {
        let newCache = reducer(self.cache_data.clone(), self.cache_data.clone());
        self.cache_data = Arc::new(newCache);
        self.validate();
    }

    async fn write_cache(&self) {
        if let Err(e) = (self.write)(&*self.cache_data).await {
            eprintln!("Error writing cache: {:#?}", e);
        }
    }

    async fn read_cache(&mut self) {
        match (self.read)().await {
            Ok(data) => self.cache_data = Arc::new(data),
            Err(e) => {
                eprintln!("Error reading cache: {:#?}", e);
            }
        };
    }

    fn validate(&mut self) {
        self.read_writes += 1;

        if let Some(max_read_writes) = self.max_read_writes {
            if self.read_writes >= max_read_writes {
                self.invalidate();
            }
        }

        if let Some(max_interval) = self.max_interval {
            if self.last_invalidation.elapsed() >= max_interval {
                self.invalidate();
            }
        }
    }

    async fn invalidate(&mut self) {
        self.write_cache().await;
        self.read_cache().await;

        self.last_invalidation = Instant::now();
        self.read_writes = 0;
    }

    fn new(
        init_data: T,
        write: fn(&T) -> Box<dyn Future<Output = Result<(), E>> + Unpin>,
        read: fn() -> Box<dyn Future<Output = Result<T, E>> + Unpin>,
        max_read_writes: Option<u32>,
        max_interval: Option<Duration>,
    ) -> Self {
        Self {
            cache_data: Arc::new(init_data),
            write,
            read,
            max_read_writes: None,
            read_writes: 0,
            max_interval: None,
            last_invalidation: Instant::now(),
        }
    }
}
