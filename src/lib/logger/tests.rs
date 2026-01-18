#[cfg(test)]
mod tests {
    use super::super::*;
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
        assert!(result.is_ok(), "Logger init should succeed in isolated test");
        assert!(log_path.exists(), "Log file should be created");
        assert!(fs::metadata(&log_path).is_ok(), "Log file should be accessible");
    }

    #[test]
    #[ignore] // tracing can only be initialized once per process, so this test conflicts with others
    fn test_main_logger_creates_log_file() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test_main.log");
        let log_path_str = log_path.to_str().unwrap();

        let verbose = Verbosity::default();
        let result = UdmLogger::init(UdmLoggerType::Main, verbose, Some(log_path_str));

        assert!(result.is_ok(), "Logger init should succeed in isolated test");
        assert!(log_path.exists(), "Log file should be created");
        assert!(fs::metadata(&log_path).is_ok(), "Log file should be accessible");
    }

    #[test]
    #[ignore] // tracing can only be initialized once per process, so this test conflicts with others
    fn test_drink_control_logger_creates_log_file() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test_dc.log");
        let log_path_str = log_path.to_str().unwrap();

        let verbose = Verbosity::default();
        let result = UdmLogger::init(UdmLoggerType::DrinkContrl, verbose, Some(log_path_str));

        assert!(result.is_ok(), "Logger init should succeed in isolated test");
        assert!(log_path.exists(), "Log file should be created");
        assert!(fs::metadata(&log_path).is_ok(), "Log file should be accessible");
    }

    #[test]
    fn test_invalid_log_path_returns_error() {
        let verbose = Verbosity::default();
        let invalid_path = "/nonexistent/directory/that/does/not/exist/test.log";
        let result = UdmLogger::init(UdmLoggerType::Daemon, verbose, Some(invalid_path));

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                e.to_string().contains("Path") || e.to_string().contains("No such file")
            );
        }
    }
}
