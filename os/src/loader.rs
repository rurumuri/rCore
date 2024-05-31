// loader.rs
// - kernel stack's and user stack's struct and implementation
// - [`load_apps`]: load all apps into different address at one time
// - [`get_app_num`]: get the num of apps from _app_num in link.app.S

use core::arch::asm;
use crate::config::*;
use log::trace;
use crate::trap::context::TrapContext;


#[repr(align(4096))]
#[derive(Clone, Copy)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Clone, Copy)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    // pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
    //     let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
    //     unsafe {
    //         *cx_ptr = cx;
    //     }
    //     unsafe { cx_ptr.as_mut().unwrap() }
    // }
    pub fn push_context(&self, trap_cx: TrapContext) -> usize {
        let trap_cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *trap_cx_ptr = trap_cx;
        }
        trap_cx_ptr as usize
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [
    KernelStack{data: [0; KERNEL_STACK_SIZE]}; 
    MAX_APP_NUM
];
static USER_STACK: [UserStack; MAX_APP_NUM] = [
    UserStack{data: [0; USER_STACK_SIZE]}; 
    MAX_APP_NUM
];


pub fn get_app_num() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe {
        (_num_app as usize as *const usize).read_volatile()
    }
}

pub fn get_app_base(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

pub fn load_apps() {
    trace!("[kernel] Loading apps");
    extern "C" {
        fn _num_app();
    }
    let num_app = unsafe{(_num_app as usize as *const usize).read_volatile()};
    trace!("[kernel] num_app: {}", num_app);
    let app_start = unsafe{ core::slice::from_raw_parts((_num_app as *const usize).add(1), num_app + 1)};
    // let mut last_app_address = APP_BASE_ADDRESS;
    // clear apps area
    unsafe {
        core::slice::from_raw_parts_mut(
            APP_BASE_ADDRESS as *mut u8, 
            APP_SIZE_LIMIT * num_app,
        ).fill(0);
    }
    // app_id starts from 0
    for app_id in 0..num_app {
        trace!("[kernel] Loading app_{}", app_id);

        let app_src = unsafe{
            core::slice::from_raw_parts(
                app_start[app_id] as *const u8,
                app_start[app_id + 1] - app_start[app_id]
            )
        };
        let app_dst = unsafe{
            core::slice::from_raw_parts_mut(
                (APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT) as *mut u8,
                app_src.len()
            )
        };
        app_dst.copy_from_slice(app_src);
        // last_app_address += app_src.len();
    }
    // if(app_id >= num_app) {
    //     panic!("[Kernel] All apps completed!");
    // }
    // print!("[kernel] Loading app_{}", app_id);
    // clear app area
    // core::slice::from_raw_parts_mut(
    //     APP_BASE_ADDRESS as *mut u8, 
    //     APP_SIZE_LIMIT,
    // ).fill(0);
    // let app_src = core::slice::from_raw_parts(
    //     self.app_start[app_id] as *const u8,
    //     self.app_start[app_id + 1] - self.app_start[app_id]
    // );
    // let app_dst = core::slice::from_raw_parts_mut(
    //     APP_BASE_ADDRESS as *mut u8,
    //     app_src.len()
    // );
    // app_dst.copy_from_slice(app_src);

    unsafe {
        asm!("fence.i");
    }
}

/// get app info with entry and sp and save `TrapContext` in kernel stack
/// return app's userstack's stack pointer
pub fn init_app_cx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(TrapContext::app_init_context(
        get_app_base(app_id),
        USER_STACK[app_id].get_sp(),
    ))
}