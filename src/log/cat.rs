/// Logger object
///
/// # Examples
///
/// ```rust
/// use rsutil::log::LogCat;
/// let logger = LogCat::new("APP");
///
/// logger.trace(format_args!("This is a trace message: {}", 2));
/// logger.trace(format_args!("This is a trace", ));
/// logger.debug(format_args!("Debugging value: {:?}", Box::new(42)));
/// logger.info(format_args!("Application started successfully."));
/// logger.warn(format_args!("This might cause an issue: {}", "low disk space"));
/// logger.error(format_args!("An error occurred: {}", "error message"));
/// ```
#[derive(Debug, Copy, Clone, Default)]
pub struct LogCat {
    tag: &'static str,
}

unsafe impl Send for LogCat {}
unsafe impl Sync for LogCat {}

impl LogCat {
    pub fn new(tag: &'static str) -> Self {
        Self { tag }
    }
}

#[cfg(all(debug_assertions, feature = "log-lineno"))]
impl LogCat {
    #[track_caller]
    #[inline(always)]
    pub fn trace(&self, args: std::fmt::Arguments) {
        let loc = std::panic::Location::caller();
        println!(
            "\x1b[95m[ TRACE] - {} - {} ({}:{})\x1b[0m",
            self.tag,
            args,
            loc.file(),
            loc.line()
        );
    }

    #[track_caller]
    #[inline(always)]
    pub fn debug(&self, args: std::fmt::Arguments) {
        let loc = std::panic::Location::caller();
        println!(
            "\x1b[96m[ DEBUG] - {} - {} ({}:{})\x1b[0m",
            self.tag,
            args,
            loc.file(),
            loc.line()
        );
    }

    #[track_caller]
    #[inline(always)]
    pub fn info(&self, args: std::fmt::Arguments) {
        let loc = std::panic::Location::caller();
        println!(
            "\x1b[32m[  INFO] - {} - {} ({}:{})\x1b[0m",
            self.tag,
            args,
            loc.file(),
            loc.line()
        );
    }

    #[track_caller]
    #[inline(always)]
    pub fn warn(&self, args: std::fmt::Arguments) {
        let loc = std::panic::Location::caller();
        println!(
            "\x1b[33m[  WARN] - {} - {} ({}:{})\x1b[0m",
            self.tag,
            args,
            loc.file(),
            loc.line()
        );
    }

    #[track_caller]
    #[inline(always)]
    pub fn error(&self, args: std::fmt::Arguments) {
        let loc = std::panic::Location::caller();
        println!(
            "\x1b[31m[ ERROR] - {} - {} ({}:{})\x1b[0m",
            self.tag,
            args,
            loc.file(),
            loc.line()
        );
    }
}

#[cfg(all(debug_assertions, not(feature = "log-lineno")))]
impl LogCat {
    #[inline(always)]
    pub fn trace(&self, args: std::fmt::Arguments) {
        println!(
            "\x1b[95m[ TRACE] - {} - {}\x1b[0m",
            self.tag,
            args,
        );
    }

    #[inline(always)]
    pub fn debug(&self, args: std::fmt::Arguments) {
        println!(
            "\x1b[96m[ DEBUG] - {} - {}\x1b[0m",
            self.tag,
            args,
        );
    }

    #[inline(always)]
    pub fn info(&self, args: std::fmt::Arguments) {
        println!(
            "\x1b[32m[  INFO] - {} - {}\x1b[0m",
            self.tag,
            args,
        );
    }

    #[inline(always)]
    pub fn warn(&self, args: std::fmt::Arguments) {
        println!(
            "\x1b[33m[  WARN] - {} - {}\x1b[0m",
            self.tag,
            args,
        );
    }

    #[inline(always)]
    pub fn error(&self, args: std::fmt::Arguments) {
        println!(
            "\x1b[31m[ ERROR] - {} - {}\x1b[0m",
            self.tag,
            args,
        );
    }
}

#[cfg(not(debug_assertions))]
impl LogCat {
    #[inline(always)]
    pub fn trace(&self, args: std::fmt::Arguments) {
        log::trace!("{} - {}", self.tag, args);
    }

    #[inline(always)]
    pub fn debug(&self, args: std::fmt::Arguments) {
        log::debug!("{} - {}", self.tag, args);
    }

    #[inline(always)]
    pub fn info(&self, args: std::fmt::Arguments) {
        log::info!("{} - {}", self.tag, args);
    }

    #[inline(always)]
    pub fn warn(&self, args: std::fmt::Arguments) {
        log::warn!("{} - {}", self.tag, args);
    }

    #[inline(always)]
    pub fn error(&self, args: std::fmt::Arguments) {
        log::error!("{} - {}", self.tag, args);
    }
}
