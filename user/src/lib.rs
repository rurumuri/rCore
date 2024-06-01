#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

pub mod console;
mod lang_items;
mod syscall;
use syscall::*;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}


#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    sys_write(fd, buffer)
}

pub fn exit(xstate: i32) -> isize {
    sys_exit(xstate)
}

pub fn yield_() -> isize {
    sys_yield()
}

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

impl TimeVal {
    pub fn new() -> Self {
        TimeVal { sec: 0, usec: 0 }
    }
}

impl Default for TimeVal {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_time_us() -> usize {
    let mut ts = TimeVal { usec: 0, sec: 0 };
    sys_get_time(&mut ts, 0);
    ts.usec
}


pub fn get_time() -> usize {
    get_time_us() / 1000
}