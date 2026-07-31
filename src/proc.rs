use std::{
    io,
    path::Path,
    process::{self, Command},
    sync::mpsc,
    thread,
    time::Duration,
};

#[cfg(windows)]
use std::{env, fs};

use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

use crate::{consts::CLI_ARGS, log::Logger};

const PROCESS_WAIT_TIMEOUT: Duration = Duration::from_secs(30);

/// Waits for the process with the given PID to terminate. Returns true if the process terminates
/// or the target PID belongs to this process.
pub fn wait_process(pid: u32, logger: &Logger) -> bool {
    let target_pid = Pid::from_u32(pid);
    let self_pid = Pid::from_u32(process::id());

    if self_pid == target_pid {
        logger.batch_verbose("当前进程 ID 等于要等待的 ID, 推断进程已经继承");
        return true;
    }

    let pid_list = [target_pid];
    let processes_to_update = ProcessesToUpdate::Some(&pid_list);

    let mut sys = System::new();
    sys.refresh_processes_specifics(processes_to_update, true, ProcessRefreshKind::nothing().without_tasks());

    let Some(process) = sys.process(target_pid) else {
        logger.batch_verbose(format_args!("进程 {} 已不存在, 推断已经结束", target_pid));
        return true;
    };

    let start_time = process.start_time();
    logger.batch_info(format_args!(
        "等待进程 {} ({})，超时: 30s",
        target_pid,
        process.name().to_string_lossy()
    ));

    let (sender, receiver) = mpsc::sync_channel(1);
    thread::spawn(move || {
        let exit_status = sys.process(target_pid).and_then(|process| process.wait());
        _ = sender.send(exit_status.is_some());
    });

    match receiver.recv_timeout(PROCESS_WAIT_TIMEOUT) {
        Ok(true) => true,
        Ok(false) | Err(_) => original_process_exited(target_pid, start_time),
    }
}

fn original_process_exited(pid: Pid, start_time: u64) -> bool {
    let pid_list = [pid];
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&pid_list),
        true,
        ProcessRefreshKind::nothing().without_tasks(),
    );

    sys.process(pid)
        .is_none_or(|process| process.start_time() != start_time)
}

#[cfg(windows)]
pub fn stop_local_yogurt(logger: &Logger) -> io::Result<usize> {
    let target_path = env::current_dir()?.join("milky").join("yogurt.exe");
    let target_path = match fs::canonicalize(&target_path) {
        Ok(path) => path,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(0),
        Err(err) => return Err(err),
    };

    let mut sys = System::new_all();
    let pids: Vec<Pid> = sys
        .processes()
        .iter()
        .filter_map(|(pid, process)| {
            let exe_path = process.exe()?;
            let exe_path = fs::canonicalize(exe_path).ok()?;
            is_same_windows_path(&exe_path, &target_path).then_some(*pid)
        })
        .collect();

    for pid in &pids {
        logger.batch_warn(format_args!("发现仍在运行的内置 Milky 进程 {}, 尝试结束", pid));
        let Some(process) = sys.process(*pid) else {
            continue;
        };
        if !process.kill() {
            return Err(io::Error::other(format!("无法结束内置 Milky 进程 {}", pid)));
        }
    }

    if pids.is_empty() {
        return Ok(0);
    }

    let processes_to_update = ProcessesToUpdate::Some(&pids);
    for _ in 0..50 {
        sys.refresh_processes(processes_to_update, true);
        if pids.iter().all(|pid| sys.process(*pid).is_none()) {
            return Ok(pids.len());
        }
        thread::sleep(Duration::from_millis(100));
    }

    let remaining = pids
        .iter()
        .filter(|pid| sys.process(**pid).is_some())
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        format!("等待内置 Milky 进程退出超时: {}", remaining),
    ))
}

#[cfg(windows)]
fn is_same_windows_path(left: &Path, right: &Path) -> bool {
    left.to_string_lossy().eq_ignore_ascii_case(&right.to_string_lossy())
}

#[cfg(windows)]
pub fn restart_sealdice(logger: &Logger) -> io::Result<()> {
    if CLI_ARGS.skip_launch {
        logger.batch_info("跳过重启主程序");
        return Ok(());
    }

    logger.batch_info("3 秒后尝试重启主程序. 跨进程指令出现的错误可能不会被记录");
    thread::sleep(Duration::from_secs(3));

    let exe_path = Path::new("./").join(&CLI_ARGS.binary_name);
    Command::new(exe_path).spawn().map(|_| ())
}

#[cfg(all(test, windows))]
mod tests {
    use super::is_same_windows_path;
    use std::path::Path;

    #[test]
    fn windows_path_comparison_is_case_insensitive() {
        assert!(is_same_windows_path(
            Path::new(r"C:\SealDice\milky\yogurt.exe"),
            Path::new(r"c:\sealdice\MILKY\YOGURT.EXE"),
        ));
    }

    #[test]
    fn windows_path_comparison_rejects_other_directories() {
        assert!(!is_same_windows_path(
            Path::new(r"C:\SealDice\milky\yogurt.exe"),
            Path::new(r"C:\OtherSealDice\milky\yogurt.exe"),
        ));
    }
}

#[cfg(unix)]
pub fn restart_sealdice(logger: &Logger) -> io::Result<()> {
    use std::{fs, os::unix::fs::PermissionsExt};

    let exe_path = Path::new("./").join(&CLI_ARGS.binary_name);

    if cfg!(target_os = "macos") {
        let output = Command::new("xattr")
            .args(["-rd", "com.apple.quarantine", CLI_ARGS.binary_name.as_str()])
            .output();
        match output {
            Err(err) => logger.batch_warn(format_args!("未能除去可执行文件隔离属性, 运行可能出错: {}", err)),
            Ok(output) => {
                if output.status.success() {
                    logger.batch_success("成功除去可执行文件隔离属性");
                } else {
                    let err = String::from_utf8(output.stderr).unwrap_or(String::from("unknown"));
                    logger.batch_warn(format_args!("未能除去可执行文件隔离属性, 运行可能出错: {}", err));
                }
            }
        }
    }

    match fs::set_permissions(&exe_path, PermissionsExt::from_mode(0o755)) {
        Ok(_) => logger.batch_info("成功设置可执行文件权限"),
        Err(err) => logger.batch_warn(format_args!("设置可执行文件权限出错, 运行可能失败: {}", err)),
    }

    if CLI_ARGS.skip_launch {
        logger.batch_info("跳过重启主程序");
        return Ok(());
    }

    logger.batch_info("3 秒后尝试重启主程序. 跨进程指令出现的错误可能不会被记录");
    thread::sleep(Duration::from_secs(3));

    Command::new(exe_path).spawn().map(|_| ())
}
