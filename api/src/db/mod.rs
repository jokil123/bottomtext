pub mod db_error;
pub mod types;

use db_error::DbError;
use types::{FrameJson, FramesJson};

use std::{fs, io::Write};

pub const DB_PATH: &str = "db";
pub const FRAME_DELIMITER: &str = "\n";
pub const SUBTEXT_DELIMITER: &str = ";";
pub const ILLEGAL_CHARACTERS: &'static [&'static str] = &["\n", "\r", "\0"];

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

pub fn write_frame(frame: FrameJson) -> Result<(), DbError> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(DB_PATH)
        .map_err(|e| DbError::IoError(e))?;

    let text = format!(
        "{}{}{}{}",
        sanitize_input(frame.text),
        SUBTEXT_DELIMITER,
        sanitize_input(frame.subtext.unwrap_or_default()),
        FRAME_DELIMITER
    );

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
