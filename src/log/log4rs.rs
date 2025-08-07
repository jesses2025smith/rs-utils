use chrono::Local;
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

/// Configuration builder for initializing log4rs-based logging.
///
/// `Log4rsConfig` allows you to flexibly configure logging output for your application,
/// including log levels, output patterns, and destinations (console and file).
///
/// # Features
///
/// - Set different log levels for root, console, and file outputs.
/// - Customize log output patterns using log4rs pattern syntax.
/// - Specify log file name and directory; log files are timestamped.
/// - Ensures log file directory exists before writing.
///
/// # Example
///
/// ```rust
/// use log::LevelFilter;
/// use rsutil::log::Log4rsConfig;
///
/// Log4rsConfig::default()
///     .set_root_level(LevelFilter::Info)
///     .set_console_level(LevelFilter::Warn)
///     .set_file_level(LevelFilter::Trace)
///     .set_filename("myapp")
///     .set_filepath("logs")
///     .set_pattern("{d} [{l}] {t}[{L}]: {m}{n}")
///     .initialize()
///     .expect("Failed to initialize logger");
/// ```
///
/// # Errors
///
/// Returns an error if the log file directory cannot be created or if log4rs fails to initialize.
///
/// # Pattern Syntax
///
/// See [log4rs pattern documentation](https://docs.rs/log4rs/latest/log4rs/encode/pattern/index.html)
/// for available pattern variables.
///
/// # Fields
///
/// - `root_level`: The global log level filter.
/// - `console_level`: Log level for console output.
/// - `filename`: Name of the log file (without extension or timestamp).
/// - `filepath`: Directory where log files are stored.
/// - `file_level`: Log level for file output.
/// - `pattern`: Log output format pattern.
///
/// # See Also
///
/// - [`logos`](https://docs.rs/log4rs)
/// - [`log`](https://docs.rs/log)
#[derive(Default, Debug)]
pub struct Log4rsConfig<'a> {
    root_level: Option<LevelFilter>,
    console_level: Option<LevelFilter>,
    filename: Option<&'a str>,
    filepath: Option<&'a str>,
    file_level: Option<LevelFilter>,
    pattern: Option<&'a str>,
}

#[allow(dead_code)]
impl<'a> Log4rsConfig<'a> {
    #[inline]
    pub fn set_root_level(&mut self, level: LevelFilter) -> &mut Self {
        self.root_level = Some(level);
        self
    }
    #[inline]
    pub fn set_console_level(&mut self, filter: LevelFilter) -> &mut Self {
        self.console_level = Some(filter);
        self
    }
    #[inline]
    pub fn set_file_level(&mut self, filter: LevelFilter) -> &mut Self {
        self.file_level = Some(filter);
        self
    }
    #[inline]
    pub fn set_filename(&mut self, filename: &'a str) -> &mut Self {
        self.filename = Some(filename);
        self
    }
    #[inline]
    pub fn set_filepath(&mut self, filepath: &'a str) -> &mut Self {
        self.filepath = Some(filepath);
        self
    }
    #[inline]
    pub fn set_pattern(&mut self, pattern: &'a str) -> &mut Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Log Trace level output to file where trace is the default level
        // and the programmatically specified level to stderr.
        let mut builder = log4rs::config::Config::builder();
        let pattern = self.pattern.unwrap_or("{d} [{l}] {t}[{L}]: {m}{n}");
        let mut root = Root::builder();

        // Build a stderr logger.
        let console = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .target(Target::Stderr)
            .build();
        builder = builder.appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(
                    self.console_level.unwrap_or(LevelFilter::Debug),
                )))
                .build("console", Box::new(console)),
        );
        root = root.appender("console");

        if let Some(filename) = self.filename {
            let filepath = self.filepath.unwrap_or("logs");
            std::fs::create_dir_all(filepath)?;
            let file = FileAppender::builder()
                // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
                .encoder(Box::new(PatternEncoder::new(pattern)))
                .append(false)
                .build(format!(
                    "{}/{}-{}.log",
                    filepath,
                    filename,
                    Local::now().format("%Y-%m-%d %H_%M_%S")
                ))?;
            builder = builder.appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(
                        self.file_level.unwrap_or(LevelFilter::Debug),
                    )))
                    .build("file", Box::new(file)),
            );
            root = root.appender("file");
        }
        let config = builder.build(root.build(self.root_level.unwrap_or(LevelFilter::Trace)))?;

        // Use this to change log levels at runtime.
        // This means you can change the default log level to trace
        // if you are trying to debug an issue and need more logs on then turn it off
        // once you are done.
        let _handle = log4rs::init_config(config)?;

        Ok(())
    }
}
