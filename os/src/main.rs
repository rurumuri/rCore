/*
 * The main procedure of the OS, mainly done tasks layed below:
 *
 * - initialize the boot stack space. (where KernelStack and UserStack will be placed in)
 * - misc: print the system info, initalizing the logger...
 * - init the trap handler and enable the time interrupt
 * - load and run user apps
*/

#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;
extern crate xmas_elf;

use core::arch::global_asm;

#[macro_use]
mod console;
mod lang_items;
mod logger;
mod sbi;
use log::*;
mod sync;
// mod batch;
mod config;
mod loader;
mod mm;
mod syscall;
mod task;
mod timer;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    logger::init(LevelFilter::Trace).expect("Logger initialize failed");

    // kernel_stack_test(0);

    os_info();

    mm::init();
    // mm::heap_allocator::init_heap();
    mm::heap_allocator::heap_test();
    // mm::frame_allocator::init_frame_allocator();
    /*
       In MultiprogOS, we split the batch mod into `task` mod and `loader` mod.
    */
    trap::init();
    // batch::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    // loader::load_apps();
    // batch::run_next_app_without_load();
    task::run_first_task();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
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
    info!(
        "[kernel] kernel\t[{:#x}, {:#x})",
        skernel as usize, ekernel as usize
    );
    info!(
        "[kernel] .text\t[{:#x}, {:#x})",
        stext as usize, etext as usize
    );
    info!(
        "[kernel] .rodata\t[{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data\t[{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    info!(
        "[kernel] .bss\t[{:#x}, {:#x})",
        sbss as usize, ebss as usize
    );
}

// fn kernel_stack_test(x: usize) {
//     info!("[kernel] stack test {}", x);
//     // if x==701 {
//     //     os_info();

//     //     trap::init();
//     //     batch::init();
//     //     loader::load_apps();
//     //     batch::run_next_app_without_load();
//     //     return;
//     // }
//     kernel_stack_test(x + 1);
// }
