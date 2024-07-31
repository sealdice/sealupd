//! Defines the expected command-line flags and arguments.

use clap::Parser;

/// Defines the command-line arguments and flags this program can accept.
#[derive(Parser, Debug)]
#[command(version)]
pub struct CliArgs {
    /// The update package
    #[arg(long, short, alias = "upgrade")]
    pub package: String,

    /// Caller's PID
    #[arg(long)]
    pub pid: u32,

    /// Skip launching SealDice after updating
    #[arg(long, alias = "skip")]
    pub skip_launch: bool,

    /// Produce no log for the update
    #[arg(long)]
    pub disable_log: bool,
}
