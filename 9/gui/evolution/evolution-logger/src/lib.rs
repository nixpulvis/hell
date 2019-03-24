extern crate log;

use std::env;
use std::thread;

use log::{
    LogRecord,
    LogMetadata,
    LogLevelFilter,
    SetLoggerError
};
use log::LogLevel::*;

/// A logger for our game.
///
/// This logger produces two forms of log messages. The normal format is used
/// for the `error`, `warn`, and `info` log levels. The module format is used
/// for the `debug` and `trace` messages.
///
/// # Normal Format
///
/// ```sh
/// < THREAD > LEVEL : MESSAGE*
///
/// # Example
/// <main> Dealing.
/// ```
///
/// # Level Format
///
/// ```sh
/// THREAD LEVEL | MESSAGE*
///
/// # Example
/// <main> ERROR | Dealing.
/// ```
///
/// # Level / Module Format
///
/// ```sh
/// THREAD LEVEL MODULE : MESSAGE*
///
/// # Example
/// <main> DEBUG evolution::game | Silly::choose({:?}) -> {:?}
/// ```
pub struct Logger;

impl Logger {
    /// Spin up a new `Logger` which will log messages to STDOUT.
    ///
    /// The max log level is configured through the environment variable
    /// `LOG`, and can be one of 5 values. The following configurations
    /// appean in order from least to greatest.
    ///
    /// ```sh
    /// LOG=error
    /// LOG=warn
    /// LOG=info
    /// LOG=debug
    /// LOG=trace
    /// ```
    pub fn init() -> Result<(), SetLoggerError> {
        let filter = match env::var("LOG") {
            Ok(ref s) if s == "error" => LogLevelFilter::Error,
            Ok(ref s) if s == "warn" => LogLevelFilter::Warn,
            Ok(ref s) if s == "info" => LogLevelFilter::Info,
            Ok(ref s) if s == "debug" => LogLevelFilter::Debug,
            Ok(ref s) if s == "trace" => LogLevelFilter::Trace,
            _ => LogLevelFilter::Off,
        };
        log::set_logger(|max_log_level| {
            max_log_level.set(filter);
            Box::new(Logger)
        })
    }

    fn normal_format(&self, record: &LogRecord) {
        println!("{} | {}",
                 self.thread_name(),
                 record.args());
    }

    fn level_format(&self, record: &LogRecord) {
        println!("{} {} | {}",
                 self.thread_name(),
                 record.level(),
                 record.args());
    }

    fn level_module_format(&self, record: &LogRecord) {
        println!("{} {} {} | {}",
                 self.thread_name(),
                 record.level(),
                 record.location().module_path(),
                 record.args());
    }

    fn thread_name(&self) -> String {
        thread::current().name().unwrap_or("<?>").into()
    }
}

impl log::Log for Logger {
    fn enabled(&self, _: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Error => self.level_format(record),
                Warn => self.level_format(record),
                Info => self.normal_format(record),
                Debug => self.level_module_format(record),
                Trace => self.level_module_format(record),
            }
        }
    }
}
