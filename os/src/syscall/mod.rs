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

mod fs;
mod process;
mod time;

use fs::*;
use process::*;
use time::*;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0], args[1]),
        SYSCALL_GET_TASKID => sys_get_taskid(),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
