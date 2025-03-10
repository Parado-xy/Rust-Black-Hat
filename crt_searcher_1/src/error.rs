// Import the Error derive macro from thiserror crate
use thiserror::Error;

/// Custom error types for the application
///
/// This enum defines all possible error types that can occur in our application.
/// By deriving Error, Debug, and Clone, we get useful functionality:
/// - Error: Provides error handling traits and custom error messages
/// - Debug: Allows printing detailed error information for debugging
/// - Clone: Enables copying errors when needed
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// Error shown when the CLI is used incorrectly
    ///
    /// Displayed when the user doesn't provide the correct command-line arguments
    #[error("Usage: tricoder <kerkour.com>")]
    CliUsage,

    /// Wrapper for HTTP request errors
    ///
    /// Captures errors from the reqwest HTTP client with a custom message
    #[error("Reqwest: {0}")]
    Reqwest(String),
}

/// Conversion from reqwest::Error to our custom Error type
///
/// This implementation allows us to use the '?' operator with reqwest
/// operations, automatically converting reqwest errors to our Error type
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        // Convert the reqwest error to a string and wrap it in our custom error type
        Error::Reqwest(err.to_string())
    }
}
