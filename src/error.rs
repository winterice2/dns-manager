// Error types module - proper error handling instead of String

use std::fmt;

/// Main error type for DNS operations
#[derive(Debug, Clone)]
pub enum DnsError {
    /// Invalid IP address format
    InvalidIpAddress(String),
    /// Command execution failed
    CommandFailed { command: String, stderr: String },
    /// Network adapter not found
    AdapterNotFound,
    /// Permission denied (needs admin rights)
    PermissionDenied,
    /// Timeout on operation
    Timeout,
    /// Parse error
    ParseError(String),
    /// Generic error with message
    Other(String),
}

impl fmt::Display for DnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DnsError::InvalidIpAddress(ip) => write!(f, "Invalid IP address: {}", ip),
            DnsError::CommandFailed { command, stderr } => {
                write!(f, "Command '{}' failed: {}", command, stderr)
            }
            DnsError::AdapterNotFound => write!(f, "Network adapter not found"),
            DnsError::PermissionDenied => {
                write!(f, "Permission denied. Please run as administrator.")
            }
            DnsError::Timeout => write!(f, "Operation timed out"),
            DnsError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DnsError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for DnsError {}

/// Convert from std::io::Error
impl From<std::io::Error> for DnsError {
    fn from(err: std::io::Error) -> Self {
        DnsError::Other(err.to_string())
    }
}

/// Result type alias for DNS operations
pub type DnsResult<T> = Result<T, DnsError>;
