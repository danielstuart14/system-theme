//! Error module for system theme operations.
use std::fmt::Display;

/// Error type for system theme operations.
#[derive(Debug)]
pub enum Error {
    /// Operation not supported by the platform.
    Unsupported,
    /// Data not available (or invalid).
    Unavailable,
    /// Main thread required error.
    MainThreadRequired,
    /// Internal platform error.
    Platform(Box<dyn std::error::Error + Send + Sync>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsupported => write!(f, "Unsupported operation"),
            Error::Unavailable => write!(f, "Unavailable data"),
            Error::MainThreadRequired => write!(f, "Main thread required"),
            Error::Platform(err) => write!(f, "Platform error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Create a new error from a platform error.
    pub fn from_platform(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Error::Platform(Box::new(err))
    }
}
