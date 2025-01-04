use crate::timer::{self, MICRO_PER_SEC};

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_get_time(ts: usize, _tz: usize) -> isize {
    let usec = timer::get_time_us();
    let time_val = TimeVal {
        sec: usec / MICRO_PER_SEC,
        usec,
    };
    unsafe {
        *(ts as *mut TimeVal) = time_val;
    }
    0
}
