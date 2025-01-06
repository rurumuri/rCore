#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::get_taskid;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world from task {}", get_taskid());
    0
}
