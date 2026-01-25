use crate::error::trace_log_error;
use crate::error::UdmError;
use crate::UdmResult;
use clap_verbosity_flag::Verbosity;
use std::env::var;
use std::fs::File;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
fn convert_log_level_to_tracing_level(level: log::Level) -> tracing::Level {
    match level {
        log::Level::Error => Level::ERROR,
        log::Level::Warn => Level::WARN,
        log::Level::Info => Level::INFO,
        log::Level::Debug => Level::DEBUG,
        log::Level::Trace => Level::TRACE,
    }
}

/// Descibes the different situations we need to account for
/// Daemon mode requires logging to file and structured format
/// ** But also requires pretty format for debug sessions on cli
/// Cli mode should only log to stderr
/// ** But log when user asks for it
pub enum UdmLoggerType {
    Main,
    DrinkContrl,
    Daemon,
    Bin,
}

pub struct UdmLogger;

impl UdmLogger {
    // writes to a file
    pub fn init(
        logger_type: UdmLoggerType,
        verbose: Verbosity,
        log_file_path: Option<&str>,
    ) -> UdmResult<()> {
        let mut layers = Vec::new();
        let mut log_level = LevelFilter::from_level(convert_log_level_to_tracing_level(
            verbose.log_level().unwrap(),
        ));
        // type of mode we are in will configure the layer
        match logger_type {
            UdmLoggerType::Daemon => {
                // Log always to file
                // optionally get "nice logs" when debug

                // Create log to file layer
                if log_file_path.is_none() {
                    return Err(trace_log_error(UdmError::LoggerError(
                        "Daemon requires a log path".to_string(),
                    )));
                }
                let file_path = log_file_path.unwrap_or("/var/log/udm/udm_daemon.log");
                let file = File::create(file_path).map_err(|e| {
                    trace_log_error(UdmError::LoggerError(format!("Path: {file_path}, {e}")))
                })?;
                layers.push(
                    tracing_subscriber::fmt::layer()
                        .with_line_number(true)
                        .with_target(true)
                        .with_thread_names(true)
                        .with_ansi(false)
                        .with_thread_ids(true)
                        .with_writer(file)
                        .with_filter(LevelFilter::DEBUG)
                        .boxed(),
                );
                // Create "pretty" Layer for stdout
                // If RUST_LOG or DEBUG
                if var("RUST_LOG").is_ok() || var("DEBUG").is_ok() {
                    log_level = LevelFilter::DEBUG;
                }
                layers.push(
                    tracing_subscriber::fmt::layer()
                        .pretty()
                        .with_file(false)
                        .with_target(false)
                        .with_thread_names(true)
                        .with_thread_ids(true)
                        .with_filter(log_level)
                        .boxed(),
                );

                tracing_subscriber::registry().with(layers).init();
                Ok(())
            }
            UdmLoggerType::Main => {
                // Log always to file
                // Create log to file layer
                if log_file_path.is_none() {
                    return Err(trace_log_error(UdmError::LoggerError(
                        "Daemon requires a log path".to_string(),
                    )));
                }
                let file = File::create(log_file_path.unwrap_or("/var/log/udm/udm_daemon.log"))
                    .map_err(|e| trace_log_error(UdmError::LoggerError(e.to_string())))?;
                layers.push(
                    tracing_subscriber::fmt::layer()
                        .with_line_number(true)
                        .with_target(true)
                        .with_thread_names(true)
                        .with_ansi(false)
                        .with_thread_ids(true)
                        .with_writer(file)
                        .with_filter(LevelFilter::DEBUG)
                        .boxed(),
                );

                // Create "pretty" Layer for stdout
                // If RUST_LOG or DEBUG
                if var("RUST_LOG").is_ok() || var("DEBUG").is_ok() {
                    log_level = LevelFilter::DEBUG;
                }
                layers.push(
                    tracing_subscriber::fmt::layer()
                        .pretty()
                        .with_file(false)
                        .with_target(false)
                        .with_thread_names(true)
                        .with_thread_ids(true)
                        .with_filter(log_level)
                        .boxed(),
                );

                tracing_subscriber::registry().with(layers).init();
                Ok(())
            }
            UdmLoggerType::Bin => {
                let cli_layer = tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_file(false)
                    .with_filter(log_level);
                tracing_subscriber::registry().with(cli_layer).init();
                Ok(())
            }
            UdmLoggerType::DrinkContrl => {
                // Log always to file
                // optionally get "nice logs" when debug

                // Create log to file layer
                if log_file_path.is_none() {
                    return Err(trace_log_error(UdmError::LoggerError(
                        "Daemon requires a log path".to_string(),
                    )));
                }
                let file = File::create(log_file_path.unwrap_or("/var/log/udm/udm_daemon.log"))
                    .map_err(|e| trace_log_error(UdmError::LoggerError(e.to_string())))?;
                layers.push(
                    tracing_subscriber::fmt::layer()
                        .with_line_number(true)
                        .with_target(true)
                        .with_thread_names(true)
                        .with_ansi(false)
                        .with_thread_ids(true)
                        .with_writer(file)
                        .with_filter(LevelFilter::DEBUG)
                        .boxed(),
                );
                // Create "pretty" Layer for stdout
                // If RUST_LOG or DEBUG
                if var("RUST_LOG").is_ok() || var("DEBUG").is_ok() {
                    log_level = LevelFilter::DEBUG;
                }
                layers.push(
                    tracing_subscriber::fmt::layer()
                        .pretty()
                        .with_file(false)
                        .with_target(false)
                        .with_thread_names(true)
                        .with_thread_ids(true)
                        .with_filter(log_level)
                        .boxed(),
                );

                tracing_subscriber::registry().with(layers).init();
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap_verbosity_flag::Verbosity;
    use std::fs;
    use tempfile::TempDir;
    use tracing::Level;

    #[test]
    fn test_convert_log_level_to_tracing_level() {
        assert_eq!(
            convert_log_level_to_tracing_level(log::Level::Error),
            Level::ERROR
        );
        assert_eq!(
            convert_log_level_to_tracing_level(log::Level::Warn),
            Level::WARN
        );
        assert_eq!(
            convert_log_level_to_tracing_level(log::Level::Info),
            Level::INFO
        );
        assert_eq!(
            convert_log_level_to_tracing_level(log::Level::Debug),
            Level::DEBUG
        );
        assert_eq!(
            convert_log_level_to_tracing_level(log::Level::Trace),
            Level::TRACE
        );
    }

    #[test]
    fn test_daemon_logger_requires_log_path() {
        let verbose = Verbosity::default();
        let result = UdmLogger::init(UdmLoggerType::Daemon, verbose, None);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Daemon requires a log path"));
        }
    }

    #[test]
    fn test_main_logger_requires_log_path() {
        let verbose = Verbosity::default();
        let result = UdmLogger::init(UdmLoggerType::Main, verbose, None);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Daemon requires a log path"));
        }
    }

    #[test]
    fn test_drink_control_logger_requires_log_path() {
        let verbose = Verbosity::default();
        let result = UdmLogger::init(UdmLoggerType::DrinkContrl, verbose, None);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Daemon requires a log path"));
        }
    }

