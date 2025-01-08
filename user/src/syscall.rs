use core::arch::asm;
use crate::TimeVal;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GET_TASKID: usize = 172; // 172 should be SYSCALL_GETPID
const SYSCALL_SBRK: usize = 214;

fn syscall(id: usize, arg: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg[0] => ret,
            in("x11") arg[1],
            in("x12") arg[2],
            in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time(ts: &mut TimeVal, _tz: usize) -> isize {
    syscall(SYSCALL_GET_TIME, [ts as *mut _ as usize, _tz, 0])
}

pub fn sys_get_taskid() -> isize {
    syscall(SYSCALL_GET_TASKID, [0, 0, 0])
}

pub fn sys_sbrk(size: i32) -> isize {
    syscall(SYSCALL_SBRK, [size as usize, 0, 0])
}
