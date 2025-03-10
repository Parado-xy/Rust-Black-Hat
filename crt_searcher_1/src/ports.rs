// Import necessary modules and types from our crate
use crate::{
    common_ports::MOST_COMMON_PORTS_100, // A list of the 100 most common ports to scan
    model::{Port, Subdomain},            // Our data models for representing ports and subdomains
};
// Import parallel iterator functionality from rayon for concurrent port scanning
use rayon::prelude::*;
// Import networking types for socket operations
use std::net::{SocketAddr, ToSocketAddrs};
// Import TcpStream for connection attempts and Duration for timeouts
use std::{net::TcpStream, time::Duration};

/// Scans the most common ports for a given subdomain and returns the subdomain
/// with the open ports populated
///
/// # Arguments
///
/// * `subdomain` - A Subdomain struct containing domain information to scan
///
/// # Returns
///
/// * The same Subdomain with open_ports field populated with scan results
pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    // Convert the domain name to a socket address with an arbitrary port (1024)
    // This is just to validate that the domain resolves to an IP address
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    // If the domain doesn't resolve to any IP addresses, return early
    if socket_addresses.is_empty() {
        return subdomain;
    }

    // Scan the most common ports in parallel using rayon
    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter() // Convert to parallel iterator
        .map(|port| scan_port(socket_addresses[0], *port)) // Scan each port (using first IP if multiple)
        .filter(|port| port.is_open) // Only keep ports that are open
        .collect(); // Collect results into a Vec<Port>
    subdomain
}

/// Attempts to connect to a specific port on a given socket address to determine
/// if the port is open
///
/// # Arguments
///
/// * `socket_address` - The base socket address (IP) to connect to
/// * `port` - The port number to check
///
/// # Returns
///
/// * A Port struct containing the port number and whether it's open
fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    // Set a 3-second timeout for connection attempts
    let timeout = Duration::from_secs(3);
    // Update the socket address to use the port we want to scan
    socket_address.set_port(port);

    // Attempt to connect to the port with the specified timeout
    // If the connection succeeds, the port is considered open
    let is_open = TcpStream::connect_timeout(&socket_address, timeout).is_ok();

    // Return a new Port struct with the scan results
    Port { port, is_open }
}