    #[test]
    #[ignore] // tracing can only be initialized once per process, so this test conflicts with others
    fn test_daemon_logger_creates_log_file() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test_daemon.log");
        let log_path_str = log_path.to_str().unwrap();

        let verbose = Verbosity::default();
        let result = UdmLogger::init(UdmLoggerType::Daemon, verbose, Some(log_path_str));

        // In a fresh process, init should succeed and file should be created
        assert!(
            result.is_ok(),
            "Logger init should succeed in isolated test"
        );
        assert!(log_path.exists(), "Log file should be created");
        assert!(
            fs::metadata(&log_path).is_ok(),
            "Log file should be accessible"
        );
    }

    #[test]
    #[ignore] // tracing can only be initialized once per process, so this test conflicts with others
    fn test_main_logger_creates_log_file() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test_main.log");
        let log_path_str = log_path.to_str().unwrap();

        let verbose = Verbosity::default();
        let result = UdmLogger::init(UdmLoggerType::Main, verbose, Some(log_path_str));

        assert!(
            result.is_ok(),
            "Logger init should succeed in isolated test"
        );
        assert!(log_path.exists(), "Log file should be created");
        assert!(
            fs::metadata(&log_path).is_ok(),
            "Log file should be accessible"
        );
    }

    #[test]
    #[ignore] // tracing can only be initialized once per process, so this test conflicts with others
    fn test_drink_control_logger_creates_log_file() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test_dc.log");
        let log_path_str = log_path.to_str().unwrap();

        let verbose = Verbosity::default();
        let result = UdmLogger::init(UdmLoggerType::DrinkContrl, verbose, Some(log_path_str));

        assert!(
            result.is_ok(),
            "Logger init should succeed in isolated test"
        );
        assert!(log_path.exists(), "Log file should be created");
        assert!(
            fs::metadata(&log_path).is_ok(),
            "Log file should be accessible"
        );
    }

    #[test]
    fn test_invalid_log_path_returns_error() {
        let verbose = Verbosity::default();
        let invalid_path = "/nonexistent/directory/that/does/not/exist/test.log";
        let result = UdmLogger::init(UdmLoggerType::Daemon, verbose, Some(invalid_path));

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Path") || e.to_string().contains("No such file"));
        }
    }
}
