use rusqlite;
use std::sync::PoisonError;

#[derive(Debug)]
pub enum Error {
    Database(rusqlite::Error),
    LockError(String),
    NotFound(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(error: PoisonError<T>) -> Self {
        Error::LockError(format!("{error}"))
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::Database(error)
    }
}
