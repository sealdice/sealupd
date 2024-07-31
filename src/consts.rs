//! Contains constants and statics that are used throughout the program.

use std::sync::LazyLock;

use clap::Parser;

use crate::cli::CliArgs;

/// The name of SealDice executable.
pub const EXE_NAME: &str =
    if cfg!(windows) { "sealdice-core.exe" } else { "sealdice-core" };

/// The name of this program.
pub const UPD_NAME: &str = if cfg!(windows) { "sealupd.exe" } else { "sealupd" };

/// The command-line arguments accepted from the caller.
pub static CLI_ARGS: LazyLock<CliArgs> = LazyLock::new(|| CliArgs::parse());
