use core::fmt;

/// logger日志
use log::{Level, LevelFilter};

use crate::console::print;

struct SimpleLogger;

/// 日志颜色
enum LogColor {
    ERROR = 31,
    WARN = 93,
    INFO = 34,
    DEBUG = 32,
    TRACE = 90,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        // Trace等级最高，所以所有log信息都显示
        metadata.level() <= LevelFilter::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();

            let color = match level {
                Level::Error => LogColor::ERROR as u8,
                Level::Warn => LogColor::WARN as u8,
                Level::Info => LogColor::INFO as u8,
                Level::Debug => LogColor::DEBUG as u8,
                Level::Trace => LogColor::TRACE as u8,
            };
            print_with_color(*record.args(), color);
        }
    }

    fn flush(&self) {}
}

fn print_with_color(args: fmt::Arguments, color: u8) {
    print(format_args!("\x1b[{}m{}\x1b[0m\n", color, args));
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}
