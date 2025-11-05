// Модуль для работы с сетевыми адаптерами

use serde::{Deserialize, Serialize};

/// Network adapter information
#[derive(Clone, Default, Debug)]
pub struct NetworkAdapter {
    pub name: String,
    pub status: String,
    pub mac_address: String,
    pub ip_addresses: Vec<String>,
    pub dns_servers: Vec<String>,
}

/// Typed structure for PowerShell JSON output
#[derive(Debug, Deserialize)]
struct AdapterJson {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "MacAddress")]
    mac_address: String,
    #[serde(rename = "IPAddress")]
    ip_address: String,
    #[serde(rename = "DNSServers")]
    dns_servers: String,
}

pub fn get_network_adapters() -> Vec<NetworkAdapter> {
    let command = r#"Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | ForEach-Object {
    $adapter = $_
    $ip_info = Get-NetIPAddress -InterfaceAlias $adapter.Name -AddressFamily IPv4 | Select-Object -First 1
    $dns_info = Get-DnsClientServerAddress -InterfaceAlias $adapter.Name -AddressFamily IPv4

    [PSCustomObject]@{
        Name = $adapter.Name
        Status = $adapter.Status.ToString()
        MacAddress = $adapter.MacAddress
        IPAddress = if ($ip_info) { $ip_info.IPAddress } else { "N/A" }
        DNSServers = if ($dns_info.ServerAddresses) { $dns_info.ServerAddresses -join ", " } else { "N/A" }
    }
} | ConvertTo-Json"#;

    match run_powershell_command(command) {
        Ok(json_result) => {
            // PERFORMANCE: Use typed deserialization instead of Value
            // Handle both single object and array of objects
            let adapters_json: Result<Vec<AdapterJson>, _> = serde_json::from_str(&json_result)
                .or_else(|_| {
                    // Try parsing as single object
                    serde_json::from_str::<AdapterJson>(&json_result)
                        .map(|single| vec![single])
                });

            match adapters_json {
                Ok(adapters_json) => {
                    adapters_json
                        .into_iter()
                        .map(|adapter_json| {
                            let ip_addresses = if adapter_json.ip_address != "N/A" {
                                vec![adapter_json.ip_address]
                            } else {
                                vec!["Не назначен".to_string()]
                            };

                            let dns_servers = if adapter_json.dns_servers != "N/A" {
                                adapter_json.dns_servers
                                    .split(", ")
                                    .map(|s| s.to_string())
                                    .collect()
                            } else {
                                vec!["Не настроен".to_string()]
                            };

                            NetworkAdapter {
                                name: adapter_json.name,
                                status: adapter_json.status,
                                mac_address: adapter_json.mac_address,
                                ip_addresses,
                                dns_servers,
                            }
                        })
                        .collect()
                }
                Err(e) => {
                    eprintln!("Failed to parse adapter JSON: {}", e);
                    vec![NetworkAdapter {
                        name: "Ошибка парсинга".to_string(),
                        status: "N/A".to_string(),
                        mac_address: "N/A".to_string(),
                        ip_addresses: vec!["N/A".to_string()],
                        dns_servers: vec!["N/A".to_string()],
                    }]
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to execute PowerShell command: {}", e);
            vec![NetworkAdapter {
                name: "Ошибка выполнения команды".to_string(),
                status: "N/A".to_string(),
                mac_address: "N/A".to_string(),
                ip_addresses: vec!["N/A".to_string()],
                dns_servers: vec!["N/A".to_string()],
            }]
        }
    }
}

fn run_powershell_command(command: &str) -> Result<String, String> {
    use std::process::Command;
    
    // SECURITY: Use environment variable instead of hardcoded path
    let system_root = std::env::var("SystemRoot")
        .unwrap_or_else(|_| "C:\\Windows".to_string());

    // Если команда содержит ipconfig или netsh - используем cmd с полным путем
    if command.contains("ipconfig") || command.contains("netsh") {
        let cmd_path = format!("{}\\System32\\cmd.exe", system_root);
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

        if output.status.success() {
            Ok(stdout)
        } else {
            Err("Command execution failed".to_string())
        }
    } else {
        // Для PowerShell команд используем PowerShell
        let ps_path = format!("{}\\System32\\WindowsPowerShell\\v1.0\\powershell.exe", system_root);
        let output = Command::new(&ps_path)
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

        if output.status.success() {
            Ok(stdout)
        } else {
            Err("Command execution failed".to_string())
        }
    }
}
