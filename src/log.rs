//! A set of logging macros that provide different levels of logging: `trace`, `debug`, `info`,
//! `warn`, and `error`. These macros behave differently depending on whether the code is compiled
//! in debug or release mode.
//!
//! In debug mode (`debug_assertions` enabled), the macros use `println!` to output messages to the
//! console. In release mode (`debug_assertions` disabled), the macros delegate to the corresponding
//! logging macros from the `log` crate (`log::trace!`, `log::debug!`, etc.).
//!
//! # Macros
//!
//! - `trace!`: Logs very detailed information, typically used for tracing program execution.
//! - `debug!`: Logs debugging information, useful during development.
//! - `info!`: Logs general informational messages.
//! - `warn!`: Logs warnings about potential issues.
//! - `error!`: Logs error messages indicating serious problems.
//!
//! # Examples
//!
//! ```rust
//!
//! rs_utils::log::trace!("This is a trace message: {}", 2);
//! rs_utils::log::debug!("Debugging value: {:?}", Box::new(42));
//! rs_utils::log::info!("Application started successfully.");
//! rs_utils::log::warn!("This might cause an issue: {}", "low disk space");
//! rs_utils::log::error!("An error occurred: {}", "error_message");
//! ```
//!
//! These macros are designed to make it easy to switch between debug and release logging behavior
//! without changing the code.

#[macro_export]
macro_rules! trace {
    ($($x:expr),*) => {
        #[cfg(debug_assertions)]
        println!($($x),*);
        #[cfg(not(debug_assertions))]
        log::trace!($($x),*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($x:expr),*) => {
        #[cfg(debug_assertions)]
        println!($($x),*);
        #[cfg(not(debug_assertions))]
        log::debug!($($x),*);
    };
}

#[macro_export]
macro_rules! info {
    ($($x:expr),*) => {
        #[cfg(debug_assertions)]
        println!($($x),*);
        #[cfg(not(debug_assertions))]
        log::info!($($x),*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($x:expr),*) => {
        #[cfg(debug_assertions)]
        println!($($x),*);
        #[cfg(not(debug_assertions))]
        log::warn!($($x),*);
    };
}

#[macro_export]
macro_rules! error {
    ($($x:expr),*) => {
        #[cfg(debug_assertions)]
        println!($($x),*);
        #[cfg(not(debug_assertions))]
        log::error!($($x),*);
    };
}
