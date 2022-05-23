//! App management syscalls
use log::warn;

use crate::batch::run_next_app;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    warn!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}
