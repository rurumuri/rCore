#![no_std]
#![no_main]

use user_lib::{get_time, println};

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let usec = get_time();
    println!(
        "uptime: {}s {}ms {}us",
        usec / 1_000_000,
        (usec / 1000) % 1000,
        (usec % 1_000_000) % 1000
    );
    0
    // println!("sys_get_time unavaliable now");
    // -1
}
