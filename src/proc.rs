//! Provides [`wait`] to wait for the caller process to terminate.

use std::{process, thread, time::Duration};

use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

/// Waits for the process with specified PID to finish. If the program is spawned
/// by that process, finishes waiting if the program has inherited the PID.
pub fn wait(pid: u32) {
    let pid = Pid::from_u32(pid);
    let mut prog_pid = Pid::from_u32(process::id());
    if prog_pid == pid {
        return;
    }

    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );

    loop {
        match sys.process(pid) {
            None => break,
            Some(proc) => {
                if proc.name() == env!("CARGO_PKG_NAME") || prog_pid == pid {
                    break;
                }
                sys.refresh_processes();
                prog_pid = Pid::from_u32(process::id());
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
