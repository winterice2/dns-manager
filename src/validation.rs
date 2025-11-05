// Input validation module - prevent command injection and security vulnerabilities

use crate::error::{DnsError, DnsResult};
use std::net::IpAddr;

/// Validates and sanitizes an IP address
/// Prevents command injection by ensuring only valid IPs pass through
pub fn validate_ip_address(ip: &str) -> DnsResult<String> {
    let trimmed = ip.trim();
    
    // Try to parse as IpAddr (v4 or v6)
    match trimmed.parse::<IpAddr>() {
        Ok(addr) => Ok(addr.to_string()),
        Err(_) => Err(DnsError::InvalidIpAddress(trimmed.to_string())),
    }
}

/// Validates a pair of DNS addresses (primary and secondary)
pub fn validate_dns_pair(primary: &str, secondary: &str) -> DnsResult<(String, String)> {
    let primary_validated = validate_ip_address(primary)?;
    let secondary_validated = validate_ip_address(secondary)?;
    Ok((primary_validated, secondary_validated))
}

/// Sanitizes PowerShell command arguments to prevent injection
/// Removes special characters that could break out of command context
pub fn sanitize_powershell_arg(arg: &str) -> String {
    // Allow only alphanumeric, dots, colons (for IPv6), and hyphens
    arg.chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == ':' || *c == '-')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ipv4() {
        assert!(validate_ip_address("1.1.1.1").is_ok());
        assert!(validate_ip_address("8.8.8.8").is_ok());
        assert!(validate_ip_address("192.168.1.1").is_ok());
    }

    #[test]
    fn test_invalid_ip() {
        assert!(validate_ip_address("999.999.999.999").is_err());
        assert!(validate_ip_address("1.1.1.1; rm -rf /").is_err());
        assert!(validate_ip_address("malicious").is_err());
    }

    #[test]
    fn test_sanitize() {
        assert_eq!(sanitize_powershell_arg("1.1.1.1"), "1.1.1.1");
        assert_eq!(sanitize_powershell_arg("1.1.1.1; rm -rf /"), "1.1.1.1rmrf");
    }
}
