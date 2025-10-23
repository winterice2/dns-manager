// Модуль для функций отрисовки вкладок

use egui::{Context, Ui};

use crate::DNSManager;

pub fn show_main_tab(app: &mut DNSManager, ui: &mut Ui, ctx: &Context) {
        ui.vertical_centered(|ui| {
            ui.heading("🚀 Основные операции");
        });
        ui.separator();

        // Satellite Control
        ui.label("🛰️ Спутниковый контроль:");
        ui.label("🔄 Интеллектуальное переключение между галактиками");

        if ui.add_sized([ui.available_width(), 40.0], egui::Button::new("🚀 Launch/Landing DNS")).clicked() {
            app.status = "🛸 Navigation system activated...".to_string();
            ctx.request_repaint();

            match crate::dns::providers::get_current_dns() {
                Ok(current) => {
                    if current.contains("1.1.1.1") || current.contains("8.8.8.8") || current.contains("9.9.9.9") || current.contains("208.67.222.222") || current.contains("94.140.14.14") {
                        // Reset to automatic
                        match crate::dns::providers::reset_dns() {
                            Ok(_) => app.status = "🔄 Returned to autopilot".to_string(),
                            Err(e) => app.status = format!("💥 System failure: {}", e),
                        }
                    } else {
                        app.status = "🌌 Coordinates received".to_string();
                    }
                }
                Err(e) => app.status = format!("💥 Hyperspace jump failed: {}", e),
            }
        }

        ui.add_space(10.0);

        if ui.add_sized([ui.available_width(), 40.0], egui::Button::new("🛰️ Space Scanning")).clicked() {
            match crate::dns::providers::get_current_dns() {
                Ok(dns) => app.status = format!("🛰️ Coordinates received: {}", dns),
                Err(e) => app.status = format!("💫 Cosmic noise: {}", e),
            }
            ctx.request_repaint();
        }

        ui.add_space(10.0);

        if ui.add_sized([ui.available_width(), 40.0], egui::Button::new("🔄 Reset to DHCP")).clicked() {
            match crate::dns::providers::reset_dns() {
                Ok(_) => app.status = "🔄 Returned to autopilot".to_string(),
                Err(e) => app.status = format!("💥 Engine failure: {}", e),
            }
            ctx.request_repaint();
        }

        ui.add_space(20.0);
        ui.label("📊 Текущий статус:");
        ui.add_space(5.0);
        let _ = ui.selectable_label(false, &app.status);
    }

    pub fn show_providers_tab(app: &mut DNSManager, ui: &mut Ui, ctx: &Context) {
        ui.vertical_centered(|ui| {
            ui.heading("🌍 Библиотека DNS провайдеров");
        });
        ui.separator();

        ui.label("⭐ Выберите DNS провайдер для космического путешествия:");
        ui.add_space(15.0);

        // Cloudflare
        if ui.add_sized([ui.available_width(), 45.0], egui::Button::new("☁️ Cloudflare DNS\n1.1.1.1, 1.0.0.1")).clicked() {
            match crate::dns::providers::set_dns("1.1.1.1", "1.0.0.1") {
                Ok(_) => app.status = "🎉 Arrived at Cloudflare: 1.1.1.1, 1.0.0.1!".to_string(),
                Err(e) => app.status = format!("💥 Ship crashed: {}", e),
            }
            ctx.request_repaint();
        }

        ui.add_space(8.0);

        // Google
        if ui.add_sized([ui.available_width(), 45.0], egui::Button::new("🔍 Google DNS\n8.8.8.8, 8.8.4.4")).clicked() {
            match crate::dns::providers::set_dns("8.8.8.8", "8.8.4.4") {
                Ok(_) => app.status = "🎉 Welcome to Google: 8.8.8.8, 8.8.4.4!".to_string(),
                Err(e) => app.status = format!("💥 System malfunction: {}", e),
            }
            ctx.request_repaint();
        }

        ui.add_space(8.0);

        // Quad9
        if ui.add_sized([ui.available_width(), 45.0], egui::Button::new("🔒 Quad9 DNS\n9.9.9.9, 149.112.112.112")).clicked() {
            match crate::dns::providers::set_dns("9.9.9.9", "149.112.112.112") {
                Ok(_) => app.status = "🎉 Secured with Quad9: 9.9.9.9, 149.112.112.112!".to_string(),
                Err(e) => app.status = format!("💥 Security breach: {}", e),
            }
            ctx.request_repaint();
        }

        ui.add_space(8.0);

        // OpenDNS
        if ui.add_sized([ui.available_width(), 45.0], egui::Button::new("👨‍👩‍👧‍👦 OpenDNS\n208.67.222.222, 208.67.220.220")).clicked() {
            match crate::dns::providers::set_dns("208.67.222.222", "208.67.220.220") {
                Ok(_) => app.status = "🎉 Family protection activated!".to_string(),
                Err(e) => app.status = format!("💥 Family emergency: {}", e),
            }
            ctx.request_repaint();
        }

        ui.add_space(8.0);

        // AdGuard
        if ui.add_sized([ui.available_width(), 45.0], egui::Button::new("🚫 AdGuard DNS\n94.140.14.14, 94.140.15.15")).clicked() {
            match crate::dns::providers::set_dns("94.140.14.14", "94.140.15.15") {
                Ok(_) => app.status = "🎉 Ads blocked: 94.140.14.14, 94.140.15.15!".to_string(),
                Err(e) => app.status = format!("💥 Ad blocking failure: {}", e),
            }
            ctx.request_repaint();
        }

        ui.add_space(8.0);

        // CleanBrowsing
        if ui.add_sized([ui.available_width(), 45.0], egui::Button::new("🧹 CleanBrowsing\n185.228.168.9, 185.228.169.9")).clicked() {
            match crate::dns::providers::set_dns("185.228.168.9", "185.228.169.9") {
                Ok(_) => app.status = "🎉 Clean browsing activated!".to_string(),
                Err(e) => app.status = format!("💥 Cleaning failure: {}", e),
            }
            ctx.request_repaint();
        }

        ui.add_space(20.0);
        ui.label("💡 Все изменения применяются ко всем активным сетевым адаптерам");
    }

    pub fn show_lab_tab(app: &mut DNSManager, ui: &mut Ui, ctx: &Context) {
        ui.vertical_centered(|ui| {
            ui.heading("🧪 Лаборатория тестирования");
        });
        ui.separator();

        ui.label("⚗️ Инструменты анализа и тестирования:");
        ui.add_space(10.0);

        // DNS Speed Test
        let button_text = if app.is_speed_testing {
            "⏳ Тестирование выполняется...".to_string()
        } else {
            "⚡ DNS Speed Test - Тестировать все провайдеры".to_string()
        };

        if ui.add_sized([ui.available_width(), 40.0], egui::Button::new(button_text)).clicked() && !app.is_speed_testing {
            app.start_speed_test();
            ctx.request_repaint();
        }

        ui.add_space(10.0);
        ui.label("🔬 Результаты тестирования:");

        // Speed Test Results
        if !app.speed_results.is_empty() {
            ui.add_space(10.0);
            ui.label("📊 Скорость DNS серверов (отсортировано по задержке):");

            ui.separator();

            for (index, result) in app.speed_results.iter().enumerate() {
                let medal = match index {
                    0 => "[1]".to_string(),
                    1 => "[2]".to_string(),
                    2 => "[3]".to_string(),
                    _ => format!("[{}]", index + 1),
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

    pub fn show_network_tab(app: &mut DNSManager, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("📡 Мониторинг сети");
        });
        ui.separator();

        ui.label("🔍 Детальная информация о сетевых адаптерах:");
        ui.add_space(10.0);

        if ui.button("🔄 Обновить информацию о сети").clicked() {
            app.network_adapters = crate::network::adapters::get_network_adapters();
            app.status = "✅ Информация о сети обновлена!".to_string();
        }

        ui.add_space(10.0);

        if app.network_adapters.is_empty() {
            ui.label("❌ Нет активных сетевых адаптеров");
        } else {
            for adapter in &app.network_adapters {
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
        ui.label("💡 Эта вкладка показывает все активные сетевые подключения");
        ui.label("🔄 Используйте кнопку обновления для получения актуальной информации");
    }

    pub fn show_stats_tab(app: &mut DNSManager, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("📊 Статистика проекта");
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
                ui.label(if app.speed_results.is_empty() { "Не выполнялось" } else { "Выполнено" });
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
        ui.label("🔗 Полезные ссылки:");
        ui.add_space(10.0);

        ui.hyperlink_to("📖 Документация проекта", "https://github.com/winterice2/dns-manager");
        ui.hyperlink_to("🌐 Cloudflare DNS", "https://1.1.1.1/");
        ui.hyperlink_to("🔍 Google Public DNS", "https://dns.google/");
        ui.hyperlink_to("🔒 Quad9", "https://www.quad9.net/");
    }
