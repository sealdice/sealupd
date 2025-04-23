use std::{fmt, io};

use zip::result::ZipError;

#[derive(Debug)]
pub enum DecompressError {
    IoError(io::Error),
    ZipError(ZipError),
    SlipError(String),
}

impl std::error::Error for DecompressError {}

impl From<io::Error> for DecompressError {
    fn from(value: io::Error) -> Self {
        DecompressError::IoError(value)
    }
}

impl From<ZipError> for DecompressError {
    fn from(value: ZipError) -> Self {
        DecompressError::ZipError(value)
    }
}

impl fmt::Display for DecompressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecompressError::IoError(io_err) => io_err.fmt(f),
            DecompressError::ZipError(zip_err) => zip_err.fmt(f),
            DecompressError::SlipError(entry) => {
                write!(f, "entry '{}' might lead to slip exploit", entry.escape_debug())
            }
        }
    }
}
