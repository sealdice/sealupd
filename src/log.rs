//! Initialzes the logger.

use chrono::Local;
use fern::{Dispatch, InitError};
use log::LevelFilter;

/// Initalizes the logger, returning the log file's name. If `disabled` is true the logger will do nothing.
pub fn init_logger(disabled: bool) -> Result<String, InitError> {
    let (llevel, lname) = if disabled {
        (LevelFilter::Off, String::new())
    } else {
        let date = Local::now().format("%F_%H%M%S").to_string();
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
