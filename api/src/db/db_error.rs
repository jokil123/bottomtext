use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("IO error")]
    IoError(std::io::Error),
    #[error("Parse error")]
    ParseError,
}
