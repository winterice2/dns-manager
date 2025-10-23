// Модуль для работы с сетевыми адаптерами

use serde_json;

#[derive(Clone, Default)]
pub struct NetworkAdapter {
    pub name: String,
    pub status: String,
    pub mac_address: String,
    pub ip_addresses: Vec<String>,
    pub dns_servers: Vec<String>,
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
            // Парсим JSON результат
            match serde_json::from_str::<Vec<serde_json::Value>>(&json_result) {
                Ok(adapters_json) => {
                    let mut adapters = Vec::new();
                    for adapter_json in adapters_json {
                        if let (Some(name), Some(status), Some(mac), Some(ip), Some(dns)) = (
                            adapter_json.get("Name").and_then(|v| v.as_str()),
                            adapter_json.get("Status").and_then(|v| v.as_str()),
                            adapter_json.get("MacAddress").and_then(|v| v.as_str()),
                            adapter_json.get("IPAddress").and_then(|v| v.as_str()),
                            adapter_json.get("DNSServers").and_then(|v| v.as_str()),
                        ) {
                            let ip_addresses = if ip != "N/A" {
                                vec![ip.to_string()]
                            } else {
                                vec!["Не назначен".to_string()]
                            };

                            let dns_servers = if dns != "N/A" {
                                dns.split(", ").map(|s| s.to_string()).collect()
                            } else {
                                vec!["Не настроен".to_string()]
                            };

                            adapters.push(NetworkAdapter {
                                name: name.to_string(),
                                status: status.to_string(),
                                mac_address: mac.to_string(),
                                ip_addresses,
                                dns_servers,
                            });
                        }
                    }
                    adapters
                }
                Err(_) => vec![NetworkAdapter {
                    name: "Ошибка получения данных".to_string(),
                    status: "N/A".to_string(),
                    mac_address: "N/A".to_string(),
                    ip_addresses: vec!["N/A".to_string()],
                    dns_servers: vec!["N/A".to_string()],
                }],
            }
        }
        Err(_) => vec![NetworkAdapter {
            name: "Ошибка выполнения команды".to_string(),
            status: "N/A".to_string(),
            mac_address: "N/A".to_string(),
            ip_addresses: vec!["N/A".to_string()],
            dns_servers: vec!["N/A".to_string()],
        }],
    }
}

fn run_powershell_command(command: &str) -> Result<String, String> {
    use std::process::Command;

    // Если команда содержит ipconfig или netsh - используем cmd с полным путем
    if command.contains("ipconfig") || command.contains("netsh") {
        let output = Command::new(r"C:\Windows\System32\cmd.exe")
            .arg("/C")
            .arg(command)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(stderr)
        }
    } else {
        // Для PowerShell команд используем PowerShell
        let output = Command::new(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe")
            .arg("-Command")
            .arg(command)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(stderr)
        }
    }
}
