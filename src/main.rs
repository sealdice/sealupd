use consts::CLI_ARGS;
use log::Logger;

mod cli;
mod consts;
mod decompress;
mod log;
mod proc;

fn main() {
    let exit_code = run();
    if exit_code != 0 && cfg!(windows) {
        use std::io::{self, Read};
        println!("\nPress ENTER to continue ...");
        _ = io::stdin().read_exact(&mut [0u8]);
    }
    std::process::exit(exit_code);
}

fn run() -> i32 {
    let logger = Logger::new();
    init_logger(&logger);
    logger.console_success("终端日志开始记录.");

    if CLI_ARGS.pid != 0 {
        logger.batch_info(format_args!("等待进程 {} 退出.", CLI_ARGS.pid));
        if !proc::wait_process(CLI_ARGS.pid, 30, &logger) {
            logger.batch_error("经等待进程仍未退出, 为避免错误, 中止操作.");
            return 1;
        }
    }

    match decompress::backup_sealdice() {
        Ok(exists) => {
            if exists {
                logger.batch_success("已经备份可执行文件.");
            } else {
                logger.batch_info("可执行文件不存在, 跳过备份.");
            }
        }
        Err(err) => {
            logger.batch_error(format_args!("备份可执行文件失败, 为安全考虑, 中止操作: {}", err));
            return 1;
        }
    }

    logger.batch_info(format_args!("尝试解压 '{}'.", CLI_ARGS.package));
    match decompress::decompress(&logger) {
        Ok(entry_count) => logger.batch_success(format_args!("解压成功, 共计 {} 条目", entry_count)),
        Err(err) => {
            logger.batch_error(format_args!("解压失败: {}", err));
            return 1;
        }
    }

    if let Err(err) = proc::restart_sealdice(&logger) {
        logger.batch_error(format_args!("重启主程序出错: {}", err));
        return 1;
    }

    0
}

fn init_logger(logger: &Logger) {
    if CLI_ARGS.quiet {
        logger.console_verbose("日志文件已关闭.");
        return;
    }

    match log::init_logger(CLI_ARGS.quiet) {
        Ok(file_name) => {
            logger.console_verbose(format_args!("已创建日志文件 '{}'.", file_name));
            logger.file_info("文件日志开始记录.");
        }
        Err(err) => logger.console_warn(format_args!("无法创建文件日志: {}.", err)),
    }
}
