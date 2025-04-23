//! Defines the expected command-line flags and arguments.

use clap::Parser;

/// Defines the command-line arguments and flags this program can accept.
#[derive(Parser, Debug)]
#[command(version)]
pub struct CliArgs {
    /// The update package.
    #[arg(long, short)]
    pub package: String,

    /// If present and not zero, wait for the process with this PID
    /// to terminate before proceeding.
    #[arg(long)]
    pub pid: u32,

    /// Skip launching SealDice after updating.
    #[arg(long = "skip", short)]
    pub skip_launch: bool,

    /// Display more information.
    #[arg(long)]
    pub verbose: bool,

    /// Produce no log file for the update.
    #[arg(long)]
    pub quiet: bool,
}
