use crate::{
    mm::page_table::translated_byte_buffer,
    task::current_user_token,
    timer::{self, get_time_us, MICRO_PER_SEC},
};

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

// pub fn sys_get_time(ts: usize, _tz: usize) -> isize {
//     let usec = timer::get_time_us();
//     let time_val = TimeVal {
//         sec: usec / MICRO_PER_SEC,
//         usec,
//     };
//     unsafe {
//         *(ts as *mut TimeVal) = time_val;
//     }
//     0
// }

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let dst_vec = translated_byte_buffer(
        current_user_token(),
        ts as *const u8,
        core::mem::size_of::<TimeVal>(),
    );
    let ref time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let src_ptr = time_val as *const TimeVal;
    for (idx, dst) in dst_vec.into_iter().enumerate() {
        let unit_len = dst.len();
        unsafe {
            dst.copy_from_slice(core::slice::from_raw_parts(
                src_ptr.wrapping_byte_add(idx * unit_len) as *const u8,
                unit_len,
            ));
        }
    }
    0
}
