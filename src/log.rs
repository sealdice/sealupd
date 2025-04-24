use std::fmt;
use std::io::{self, Stderr, Stdout};

use chrono::Local;
use fern::Dispatch;
use log::{LevelFilter, debug, error, info, warn};

use crate::consts::{CLI_ARGS, IS_ASCII_SUPPORTED};

/// A convenient struct grouping console and file logging.
pub struct Logger {
    stdout: Stdout,
    stderr: Stderr,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            stderr: io::stderr(),
            stdout: io::stdout(),
        }
    }

    pub fn console_error<D: fmt::Display>(&self, message: D) {
        let header: &str = if *IS_ASCII_SUPPORTED { "\x1b[1;31mE\x1b[0m" } else { "E" };
        self.console_write(&self.stderr, format_args!("{} {}\n", header, message));
    }

    pub fn console_success<D: fmt::Display>(&self, message: D) {
        let header: &str = if *IS_ASCII_SUPPORTED { "\x1b[1;32mO\x1b[0m" } else { "O" };
        self.console_write(&self.stdout, format_args!("{} {}\n", header, message));
    }

    pub fn console_warn<D: fmt::Display>(&self, message: D) {
        let header: &str = if *IS_ASCII_SUPPORTED { "\x1b[1;33mW\x1b[0m" } else { "W" };
        self.console_write(&self.stderr, format_args!("{} {}\n", header, message));
    }

    pub fn console_info<D: fmt::Display>(&self, message: D) {
        self.console_write(&self.stderr, format_args!("{}\n", message));
    }

    /// No-op if CLI_ARGS.verbose is false.
    pub fn console_verbose<D: fmt::Display>(&self, message: D) {
        if CLI_ARGS.verbose {
            self.console_write(&self.stdout, format_args!("{}\n", message));
        }
    }

    pub fn batch_error<D: fmt::Display>(&self, message: D) {
        self.console_error(&message);
        self.file_error(message);
    }

    pub fn batch_success<D: fmt::Display>(&self, message: D) {
        self.console_success(&message);
        self.file_info(message);
    }

    pub fn batch_warn<D: fmt::Display>(&self, message: D) {
        self.console_warn(&message);
        self.file_warn(message);
    }

    pub fn batch_info<D: fmt::Display>(&self, message: D) {
        self.console_info(&message);
        self.file_info(message);
    }

    /// No-op if CLI_ARGS.verbose is false.
    pub fn batch_verbose<D: fmt::Display>(&self, message: D) {
        self.console_verbose(&message);
        self.file_verbose(message);
    }

    pub fn file_error<D: fmt::Display>(&self, message: D) {
        error!("{}", message);
    }

    pub fn file_info<D: fmt::Display>(&self, message: D) {
        info!("{}", message);
    }

    pub fn file_warn<D: fmt::Display>(&self, message: D) {
        warn!("{}", message);
    }

    /// No-op if CLI_ARGS.verbose is false.
    pub fn file_verbose<D: fmt::Display>(&self, message: D) {
        if CLI_ARGS.verbose {
            debug!("{}", message);
        }
    }

    fn console_write<W: io::Write, D: fmt::Display>(&self, mut device: W, message: D) {
        _ = write!(device, "{}", message);
    }
}

/// Initializes the file logger. Upon success, the log file's name is returned.
pub fn init_logger(disabled: bool) -> Result<String, fern::InitError> {
    let (level_filter, log_name) = if disabled {
        (LevelFilter::Off, String::new())
    } else {
        let date = Local::now().format("%y%m%d_%H%M%S").to_string();
        (LevelFilter::Debug, format!("updater_{}.txt", date))
    };

    let cfg = Dispatch::new()
        .format(|out, msg, rec| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%F %H:%M:%S%.3f"),
                rec.level(),
                msg
            ))
        })
        .level(level_filter);

    if !disabled {
        cfg.chain(fern::log_file(&log_name)?).apply()?;
    } else {
        cfg.apply()?;
    }

    Ok(log_name)
}
