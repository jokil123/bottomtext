mod db_error;
use db_error::DbError;
use rocket::http::private::cookie::ParseError;

use std::{error::Error, fs};

pub struct FramesJson {
    pub frames: Vec<FrameJson>,
}

pub struct FrameJson {
    pub text: String,
    pub subtext: Option<String>,
}

pub fn get_frames() -> Result<FramesJson, DbError> {
    let contents = fs::read_to_string("db").map_err(|e| DbError::IoError(e))?;

    let lines = contents
        .split("\n")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let frames = lines
        .iter()
        .map(|frame| {
            let splits = frame
                .split(",")
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
