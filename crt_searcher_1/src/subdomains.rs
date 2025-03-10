// Import necessary modules and types
use crate::{
    model::{CrtShEntry, Subdomain}, // Data models for certificate transparency entries and subdomains
    Error,                          // Custom error type for the application
};
use reqwest::blocking::Client; // HTTP client for making web requests
use std::{collections::HashSet, time::Duration}; // Standard library imports for deduplication and timeouts
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts}, // DNS resolver configuration
    Resolver,                               // DNS resolver for checking if domains resolve
};

/// Enumerates subdomains for a given target domain using certificate transparency logs
///
/// # Arguments
///
/// * `http_client` - The HTTP client used to make requests to the crt.sh service
/// * `target` - The base domain to find subdomains for (e.g., "example.com")
///
/// # Returns
///
/// * A Result containing either a Vec of Subdomain structs or an Error
pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    // Query crt.sh for certificate records related to the target domain
    // The '%25.' is URL-encoded '%.' which is a wildcard search in crt.sh
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()? // Send the HTTP request
        .json()?; // Parse the JSON response into CrtShEntry structs

    // Process and deduplicate the subdomains found in certificates
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value // Get the domain names from each entry
                .split('\n') // Split on newlines (crt.sh returns multiple domains per entry)
                .map(|subdomain| subdomain.trim().to_string()) // Clean up whitespace
                .collect::<Vec<String>>() // Collect into a vector
        })
        .filter(|subdomain: &String| subdomain != target) // Remove exact matches with the target domain
        .filter(|subdomain: &String| !subdomain.contains('*')) // Remove wildcard domains
        .collect(); // Collect into a HashSet for deduplication

    // Add the target domain itself to the set of domains to scan
    subdomains.insert(target.to_string());

    // Create Subdomain structs for each domain and filter out those that don't resolve
    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,                 // Set the domain name
            open_ports: Vec::new(), // Initialize with empty list of open ports
        })
        .filter(resolves) // Only keep domains that resolve to an IP address
        .collect(); // Collect into the final Vec

    Ok(subdomains)
}

/// Checks if a subdomain resolves to an IP address
///
/// # Arguments
///
/// * `domain` - The Subdomain struct to check
///
/// # Returns
///
/// * A boolean indicating whether the domain resolves (true) or not (false)
pub fn resolves(domain: &Subdomain) -> bool {
    // Configure DNS resolver options
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4); // Set 4-second timeout for DNS lookups

    // Create a new DNS resolver with default configuration and our custom options
    let dns_resolver = Resolver::new(ResolverConfig::default(), opts)
        .expect("subdomain resolver: building DNS client");

    // Attempt to look up the IP address for the domain
    // Return true if successful, false if it fails
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
