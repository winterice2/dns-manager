// Модуль для DNS провайдеров

use std::collections::HashSet;
use crate::error::{DnsError, DnsResult};
use crate::validation::{validate_dns_pair, sanitize_powershell_arg};

#[derive(Clone)]
pub struct DNSProvider {
    pub name: String,
    pub primary: String,
    pub secondary: String,
    pub description: String,
}

#[derive(Clone, Default)]
pub struct SpeedTestResult {
    pub provider: String,
    pub primary_ping: Option<f64>,
    pub secondary_ping: Option<f64>,
    pub avg_ping: Option<f64>,
}

pub fn get_dns_providers() -> Vec<DNSProvider> {
    vec![
        DNSProvider {
            name: "Cloudflare".to_string(),
            primary: "1.1.1.1".to_string(),
            secondary: "1.0.0.1".to_string(),
            description: "Быстрый и приватный DNS от Cloudflare".to_string(),
        },
        DNSProvider {
            name: "Google".to_string(),
            primary: "8.8.8.8".to_string(),
            secondary: "8.8.4.4".to_string(),
            description: "Надежный DNS от Google".to_string(),
        },
        DNSProvider {
            name: "Quad9".to_string(),
            primary: "9.9.9.9".to_string(),
            secondary: "149.112.112.112".to_string(),
            description: "Защита от вредоносных сайтов".to_string(),
        },
        DNSProvider {
            name: "OpenDNS".to_string(),
            primary: "208.67.222.222".to_string(),
            secondary: "208.67.220.220".to_string(),
            description: "Семейная фильтрация контента".to_string(),
        },
        DNSProvider {
            name: "AdGuard".to_string(),
            primary: "94.140.14.14".to_string(),
            secondary: "94.140.15.15".to_string(),
            description: "Блокировка рекламы".to_string(),
        },
        DNSProvider {
            name: "CleanBrowsing".to_string(),
            primary: "185.228.168.9".to_string(),
            secondary: "185.228.169.9".to_string(),
            description: "Безопасный интернет для детей".to_string(),
        },
    ]
}

pub fn ping_dns_server(ip: &str) -> Option<f64> {
    // SECURITY: Validate IP address first
    if crate::validation::validate_ip_address(ip).is_err() {
        eprintln!("Invalid IP address for ping: {}", ip);
        return None;
    }
    
    let safe_ip = sanitize_powershell_arg(ip);
    let command = format!("Test-Connection -ComputerName {} -Count 1 | Select-Object -ExpandProperty ResponseTime", safe_ip);

    match run_powershell_command(&command) {
        Ok(result) => {
            // Парсим результат
            if let Ok(ms) = result.trim().parse::<f64>() {
                println!("PowerShell ping to {}: {:.1}ms", safe_ip, ms);
                Some(ms)
            } else {
                eprintln!("Failed to parse PowerShell ping result for {}: {}", safe_ip, result);
                None
            }
        }
        Err(e) => {
            eprintln!("PowerShell ping to {} failed: {}", safe_ip, e);
            None
        }
    }
}

pub fn get_current_dns() -> Result<String, String> {
    // Сначала пробуем PowerShell для получения текущих DNS серверов
    let ps_output = run_powershell_command("Get-DnsClientServerAddress | Where-Object { $_.AddressFamily -eq 2 } | Select-Object -ExpandProperty ServerAddresses");

    if let Ok(dns_list) = ps_output {
        if !dns_list.is_empty() && dns_list != "" {
            let dns_servers: Vec<&str> = dns_list.split_whitespace().collect();
            if !dns_servers.is_empty() {
                // Дедупликация DNS адресов с помощью HashSet
                let unique_servers: HashSet<&str> = dns_servers.into_iter().collect();
                let addresses = unique_servers.into_iter().collect::<Vec<&str>>().join(", ");
                // Проверяем, является ли это DHCP настройками
                if is_dhcp_dns(&addresses) {
                    return Ok(format!("Автопилот (DHCP): {}", addresses));
                } else {
                    return Ok(addresses);
                }
            }
        }
    }

    // Fallback - используем ipconfig напрямую с полным путем
    let output = run_cmd_command("ipconfig /all");

    match output {
        Ok(stdout) => {
            // Парсим результат
            let lines: Vec<&str> = stdout.lines().collect();
            let mut dns_servers = Vec::new();
            let mut is_dhcp = false;

            for line in lines {
                if line.contains("DNS servers configured through DHCP") {
                    is_dhcp = true;
                }
                if line.contains("DNS Servers") && !line.contains("configured through DHCP") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() > 1 {
                        let ip_part = parts[1].trim();
                        if !ip_part.is_empty() && !ip_part.contains("None") {
                            // Разбиваем на отдельные IP адреса
                            let ips: Vec<&str> = ip_part.split(',').collect();
                            for ip in ips {
                                let clean_ip = ip.trim();
                                if !clean_ip.is_empty() && clean_ip != "0.0.0.0" {
                                    dns_servers.push(clean_ip.to_string());
                                }
                            }
                        }
                    }
                }
            }

            if dns_servers.is_empty() {
                Ok("Автопилот (DHCP): адреса не получены".to_string())
            } else if is_dhcp {
                Ok(format!("Автопилот (DHCP): {}", dns_servers.join(", ")))
            } else {
                Ok(dns_servers.join(", "))
            }
        }
        Err(e) => Err(e),
    }
}

