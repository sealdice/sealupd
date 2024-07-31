//! Initialzes the logger.

use std::error::Error;

use chrono::Local;
use fern::{Dispatch, InitError};
use log::{error, info, warn, LevelFilter};

use crate::color::Colorable;

/// Initalizes the logger, returning the log file's name. If `disabled` is true the logger will do nothing.
pub fn init_logger(disabled: bool) -> Result<String, InitError> {
    let (llevel, lname) = if disabled {
        (LevelFilter::Off, String::new())
    } else {
        let date = Local::now().format("%y%m%d_%H%M%S").to_string();
        (LevelFilter::Debug, format!("updater_{}.txt", date))
    };

    let cfg = Dispatch::new()
        .format(|out, msg, rec| {
            out.finish(format_args!(
                "{} [{}] {}:{} {}",
                Local::now().format("%F %H:%M:%S%.3f"),
                rec.level(),
                rec.file().unwrap_or("<unknown file>"),
                rec.line().unwrap_or_default(),
                msg
            ))
        })
        .level(llevel);

    if !disabled {
        cfg.chain(fern::log_file(&lname)?).apply()?;
    } else {
        cfg.apply()?;
    }

    Ok(lname)
}

/// Logs the error and prints it to standard error.
pub fn display_error<E>(header: &str, err: E)
where
    E: Error,
{
    error!("{}: {}", header, err);
    eprintln!("[ERROR] {}: {}", header.error(), err);
}

/// Logs the information and prints to standard output.
pub fn display_info(d: &str) {
    info!("{}", d);
    println!("[INFO] {}", d);
}

/// Logs the warning and prints to standard output.
pub fn display_warn(d: &str) {
    warn!("{}", d);
    println!("[WARN] {}", d.warn());
}

/// Same as [`display_info`], but changes terminal color.
pub fn display_success(d: &str) {
    info!("{}", d);
    println!("[INFO] {}", d.success());
}
