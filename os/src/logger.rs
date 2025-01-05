/*!
 * 简单的logger，支持log等级控制和彩色输出
 * 实现参考：https://docs.rs/log/0.4.14/log/#implementing-a-logger
*/

use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

use crate::println;

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                Level::Error => 31,
                Level::Warn => 93,
                Level::Info => 34,
                Level::Debug => 32,
                Level::Trace => 90,
            };

            println!(
                "\x1b[{}m[{}]\t{}\x1b[0m",
                color,
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: MyLogger = MyLogger;

pub fn init(log_level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level))
}
