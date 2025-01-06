/*!
 * Old loader implement for batchOS, apps will be loaded to `APP_BASE_ADDRESS` and will be executed ONE BY ONE.
 * Only one app will be in memory at one time. After all apps have been done, kernel will panic.
 * 
 * Defined `KernelStack` and `UserStack`, which will be placed in the kernel's bss segment (We reserved 4096 * 16 Byte for that in `entry.asm`).
 * KernelStack is for reserving the trap info, and UserStack is for ALL user apps' function invoking.
 * 
 * So far we haven‘t implemented dynamic memory allocation (such as the mmap syscall for user apps), so there isn't a "UserHeap" structure now.
*/

// use core::arch::asm;
// use crate::sync::UPSafeCell;
// use lazy_static::*;
// use crate::trap::context::TrapContext;
// use crate::config::*;
// use log::info;

/*
    Consts has been moved to `config.rs`
*/
// // const USER_STACK_SIZE: usize = 4096 * 2;
// // const KERNEL_STACK_SIZE: usize = 4096 * 2;

// // const MAX_APP_NUM: usize = 16;
// // const APP_BASE_ADDRESS: usize = 0x80400000;
// // const APP_SIZE_LIMIT: usize = 0x20000;

// #[repr(align(4096))]
// struct KernelStack {
//     data: [u8; KERNEL_STACK_SIZE],
// }

// #[repr(align(4096))]
// struct UserStack {
//     data: [u8; USER_STACK_SIZE],
// }

// impl KernelStack {
//     fn get_sp(&self) -> usize {
//         self.data.as_ptr() as usize + KERNEL_STACK_SIZE
//     }
//     pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
//         let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
//         unsafe {
//             *cx_ptr = cx;
//         }
//         unsafe { cx_ptr.as_mut().unwrap() }
//     }
// }

// impl UserStack {
//     fn get_sp(&self) -> usize {
//         self.data.as_ptr() as usize + USER_STACK_SIZE
//     }
// }


// static KERNEL_STACK: KernelStack = KernelStack{data: [0; KERNEL_STACK_SIZE]};
// static USER_STACK: UserStack = UserStack{data: [0; USER_STACK_SIZE]};

// struct AppManager {
//     num_app: usize,
//     cur_app: usize,
//     app_start: [usize; MAX_APP_NUM + 1],
// }

// impl AppManager {
//     pub fn print_app_info(&self) {
//         info!("[kernel] num_app = {}", self.num_app);
//         for i in 0..self.num_app {
//             info!(
//                 "[kernel] app_{} [{:#x}, {:#x})",
//                 i,
//                 self.app_start[i],
//                 self.app_start[i + 1]
//             );
//         }
//     }
//     pub fn get_current_app(&self) -> usize {
//         self.cur_app
//     }
//     pub fn move_to_next_app(&mut self) {
//         self.cur_app += 1;
//     }
//     unsafe fn load_app(&self, app_id: usize) {
//         // app_id starts from 0
//         if(app_id >= self.num_app) {
//             panic!("[Kernel] All apps completed!");
//         }
//         info!("[kernel] Loading app_{}", app_id);
//         // clear app area
//         core::slice::from_raw_parts_mut(
//             APP_BASE_ADDRESS as *mut u8, 
//             APP_SIZE_LIMIT,
//         ).fill(0);
//         let app_src = core::slice::from_raw_parts(
//             self.app_start[app_id] as *const u8,
//             self.app_start[app_id + 1] - self.app_start[app_id]
//         );
//         let app_dst = core::slice::from_raw_parts_mut(
//             APP_BASE_ADDRESS as *mut u8,
//             app_src.len()
//         );
//         app_dst.copy_from_slice(app_src);
//         asm!("fence.i");
//     }
// }

// lazy_static! {
//     static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
//         extern "C" {
//             fn _num_app();
//         }
//         let app_num_ptr = _num_app as usize as *const usize;
//         let app_num = app_num_ptr.read_volatile();

//         let app_start_ptr = app_num_ptr.add(1);
//         let app_start_raw = core::slice::from_raw_parts(app_start_ptr, app_num + 1);
//         let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
//         app_start[..=app_num].copy_from_slice(app_start_raw);

//         UPSafeCell::new(
//             AppManager {
//                 num_app: app_num,
//                 cur_app: 0,
//                 app_start: app_start,
//             }
//         )
//     };
// }

// pub fn init(){
//     print_app_info();
// }

// pub fn print_app_info() {
//     APP_MANAGER.exclusive_access().print_app_info();
// }

// pub fn run_next_app() -> ! {
//     let mut app_manager = APP_MANAGER.exclusive_access();
//     let current_app = app_manager.get_current_app();
//     unsafe{ app_manager.load_app(current_app); }
//     app_manager.move_to_next_app();
//     drop(app_manager);

//     extern "C" {fn __restore(cx_addr: usize);}
//     unsafe {
//         __restore(KERNEL_STACK.push_context(
//             TrapContext::app_init_context(APP_BASE_ADDRESS, USER_STACK.get_sp())
//         ) as *const _ as usize);
//     }
//     panic!("Unreachable in batch::run_current_app!");
// }

// pub fn run_next_app_without_load() -> ! {
//     let mut app_manager = APP_MANAGER.exclusive_access();
//     let current_app = app_manager.get_current_app();

//     extern "C" {
//         fn _num_app();
//     }
//     let app_id = unsafe{(_num_app as usize as *const usize).read_volatile()};
//     if(current_app >= app_id) {
//         panic!("[Kernel] All apps completed!");
//     }
//     info!("[kernel] Running app_{}", current_app);

//     // unsafe{ app_manager.load_app(current_app); }
//     app_manager.move_to_next_app();
//     drop(app_manager);

//     extern "C" {fn __restore(cx_addr: usize);}
//     unsafe {
//         __restore(KERNEL_STACK.push_context(
//             TrapContext::app_init_context(APP_BASE_ADDRESS + current_app * APP_SIZE_LIMIT, USER_STACK.get_sp())
//         ) as *const _ as usize);
//     }
//     panic!("Unreachable in batch::run_current_app!");
// }