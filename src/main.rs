// Модули для организации кода
mod ui;
mod network;
mod dns;

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

// Импортируем типы из модулей
use dns::providers::{DNSProvider, SpeedTestResult};
use network::adapters::NetworkAdapter;

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
    speed_test_frame_counter: u32, // Счетчик кадров для управления скоростью тестирования
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
        let network_adapters = network::adapters::get_network_adapters();

                Self {
                    status: "🚀 Ready for space launch!".to_string(),
                    current_dns: String::new(),
                    speed_results: Vec::new(),
                    custom_primary: String::new(),
                    custom_secondary: String::new(),
                    selected_tab: 0,
                    is_speed_testing: false,
                    network_adapters,
                    speed_test_frame_counter: 0,
                }
    }



    // Вспомогательные функции-обертки для модулей
    fn get_current_dns() -> Result<String, String> {
        dns::providers::get_current_dns()
    }

    fn set_dns(primary: &str, secondary: &str) -> Result<String, String> {
        dns::providers::set_dns(primary, secondary)
    }

    fn reset_dns() -> Result<String, String> {
        dns::providers::reset_dns()
    }

    fn ping_dns_server(ip: &str) -> Option<f64> {
        dns::providers::ping_dns_server(ip)
    }

    fn get_dns_providers() -> Vec<DNSProvider> {
        dns::providers::get_dns_providers()
    }

    fn get_network_adapters() -> Vec<NetworkAdapter> {
        network::adapters::get_network_adapters()
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

        // Выполняем тестирование только каждый 10-й кадр, чтобы не блокировать UI
        self.speed_test_frame_counter += 1;
        if self.speed_test_frame_counter % 10 != 0 {
            return false; // Пропускаем этот кадр
        }

        let providers = dns::providers::get_dns_providers();
        let current_count = self.speed_results.len();

        if current_count < providers.len() {
            // Тестируем следующий провайдер
            let provider = &providers[current_count];
            self.status = format!("🧪 Тестирование {}... ({}/{})", provider.name, current_count + 1, providers.len());

            // Выполняем тестирование (теперь только каждый 10-й кадр)
            let primary_ping = dns::providers::ping_dns_server(&provider.primary);
            let secondary_ping = dns::providers::ping_dns_server(&provider.secondary);

            let mut result = dns::providers::SpeedTestResult {
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
            self.speed_test_frame_counter = 0; // Сбрасываем счетчик

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
                if ui.selectable_label(self.selected_tab == 3, "📡 Сеть").clicked() {
                    self.selected_tab = 3;
                }
                if ui.selectable_label(self.selected_tab == 4, "📊 Статистика").clicked() {
                    self.selected_tab = 4;
                }
            });

            ui.separator();

            match self.selected_tab {
                0 => self.show_main_tab(ui, ctx),
                1 => self.show_providers_tab(ui, ctx),
                2 => self.show_lab_tab(ui, ctx),
                3 => self.show_network_tab(ui),
                4 => self.show_stats_tab(ui),
                _ => self.show_main_tab(ui, ctx),
            }
        });
    }
}

impl DNSManager {
    // Обертки для функций из модулей UI
    fn show_main_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui::tabs::show_main_tab(self, ui, ctx);
    }

    fn show_providers_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui::tabs::show_providers_tab(self, ui, ctx);
    }

    fn show_lab_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui::tabs::show_lab_tab(self, ui, ctx);
    }

    fn show_network_tab(&mut self, ui: &mut egui::Ui) {
        ui::tabs::show_network_tab(self, ui);
    }

    fn show_stats_tab(&mut self, ui: &mut egui::Ui) {
        ui::tabs::show_stats_tab(self, ui);
    }
}
