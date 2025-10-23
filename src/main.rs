use eframe::egui;
use std::process::Command;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_title("DNS Manager - Windows"),
        ..Default::default()
    };

    eframe::run_native(
        "DNS Manager",
        options,
        Box::new(|cc| Box::new(DNSManager::new(cc))),
    )
}

#[derive(Clone)]
struct DNSProvider {
    name: String,
    primary: String,
    secondary: String,
    description: String,
}

#[derive(Clone, Default)]
struct SpeedTestResult {
    provider: String,
    primary_ping: Option<f64>,
    secondary_ping: Option<f64>,
    avg_ping: Option<f64>,
}

#[derive(Clone, Default)]
struct NetworkAdapter {
    name: String,
    status: String,
    mac_address: String,
    ip_addresses: Vec<String>,
    dns_servers: Vec<String>,
}

#[derive(Default)]
struct DNSManager {
    status: String,
    current_dns: String,
    speed_results: Vec<SpeedTestResult>,
    custom_primary: String,
    custom_secondary: String,
    selected_tab: usize,
    is_speed_testing: bool,
    network_adapters: Vec<NetworkAdapter>,
}

impl DNSManager {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 🚀 КОСМИЧЕСКАЯ ЭМОЦИОНАЛЬНАЯ ТЕМА 2025 - Путь Андромеды 🌌
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals.dark_mode = true; // Темная тема для космоса

        // 🌌 Глубокий космос - фиолетово-синий фон
        style.visuals.window_fill = egui::Color32::from_rgb(15, 23, 42); // Темно-синий космос
        style.visuals.panel_fill = egui::Color32::from_rgb(30, 41, 59); // Более светлый синий
        style.visuals.faint_bg_color = egui::Color32::from_rgb(51, 65, 85);

        // 🛰️ Кибер-панк кнопки - фиолетовый с бирюзовым свечением
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(147, 51, 234); // Фиолетовый (Claude-style)
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(59, 130, 246); // Светло-голубой
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(6, 182, 212); // Бирюзовый

        // 🔮 Футуристические закругления - как космические корабли
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(16.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(16.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(16.0);

        // ⚡ Неоновый текст - кибер-панк стиль
        style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(236, 254, 255); // Почти белый
        style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(6, 182, 212); // Бирюзовый
        style.visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;

        // 🌟 Эмоциональный текст для лейблов
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(148, 163, 184)); // Светло-синий

        cc.egui_ctx.set_style(style);

        // Получаем информацию о сетевых адаптерах при запуске
        let network_adapters = Self::get_network_adapters();

        Self {
            status: "🚀 Ready for space launch!".to_string(),
            current_dns: String::new(),
            speed_results: Vec::new(),
            custom_primary: String::new(),
            custom_secondary: String::new(),
            selected_tab: 0,
            is_speed_testing: false,
            network_adapters,
        }
    }

