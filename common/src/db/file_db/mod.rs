use async_trait::async_trait;

use std::{
    collections::HashMap, fmt::Debug, fs, future::Future, hash::Hash, io::Write, pin::Pin,
    ptr::read, sync::Arc, time::Duration,
};

use super::{
    db_error::DbError,
    pool::{DbConnection, DbPool},
    types::{FrameJson, FramesJson},
};

use crate::cache::Cache;

struct FileDbPool {
    connections: Vec<FileDbConnection>,
    cache: Cache<FramesJson, DbError>,
}

#[async_trait]
impl DbPool<FileDbConnection> for FileDbPool {
    async fn get(&self) -> Result<FileDbConnection, DbError> {
        todo!();
    }

    async fn put(&self, conn: FileDbConnection) -> Result<(), DbError> {
        todo!();
    }
}

impl FileDbPool {
    fn new() -> Self {
        FileDbPool {
            connections: vec![],
            cache: Cache::new(
                FramesJson { frames: vec![] },
                write_frames,
                read_frames,
                Some(100),
                Some(Duration::from_secs(10)),
            ),
        }
    }
}

struct FileDbConnection {
    pool: &'static FileDbPool,
    id: u32,
}

#[async_trait]
impl DbConnection for FileDbConnection {
    async fn query(&self, count: i32) -> Result<FramesJson, DbError> {
        todo!()
    }
    async fn insert(&self, frame: FrameJson) -> Result<(), DbError> {
        todo!()
    }
}

const DB_PATH: &str = "db";
const FRAME_DELIMITER: &str = "\n";
const SUBTEXT_DELIMITER: &str = ";";
const ILLEGAL_CHARACTERS: &'static [&'static str] = &["\n", "\r", "\0"];

pub fn read_frames() -> Result<FramesJson, DbError> {
    let contents = fs::read_to_string(DB_PATH).map_err(|e| DbError::IoError(e))?;

    let lines = contents
        .split(FRAME_DELIMITER)
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let frames = lines
        .iter()
        .map(|frame| {
            let splits = frame
                .split(SUBTEXT_DELIMITER)
                .map(str::to_string)
                .collect::<Vec<String>>();

            match splits.get(0) {
                Some(s) => Ok(FrameJson {
                    text: s.to_string(),
                    subtext: splits.get(1).map(|s| s.to_owned()),
                }),
                None => Err(DbError::ParseError),
            }
        })
        .collect::<Result<Vec<FrameJson>, DbError>>()?;

    Ok(FramesJson { frames })
}

pub fn write_frames(frames: FramesJson) -> Result<(), DbError> {
    let mut file = fs::OpenOptions::new()
        .append(false)
        .open(DB_PATH)
        .map_err(|e| DbError::IoError(e))?;

    let text = frames
        .frames
        .into_iter()
        .map(|f| {
            format!(
                "{}{}{}{}",
                sanitize_input(f.text),
                SUBTEXT_DELIMITER,
                sanitize_input(f.subtext.unwrap_or_default()),
                FRAME_DELIMITER
            )
        })
        .fold(String::new(), |a, b| a + &b);

    file.write_all(text.as_bytes())
        .map_err(|e| DbError::IoError(e))?;

    Ok(())
}

pub fn sanitize_input(input: String) -> String {
    let input = input
        .replace(FRAME_DELIMITER, "")
        .replace(SUBTEXT_DELIMITER, "");

    ILLEGAL_CHARACTERS
        .iter()
        .fold(input, |acc, c| acc.replace(c, ""))
}
