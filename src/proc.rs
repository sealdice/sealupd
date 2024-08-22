//! Provides [`wait`] to wait for the caller process to terminate.

use std::{process, thread, time::Duration};

use log::debug;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

struct RefreshSpecs {
    refresh_kind: RefreshKind,
    proc_refresh_kind: ProcessRefreshKind,
}

impl RefreshSpecs {
    pub fn new() -> Self {
        let proc_refresh_kind = ProcessRefreshKind::new();
        RefreshSpecs {
            refresh_kind: RefreshKind::new().with_processes(proc_refresh_kind),
            proc_refresh_kind,
        }
    }
}

/// Waits for the process with specified PID to finish. If the program is spawned
/// by that process, finishes waiting if the program has inherited the PID.
pub fn wait(pid: u32) {
    let pid = Pid::from_u32(pid);
    let mut prog_pid = Pid::from_u32(process::id());
    if prog_pid == pid {
        return;
    }

    let specs = RefreshSpecs::new();
    let mut sys = System::new_with_specifics(specs.refresh_kind);

    loop {
        match sys.process(pid) {
            None => break,
            Some(proc) => {
                let pname = proc.name();
                debug!(
                    "Found process {} \"{}\"",
                    pid,
                    pname.to_string_lossy().escape_debug()
                );
                if pname == env!("CARGO_PKG_NAME") || prog_pid == pid {
                    break;
                }

                sys.refresh_processes_specifics(ProcessesToUpdate::All, specs.proc_refresh_kind);
                prog_pid = Pid::from_u32(process::id());
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
