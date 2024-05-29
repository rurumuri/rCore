use core::arch::asm;
use crate::config::*;
use log::trace;

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