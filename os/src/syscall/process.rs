use crate::batch::run_next_app_without_load;
use log::info;

pub fn sys_exit(xstate: i32) -> ! {
    info!("[kernel] Application exited with code {}", xstate);
    run_next_app_without_load()
}

pub fn sys_yield() -> isize {
    1
}