//! Contains other operations needed by the program.

use std::{
    fs, io,
    path::Path,
    process::{self, Command},
    thread,
    time::Duration,
};

use log::{debug, error, info};

use crate::{
    consts::{CLI_ARGS, EXE_NAME},
    term_color::Colorable,
};

/// Asks for user confirmation before exiting on Windows.
pub fn graceful_exit(code: i32) {
    if cfg!(windows) && code != 0 {
        use std::io::{self, Read};
        println!("\nPress ENTER to exit ...");
        _ = io::stdin().read_exact(&mut [0u8]);
    }
    process::exit(code);
}

pub fn backup_sealdice() -> Result<(), io::Error> {
    let cwd = Path::new(".");
    if !cwd.join(EXE_NAME).exists() {
        return Ok(());
    }

    let old = if cfg!(windows) {
        format!("{}.old", EXE_NAME)
    } else {
        format!("{}_old", EXE_NAME)
    };

    fs::rename(cwd.join(EXE_NAME), cwd.join(old))
}

#[cfg(target_family = "unix")]
pub fn restart_sealdice() {
    use std::os::unix::{fs::PermissionsExt, process::CommandExt};

    let dest = Path::new("./").join(EXE_NAME);

    match fs::set_permissions(&dest, PermissionsExt::from_mode(0o755)) {
        Ok(_) => info!("chmod 755 {:?}", dest),
        Err(err) => {
            error!("Failed to set executable permission: {}", err);
            eprintln!("{}: {}", "chmod error".error(), err);
            graceful_exit(1);
        }
    }

    if CLI_ARGS.skip_launch {
        info!("Exiting due to --skip flag");
        println!("{}", "Exiting due to --skip flag".warn());
        graceful_exit(0);
    }

    debug!("Sleep for 2 seconds, then starting SealDice.");
    println!(
        "{}",
        "Everything is done, launching in 2 seconds ...".success()
    );
    thread::sleep(Duration::from_secs(2));

    let err = Command::new(dest).current_dir(Path::new("./")).exec();
    error!("Launching SealDice failed: {}", err);
    eprintln!("{}: {}", "Failed to start core".error(), err);
    graceful_exit(1);
}

#[cfg(target_family = "windows")]
pub fn restart_sealdice() {
    let dest = Path::new("./").join(EXE_NAME);

    if CLI_ARGS.skip_launch {
        info!("Exiting due to --skip flag");
        println!("{}", "Exiting due to --skip flag".warn());
        graceful_exit(0);
    }

    debug!("Sleep for 2 seconds, then starting SealDice.");
    println!(
        "{}",
        "Everything is done, launching in 2 seconds ...".success()
    );
    thread::sleep(Duration::from_secs(2));

    let res = Command::new("cmd")
        .current_dir(Path::new("./"))
        .args(["/C", "start", "", &dest.to_string_lossy()])
        .spawn();
    if let Err(err) = res {
        error!("Launching SealDice failed: {}", err);
        eprintln!("{}: {}", "Failed to start core".error(), err);
        graceful_exit(1);
    }
}
