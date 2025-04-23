use core::time;
use std::{
    io,
    path::Path,
    process::{self, Command},
    thread,
};

use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

use crate::{
    consts::{CLI_ARGS, EXE_NAME},
    log::Logger,
};

/// Waits for the process with given PID to terminate. Returns true if the process terminates
/// or has the same PID with this process.
pub fn wait_process(pid: u32, max_retries: usize, logger: &Logger) -> bool {
    let target_pid = Pid::from_u32(pid);
    let self_pid = Pid::from_u32(process::id());

    if self_pid == target_pid {
        logger.batch_verbose("当前进程 ID 等于要等待的 ID, 推断进程已经继承.");
        return true;
    }

    let pid_list = [target_pid];
    let processes_to_update = ProcessesToUpdate::Some(&pid_list);

    let mut sys = System::new();
    sys.refresh_processes_specifics(processes_to_update, true, ProcessRefreshKind::nothing());

    for i in 0..max_retries {
        match sys.process(target_pid) {
            None => {
                logger.batch_verbose(format_args!("进程 {} 已不存在, 推断已经结束.", target_pid));
                return true;
            }
            Some(process) => {
                if process.pid() == self_pid {
                    logger.batch_verbose("进程名称等于升级器名称, 推断进程已经继承.");
                    return true;
                }

                let name = process.name();
                logger.batch_verbose(format_args!(
                    "找到进程 {}, 尝试次数 {}/{}",
                    name.to_string_lossy(),
                    i + 1,
                    max_retries
                ));

                sys.refresh_processes(processes_to_update, true);
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    }

    false
}

#[cfg(windows)]
pub fn restart_sealdice(logger: &Logger) -> io::Result<()> {
    if CLI_ARGS.skip_launch {
        logger.batch_info("跳过重启主程序.");
        return Ok(());
    }

    logger.batch_info("3 秒后尝试重启主程序. 跨进程指令出现的错误可能不会被记录.");
    thread::sleep(time::Duration::from_secs(3));

    let exe_path = Path::new("./").join(EXE_NAME);
    let mut command = Command::new(exe_path);

    command.spawn().map(|_| ())
}

#[cfg(unix)]
pub fn restart_sealdice(logger: &Logger) -> io::Result<()> {
    use std::{fs, os::unix::fs::PermissionsExt};

    let exe_path = Path::new("./").join(EXE_NAME);

    if cfg!(target_os = "macos") {
        let output = Command::new("xattr")
            .args(&["-rd", "com.apple.quarantine", EXE_NAME])
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
        Ok(_) => logger.batch_info("成功设置可执行文件权限."),
        Err(err) => logger.batch_warn(format_args!("设置可执行文件权限出错, 运行可能失败: {}", err)),
    }

    if CLI_ARGS.skip_launch {
        logger.batch_info("跳过重启主程序.");
        return Ok(());
    }

    logger.batch_info("3 秒后尝试重启主程序. 跨进程指令出现的错误可能不会被记录.");
    thread::sleep(time::Duration::from_secs(3));

    let mut command = Command::new(exe_path);
    command.spawn().map(|_| ())
}
