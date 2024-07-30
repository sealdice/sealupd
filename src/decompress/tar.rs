//! Handles tarball decompression.

use std::{
    error::Error,
    io::{self, Read, Seek, SeekFrom},
    path::{Component, Components, Path},
};

use flate2::read::GzDecoder;
use tar::Archive;

use super::{super::consts::UPD_NAME, util};

/// Helper type that effectively resets the underlying reader's position after collecting
/// the entry count of a [`tar::Archive`], since getting the count relies on exhausting the iterator.
/// The reader must also implement [`Seek`].
struct ReseekableArchive<R: Read + Seek>(R);

impl<R> ReseekableArchive<R>
where
    R: Read + Seek,
{
    pub fn new(reader: R) -> Self {
        ReseekableArchive(reader)
    }

    pub fn count(&mut self) -> io::Result<usize> {
        let dec = GzDecoder::new(&mut self.0);
        let mut archive = Archive::new(dec);

        let count = archive.entries()?.count();
        self.0.seek(SeekFrom::Start(0))?;

        Ok(count)
    }

    pub fn into_archive(self) -> Archive<GzDecoder<R>> {
        let dec = GzDecoder::new(self.0);
        Archive::new(dec)
    }
}

/// Decompresses the provided reader assuming it is associated with a GZIP-compressed
/// TAR archive (`.tar.gz`). Decompressed files will be written under `dest`. Terminates
/// if any entry is not safe, i.e. contains the parent path `../` or root path `/`.
pub fn decompress<R>(reader: R, dest: &Path) -> Result<(), Box<dyn Error>>
where
    R: Read + Seek,
{
    let mut archive = ReseekableArchive::new(reader);
    let ecount = archive.count()?;

    let mut archive = archive.into_archive();
    let entries = archive.entries()?;

    for (index, entry) in entries.enumerate() {
        let mut entry = entry?;

        let ename = entry.path()?;
        if !is_safe_path(ename.components()) {
            Err(format!("unsafe entry: {:?}", ename))?;
        }

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

fn is_safe_path(com: Components) -> bool {
    com.into_iter()
        .any(|c| matches!(c, Component::ParentDir) || matches!(c, Component::RootDir))
}
