#![no_main] // no main()
#![no_std] // use rust core library rather than std (which need OS environment)
#![feature(panic_info_message)]

// at the module's root file (main.rs), we need to list all submod we will use
#[macro_use] // use macros from marco. Just available for the next line!
mod console;
mod sbi;
mod lang_items;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm")); // import entry.asm as string, then insert it


#[no_mangle] // tell the compiler not to change the function's name
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello rCore");
    panic!("Bye");
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