    fn run_powershell_command(command: &str) -> Result<String, String> {
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

    fn get_current_dns() -> Result<String, String> {
        // Сначала пробуем PowerShell для получения текущих DNS серверов
        let ps_output = Self::run_powershell_command("Get-DnsClientServerAddress | Where-Object { $_.AddressFamily -eq 2 } | Select-Object -ExpandProperty ServerAddresses");

        if let Ok(dns_list) = ps_output {
            if !dns_list.is_empty() && dns_list != "" {
                let dns_servers: Vec<&str> = dns_list.split_whitespace().collect();
                if !dns_servers.is_empty() {
                    // Дедупликация DNS адресов с помощью HashSet
                    use std::collections::HashSet;
                    let unique_servers: HashSet<&str> = dns_servers.into_iter().collect();
                    let addresses = unique_servers.into_iter().collect::<Vec<&str>>().join(", ");
                    // Проверяем, является ли это DHCP настройками
                    if Self::is_dhcp_dns(&addresses) {
                        return Ok(format!("Автопилот (DHCP): {}", addresses));
                    } else {
                        return Ok(addresses);
                    }
                }
            }
        }

        // Fallback - используем ipconfig напрямую с полным путем
        let output = Command::new(r"C:\Windows\System32\ipconfig.exe")
            .arg("/all")
            .output()
            .map_err(|e| format!("Failed to execute ipconfig: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if output.status.success() {
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
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(stderr)
        }
    }

    fn is_dhcp_dns(addresses: &str) -> bool {
        // Проверяем, содержат ли адреса типичные провайдерские DNS
        let dhcp_indicators = ["192.168.", "10.", "172."];
        addresses.split(", ").any(|addr| {
            dhcp_indicators.iter().any(|indicator| addr.starts_with(indicator))
        })
    }

    fn get_dns_providers() -> Vec<DNSProvider> {
        vec![
            DNSProvider {
                name: "Cloudflare".to_string(),
                primary: "1.1.1.1".to_string(),
                secondary: "1.0.0.1".to_string(),
                description: "Быстрый и приватный DNS".to_string(),
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
                description: "Безопасный DNS с блокировкой вредоносных сайтов".to_string(),
            },
            DNSProvider {
                name: "OpenDNS".to_string(),
                primary: "208.67.222.222".to_string(),
                secondary: "208.67.220.220".to_string(),
                description: "Семейный DNS с фильтрацией контента".to_string(),
            },
            DNSProvider {
                name: "AdGuard".to_string(),
                primary: "94.140.14.14".to_string(),
                secondary: "94.140.15.15".to_string(),
                description: "DNS с блокировкой рекламы".to_string(),
            },
            DNSProvider {
                name: "CleanBrowsing".to_string(),
                primary: "185.228.168.9".to_string(),
                secondary: "185.228.169.9".to_string(),
                description: "Семейный DNS без рекламы".to_string(),
            },
        ]
    }

    fn get_network_adapters() -> Vec<NetworkAdapter> {
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

        match Self::run_powershell_command(command) {
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

    fn ping_dns_server(ip: &str) -> Option<f64> {
        // Используем PowerShell для измерения задержки через Test-Connection
        let command = format!("Test-Connection -ComputerName {} -Count 1 | Select-Object -ExpandProperty ResponseTime", ip);

        match Self::run_powershell_command(&command) {
            Ok(result) => {
                // Парсим результат
                if let Ok(ms) = result.trim().parse::<f64>() {
                    println!("PowerShell ping to {}: {:.1}ms", ip, ms);
                    Some(ms)
        } else {
                    println!("Failed to parse PowerShell ping result for {}: {}", ip, result);
                    None
                }
            }
            Err(e) => {
                println!("PowerShell ping to {} failed: {}", ip, e);
                None
            }
        }
    }

    fn start_speed_test(&mut self) {
        if !self.is_speed_testing {
            self.is_speed_testing = true;
            self.status = "🧪 Запуск тестирования скорости DNS...".to_string();
            self.speed_results.clear();
        }
    }

    fn update_speed_test(&mut self) -> bool {
        if !self.is_speed_testing {
            return false;
        }

        let providers = Self::get_dns_providers();
        let current_count = self.speed_results.len();

        if current_count < providers.len() {
            // Тестируем следующий провайдер
            let provider = &providers[current_count];
            self.status = format!("🧪 Тестирование {}... ({}/{})", provider.name, current_count + 1, providers.len());

            let primary_ping = Self::ping_dns_server(&provider.primary);
            let secondary_ping = Self::ping_dns_server(&provider.secondary);

            let mut result = SpeedTestResult {
                provider: provider.name.clone(),
                primary_ping,
                secondary_ping,
                avg_ping: None,
            };

            // Вычисляем среднее значение
            let mut pings = Vec::new();
            if let Some(p) = result.primary_ping { pings.push(p); }
            if let Some(p) = result.secondary_ping { pings.push(p); }

            if !pings.is_empty() {
                result.avg_ping = Some(pings.iter().sum::<f64>() / pings.len() as f64);
            }

            self.speed_results.push(result);
            return false; // Продолжаем тестирование
        } else {
            // Тестирование завершено
            self.is_speed_testing = false;

            // Сортируем по средней задержке
            self.speed_results.sort_by(|a, b| {
                match (a.avg_ping, b.avg_ping) {
                    (Some(a_ping), Some(b_ping)) => a_ping.partial_cmp(&b_ping).unwrap_or(std::cmp::Ordering::Equal),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            });

            self.status = format!("✅ Тестирование завершено! Получено {} результатов.", self.speed_results.len());
            return true; // Тестирование завершено
        }
    }

    fn set_dns(primary: &str, secondary: &str) -> Result<String, String> {
        // Получаем список активных сетевых адаптеров и устанавливаем DNS для всех
        let command = format!(
            r#"Get-NetAdapter | Where-Object {{ $_.Status -eq 'Up' }} | ForEach-Object {{
    Set-DnsClientServerAddress -InterfaceAlias $_.Name -ServerAddresses ('{0}','{1}')
}}"#,
            primary, secondary
        );
        Self::run_powershell_command(&command)
    }

    fn reset_dns() -> Result<String, String> {
        // Полностью сбрасываем DNS для всех активных адаптеров к DHCP
        let command = r#"Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | ForEach-Object {
    # Сбрасываем все DNS настройки и устанавливаем получение от DHCP
    Set-DnsClientServerAddress -InterfaceAlias $_.Name -ResetServerAddresses
    # Явно включаем DHCP для DNS
    Set-NetIPInterface -InterfaceAlias $_.Name -Dhcp Enabled
}"#;
        Self::run_powershell_command(command)
    }
}

impl eframe::App for DNSManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Обновляем тестирование скорости, если оно активно
        if self.is_speed_testing {
            self.update_speed_test();
            ctx.request_repaint(); // Запрашиваем перерисовку для обновления UI
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Создаем вкладки для организации интерфейса
            ui.horizontal(|ui| {
                ui.heading("🌌 DNS Manager - Расширенная Вселенная");
            });
            ui.separator();

            // Простая система вкладок через условный рендеринг
            ui.horizontal(|ui| {
                if ui.selectable_label(self.selected_tab == 0, "🌌 Главная").clicked() {
                    self.selected_tab = 0;
                }
                if ui.selectable_label(self.selected_tab == 1, "🌍 Провайдеры").clicked() {
                    self.selected_tab = 1;
                }
                if ui.selectable_label(self.selected_tab == 2, "🧪 Лаборатория").clicked() {
                    self.selected_tab = 2;
                }
                if ui.selectable_label(self.selected_tab == 3, "📊 Статистика").clicked() {
                    self.selected_tab = 3;
                }
            });

            ui.separator();

            match self.selected_tab {
                0 => self.show_main_tab(ui, ctx),
                1 => self.show_providers_tab(ui, ctx),
                2 => self.show_lab_tab(ui, ctx),
                3 => self.show_stats_tab(ui),
                _ => self.show_main_tab(ui, ctx),
            }
        });
    }
}

