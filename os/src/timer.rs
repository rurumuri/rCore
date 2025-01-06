/*!
 * timer
 * read time or set next trigger, enabling timer interrupt
*/

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100; // 100 times timer interrupt per second
pub const MICRO_PER_SEC: usize = 1_000_000;

/// get mtime's value
pub fn get_time() -> usize {
    time::read()
}

/// get mtime's value in microsecond
pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

/// set timer interrupt's interval
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC); // 10ms
}
