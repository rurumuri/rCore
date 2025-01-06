#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    //user_stack_test(0);
    0
}


fn user_stack_test(x: usize) {
    print!("stack test {}\n", x);
    user_stack_test(x+1);
}