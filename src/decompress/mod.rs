use std::{
    fs::{self, File},
    io::{self, Read, Seek},
    path::{Component, Components, Path, PathBuf},
};

use flate2::read::GzDecoder as GzReader;
use tar::Archive as TarArchive;
use zip::ZipArchive;

use crate::{
    consts::{CLI_ARGS, EXE_NAME, UPDATER_NAME},
    log::Logger,
};
use error::DecompressError;

mod error;

pub fn backup_sealdice() -> io::Result<bool> {
    let exe_path = Path::new(EXE_NAME);
    if !exe_path.exists() {
        return Ok(false);
    }

    let old_name = if cfg!(windows) {
        format!("{}.old", EXE_NAME)
    } else {
        format!("{}_old", EXE_NAME)
    };

    fs::rename(exe_path, &old_name)?;
    Ok(true)
}

/// Decompresses the package, as provided by CLI_ARGS.package, to the current directory.
pub fn decompress(logger: &Logger) -> Result<usize, DecompressError> {
    let package_path = Path::new(&CLI_ARGS.package);
    let mut file = File::open(package_path)?;

    let is_zip = package_path
        .extension()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("gz")
        .eq("zip");

    if is_zip {
        decompress_zip(&mut file, logger)
    } else {
        decompress_tarball(&mut file, logger)
    }
}

fn make_file<R: Read>(mut src: R, dest: &Path) -> Result<(), io::Error> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    if dest.is_dir() || dest.to_string_lossy().ends_with('/') {
        fs::create_dir_all(dest)?;
    } else {
        let mut out_file = File::create(dest)?;
        io::copy(&mut src, &mut out_file)?;
    }

    Ok(())
}

/// Upon success, return the count of entries decompressed.
fn decompress_zip<R: Read + Seek>(mut file: R, logger: &Logger) -> Result<usize, DecompressError> {
    let mut archive = ZipArchive::new(&mut file)?;
    let entry_count = archive.len();

    let updater_path = Path::new(UPDATER_NAME);

    for index in 0..entry_count {
        let mut entry = archive.by_index(index)?;
        let entry_name = entry
            .enclosed_name()
            .ok_or(DecompressError::SlipError(entry.name().to_owned()))?;

        let mut dest = PathBuf::new();
        if entry_name == updater_path {
            dest.push("new-updater");
        }
        dest.push(entry_name);

        logger.console_verbose(format_args!("[{}/{}] {:?}", index + 1, entry_count, dest));
        make_file(&mut entry, &dest)?;
    }

    Ok(entry_count)
}

struct SeekableTarball<R: Read + Seek> {
    inner: R,
}

impl<R: Read + Seek> SeekableTarball<R> {
    pub fn new(inner: R) -> SeekableTarball<R> {
        SeekableTarball { inner }
    }

    pub fn count(&mut self) -> io::Result<usize> {
        let gz_decoder = GzReader::new(&mut self.inner);
        let mut tar_archive = TarArchive::new(gz_decoder);

        let count = tar_archive.entries()?.count();
        self.inner.seek(io::SeekFrom::Start(0))?;

        Ok(count)
    }

    pub fn into_tarball(self) -> TarArchive<GzReader<R>> {
        let gz_decoder = GzReader::new(self.inner);
        TarArchive::new(gz_decoder)
    }
}

/// Upon success, return the count of entries decompressed.
fn decompress_tarball<R: Read + Seek>(mut reader: R, logger: &Logger) -> Result<usize, DecompressError> {
    let mut archive = SeekableTarball::new(&mut reader);
    let entry_count = archive.count()?;

    let updater_path = Path::new(UPDATER_NAME);

    let mut archive = archive.into_tarball();
    let entries = archive.entries()?;

    for (index, entry) in entries.enumerate() {
        let mut entry = entry?;
        let entry_name = entry.path()?;

        if is_suspicious_path(entry_name.components()) {
            let p = String::from(entry_name.to_string_lossy());
            return Err(DecompressError::SlipError(p));
        }

        let mut dest = PathBuf::new();
        if entry_name == updater_path {
            dest.push("new-updater");
        }
        dest.push(entry_name);

        logger.console_verbose(format_args!("[{}/{}] {:?}", index + 1, entry_count, dest));
        make_file(&mut entry, &dest)?;
    }

    Ok(entry_count)
}

fn is_suspicious_path(components: Components) -> bool {
    components
        .into_iter()
        .any(|c| matches!(c, Component::ParentDir | Component::RootDir))
}
