//! Handles ZIP decompression.

use std::{
    error::Error,
    io::{Read, Seek},
    path::Path,
};

use zip::ZipArchive;

use super::{super::consts::UPD_NAME, util};

/// Decompresses the provided reader assuming it is associated with ZIP archive data.
/// Decompressed files will be written under `dest`. Terminates if any entry is not safe,
/// i.e. contains the parent path `../` or root path `/`.
pub fn decompress<R>(reader: R, dest: &Path) -> Result<(), Box<dyn Error>>
where
    R: Read + Seek,
{
    let mut archive = ZipArchive::new(reader)?;
    let ecount = archive.len();

    for index in 0..ecount {
        let mut entry = archive.by_index(index)?;
        let ename = entry
            .enclosed_name()
            .ok_or(format!("unsafe entry: \"{}\"", entry.name().escape_debug()))?;

        let dest = if ename.to_string_lossy() != UPD_NAME {
            dest.join(ename)
        } else {
            dest.join("new_updater").join(ename)
        };

        println!("[{}/{}] {:?}", index + 1, ecount, dest);
        util::make_file(&mut entry, &dest).map_err(|e| format!("make file: {}", e))?;
    }

    Ok(())
}
