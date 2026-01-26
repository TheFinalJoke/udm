use regex::Error as RegexError;
use rusqlite::Error as rusqlite_error;
use std::error::Error as GenericError;
use std::fmt::Display;
use thiserror::Error;
use tokio_postgres::Error as PostgresError;

#[derive(Error, Debug)]
pub enum UdmError {
    #[error("Invalid Configuration {0}")]
    InvalidateConfiguration(String),
    #[error("An Error from Sqlite")]
    RusqliteError(#[from] rusqlite_error),
    #[error("An Error from Postgres {0}")]
    PostgresError(#[from] PostgresError),
    #[error("Invalid Input {0}")]
    InvalidInput(String),
    #[error("Api Failure: {0}")]
    ApiFailure(String),
    #[error("Error Parsing: {0}")]
    ParsingError(#[from] RegexError),
    #[error("Error Setting Up Logger: {0}")]
    LoggerError(String),
    #[error("Error collecting GpioPin: {0}")]
    GpioError(String),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Tonic Transport Error: {0}")]
    TonicTransportError(#[from] tonic::transport::Error),
    #[error("JSON Serialization Error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("UUID Error: {0}")]
    UuidError(#[from] uuid::Error),
    #[error("Configuration Error: {0}")]
    ConfigError(#[from] config::ConfigError),
}

impl From<String> for UdmError {
    fn from(value: String) -> Self {
        Self::InvalidateConfiguration(value)
    }
}

// Auto-convert ParseIntError to InvalidInput
impl From<std::num::ParseIntError> for UdmError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::InvalidInput(format!("Parse error: {err}"))
    }
}

// Auto-convert ParseFloatError to InvalidInput
impl From<std::num::ParseFloatError> for UdmError {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::InvalidInput(format!("Parse error: {err}"))
    }
}

// Auto-convert tonic::Status to ApiFailure
impl From<tonic::Status> for UdmError {
    fn from(status: tonic::Status) -> Self {
        Self::ApiFailure(format!("gRPC error: {}", status.message()))
    }
}

// Auto-convert anyhow::Error to ApiFailure
impl From<anyhow::Error> for UdmError {
    fn from(err: anyhow::Error) -> Self {
        Self::ApiFailure(format!("{err:#}"))
    }
}

// Auto-convert boxed errors to ApiFailure
impl From<Box<dyn GenericError + Send + Sync>> for UdmError {
    fn from(err: Box<dyn GenericError + Send + Sync>) -> Self {
        Self::ApiFailure(err.to_string())
    }
}

impl UdmError {
    pub fn log_and_exit(msg: Box<dyn GenericError>, exit_code: i32) {
        tracing::error!("{}", format!("{}", msg));
        std::process::exit(exit_code)
    }
    pub fn log(&self) {
        tracing::error!("{}", self);
    }
}

pub fn trace_log_error<T: Display>(error: T) -> T {
    let temp_error = error;
    tracing::error!("{}", temp_error);
    temp_error
}
