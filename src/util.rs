//! Contains other operations needed by the program.

use std::{
    fs, io,
    path::Path,
    process::{self, Command},
    thread,
    time::Duration,
};

use crate::{
    consts::{CLI_ARGS, EXE_NAME},
    log,
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

/// Copies existing executable to `sealdice-core_old` (Unix) or `sealdice-core.exe.old`
/// (Windows), returning whether a copy is made or any error that occurs.
pub fn backup_sealdice() -> Result<bool, io::Error> {
    let cwd = Path::new(".");
    if !cwd.join(EXE_NAME).exists() {
        return Ok(false);
    }

    let old = if cfg!(windows) {
        format!("{}.old", EXE_NAME)
    } else {
        format!("{}_old", EXE_NAME)
    };

    fs::rename(cwd.join(EXE_NAME), cwd.join(old))?;
    Ok(true)
}

/// Attempts to run `chmod 755` on `sealdice-core` and then start it. Exit the program
/// if anything goes wrong.
#[cfg(target_family = "unix")]
pub fn restart_sealdice() {
    use std::os::unix::{fs::PermissionsExt, process::CommandExt};

    let dest = Path::new(".").join(EXE_NAME);

    match fs::set_permissions(&dest, PermissionsExt::from_mode(0o755)) {
        Ok(_) => info!("Executed chmod 755 {:?}", dest),
        Err(err) => {
            log::display_error("chmod failure", err);
            graceful_exit(1);
        }
    }

    if CLI_ARGS.skip_launch {
        log::display_warn("SealDice will not be launched due to --skip-launch");
        graceful_exit(0);
    }

    log::display_success("Update completed, launching SealDice in a few seconds.");
    log::display_warn("If SealDice is not run, check any console output and the update log, and refer the situation to the developers.");
    thread::sleep(Duration::from_secs(2));

    debug!("Launching with command {:?}", dest);
    let err = Command::new(dest).current_dir(Path::new("./")).exec();
    log::display_error("Launching failed", err);
    graceful_exit(1);
}

/// Attempts to start `sealdice-core`. Exit the program if anything goes wrong.
#[cfg(target_family = "windows")]
pub fn restart_sealdice() {
    let dest = Path::new(".").join(EXE_NAME);

    if CLI_ARGS.skip_launch {
        log::display_warn("SealDice will not be launched due to --skip-launch");
        graceful_exit(0);
    }

    log::display_success("Update completed, launching SealDice in a few seconds.");
    log::display_warn("If SealDice is not run, check any console output and the update log, and refer the situation to the developers.");
    thread::sleep(Duration::from_secs(2));

    let res = Command::new("cmd")
        .current_dir(Path::new("./"))
        .args(["/C", "start", "", &dest.to_string_lossy()])
        .spawn();
    if let Err(err) = res {
        log::display_error("Launching failed", err);
        graceful_exit(1);
    }
}
