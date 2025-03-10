# Certificate Transparency Searcher

A Rust-based subdomain discovery and port scanning tool that leverages Certificate Transparency logs to identify subdomains for a target domain and scan them for open ports.

## Overview

This tool utilizes the certificate transparency logs (via crt.sh) to discover subdomains associated with a target domain. It then performs port scanning to identify open services on those subdomains, helping to map the attack surface of a domain.

## Features

- Subdomain discovery using Certificate Transparency logs
- Parallel port scanning of discovered subdomains
- Focused scanning of the 100 most common ports
- Clean, organized output of results

## Prerequisites

- Rust and Cargo installed on your system

## Dependencies

- `serde`: For JSON deserialization of CT log responses
- `reqwest`: For HTTP requests to the crt.sh service
- `rayon`: For parallel port scanning
- `trust-dns-resolver`: For DNS resolution

## Usage

Run the program with a target domain to scan:

```sh
cargo run -- example.com
```

The program will:

1. Query the Certificate Transparency logs for subdomains
2. Scan each discovered subdomain for open ports
3. Display the results with each subdomain and its open ports

## Project Structure

- main.rs: Entry point of the application
- model.rs: Data structures for subdomains, ports, and CT log entries
- `src/subdomains.rs`: Functionality for discovering subdomains
- `src/ports.rs`: Port scanning implementation
- `src/common_ports.rs`: List of common ports to scan
- `src/error.rs`: Custom error handling

## How It Works

1. The program queries the Certificate Transparency logs via crt.sh API
2. It extracts unique subdomains from the certificate data
3. For each subdomain, it attempts to resolve the IP address
4. If resolution succeeds, it scans the most common ports using TCP connections
5. Results are collected and displayed showing each subdomain and its open ports

## Security Note

This tool is for educational and legitimate security assessment purposes only. Always ensure you have permission to scan domains and ports.

## License

MIT
