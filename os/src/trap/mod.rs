/*!
 * trap hanler.
*/

pub(crate) mod context;

// use crate::batch::run_next_app_without_load;
use crate::syscall::syscall;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::set_next_trigger;
use crate::trap::context::TrapContext;
use log::{error, trace, warn};

use core::{arch::global_asm, f32::consts::E};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

///
/// Initialize the trap handler, by setting stvec register's address as `__alltraps` defined in `trap.S`
///
/// Then, when executing `ecall` from user mode, `__alltraps` will be executed, switching sp from user stack to kernel stack,
/// and save all user context as a TrapContext before registers being modified,
/// then turn to `trap_handler` with the unmodified TrapContext at a0(x10) as argument.
///
/// We just need to set an initial value for `stvec` oncely.
///
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

/// timer interrupt enabled
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

///
/// Will be called by `__alltraps` with the unmodified TrapContext at `a0(x10)` as argument.
/// Handle the trap according to trap's exception type.
/// After that, `__restore` in `trap.S` will be executed continually, then return to U mode by `sret`.
///
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        // syscall
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] PageFault in application, kernel killed it.");
            // run_next_app();
            // run_next_app_without_load();
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, kernel killed it.");
            // run_next_app();
            // run_next_app_without_load();
            exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // trace!("[kernel] Timer Interrupt.");
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}
