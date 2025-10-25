// Модуль для работы с сетевыми адаптерами

use serde_json;

#[derive(Clone, Default)]
pub struct NetworkAdapter {
    pub name: String,
    pub mac_address: String,
    pub ip_addresses: Vec<String>,
    pub dns_servers: Vec<String>,
    pub connection_type: String, // WiFi, Ethernet, etc.
    pub gateway: String,
    pub is_online: bool,
    pub ping_to_gateway: Option<f64>, // в ms
    pub connection_speed: String, // Mbps или качественная оценка
}

pub fn get_network_adapters() -> Vec<NetworkAdapter> {
    let command = r#"Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | ForEach-Object {
    $adapter = $_
    $ip_info = Get-NetIPAddress -InterfaceAlias $adapter.Name -AddressFamily IPv4 | Select-Object -First 1
    $dns_info = Get-DnsClientServerAddress -InterfaceAlias $adapter.Name -AddressFamily IPv4

    # Получаем информацию о шлюзе
    $route_info = Get-NetRoute -InterfaceAlias $adapter.Name -AddressFamily IPv4 | Where-Object { $_.DestinationPrefix -eq '0.0.0.0/0' } | Select-Object -First 1

    # Определяем тип подключения
    $connection_type = "Неизвестно"
    if ($adapter.Name -like "*Wi-Fi*" -or $adapter.Name -like "*Wireless*" -or $adapter.Name -like "*WiFi*") {
        $connection_type = "WiFi"
    } elseif ($adapter.Name -like "*Ethernet*" -or $adapter.Name -like "*LAN*" -or $adapter.PhysicalMediaType -eq "802.3") {
        $connection_type = "Ethernet"
    } elseif ($adapter.PhysicalMediaType -eq "Wireless LAN") {
        $connection_type = "WiFi"
    } elseif ($adapter.PhysicalMediaType -eq "802.11") {
        $connection_type = "WiFi"
    }

    [PSCustomObject]@{
        Name = $adapter.Name
        Status = $adapter.Status.ToString()
        MacAddress = $adapter.MacAddress
        IPAddress = if ($ip_info) { $ip_info.IPAddress } else { "N/A" }
        DNSServers = if ($dns_info.ServerAddresses) { $dns_info.ServerAddresses -join ", " } else { "N/A" }
        ConnectionType = $connection_type
        Gateway = if ($route_info) { $route_info.NextHop } else { "N/A" }
    }
} | ConvertTo-Json"#;

    match run_powershell_command(command) {
        Ok(json_result) => {
            // Парсим JSON результат
            match serde_json::from_str::<Vec<serde_json::Value>>(&json_result) {
                Ok(adapters_json) => {
                    let mut adapters = Vec::new();
                    for adapter_json in adapters_json {
                        if let (Some(name), Some(_status), Some(mac), Some(ip), Some(dns), Some(conn_type), Some(gateway)) = (
                            adapter_json.get("Name").and_then(|v| v.as_str()),
                            adapter_json.get("Status").and_then(|v| v.as_str()),
                            adapter_json.get("MacAddress").and_then(|v| v.as_str()),
                            adapter_json.get("IPAddress").and_then(|v| v.as_str()),
                            adapter_json.get("DNSServers").and_then(|v| v.as_str()),
                            adapter_json.get("ConnectionType").and_then(|v| v.as_str()),
                            adapter_json.get("Gateway").and_then(|v| v.as_str()),
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

                            // Проверяем доступность шлюза
                            let (is_online, ping_time) = if gateway != "N/A" && !gateway.is_empty() {
                                let ping_result = ping_gateway(gateway);
                                (ping_result.is_some(), ping_result)
                            } else {
                                (false, None)
                            };

                            // Определяем качество соединения на основе пинга
                            let connection_speed = match ping_time {
                                Some(ping) if ping < 10.0 => "Отличное (🌟)".to_string(),
                                Some(ping) if ping < 50.0 => "Хорошее (✅)".to_string(),
                                Some(ping) if ping < 150.0 => "Среднее (⚠️)".to_string(),
                                Some(ping) if ping < 500.0 => "Плохое (❌)".to_string(),
                                Some(_) => "Очень плохое (💀)".to_string(),
                                None => "Недоступно (🚫)".to_string(),
                            };

                            adapters.push(NetworkAdapter {
                                name: name.to_string(),
                                mac_address: mac.to_string(),
                                ip_addresses,
                                dns_servers,
                                connection_type: conn_type.to_string(),
                                gateway: gateway.to_string(),
                                is_online,
                                ping_to_gateway: ping_time,
                                connection_speed,
                            });
                        }
                    }
                    adapters
                }
                Err(_) => vec![NetworkAdapter {
                    name: "Ошибка получения данных".to_string(),
                    mac_address: "N/A".to_string(),
                    ip_addresses: vec!["N/A".to_string()],
                    dns_servers: vec!["N/A".to_string()],
                    connection_type: "N/A".to_string(),
                    gateway: "N/A".to_string(),
                    is_online: false,
                    ping_to_gateway: None,
                    connection_speed: "Недоступно (🚫)".to_string(),
                }],
            }
        }
        Err(_) => vec![NetworkAdapter {
            name: "Ошибка выполнения команды".to_string(),
            mac_address: "N/A".to_string(),
            ip_addresses: vec!["N/A".to_string()],
            dns_servers: vec!["N/A".to_string()],
            connection_type: "N/A".to_string(),
            gateway: "N/A".to_string(),
            is_online: false,
            ping_to_gateway: None,
            connection_speed: "Недоступно (🚫)".to_string(),
        }],
    }
}

// Функция для измерения пинга до шлюза
fn ping_gateway(gateway: &str) -> Option<f64> {
    use std::process::Command;

    // Используем ping.exe для измерения задержки до шлюза
    match Command::new("ping")
        .arg("-n").arg("1")  // Один пакет
        .arg("-w").arg("2000")  // Таймаут 2 секунды
        .arg(gateway)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);

            // Парсим результат ping.exe
            if let Some(time_line) = stdout.lines().find(|line| line.contains("time=")) {
                if let Some(time_part) = time_line.split("time=").nth(1) {
                    if let Some(ms_str) = time_part.split("ms").next() {
                        if let Ok(ms) = ms_str.trim().parse::<f64>() {
                            return Some(ms);
                        }
                    }
                }
            }

            // Альтернативный парсинг для русской локали
            if let Some(time_line) = stdout.lines().find(|line| line.contains("время=")) {
                if let Some(time_part) = time_line.split("время=").nth(1) {
                    if let Some(ms_str) = time_part.split("мс").next() {
                        if let Ok(ms) = ms_str.trim().parse::<f64>() {
                            return Some(ms);
                        }
                    }
                }
            }

            None
        }
        Err(_) => None,
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
