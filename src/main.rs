// Модули для организации кода
mod ui;
mod network;
mod dns;
mod error;
mod validation;
mod executor;

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
use dns::providers::DNSProvider;
use network::adapters::NetworkAdapter;
use executor::{AsyncExecutor, AsyncSpeedTestResult, SpeedTestState};

/// Main application state
/// ARCHITECTURE: Separated concerns - this struct only holds UI state
struct DNSManager {
    status: String,
    current_dns: String,
    speed_results: Vec<AsyncSpeedTestResult>,
    custom_primary: String,
    custom_secondary: String,
    selected_tab: usize,
    network_adapters: Vec<NetworkAdapter>,
    // PERFORMANCE: Async executor for non-blocking operations
    executor: AsyncExecutor,
}

impl Default for DNSManager {
    fn default() -> Self {
        Self {
            status: "🚀 Ready for space launch!".to_string(),
            current_dns: String::new(),
            speed_results: Vec::new(),
            custom_primary: String::new(),
            custom_secondary: String::new(),
            selected_tab: 0,
            network_adapters: Vec::new(),
            executor: AsyncExecutor::new(),
        }
    }
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
            network_adapters,
            ..Default::default()
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

    // PERFORMANCE: Speed test now runs in background thread via AsyncExecutor
    fn start_speed_test(&mut self) {
        self.status = "🧪 Запуск тестирования скорости DNS...".to_string();
        self.speed_results.clear();
        let providers = Self::get_dns_providers();
        self.executor.start_speed_test(providers);
    }

    // PERFORMANCE: Non-blocking check of speed test state
    fn update_speed_test_ui(&mut self) {
        match self.executor.get_speed_test_state() {
            SpeedTestState::Idle => {},
            SpeedTestState::Running { progress, total } => {
                self.status = format!("🧪 Тестирование... ({}/{})", progress, total);
            },
            SpeedTestState::Completed(results) => {
                self.speed_results = results;
                self.status = format!("✅ Тестирование завершено! Получено {} результатов.", self.speed_results.len());
                self.executor.reset_speed_test();
            },
            SpeedTestState::Failed(err) => {
                self.status = format!("❌ Ошибка тестирования: {}", err);
                self.executor.reset_speed_test();
            },
        }
    }

    // Helper to check if speed test is running
    fn is_speed_test_running(&self) -> bool {
        !matches!(self.executor.get_speed_test_state(), SpeedTestState::Idle)
    }

}

impl eframe::App for DNSManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // PERFORMANCE: Non-blocking update of speed test UI
        self.update_speed_test_ui();
        
        // PERFORMANCE: Only request repaint if speed test is running
        if !matches!(self.executor.get_speed_test_state(), SpeedTestState::Idle) {
            ctx.request_repaint();
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
