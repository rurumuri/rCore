/*!
 * trap hanler.
*/

pub(crate) mod context;

use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
// use crate::batch::run_next_app_without_load;
use crate::syscall::syscall;
use crate::task::{
    current_trap_cx, current_user_token, exit_current_and_run_next, get_cur_task_id,
    suspend_current_and_run_next,
};
use crate::timer::set_next_trigger;
use crate::trap::context::TrapContext;
use log::{error, trace, warn};

use core::arch::asm;
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

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
    panic!("Unreachable in back_to_user!");
}

///
/// Will be called by `__alltraps` with the unmodified TrapContext at `a0(x10)` as argument.
/// Handle the trap according to trap's exception type.
/// After that, `__restore` in `trap.S` will be executed continually (if no panic), then return to U mode by `sret`.
///
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    set_kernel_trap_entry();
    let cx = current_trap_cx();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        // syscall
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!(
                "[kernel] [{}] PageFault in application, kernel killed it.",
                get_cur_task_id()
            );
            // run_next_app();
            // run_next_app_without_load();
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!(
                "[kernel] [{}] IllegalInstruction in application, kernel killed it.",
                get_cur_task_id()
            );
            // run_next_app();
            // run_next_app_without_load();
            exit_current_and_run_next();
        }
        // currently we don't support S mode timer interrupt
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
    // cx
    trap_return();
}
