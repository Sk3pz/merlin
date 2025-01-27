use std::{fmt, time::Duration};

use better_term::{flush_styles, Color};

use crate::DEBUG_OUTPUT;

pub(crate) fn timed<F: FnOnce() -> R, R>(f: F) -> (R, Duration) {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    (result, elapsed)
}

pub(crate) fn _debug(args: fmt::Arguments) {
    if !DEBUG_OUTPUT {
        return;
    }
    println!(
        "{}# DBG > {}{}",
        Color::BrightBlack,
        Color::White,
        args,
    );
    flush_styles();
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logging::_debug(format_args!($($arg)*));
    };
}

pub(crate) fn _info(args: fmt::Arguments) {
    println!(
        "{}: INF > {}{}",
        Color::Cyan,
        Color::BrightWhite,
        args,
    );
    flush_styles();
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logging::_info(format_args!($($arg)*));
    };
}

pub(crate) fn _warn(args: fmt::Arguments) {
    println!(
        "{}! WRN > {}{}",
        Color::Yellow,
        Color::BrightYellow,
        args,
    );
    flush_styles();
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logging::_warn(format_args!($($arg)*));
    };
}

pub(crate) fn _error(args: fmt::Arguments) {
    println!(
        "{}X ERR > {}{}",
        Color::Red,
        Color::BrightRed,
        args,
    );
    flush_styles();
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logging::_error(format_args!($($arg)*));
    };
}