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

#[derive(Default)]
struct DNSManager {
    status: String,
    current_dns: String,
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

        Self {
            status: "🚀 Ready for space launch!".to_string(),
            current_dns: String::new(),
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
                    let addresses = dns_servers.join(", ");
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Заголовок - Space Mission
                ui.add_space(10.0);
                ui.heading("🚀 DNS Manager - Andromeda Mission");
                ui.add_space(5.0);
                ui.label("🌌 DNS server management in deep space");
                ui.add_space(20.0);

                // Кнопки
                let button_width = 350.0;
                let button_height = 35.0;

                // 🛰️ Satellite Control
                ui.label("🛰️ Satellite control:");
                ui.label("🔄 Intelligent switching between galaxies");
                if ui.add_sized([button_width, button_height], egui::Button::new("🚀 Launch/Landing DNS")).clicked() {
                    self.status = "🛸 Navigation system activated...".to_string();
                    ctx.request_repaint();

                    match Self::get_current_dns() {
                        Ok(current) => {
                            if current.contains("1.1.1.1") || current.contains("8.8.8.8") {
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
                ui.label("🪐 Known DNS planets:");

                // Кнопка 2: Cloudflare - Cloud Planet
                if ui.add_sized([button_width, button_height], egui::Button::new("☁️ Cloudflare - Cloud Planet (1.1.1.1)")).clicked() {
                    self.status = "🛸 Entering Cloudflare orbit: 1.1.1.1, 1.0.0.1...".to_string();
                    ctx.request_repaint();

                    match Self::set_dns("1.1.1.1", "1.0.0.1") {
                        Ok(_) => self.status = "🎉 Arrived at Cloudflare: 1.1.1.1, 1.0.0.1! Mission accomplished!".to_string(),
                        Err(e) => self.status = format!("💥 Ship crashed: {}", e),
                    }
                }

                ui.add_space(10.0);

                // Кнопка 3: Google - Search Planet
                if ui.add_sized([button_width, button_height], egui::Button::new("🔍 Google - Search Planet (8.8.8.8)")).clicked() {
                    self.status = "🛸 Activating Google hyperdrive: 8.8.8.8, 8.8.4.4...".to_string();
                    ctx.request_repaint();

                    match Self::set_dns("8.8.8.8", "8.8.4.4") {
                        Ok(_) => self.status = "🎉 Welcome to Google: 8.8.8.8, 8.8.4.4! Data found!".to_string(),
                        Err(e) => self.status = format!("💥 Hyperspace jump failed: {}", e),
                    }
                }

                ui.add_space(15.0);
                ui.label("🌠 Space telemetry:");

                // Кнопка 4: Future Missions
                ui.add_sized([button_width, button_height], egui::Button::new("🛸 Future Missions").sense(egui::Sense::hover()));

                ui.add_space(20.0);

                // Кнопка обновления статуса - Space Scanning
                if ui.add_sized([button_width, button_height], egui::Button::new("🔭 Space Scanning")).clicked() {
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

                ui.add_space(15.0);

                // Отображение текущего DNS - Space Navigation
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
            });
        });
    }
}