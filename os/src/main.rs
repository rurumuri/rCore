#![no_main]
#![no_std]
#![feature(panic_info_message)]


use core::arch::global_asm;


#[macro_use]
mod console;
mod sbi;
mod lang_items;
mod logger;
use log::*;


global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init(LevelFilter::Trace).expect("Logger initialize failed");

    os_info();
    logger_test();

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

fn os_info() {
    extern "C" {
        fn skernel();
        fn ekernel();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
    }
    info!("kernel\t[{:#x}, {:#x})", skernel as usize, ekernel as usize);
    info!(".text\t[{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata\t[{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data\t[{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss\t[{:#x}, {:#x})", sbss as usize, ebss as usize);
}

fn logger_test() {
    error!("very serious errors");
    warn!("hazardous situations");
    info!("useful information");
    debug!("lower priority information");
    trace!("very low priority, often extremely verbose, information");
}