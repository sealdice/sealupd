use std::{fmt, io};

use zip::result::ZipError;

#[derive(Debug)]
pub enum DecompressError {
    Io(io::Error),
    Zip(ZipError),
    Slip(String),
}

impl std::error::Error for DecompressError {}

impl From<io::Error> for DecompressError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ZipError> for DecompressError {
    fn from(value: ZipError) -> Self {
        Self::Zip(value)
    }
}

impl fmt::Display for DecompressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecompressError::Io(io_err) => io_err.fmt(f),
            DecompressError::Zip(zip_err) => zip_err.fmt(f),
            DecompressError::Slip(entry) => {
                write!(f, "entry '{}' might lead to slip exploit", entry.escape_debug())
            }
        }
    }
}
