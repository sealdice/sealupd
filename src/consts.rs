use std::{env, sync::LazyLock};

use clap::Parser;

use crate::cli::CliArgs;

/// The name of SealDice executable.
pub const EXE_NAME: &str = if cfg!(windows) { "sealdice-core.exe" } else { "sealdice-core" };

/// The name of this program.
pub const UPDATER_NAME: &str = if cfg!(windows) { "sealupd.exe" } else { "sealupd" };

/// The command-line arguments accepted from the caller.
pub static CLI_ARGS: LazyLock<CliArgs> = LazyLock::new(|| CliArgs::parse());

/// Whether ASCII colour codes are supported by the current environment.
/// On Windows, it checks whether the current terminal is Windows Terminal by looking for
/// the WT_SESSION variable.
pub const IS_ASCII_SUPPORTED: LazyLock<bool> = LazyLock::new(|| cfg!(unix) || env::var("WT_SESSION").is_ok());
