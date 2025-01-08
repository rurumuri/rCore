/*!
 * syscall implements
 * will be invoked from user apps.
 *
 * according to Linux 6.13.0-rc1 kernel source
 * https://gpages.juszkiewicz.com.pl/syscalls-table/syscalls.html
 */

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GET_TASKID: usize = 172; // 172 should be SYSCALL_GETPID
const SYSCALL_SBRK: usize = 214;

mod fs;
mod process;
mod time;

use fs::*;
use process::*;
use time::*;

use crate::task::get_cur_task_id;
use log::trace;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    #[cfg(feature = "trace_syscalls")]
    {
        match syscall_id {
            SYSCALL_EXIT => {
                trace!(
                    "[kernel] [{}] Application syscall trace: {}",
                    get_cur_task_id(),
                    syscall_id
                );
            }
            _ => {}
        }
    }
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_GET_TASKID => sys_get_taskid(),
        SYSCALL_SBRK => sys_sbrk(args[0] as i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
