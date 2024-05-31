// use crate::batch::run_next_app_without_load;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use log::info;

pub fn sys_exit(xstate: i32) -> ! {
    info!("[kernel] Application exited with code {}", xstate);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}