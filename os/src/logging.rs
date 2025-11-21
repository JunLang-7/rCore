use core::fmt;
use crate::println;

/// Logging level
pub const ERROR: u8 = 1;
pub const WARN: u8 = 2;
pub const INFO: u8 = 3;
pub const DEBUG: u8 = 4;
pub const TRACE: u8 = 5;

const fn get_log_level() -> u8 {
    match option_env!("LOG") {
        Some(env) => {
            let bytes = env.as_bytes();
            if bytes.is_empty() {
                return INFO;
            }
            match bytes[0] {
                b'E' => ERROR,
                b'W' => WARN,
                b'I' => INFO,
                b'D' => DEBUG,
                b'T' => TRACE,
                _ => INFO,
            }
        },
        None => INFO,
    }
}

const LOG_LEVEL: u8 = get_log_level();

pub fn print_log(level: u8, args: fmt::Arguments) {
    if level <= LOG_LEVEL {
        let (level_str, color_code) = match level {
            ERROR => ("ERROR", 31),
            WARN => ("WARN", 93),
            INFO => ("INFO", 34),
            DEBUG => ("DEBUG", 32),
            TRACE => ("TRACE", 90),
            _ => ("LOG", 0),
        };
        println!("\x1b[{}m[{}]{}\x1b[0m", color_code, level_str, args);
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::print_log($crate::logging::ERROR, format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::print_log($crate::logging::WARN, format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::print_log($crate::logging::INFO, format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::print_log($crate::logging::DEBUG, format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::print_log($crate::logging::TRACE, format_args!($fmt $(, $($arg)+)?));
    }
}
