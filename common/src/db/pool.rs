use async_trait::async_trait;

use super::db_error::DbError;

use crate::frame::{FrameJson, FramesJson};

#[async_trait]
pub trait DbPool<T: DbConnection> {
    async fn get(&self) -> Result<T, DbError>;
    async fn put(&self, conn: T) -> Result<(), DbError>;
}

#[async_trait]
pub trait DbConnection {
    async fn query(&self, count: i32) -> Result<FramesJson, DbError>;
    async fn insert(&self, frame: FrameJson) -> Result<(), DbError>;
}
