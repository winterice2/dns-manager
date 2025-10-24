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
                        match app.reset_dns() {
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
            match app.reset_dns() {
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
        ui.add_space(10.0);

        // Прокручиваемая область для провайдеров
        egui::ScrollArea::vertical().show(ui, |ui| {
            let providers = crate::dns::providers::get_dns_providers();

            for provider in providers {
                ui.add_space(5.0);

                let button_text = format!("{}\n{}, {}", provider.name, provider.primary, provider.secondary);
                let emoji = match provider.name.as_str() {
                    "Cloudflare" => "☁️",
                    "Google" => "🔍",
                    "Quad9" => "🔒",
                    "OpenDNS" => "👨‍👩‍👧‍👦",
                    "AdGuard" => "🚫",
                    "CleanBrowsing" => "🧹",
                    "Comodo" => "🔐",
                    "Yandex" => "🇷🇺",
                    "DNS.WATCH" => "👁️",
                    "UncensoredDNS" => "🆓",
                    "Freenom" => "💰",
                    "Level3" => "🏢",
                    _ => "🌐",
                };

                let full_button_text = format!("{} {}", emoji, button_text);

                if ui.add_sized([ui.available_width(), 50.0], egui::Button::new(full_button_text)).clicked() {
                    match app.set_dns(&provider.primary, &provider.secondary) {
                        Ok(_) => {
                            let success_msg = match provider.name.as_str() {
                                "Cloudflare" => "🎉 Arrived at Cloudflare!",
                                "Google" => "🎉 Welcome to Google!",
                                "Quad9" => "🎉 Secured with Quad9!",
                                "OpenDNS" => "🎉 Family protection activated!",
                                "AdGuard" => "🎉 Ads blocked!",
                                "CleanBrowsing" => "🎉 Clean browsing activated!",
                                "Comodo" => "🎉 Secured with Comodo!",
                                "Yandex" => "🎉 Welcome to Yandex!",
                                "DNS.WATCH" => "🎉 DNS.WATCH activated!",
                                "UncensoredDNS" => "🎉 UncensoredDNS activated!",
                                "Freenom" => "🎉 Freenom DNS activated!",
                                "Level3" => "🎉 Level3 DNS activated!",
                                _ => "🎉 DNS changed successfully!",
                            };
                            app.status = format!("{} {}, {}", success_msg, provider.primary, provider.secondary);
                        }
                        Err(e) => {
                            let error_emoji = match provider.name.as_str() {
                                "Cloudflare" => "💥 Ship crashed",
                                "Google" => "💥 System malfunction",
                                "Quad9" => "💥 Security breach",
                                "OpenDNS" => "💥 Family emergency",
                                "AdGuard" => "💥 Ad blocking failure",
                                "CleanBrowsing" => "💥 Cleaning failure",
                                _ => "💥 DNS change failed",
                            };
                            app.status = format!("{}: {}", error_emoji, e);
                        }
                    }
                    ctx.request_repaint();
                }

                // Показываем описание при наведении
                ui.small(&provider.description);
            }
        });

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

                // Заголовок с статусом подключения
                let status_emoji = if adapter.is_online { "🟢" } else { "🔴" };
                let type_emoji = match adapter.connection_type.as_str() {
                    "WiFi" => "📶",
                    "Ethernet" => "🔌",
                    _ => "🌐",
                };

                ui.horizontal(|ui| {
                    ui.label(format!("{} {} **{}**", status_emoji, type_emoji, adapter.name));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(&adapter.connection_speed);
                    });
                });

                ui.add_space(5.0);

                // Основная информация
                ui.group(|ui| {
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
                        ui.label("🚪 Шлюз:");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(&adapter.gateway);
                        });
                    });

                    if let Some(ping) = adapter.ping_to_gateway {
                        ui.horizontal(|ui| {
                            ui.label("⚡ Пинг до шлюза:");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(format!("{:.1}ms", ping));
                            });
                        });
                    }

                    ui.horizontal(|ui| {
                        ui.label("🔧 DNS серверы:");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(adapter.dns_servers.join(", "));
                        });
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
                ui.label("v1.5.0 - Полный Функционал");
            });
        });

        ui.horizontal(|ui| {
            ui.label("🔧 Провайдеров DNS:");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("12 доступных");
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
        ui.label("🔐 **Comodo (8.26.56.26)**: Безопасный DNS с фильтрацией");
        ui.label("🇷🇺 **Yandex (77.88.8.8)**: DNS от Яндекса для русскоязычных");
        ui.label("👁️ **DNS.WATCH (84.200.69.80)**: Независимый DNS без логирования");
        ui.label("🆓 **UncensoredDNS (91.239.100.100)**: DNS без цензуры");
        ui.label("💰 **Freenom (80.80.80.80)**: Бесплатный DNS от Freenom");
        ui.label("🏢 **Level3 (209.244.0.3)**: DNS от Level 3 Communications");

        ui.add_space(20.0);
        ui.label("⌚ Горячие клавиши:");
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("💡 **Горячие клавиши для быстрого управления DNS:**");
            ui.add_space(5.0);

            ui.small("• **Ctrl+1**: Cloudflare DNS (1.1.1.1)");
            ui.small("• **Ctrl+2**: Google DNS (8.8.8.8)");
            ui.small("• **Ctrl+3**: Quad9 DNS (9.9.9.9)");
            ui.small("• **Ctrl+4**: OpenDNS (208.67.222.222)");
            ui.small("• **Ctrl+5**: AdGuard DNS (94.140.14.14)");
            ui.small("• **Ctrl+6**: CleanBrowsing (185.228.168.9)");
            ui.small("• **Ctrl+0**: Сброс на DHCP");
            ui.small("• **F5**: Обновить статус DNS");
        });

        ui.add_space(20.0);
        ui.label("🔗 Полезные ссылки:");
        ui.add_space(10.0);

        ui.hyperlink_to("📖 Документация проекта", "https://github.com/winterice2/dns-manager");
        ui.hyperlink_to("🌐 Cloudflare DNS", "https://1.1.1.1/");
        ui.hyperlink_to("🔍 Google Public DNS", "https://dns.google/");
        ui.hyperlink_to("🔒 Quad9", "https://www.quad9.net/");
        ui.hyperlink_to("👨‍👩‍👧‍👦 OpenDNS", "https://www.opendns.com/");
        ui.hyperlink_to("🚫 AdGuard DNS", "https://adguard-dns.io/");
        ui.hyperlink_to("🧹 CleanBrowsing", "https://cleanbrowsing.org/");
        ui.hyperlink_to("🇷🇺 Yandex DNS", "https://dns.yandex.ru/");
    }
