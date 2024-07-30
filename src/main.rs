use ::log::{debug, error, info};
use consts::CLI_ARGS;
use term_color::Colorable;

mod cli;
mod consts;
mod decompress;
mod log;
mod proc_wait;
mod term_color;
mod util;

fn main() {
    init_log();
    debug!("Program launched successfully.");
    print_appinfo();

    if CLI_ARGS.pid != 0 {
        debug!("Starting to wait for process {}.", CLI_ARGS.pid);
        proc_wait::wait(CLI_ARGS.pid);
    }

    debug!("Backing up old sealdice-core.");
    if let Err(err) = util::backup_sealdice() {
        error!("Backing up old executable failed: {}", err);
        eprintln!("{}: {}", "Backup failed".error(), err);
    }
    info!("Backup finished.");
    println!("{}", "Backup finished.".success());

    debug!(
        "Starting to decompress archive \"{}\"",
        CLI_ARGS.package.escape_debug()
    );
    if let Err(err) = decompress::decompress(CLI_ARGS.package.as_str(), ".") {
        error!("Failed to decompress: {}", err);
        eprintln!("{}: {}\nExiting.", "Extraction failed".error(), err);
        util::graceful_exit(1);
    }
    info!("Decompression succeeded.");
    println!("{}", "All files extracted.".success());

    util::restart_sealdice();
}

fn print_appinfo() {
    println!(
        "{} v{} --- Updater for SealDice.",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}

fn init_log() {
    match log::init_logger(CLI_ARGS.disable_log) {
        Ok(lname) => {
            if CLI_ARGS.disable_log {
                println!(
                    "{}",
                    "No log will be produced due to --disable-log flag.".warn()
                );
                return;
            }
            println!("{}: {}", "Update log is stored at".warn(), lname);
        }
        Err(err) => {
            eprintln!("{}: {}", "Failed to initialize log".error(), err);
        }
    }
}
