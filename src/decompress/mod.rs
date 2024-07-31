//! Provides functionalities to decompresses a ZIP archive or tarball.

use std::{
    error::Error,
    fs::File,
    io::{Seek, SeekFrom},
    path::Path,
};

use ::zip::result::ZipError;
use log::debug;

mod tar;
mod util;
mod zip;

/// First tries decompressing the source file as a ZIP archive, and on [`ZipError::InvalidArchive`] tries
/// decompressing it as a tarball.
pub fn decompress<P>(src: P, dest: P) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut file =
        File::open(src.as_ref()).map_err(|e| format!("open {:?}: {}", src.as_ref(), e))?;
    let dest = dest.as_ref();

    if let Err(err) = zip::decompress(&mut file, dest) {
        if !matches!(err.downcast_ref(), Some(ZipError::InvalidArchive(_))) {
            Err(format!("decompress zip: {}", err))?;
        }

        debug!("Invalid ZIP archive detected, falling back to tarball.");
        file.seek(SeekFrom::Start(0))?;
        tar::decompress(file, dest).map_err(|e| format!("decompress tar: {}", e))?;
    }

    Ok(())
}