pub fn set_dns(primary: &str, secondary: &str) -> Result<String, String> {
    // SECURITY: Validate IPs to prevent command injection
    let (validated_primary, validated_secondary) = validate_dns_pair(primary, secondary)
        .map_err(|e| e.to_string())?;
    
    // SECURITY: Additional sanitization
    let safe_primary = sanitize_powershell_arg(&validated_primary);
    let safe_secondary = sanitize_powershell_arg(&validated_secondary);
    
    // Получаем список активных сетевых адаптеров и устанавливаем DNS для всех
    let command = format!(
        r#"Get-NetAdapter | Where-Object {{ $_.Status -eq 'Up' }} | ForEach-Object {{
    Set-DnsClientServerAddress -InterfaceAlias $_.Name -ServerAddresses ('{0}','{1}')
}}"#,
        safe_primary, safe_secondary
    );
    run_powershell_command(&command)
}

pub fn reset_dns() -> Result<String, String> {
    // Полностью сбрасываем DNS для всех активных адаптеров к DHCP
    let command = r#"Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | ForEach-Object {
    # Сбрасываем все DNS настройки и устанавливаем получение от DHCP
    Set-DnsClientServerAddress -InterfaceAlias $_.Name -ResetServerAddresses
    # Явно включаем DHCP для DNS
    Set-NetIPInterface -InterfaceAlias $_.Name -Dhcp Enabled
}"#;
    run_powershell_command(command)
}

fn is_dhcp_dns(addresses: &str) -> bool {
    // Простая проверка на DHCP адреса
    addresses.contains("192.168.") || addresses.contains("10.") || addresses.contains("172.")
}

fn run_powershell_command(command: &str) -> Result<String, String> {
    use std::process::Command;
    use std::time::Duration;
    
    // SECURITY: Use environment variable instead of hardcoded path
    let powershell_path = std::env::var("SystemRoot")
        .unwrap_or_else(|_| "C:\\Windows".to_string()) + "\\System32\\WindowsPowerShell\\v1.0\\powershell.exe";

    let output = Command::new(&powershell_path)
        .arg("-NoProfile") // SECURITY: Don't load user profile
        .arg("-NonInteractive") // SECURITY: No interactive prompts
        .arg("-Command")
        .arg(command)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                "Permission denied. Please run as administrator.".to_string()
            } else {
                format!("Failed to execute command: {}", e)
            }
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        // SECURITY: Don't expose full stderr to prevent info leakage
        if stderr.contains("Access is denied") {
            Err("Access denied. Administrator rights required.".to_string())
        } else {
            Err("Command execution failed".to_string())
        }
    }
}

fn run_cmd_command(command: &str) -> Result<String, String> {
    use std::process::Command;
    
    // SECURITY: Use environment variable instead of hardcoded path
    let cmd_path = std::env::var("SystemRoot")
        .unwrap_or_else(|_| "C:\\Windows".to_string()) + "\\System32\\cmd.exe";

    let output = Command::new(&cmd_path)
        .arg("/C")
        .arg(command)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                "Permission denied. Please run as administrator.".to_string()
            } else {
                format!("Failed to execute command: {}", e)
            }
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err("Command execution failed".to_string())
    }
}
