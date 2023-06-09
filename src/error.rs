pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Raised IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed converting [u8] to String: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("Failed parsing as int: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Failed serializing to json: {0}")]
    SerdeJson(#[from] serde_json::Error),
    // Application specific
    #[error("Given golumn is too high: {given} / {limit}")]
    ColumnTooHigh { given: usize, limit: usize },
    #[error("Given row is too high: {given} / {limit}")]
    RowTooHigh { given: usize, limit: usize },
    #[error("Unknown error")]
    Unknown,
}