impl DNSManager {
    fn show_main_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
            ui.vertical_centered(|ui| {
            ui.heading("🚀 Основные операции");
        });
        ui.separator();

        // Satellite Control
        ui.label("🛰️ Спутниковый контроль:");
        ui.label("🔄 Интеллектуальное переключение между галактиками");

        if ui.add_sized([ui.available_width(), 40.0], egui::Button::new("🚀 Launch/Landing DNS")).clicked() {
            self.status = "🛸 Navigation system activated...".to_string();
            ctx.request_repaint();

            match Self::get_current_dns() {
                Ok(current) => {
                    if current.contains("1.1.1.1") || current.contains("8.8.8.8") || current.contains("9.9.9.9") || current.contains("208.67.222.222") || current.contains("94.140.14.14") {
                        // Reset to automatic
                        match Self::reset_dns() {
                            Ok(_) => self.status = "🔄 Returned to autopilot".to_string(),
                            Err(e) => self.status = format!("💥 System failure: {}", e),
                        }
                    } else {
                        // Set Cloudflare
                        match Self::set_dns("1.1.1.1", "1.0.0.1") {
                            Ok(_) => self.status = "🎉 Course set to Cloudflare!".to_string(),
                            Err(e) => self.status = format!("💥 Engines failed to start: {}", e),
                        }
                    }
                }
                Err(e) => self.status = format!("💥 Navigation system failure: {}", e),
            }
        }

