// Модуль для DNS провайдеров

use std::collections::HashSet;
use encoding_rs;
use std::io::{self, Write};

// Функция для вывода в CP866 кодировке (для корректного отображения в Windows PowerShell)
fn println_cp866(message: &str) {
    // Конвертируем UTF-8 в CP866 (DOS Russian)
    let (cow, _encoding, _had_errors) = encoding_rs::WINDOWS_1251.encode(message);
    // Если CP866 недоступна, используем Windows-1251 как альтернативу
    let bytes = cow.as_ref();

    // Выводим как есть (Windows консоль должна правильно интерпретировать)
    let _ = io::stdout().write_all(bytes);
    let _ = io::stdout().write_all(b"\r\n");
}

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
        DNSProvider {
            name: "Comodo".to_string(),
            primary: "8.26.56.26".to_string(),
            secondary: "8.20.247.20".to_string(),
            description: "Безопасный DNS с фильтрацией".to_string(),
        },
        DNSProvider {
            name: "Yandex".to_string(),
            primary: "77.88.8.8".to_string(),
            secondary: "77.88.8.1".to_string(),
            description: "DNS от Яндекса для русскоязычных".to_string(),
        },
        DNSProvider {
            name: "DNS.WATCH".to_string(),
            primary: "84.200.69.80".to_string(),
            secondary: "84.200.70.40".to_string(),
            description: "Независимый DNS без логирования".to_string(),
        },
        DNSProvider {
            name: "UncensoredDNS".to_string(),
            primary: "91.239.100.100".to_string(),
            secondary: "89.233.43.71".to_string(),
            description: "DNS без цензуры".to_string(),
        },
        DNSProvider {
            name: "Freenom".to_string(),
            primary: "80.80.80.80".to_string(),
            secondary: "80.80.81.81".to_string(),
            description: "Бесплатный DNS от Freenom".to_string(),
        },
        DNSProvider {
            name: "Level3".to_string(),
            primary: "209.244.0.3".to_string(),
            secondary: "209.244.0.4".to_string(),
            description: "DNS от Level 3 Communications".to_string(),
        },
    ]
}

pub fn ping_dns_server(ip: &str) -> Option<f64> {
    // Пробуем несколько подходов для измерения задержки

    // Сначала пробуем Test-Connection с более простой командой
    let command1 = format!("(Test-Connection -ComputerName {} -Count 1 -TimeoutSeconds 5).ResponseTime", ip);

    match run_powershell_command(&command1) {
        Ok(result) => {
            if let Ok(ms) = result.trim().parse::<f64>() {
                #[cfg(debug_assertions)]
                println_cp866(&format!("PowerShell Test-Connection ping to {}: {:.1}ms", ip, ms));
                return Some(ms);
            }
        }
        Err(_) => {
            // Если Test-Connection не работает, пробуем ping.exe
            #[cfg(debug_assertions)]
            println_cp866(&format!("Test-Connection failed for {}, trying ping.exe", ip));
        }
    }

    // Резервный вариант - используем ping.exe напрямую
    match run_executable("ping", &["-n", "1", "-w", "5000", ip]) {
        Ok(result) => {

            // Сначала пробуем распарсить английский вывод: "Reply from 1.1.1.1: bytes=32 time=14ms TTL=57"
            if let Some(time_part) = result.split("time=").nth(1) {
                if let Some(ms_str) = time_part.split("ms").next() {
                    if let Ok(ms) = ms_str.trim().parse::<f64>() {
                        #[cfg(debug_assertions)]
                        println_cp866(&format!("ping.exe ping to {}: {:.1}ms", ip, ms));
                        return Some(ms);
                    }
                }
            }

            // Ищем паттерн "время=числа�" в сыром выводе
            // Из логов видно: "�۶�=19��", "�۶�=47��" и т.д.
            // Ищем "=" за которым идут цифры, а потом специфический символ
            if let Some(eq_pos) = result.find("�۶�=") {
                let after_eq = &result[eq_pos + 4..]; // 4 = длина "�۶�="
                // Ищем следующий не-цифровой символ
                if let Some(end_pos) = after_eq.chars().position(|c| !c.is_ascii_digit()) {
                    let number_str = &after_eq[..end_pos];
                    if let Ok(ms) = number_str.parse::<f64>() {
                        println_cp866(&format!("ping.exe ping to {}: {:.1}ms (CP866 decoded)", ip, ms));
                        return Some(ms);
                    }
                }
            }

            // Исправленный поиск: ищем второе число после "=" (время, а не размер пакета)
            let eq_positions: Vec<usize> = result.match_indices('=').map(|(pos, _)| pos).collect();
            if eq_positions.len() >= 2 {
                // Второе "=" должно быть у времени (после "число байт=32 время=22мс")
                let time_eq_pos = eq_positions[1];
                let after_time_eq = &result[time_eq_pos + 1..];
                // Ищем цифры до первого не-цифрового символа
                if let Some(end_pos) = after_time_eq.chars().position(|c| !c.is_ascii_digit() && c != '.') {
                    let number_str = &after_time_eq[..end_pos];
                    if let Ok(ms) = number_str.parse::<f64>() {
                        // Проверяем что это разумное значение пинга (1-10000мс)
                        if ms >= 1.0 && ms < 10000.0 {
                            #[cfg(debug_assertions)]
                            println_cp866(&format!("ping.exe ping to {}: {:.1}ms (time parser)", ip, ms));
                            return Some(ms);
                        }
                    }
                }
            }

            // Запасной вариант: ищем числа между специфическими символами
            if let Some(time_marker_pos) = result.find("�६�=") {
                let after_marker = &result[time_marker_pos + 4..];
                if let Some(end_pos) = after_marker.chars().position(|c| !c.is_ascii_digit() && c != '.') {
                    let number_str = &after_marker[..end_pos];
                    if let Ok(ms) = number_str.parse::<f64>() {
                        if ms >= 1.0 && ms < 10000.0 {
                            #[cfg(debug_assertions)]
                            println_cp866(&format!("ping.exe ping to {}: {:.1}ms (CP866 marker)", ip, ms));
                            return Some(ms);
                        }
                    }
                }
            }

            #[cfg(debug_assertions)]
            println_cp866(&format!("Failed to parse ping.exe result for {}: {}", ip, result));
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            println_cp866(&format!("ping.exe ping to {} failed: {}", ip, e));
        }
    }

    None
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
    // Получаем список активных сетевых адаптеров и устанавливаем DNS для всех
    let command = format!(
        r#"Get-NetAdapter | Where-Object {{ $_.Status -eq 'Up' }} | ForEach-Object {{
    Set-DnsClientServerAddress -InterfaceAlias $_.Name -ServerAddresses ('{0}','{1}')
}}"#,
        primary, secondary
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

fn run_cmd_command(command: &str) -> Result<String, String> {
    use std::process::Command;

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
}

fn run_executable(program: &str, args: &[&str]) -> Result<String, String> {
    use std::process::Command;

    let mut command = Command::new(program);
    for arg in args {
        command.arg(arg);
    }

    let output = command.output()
        .map_err(|e| format!("Failed to execute {}: {}", program, e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(stderr)
    }
}

// Валидация IP адреса (IPv4)
pub fn validate_ip_address(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();

    // Должен быть ровно 4 части
    if parts.len() != 4 {
        return false;
    }

    // Каждая часть должна быть числом от 0 до 255
    for part in parts {
        match part.parse::<u8>() {
            Ok(_) => continue,
            Err(_) => return false,
        }
    }

    true
}
