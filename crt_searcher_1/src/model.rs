// Import Deserialize trait from serde for JSON deserialization
use serde::Deserialize;

/// Represents a subdomain that we'll scan
///
/// This struct contains:
/// - The domain name as a string
/// - A vector of Port structs representing open ports found during scanning
///
/// Derives:
/// - Debug: For formatted output during development
/// - Clone: To allow the struct to be duplicated when needed
#[derive(Debug, Clone)]
pub struct Subdomain {
    /// The domain name string (e.g., "subdomain.example.com")
    pub domain: String,
    /// A list of ports that were found to be open during scanning
    pub open_ports: Vec<Port>,
}

/// Represents a network port and its state
///
/// This struct contains:
/// - The port number
/// - A boolean indicating whether the port is open
///
/// Derives:
/// - Debug: For formatted output during development
/// - Clone: To allow the struct to be duplicated when needed
#[derive(Debug, Clone)]
pub struct Port {
    /// The port number (0-65535)
    pub port: u16,
    /// Whether the port is open (true) or closed/filtered (false)
    pub is_open: bool,
}

/// Represents an entry from the Certificate Transparency logs via crt.sh
///
/// This struct is designed to match the JSON structure returned by the crt.sh API
///
/// Derives:
/// - Debug: For formatted output during development
/// - Deserialize: Allows automatic conversion from JSON to this struct
/// - Clone: To allow the struct to be duplicated when needed
#[derive(Debug, Deserialize, Clone)]
pub struct CrtShEntry {
    /// The domain name(s) found in the certificate
    /// Often contains multiple domains separated by newlines
    pub name_value: String,
}
