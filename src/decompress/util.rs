//! Contains common functions used in the decompress module.

use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

/// Creates all files and directories in the destination path and copies
/// from the source if it is a file.
pub fn make_file<R>(src: &mut R, dest: &Path) -> Result<(), Box<dyn Error>>
where
    R: Read,
{
    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    if dest.is_dir() || dest.to_string_lossy().ends_with("/") {
        fs::create_dir_all(dest)?;
    } else {
        let mut out_file = File::create(dest)?;
        io::copy(src, &mut out_file)?;
    }

    Ok(())
}
