use ::log::debug;
use color::{Colorable, ColoredString};
use consts::CLI_ARGS;

mod cli;
mod color;
mod consts;
mod decompress;
mod log;
mod proc;
mod util;

fn main() {
    init_log();
    debug!("Program launched successfully.");
    print_appinfo();

    if CLI_ARGS.pid != 0 {
        log::display_info(&format!("Waiting process {} termination.", CLI_ARGS.pid));
        proc::wait(CLI_ARGS.pid);
    }

    match util::backup_sealdice() {
        Ok(_) => log::display_success("Old executable backed up."),
        Err(err) => log::display_error("Backup of old executable failed", err),
    }

    log::display_warn(&format!("Archive: \"{}\"", CLI_ARGS.package.escape_debug()));
    if let Err(err) = decompress::decompress(CLI_ARGS.package.as_str(), "") {
        log::display_error("Decompression failed", err.as_ref());
        util::graceful_exit(1);
    }
    log::display_success("All files extracted.");

    println!();
    util::restart_sealdice();
}

fn print_appinfo() {
    println!(
        ">>> {} v{}    Updater for SealDice. <<<",
        ColoredString::new(env!("CARGO_PKG_NAME"), &[33, 1]),
        env!("CARGO_PKG_VERSION")
    );
}

fn init_log() {
    match log::init_logger(CLI_ARGS.disable_log) {
        Ok(lname) => {
            if CLI_ARGS.disable_log {
                println!("{}", "Log disabled due to --disable-log flag.".warn());
                return;
            }
            println!("{}: {}", "Update log stored at".warn(), lname);
        }
        Err(err) => eprintln!("{}: {}", "Failed to initialize log".error(), err),
    }
}