        ui.add_space(15.0);

        // Status display
        let dns_text = if !self.current_dns.is_empty() && !self.current_dns.contains("Автопилот (DHCP)") {
            format!("🛰️ Current coordinates: {}", self.current_dns)
        } else if self.current_dns.contains("Автопилот (DHCP)") {
            format!("🛰️ Current coordinates: {}", self.current_dns)
        } else {
            "🔭 Use telescope to scan coordinates".to_string()
        };

        // DNS with context menu
        let dns_label = ui.selectable_label(true, &dns_text);
        dns_label.context_menu(|ui| {
            if ui.button("Copy DNS").clicked() {
                ui.output_mut(|o| o.copied_text = self.current_dns.clone());
                ui.close_menu();
            }
            if ui.button("Copy full text").clicked() {
                ui.output_mut(|o| o.copied_text = dns_text.clone());
                ui.close_menu();
            }
        });

                ui.add_space(10.0);

        // Status with context menu
        let status_label = ui.selectable_label(true, &self.status);
        status_label.context_menu(|ui| {
            if ui.button("Copy status").clicked() {
                ui.output_mut(|o| o.copied_text = self.status.clone());
                ui.close_menu();
            }
        });

                ui.add_space(20.0);

        // Quick actions
        ui.label("⚡ Быстрые действия:");
        ui.horizontal(|ui| {
            if ui.button("🔭 Сканировать DNS").clicked() {
                self.status = "🔭 Telescope activating...".to_string();
                ctx.request_repaint();

                match Self::get_current_dns() {
                            Ok(dns) => {
                        self.current_dns = dns.clone();
                        if dns.contains("Автоматический") {
                            self.status = format!("🌌 Signal received: {}", dns);
                                } else {
                            self.status = format!("🛰️ Coordinates received: {}", dns);
                        }
                    }
                    Err(e) => {
                        self.status = format!("💫 Cosmic noise: {}", e);
                    }
                }
            }

            if ui.button("🔄 Сброс на DHCP").clicked() {
                match Self::reset_dns() {
                    Ok(_) => {
                        self.status = "✅ DNS сброшен на автоматический режим".to_string();
                        self.current_dns = String::new();
                    }
                    Err(e) => self.status = format!("❌ Ошибка сброса: {}", e),
                }
            }
        });
    }

    fn show_providers_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.heading("🌍 Библиотека DNS провайдеров");
        });
        ui.separator();

        ui.label("⭐ Популярные провайдеры:");
                ui.add_space(10.0);

        // Cloudflare
        if ui.add_sized([ui.available_width(), 35.0], egui::Button::new("☁️ Cloudflare - Быстрый и приватный (1.1.1.1)")).clicked() {
            self.status = "🛸 Entering Cloudflare orbit: 1.1.1.1, 1.0.0.1...".to_string();
            ctx.request_repaint();

            match Self::set_dns("1.1.1.1", "1.0.0.1") {
                Ok(_) => self.status = "🎉 Arrived at Cloudflare: 1.1.1.1, 1.0.0.1! Mission accomplished!".to_string(),
                Err(e) => self.status = format!("💥 Ship crashed: {}", e),
            }
        }

        ui.add_space(8.0);

        // Google
        if ui.add_sized([ui.available_width(), 35.0], egui::Button::new("🔍 Google - Надежный DNS (8.8.8.8)")).clicked() {
            self.status = "🛸 Activating Google hyperdrive: 8.8.8.8, 8.8.4.4...".to_string();
            ctx.request_repaint();

            match Self::set_dns("8.8.8.8", "8.8.4.4") {
                Ok(_) => self.status = "🎉 Welcome to Google: 8.8.8.8, 8.8.4.4! Data found!".to_string(),
                Err(e) => self.status = format!("💥 Hyperspace jump failed: {}", e),
            }
                }

                ui.add_space(15.0);
        ui.label("🔒 Специализированные провайдеры:");

        // Quad9
        if ui.add_sized([ui.available_width(), 35.0], egui::Button::new("🔒 Quad9 - Защита от угроз (9.9.9.9)")).clicked() {
            self.status = "🛸 Approaching Quad9 security zone: 9.9.9.9, 149.112.112.112...".to_string();
            ctx.request_repaint();

            match Self::set_dns("9.9.9.9", "149.112.112.112") {
                Ok(_) => self.status = "🎉 Secured with Quad9: 9.9.9.9, 149.112.112.112! Threats blocked!".to_string(),
                Err(e) => self.status = format!("💥 Security breach: {}", e),
            }
        }

        ui.add_space(8.0);

        // OpenDNS
        if ui.add_sized([ui.available_width(), 35.0], egui::Button::new("👨‍👩‍👧‍👦 OpenDNS - Семейная защита (208.67.222.222)")).clicked() {
            self.status = "🛸 Entering family-friendly zone: 208.67.222.222, 208.67.220.220...".to_string();
            ctx.request_repaint();

            match Self::set_dns("208.67.222.222", "208.67.220.220") {
                Ok(_) => self.status = "🎉 Family protection activated: 208.67.222.222, 208.67.220.220!".to_string(),
                Err(e) => self.status = format!("💥 Family shield failure: {}", e),
            }
        }

        ui.add_space(8.0);

        // AdGuard
        if ui.add_sized([ui.available_width(), 35.0], egui::Button::new("🚫 AdGuard - Без рекламы (94.140.14.14)")).clicked() {
            self.status = "🛸 Entering ad-free zone: 94.140.14.14, 94.140.15.15...".to_string();
            ctx.request_repaint();

            match Self::set_dns("94.140.14.14", "94.140.15.15") {
                Ok(_) => self.status = "🎉 Ads blocked: 94.140.14.14, 94.140.15.15! Clean browsing!".to_string(),
                Err(e) => self.status = format!("💥 Ad blocking failure: {}", e),
            }
        }

        ui.add_space(8.0);

        // CleanBrowsing
        if ui.add_sized([ui.available_width(), 35.0], egui::Button::new("🧹 CleanBrowsing - Чистый интернет (185.228.168.9)")).clicked() {
            self.status = "🛸 Entering clean zone: 185.228.168.9, 185.228.169.9...".to_string();
            ctx.request_repaint();

            match Self::set_dns("185.228.168.9", "185.228.169.9") {
                Ok(_) => self.status = "🎉 Clean browsing activated: 185.228.168.9, 185.228.169.9!".to_string(),
                Err(e) => self.status = format!("💥 Clean zone failure: {}", e),
            }
        }
    }

    fn show_lab_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.heading("🧪 Лаборатория тестирования");
        });
        ui.separator();

        ui.label("⚡ Инструменты анализа и тестирования:");
                ui.add_space(10.0);

        // DNS Speed Test
        let button_text = if self.is_speed_testing {
            "⏳ Тестирование выполняется...".to_string()
                } else {
            "⚡ DNS Speed Test - Тестировать все провайдеры".to_string()
        };

        if ui.add_sized([ui.available_width(), 40.0], egui::Button::new(button_text)).clicked() && !self.is_speed_testing {
            self.start_speed_test();
            ctx.request_repaint();
                }

                ui.add_space(10.0);
        ui.label("🔬 Результаты тестирования:");


        // Speed Test Results
        if !self.speed_results.is_empty() {
            ui.add_space(10.0);
            ui.label("📊 Скорость DNS серверов (отсортировано по задержке):");

            ui.separator();

            for (index, result) in self.speed_results.iter().enumerate() {
                let medal = match index {
                    0 => "🥇",
                    1 => "🥈",
                    2 => "🥉",
                    _ => "📍",
                };

                let avg_text = match result.avg_ping {
                    Some(avg) => format!("{:.1}ms", avg),
                    None => "N/A".to_string(),
                };

                let primary_text = match result.primary_ping {
                    Some(p) => format!("{:.1}ms", p),
                    None => "❌".to_string(),
                };

                let secondary_text = match result.secondary_ping {
                    Some(p) => format!("{:.1}ms", p),
                    None => "❌".to_string(),
                };

                ui.horizontal(|ui| {
                    ui.label(format!("{} {}:", medal, result.provider));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("Avg: {} | P1: {} | P2: {}", avg_text, primary_text, secondary_text));
            });
        });
            }

            ui.add_space(5.0);
            ui.small("💡 Чем меньше задержка - тем быстрее DNS!");
        } else {
            ui.add_space(10.0);
            ui.label("🔭 Запустите тестирование, чтобы увидеть результаты");
        }
    }

    fn show_stats_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("📊 Статистика и информация");
        });
        ui.separator();

        ui.label("📈 Общая статистика проекта:");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("🎯 Версия:");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("v1.2.1 - Мониторинг Сети");
            });
        });

        ui.horizontal(|ui| {
            ui.label("🔧 Провайдеров DNS:");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("6 доступных");
            });
        });

        ui.horizontal(|ui| {
            ui.label("⚡ Тестов скорости:");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(if self.speed_results.is_empty() { "Не выполнялось" } else { "Выполнено" });
            });
        });

        ui.add_space(20.0);
        ui.label("ℹ️ Информация о DNS:");
        ui.add_space(10.0);

        ui.label("☁️ **Cloudflare (1.1.1.1)**: Быстрый и приватный DNS от Cloudflare");
        ui.label("🔍 **Google (8.8.8.8)**: Надежный DNS от Google");
        ui.label("🔒 **Quad9 (9.9.9.9)**: Защита от вредоносных сайтов");
        ui.label("👨‍👩‍👧‍👦 **OpenDNS (208.67.222.222)**: Семейная фильтрация контента");
        ui.label("🚫 **AdGuard (94.140.14.14)**: Блокировка рекламы");
        ui.label("🧹 **CleanBrowsing (185.228.168.9)**: Безопасный интернет для детей");

        ui.add_space(20.0);
        ui.label("📡 Информация о сети:");
        ui.add_space(10.0);

        if ui.button("🔄 Обновить информацию о сети").clicked() {
            self.network_adapters = Self::get_network_adapters();
            self.status = "✅ Информация о сети обновлена!".to_string();
                }

                ui.add_space(10.0);

        if self.network_adapters.is_empty() {
            ui.label("❌ Нет активных сетевых адаптеров");
                } else {
            for adapter in &self.network_adapters {
                ui.add_space(5.0);
                ui.label(format!("🔌 **{}** ({})", adapter.name, adapter.status));

                ui.horizontal(|ui| {
                    ui.label("📍 MAC адрес:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(&adapter.mac_address);
                    });
                });

                ui.horizontal(|ui| {
                    ui.label("🌐 IP адрес:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(adapter.ip_addresses.join(", "));
                    });
                });

                ui.horizontal(|ui| {
                    ui.label("🔧 DNS серверы:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(adapter.dns_servers.join(", "));
            });
        });

                ui.add_space(5.0);
                ui.separator();
            }
        }

        ui.add_space(20.0);
        ui.label("🔗 Полезные ссылки:");
        ui.add_space(10.0);

        ui.hyperlink_to("📖 Документация проекта", "https://github.com/winterice2/dns-manager");
        ui.hyperlink_to("🌐 Cloudflare DNS", "https://1.1.1.1/");
        ui.hyperlink_to("🔍 Google Public DNS", "https://dns.google/");
        ui.hyperlink_to("🔒 Quad9", "https://www.quad9.net/");
    }
}

