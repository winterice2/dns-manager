// Модули для организации кода
mod ui;
mod network;
mod dns;
mod tray;
mod notifications;
mod settings;
mod auto_startup;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([550.0, 650.0])
            .with_min_inner_size([450.0, 500.0])
            .with_title("DNS Manager - Расширенная Вселенная"),
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
use std::time::{Duration, Instant};

#[derive(Clone)]
struct DNSSchedule {
    primary: String,
    secondary: String,
    name: String,
}

#[derive(Clone)]
struct HistoryEntry {
    timestamp: String,
    action: String,
    dns_before: String,
    dns_after: String,
}

#[derive(Clone)]
struct CommandResult {
    timestamp: String,
    command: String,
    success: bool,
    result: String,
    error_message: Option<String>,
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
    speed_test_frame_counter: u32, // Счетчик кадров для управления скоростью тестирования
    custom_dns_name: String, // Название кастомного DNS сервера
    custom_dns_description: String, // Описание кастомного DNS сервера

    // Планировщик DNS
    scheduler_enabled: bool,
    scheduler_interval: u32, // в минутах
    scheduler_dns_list: Vec<DNSSchedule>,
    scheduler_current_index: usize,
    scheduler_last_switch: Option<Instant>,

    // История изменений
    history: Vec<HistoryEntry>,

    // Темы интерфейса
    theme_dark: bool,
    theme_custom_colors: bool,
    theme_primary: [u8; 3],
    theme_secondary: [u8; 3],
    theme_accent: [u8; 3],

    // Системный трей
    tray_enabled: bool,
    tray_manager: Option<tray::TrayManager>,
    window_visible: bool,
    is_background_mode: bool,

    // Уведомления
    notification_manager: notifications::NotificationManager,
    silent_mode: bool,

    // Горячие клавиши
    hotkeys_enabled: bool,

    // Автозапуск
    auto_startup_enabled: bool,

    // Для планировщика - временные поля ввода
    scheduler_new_name: String,
    scheduler_new_primary: String,
    scheduler_new_secondary: String,

    // Отслеживание результатов команд
    command_results: Vec<CommandResult>,
}

impl DNSManager {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Загружаем сохраненные настройки
        let saved_settings = settings::AppSettings::load();

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

        // Инициализируем планировщик с предустановленными DNS
        let scheduler_dns_list = vec![
            DNSSchedule {
                primary: "1.1.1.1".to_string(),
                secondary: "1.0.0.1".to_string(),
                name: "Cloudflare".to_string(),
            },
            DNSSchedule {
                primary: "8.8.8.8".to_string(),
                secondary: "8.8.4.4".to_string(),
                name: "Google".to_string(),
            },
            DNSSchedule {
                primary: "9.9.9.9".to_string(),
                secondary: "149.112.112.112".to_string(),
                name: "Quad9".to_string(),
            },
        ];

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
            custom_dns_name: String::new(),
            custom_dns_description: String::new(),

            // Планировщик DNS
            scheduler_enabled: saved_settings.scheduler_enabled,
            scheduler_interval: saved_settings.scheduler_interval,
            scheduler_dns_list,
            scheduler_current_index: 0,
            scheduler_last_switch: None,

            // История изменений
            history: Vec::new(),

            // Темы интерфейса
            theme_dark: saved_settings.theme_dark,
            theme_custom_colors: saved_settings.theme_custom_colors,
            theme_primary: saved_settings.theme_primary,
            theme_secondary: saved_settings.theme_secondary,
            theme_accent: saved_settings.theme_accent,

            // Системный трей
            tray_enabled: saved_settings.tray_enabled,
            tray_manager: None,
            window_visible: saved_settings.window_visible,
            is_background_mode: false,

            // Уведомления
            notification_manager: {
                let mut nm = notifications::NotificationManager::new();
                nm.set_silent_mode(saved_settings.silent_mode);
                nm
            },
            silent_mode: saved_settings.silent_mode,

        // Горячие клавиши
        hotkeys_enabled: saved_settings.hotkeys_enabled,

        // Автозапуск
        auto_startup_enabled: auto_startup::AutoStartupManager::is_enabled(),

        // Для планировщика - временные поля ввода
        scheduler_new_name: "Мой DNS".to_string(),
        scheduler_new_primary: String::new(),
        scheduler_new_secondary: String::new(),

        // Отслеживание результатов команд
        command_results: Vec::new(),

        }
    }



    // Вспомогательные функции-обертки для модулей
    fn get_current_dns() -> Result<String, String> {
        dns::providers::get_current_dns()
    }

    fn set_dns(&mut self, primary: &str, secondary: &str) -> Result<String, String> {
        let dns_before = dns::providers::get_current_dns().unwrap_or_else(|_| "Неизвестно".to_string());
        let result = dns::providers::set_dns(primary, secondary);

        if let Ok(ref _success_msg) = result {
            let dns_after = format!("{}, {}", primary, secondary);
            self.log_history("Ручная установка DNS", &dns_before, &dns_after);

            // Отправляем уведомление (если не в тихом режиме)
            let _ = self.notification_manager.send_dns_change_notification("DNS", primary, secondary);
        }

        result
    }

    fn reset_dns(&mut self) -> Result<String, String> {
        let dns_before = dns::providers::get_current_dns().unwrap_or_else(|_| "Неизвестно".to_string());
        let result = dns::providers::reset_dns();

        if let Ok(ref _success_msg) = result {
            let dns_after = "DHCP (автоматические)".to_string();
            self.log_history("Сброс на DHCP", &dns_before, &dns_after);
        }

        result
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

        // Адаптивный интервал тестирования в зависимости от режима
        let frame_divisor = if self.is_background_mode { 40 } else { 10 };

        // Выполняем тестирование только каждый N-й кадр, чтобы не блокировать UI
        self.speed_test_frame_counter += 1;
        if self.speed_test_frame_counter % frame_divisor != 0 {
            return false; // Пропускаем этот кадр
        }

        let providers = dns::providers::get_dns_providers();
        let current_count = self.speed_results.len();

        if current_count < providers.len() {
            // Тестируем следующий провайдер
            let provider = &providers[current_count];
            self.status = format!("🧪 Тестирование {}... ({}/{})", provider.name, current_count + 1, providers.len());

            // Выполняем тестирование
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

    // Планировщик DNS
    fn check_scheduler(&mut self) {
        if !self.scheduler_enabled || self.scheduler_dns_list.is_empty() {
            return;
        }

        let now = Instant::now();
        let should_switch = match self.scheduler_last_switch {
            Some(last) => now.duration_since(last) >= Duration::from_secs(self.scheduler_interval as u64 * 60),
            None => true, // Первый запуск
        };

        if should_switch {
            self.switch_to_next_scheduled_dns();
            self.scheduler_last_switch = Some(now);
        }
    }

    fn switch_to_next_scheduled_dns(&mut self) {
        if self.scheduler_dns_list.is_empty() {
            return;
        }

        let dns_before = dns::providers::get_current_dns().unwrap_or_else(|_| "Неизвестно".to_string());

        // Копируем данные перед вызовом mutable метода
        let primary = self.scheduler_dns_list[self.scheduler_current_index].primary.clone();
        let secondary = self.scheduler_dns_list[self.scheduler_current_index].secondary.clone();
        let name = self.scheduler_dns_list[self.scheduler_current_index].name.clone();

        let result = self.set_dns(&primary, &secondary);

        match result {
            Ok(_) => {
                let dns_after = format!("{} ({}, {})", name, primary, secondary);
                self.status = format!("🕒 Автопереключение: {}", dns_after);
                self.log_history("Автопереключение планировщика", &dns_before, &dns_after);

                // Уведомление планировщика
                let _ = self.notification_manager.send_scheduler_notification(&name);
            }
            Err(e) => {
                self.status = format!("❌ Ошибка автопереключения: {}", e);
            }
        }

        // Переходим к следующему DNS в списке
        self.scheduler_current_index = (self.scheduler_current_index + 1) % self.scheduler_dns_list.len();
    }

    // История изменений
    fn log_history(&mut self, action: &str, dns_before: &str, dns_after: &str) {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Форматируем время в читаемый вид
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
            .unwrap_or_else(|| chrono::DateTime::UNIX_EPOCH);

        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        let entry = HistoryEntry {
            timestamp: formatted_time,
            action: action.to_string(),
            dns_before: dns_before.to_string(),
            dns_after: dns_after.to_string(),
        };

        self.history.push(entry);

        // Ограничиваем историю 100 записями
        if self.history.len() > 100 {
            self.history.remove(0);
        }
    }

    // Темы интерфейса
    fn apply_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        style.visuals.dark_mode = self.theme_dark;

        if self.theme_custom_colors {
            style.visuals.window_fill = egui::Color32::from_rgb(
                self.theme_primary[0],
                self.theme_primary[1],
                self.theme_primary[2]
            );
            style.visuals.panel_fill = egui::Color32::from_rgb(
                self.theme_secondary[0],
                self.theme_secondary[1],
                self.theme_secondary[2]
            );
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(
                self.theme_primary[0],
                self.theme_primary[1],
                self.theme_primary[2]
            );
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(
                self.theme_secondary[0],
                self.theme_secondary[1],
                self.theme_secondary[2]
            );
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(
                self.theme_accent[0],
                self.theme_accent[1],
                self.theme_accent[2]
            );
        } else {
            // Стандартная космическая тема
            style.visuals.window_fill = egui::Color32::from_rgb(15, 23, 42);
            style.visuals.panel_fill = egui::Color32::from_rgb(30, 41, 59);
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(147, 51, 234);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(59, 130, 246);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(6, 182, 212);
        }

        ctx.set_style(style);
    }

    // Сохранение настроек
    fn save_settings(&self) {
        let settings = settings::AppSettings {
            tray_enabled: self.tray_enabled,
            window_visible: self.window_visible,
            silent_mode: self.silent_mode,
            scheduler_enabled: self.scheduler_enabled,
            scheduler_interval: self.scheduler_interval,
            theme_dark: self.theme_dark,
            theme_custom_colors: self.theme_custom_colors,
            theme_primary: self.theme_primary,
            theme_secondary: self.theme_secondary,
            theme_accent: self.theme_accent,
            hotkeys_enabled: self.hotkeys_enabled,
            auto_startup_enabled: self.auto_startup_enabled,
        };

        if let Err(e) = settings.save() {
            eprintln!("Failed to save settings: {}", e);
        }
    }

}

impl eframe::App for DNSManager {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Применяем текущую тему
        self.apply_theme(ctx);

        // Инициализируем tray manager при первом запуске (если включен)
        if self.tray_enabled && self.tray_manager.is_none() {
            match tray::TrayManager::new() {
                Ok(tray) => {
                    self.tray_manager = Some(tray);
                    self.status = "✅ Системный трей активирован".to_string();
                }
                Err(e) => {
                    self.status = format!("❌ Ошибка создания трея: {}", e);
                    self.tray_enabled = false;
                }
            }
        }

        // Обработка событий трея
        if let Some(ref tray_manager) = self.tray_manager {
            for event in tray_manager.poll_events() {
                match event {
                    tray::TrayEvent::Show => {
                        self.window_visible = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                        self.is_background_mode = false;
                    }
                    tray::TrayEvent::Hide => {
                        self.window_visible = false;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                        self.is_background_mode = true;
                    }
                    tray::TrayEvent::Quit => {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            }
        }

        // Проверяем планировщик DNS
        self.check_scheduler();

        // Обработка горячих клавиш
        if self.hotkeys_enabled {
            self.handle_hotkeys(ctx);
        }

        // Обновляем тестирование скорости, если оно активно
        if self.is_speed_testing {
            self.update_speed_test();
            ctx.request_repaint(); // Запрашиваем перерисовку для обновления UI
        }

        // Адаптивная частота обновления в зависимости от режима
        if self.is_background_mode {
            // В фоновом режиме обновляемся реже (каждые 2 секунды)
            ctx.request_repaint_after(std::time::Duration::from_secs(2));
        } else {
            // В активном режиме - каждые 0.5 секунды
            ctx.request_repaint_after(std::time::Duration::from_millis(500));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Создаем вкладки для организации интерфейса
            ui.horizontal(|ui| {
                ui.heading("🌌 DNS Manager - Расширенная Вселенная");
            });
            ui.separator();

            // Система вкладок с прокруткой
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0; // Уменьшаем расстояние между вкладками

                if ui.selectable_label(self.selected_tab == 0, "🌌 Главная").clicked() {
                    self.selected_tab = 0;
                }
                if ui.selectable_label(self.selected_tab == 1, "🌍 Провайдеры").clicked() {
                    self.selected_tab = 1;
                }
                if ui.selectable_label(self.selected_tab == 2, "🔧 Кастомные").clicked() {
                    self.selected_tab = 2;
                }
                if ui.selectable_label(self.selected_tab == 3, "🧪 Лаборатория").clicked() {
                    self.selected_tab = 3;
                }
                if ui.selectable_label(self.selected_tab == 4, "📡 Сеть").clicked() {
                    self.selected_tab = 4;
                }
                if ui.selectable_label(self.selected_tab == 5, "🕒 Планировщик").clicked() {
                    self.selected_tab = 5;
                }
                if ui.selectable_label(self.selected_tab == 6, "📅 История").clicked() {
                    self.selected_tab = 6;
                }
                if ui.selectable_label(self.selected_tab == 7, "⚙️ Настройки").clicked() {
                    self.selected_tab = 7;
                }
                if ui.selectable_label(self.selected_tab == 8, "🎨 Темы").clicked() {
                    self.selected_tab = 8;
                }
                if ui.selectable_label(self.selected_tab == 9, "📊 Статистика").clicked() {
                    self.selected_tab = 9;
                }
                if ui.selectable_label(self.selected_tab == 10, "🏓 Монитор").clicked() {
                    self.selected_tab = 10;
                }
            });

            ui.separator();

            match self.selected_tab {
                0 => self.show_main_tab(ui, ctx),
                1 => self.show_providers_tab(ui, ctx),
                2 => self.show_custom_tab(ui, ctx),
                3 => self.show_lab_tab(ui, ctx),
                4 => self.show_network_tab(ui),
                5 => self.show_scheduler_tab(ui),
                6 => self.show_history_tab(ui),
                7 => self.show_settings_tab(ui),
                8 => self.show_themes_tab(ui),
                9 => self.show_stats_tab(ui),
                10 => self.show_monitor_tab(ui),
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

    fn show_scheduler_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("🕒 Планировщик DNS");
        });
        ui.separator();

        ui.label("⏰ Автоматическое переключение между DNS серверами по расписанию");
        ui.add_space(15.0);

        // Включение/отключение планировщика
        ui.horizontal(|ui| {
            ui.label("🟢 Включить планировщик:");
            if ui.checkbox(&mut self.scheduler_enabled, "").changed() {
                if self.scheduler_enabled {
                    self.status = "🕒 Планировщик DNS включен".to_string();
                } else {
                    self.status = "⏸️ Планировщик DNS отключен".to_string();
                }
            }
        });

        ui.add_space(10.0);

        // Настройка интервала
        ui.horizontal(|ui| {
            ui.label("⏱️ Интервал переключения (минуты):");
            let mut interval = self.scheduler_interval as i32;
            if ui.add(egui::DragValue::new(&mut interval).clamp_range(5..=1440)).changed() {
                self.scheduler_interval = interval as u32;
            }
        });

        ui.add_space(20.0);

        // Список DNS для ротации
        ui.label("🔄 DNS серверы для ротации:");
        ui.add_space(10.0);

        let mut to_remove = None;
        for (i, schedule) in self.scheduler_dns_list.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("{}. {}: {}, {}", i + 1, schedule.name, schedule.primary, schedule.secondary));

                if ui.small_button("🗑️").on_hover_text("Удалить из списка").clicked() {
                    to_remove = Some(i);
                }
            });
        }

        // Удаляем элемент после итерации
        if let Some(index) = to_remove {
            if index < self.scheduler_dns_list.len() {
                self.scheduler_dns_list.remove(index);
            }
        }

        ui.add_space(15.0);

        // Добавление нового DNS в список
        ui.label("➕ Добавить DNS в ротацию:");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("🏷️ Название:");
            ui.text_edit_singleline(&mut self.scheduler_new_name);
        });

        ui.horizontal(|ui| {
            ui.label("🔵 Primary:");
            ui.text_edit_singleline(&mut self.scheduler_new_primary);
        });

        ui.horizontal(|ui| {
            ui.label("🔵 Secondary:");
            ui.text_edit_singleline(&mut self.scheduler_new_secondary);
        });

        ui.add_space(10.0);

        let can_add = !self.scheduler_new_name.is_empty() &&
                      dns::providers::validate_ip_address(&self.scheduler_new_primary) &&
                      (self.scheduler_new_secondary.is_empty() || dns::providers::validate_ip_address(&self.scheduler_new_secondary));

        if ui.button("➕ Добавить в список").clicked() && can_add {
            self.scheduler_dns_list.push(DNSSchedule {
                name: self.scheduler_new_name.clone(),
                primary: self.scheduler_new_primary.clone(),
                secondary: self.scheduler_new_secondary.clone(),
            });

            // Очищаем поля после добавления
            self.scheduler_new_name = "Мой DNS".to_string();
            self.scheduler_new_primary.clear();
            self.scheduler_new_secondary.clear();

            self.status = "✅ DNS добавлен в планировщик".to_string();
        }

        ui.add_space(20.0);

        // Информация о текущем состоянии
        ui.label("📊 Состояние планировщика:");
        ui.add_space(5.0);

        ui.label(format!("🔄 Статус: {}", if self.scheduler_enabled { "Включен" } else { "Отключен" }));
        ui.label(format!("⏱️ Интервал: {} минут", self.scheduler_interval));
        ui.label(format!("📋 DNS в списке: {}", self.scheduler_dns_list.len()));
        ui.label(format!("🎯 Текущий индекс: {}", self.scheduler_current_index + 1));

        if let Some(last_switch) = self.scheduler_last_switch {
            let elapsed = last_switch.elapsed().as_secs() / 60;
            ui.label(format!("🕐 Прошло с последнего переключения: {} мин", elapsed));
        } else {
            ui.label("🕐 Последнее переключение: никогда");
        }
    }

    fn show_history_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("📅 История изменений DNS");
        });
        ui.separator();

        ui.label("📜 Журнал всех изменений DNS настроек с временными метками");
        ui.add_space(15.0);

        if ui.button("🔄 Обновить историю").clicked() {
            // Просто для интерфейса
        }

        ui.add_space(10.0);

        if self.history.is_empty() {
            ui.label("📝 История пуста. Изменения DNS будут отображаться здесь.");
        } else {
            ui.label(format!("📊 Всего записей: {}", self.history.len()));

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, entry) in self.history.iter().enumerate().rev() { // Показываем с конца
                    ui.add_space(5.0);
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("🕐 {}", entry.timestamp));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                ui.small(format!("#{}", self.history.len() - i));
                            });
                        });

                        ui.label(format!("🎯 {}", entry.action));
                        ui.add_space(2.0);

                        ui.horizontal(|ui| {
                            ui.colored_label(egui::Color32::RED, "❌ До:");
                            ui.label(&entry.dns_before);
                        });

                        ui.horizontal(|ui| {
                            ui.colored_label(egui::Color32::GREEN, "✅ После:");
                            ui.label(&entry.dns_after);
                        });
                    });
                }
            });
        }

        ui.add_space(20.0);

        // Очистка истории
        if ui.button("🗑️ Очистить историю").clicked() {
            self.history.clear();
            self.status = "🗑️ История очищена".to_string();
        }

        ui.add_space(5.0);
        ui.small("💡 История ограничена 100 последними записями");
    }

    fn show_settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("⚙️ Настройки приложения");
        });
        ui.separator();

        ui.label("🔧 Основные параметры работы DNS Manager");
        ui.add_space(15.0);

        // Экспорт настроек
        ui.label("💾 Экспорт/Импорт настроек:");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            if ui.button("💾 Сохранить настройки").clicked() {
                self.save_settings();
                self.status = "💾 Настройки сохранены в ~/.dns-manager/settings.json".to_string();
            }

            if ui.button("📂 Показать файл настроек").clicked() {
                #[cfg(target_os = "windows")]
                {
                    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
                    let path = home.join(".dns-manager");
                    let _ = std::process::Command::new("explorer").arg(path).spawn();
                }
                self.status = "📂 Открыта папка настроек".to_string();
            }
        });

        ui.add_space(20.0);

        // Системный трей
        ui.label("📱 Интеграция с системой:");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("🔔 Системный трей:");
            ui.checkbox(&mut self.tray_enabled, "");
            ui.small(if self.tray_enabled { "✅ Включен" } else { "⏸️ Отключен" });
        });

        if self.tray_enabled {
            ui.add_space(5.0);
            ui.small("💡 Приложение будет доступно в системном трее");
            ui.small("   • Меню 'Показать': открыть окно");
            ui.small("   • Меню 'Скрыть': свернуть в трей");
            ui.small("   • Меню 'Выход': закрыть приложение");

            ui.add_space(10.0);

            // Кнопка для сворачивания в трей
            if ui.button("📥 Свернуть в трей").clicked() {
                self.window_visible = false;
                self.is_background_mode = true;
                self.status = "📥 Приложение свернуто в системный трей".to_string();
            }
        }

        ui.add_space(10.0);

        // Уведомления
        ui.horizontal(|ui| {
            ui.label("🔔 Уведомления:");
            if ui.checkbox(&mut self.silent_mode, "Тихий режим").changed() {
                self.notification_manager.set_silent_mode(self.silent_mode);
                if self.silent_mode {
                    self.status = "🔕 Тихий режим включен - уведомления отключены".to_string();
                } else {
                    self.status = "🔔 Уведомления включены".to_string();
                }
            }
        });

        if !self.silent_mode {
            ui.add_space(5.0);
            ui.small("💡 Вы будете получать уведомления при:");
            ui.small("   • Ручной смене DNS");
            ui.small("   • Автоматическом переключении планировщика");
            ui.small("   • Завершении теста скорости");
        }

        ui.add_space(10.0);

        // Автозапуск
        ui.horizontal(|ui| {
            ui.label("🚀 Автозапуск:");
            if ui.checkbox(&mut self.auto_startup_enabled, "Запускать при загрузке Windows").changed() {
                if self.auto_startup_enabled {
                    match auto_startup::AutoStartupManager::enable() {
                        Ok(_) => self.status = "✅ Автозапуск включен".to_string(),
                        Err(e) => {
                            self.status = format!("❌ Ошибка включения автозапуска: {}", e);
                            self.auto_startup_enabled = false;
                        }
                    }
                } else {
                    match auto_startup::AutoStartupManager::disable() {
                        Ok(_) => self.status = "⏸️ Автозапуск отключен".to_string(),
                        Err(e) => self.status = format!("❌ Ошибка отключения автозапуска: {}", e),
                    }
                }
                self.save_settings();
            }
        });

        if self.auto_startup_enabled {
            ui.add_space(5.0);
            ui.small("💡 DNS Manager будет запускаться при загрузке Windows");
            ui.small("   и сворачиваться в системный трей");
        }

        ui.add_space(10.0);

        // Горячие клавиши
        ui.horizontal(|ui| {
            ui.label("⌨️ Горячие клавиши:");
            if ui.checkbox(&mut self.hotkeys_enabled, "").changed() {
                if self.hotkeys_enabled {
                    self.status = "⌨️ Горячие клавиши включены".to_string();
                } else {
                    self.status = "🚫 Горячие клавиши отключены".to_string();
                }
            }
        });

        ui.add_space(20.0);

        // Информация о горячих клавишах
        ui.label("⌨️ Доступные горячие клавиши:");
        ui.add_space(5.0);

        ui.small("• Ctrl+1: Cloudflare DNS (1.1.1.1)");
        ui.small("• Ctrl+2: Google DNS (8.8.8.8)");
        ui.small("• Ctrl+3: Quad9 DNS (9.9.9.9)");
        ui.small("• Ctrl+4: OpenDNS (208.67.222.222)");
        ui.small("• Ctrl+5: AdGuard DNS (94.140.14.14)");
        ui.small("• Ctrl+6: CleanBrowsing (185.228.168.9)");
        ui.small("• Ctrl+0: Сброс на DHCP");
        ui.small("• F5: Обновить статус DNS");

        ui.add_space(20.0);

        // Сброс настроек
        ui.label("🔄 Сброс настроек:");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            if ui.button("⚠️ Сброс к defaults").clicked() {
                self.reset_to_defaults();
                self.status = "🔄 Настройки сброшены к значениям по умолчанию".to_string();
            }
        });

        ui.add_space(5.0);
        ui.small("⚠️ Это действие нельзя отменить!");
    }

    fn show_themes_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("🎨 Темы интерфейса");
        });
        ui.separator();

        ui.label("🖌️ Настройка внешнего вида DNS Manager");
        ui.add_space(15.0);

        // Переключение светлая/темная тема
        ui.horizontal(|ui| {
            ui.label("🌙 Темная тема:");
            if ui.checkbox(&mut self.theme_dark, "").changed() {
                self.status = format!("🎨 Тема изменена на {}", if self.theme_dark { "темную" } else { "светлую" });
            }
        });

        ui.add_space(15.0);

        // Предустановленные темы
        ui.label("🎭 Предустановленные темы:");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("🌌 Космос").clicked() {
                self.apply_preset_theme("cosmos");
                self.status = "🌌 Применена космическая тема".to_string();
            }

            if ui.button("🔮 Киберпанк").clicked() {
                self.apply_preset_theme("cyberpunk");
                self.status = "🔮 Применена кибер-панк тема".to_string();
            }

            if ui.button("🌸 Розовая").clicked() {
                self.apply_preset_theme("pink");
                self.status = "🌸 Применена розовая тема".to_string();
            }

            if ui.button("🌊 Океан").clicked() {
                self.apply_preset_theme("ocean");
                self.status = "🌊 Применена океаническая тема".to_string();
            }
        });

        ui.add_space(20.0);

        // Кастомные цвета
        ui.horizontal(|ui| {
            ui.label("🎨 Кастомные цвета:");
            if ui.checkbox(&mut self.theme_custom_colors, "").changed() {
                self.status = format!("🎨 Кастомные цвета {}", if self.theme_custom_colors { "включены" } else { "отключены" });
            }
        });

        if self.theme_custom_colors {
            ui.add_space(15.0);
            ui.label("🖍️ Настройка цветов:");

            // Primary color
            ui.horizontal(|ui| {
                ui.label("🎯 Primary:");
                ui.color_edit_button_srgb(&mut self.theme_primary);
                ui.label(format!("RGB({}, {}, {})", self.theme_primary[0], self.theme_primary[1], self.theme_primary[2]));
            });

            // Secondary color
            ui.horizontal(|ui| {
                ui.label("🎪 Secondary:");
                ui.color_edit_button_srgb(&mut self.theme_secondary);
                ui.label(format!("RGB({}, {}, {})", self.theme_secondary[0], self.theme_secondary[1], self.theme_secondary[2]));
            });

            // Accent color
            ui.horizontal(|ui| {
                ui.label("⚡ Accent:");
                ui.color_edit_button_srgb(&mut self.theme_accent);
                ui.label(format!("RGB({}, {}, {})", self.theme_accent[0], self.theme_accent[1], self.theme_accent[2]));
            });
        }

        ui.add_space(20.0);

        // Предварительный просмотр
        ui.label("👀 Предварительный просмотр:");
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("🎛️ Кнопка:");
            let _ = ui.button("Пример кнопки");

            ui.add_space(5.0);
            ui.label("📝 Текст:");
            ui.label("Это пример текста для предварительного просмотра темы.");

            ui.add_space(5.0);
            ui.label("📊 Прогресс:");
            let _ = ui.add(egui::ProgressBar::new(0.7).text("70%"));
        });
    }

    fn apply_preset_theme(&mut self, theme: &str) {
        self.theme_custom_colors = false; // Отключаем кастомные цвета

        match theme {
            "cosmos" => {
                self.theme_dark = true;
                // Космическая тема уже установлена по умолчанию
            }
            "cyberpunk" => {
                self.theme_dark = true;
                self.theme_custom_colors = true;
                self.theme_primary = [255, 0, 255]; // Магента
                self.theme_secondary = [0, 255, 255]; // Циан
                self.theme_accent = [255, 255, 0]; // Желтый
            }
            "pink" => {
                self.theme_dark = false;
                self.theme_custom_colors = true;
                self.theme_primary = [255, 192, 203]; // Розовый
                self.theme_secondary = [255, 218, 221]; // Светло-розовый
                self.theme_accent = [255, 105, 180]; // Ярко-розовый
            }
            "ocean" => {
                self.theme_dark = true;
                self.theme_custom_colors = true;
                self.theme_primary = [0, 191, 255]; // Глубокий небесно-голубой
                self.theme_secondary = [70, 130, 180]; // Стальной синий
                self.theme_accent = [0, 255, 255]; // Аква
            }
            _ => {}
        }
    }

    fn reset_to_defaults(&mut self) {
        self.scheduler_enabled = false;
        self.scheduler_interval = 60;
        self.scheduler_current_index = 0;
        self.scheduler_last_switch = None;

        self.theme_dark = true;
        self.theme_custom_colors = false;
        self.theme_primary = [147, 51, 234];
        self.theme_secondary = [59, 130, 246];
        self.theme_accent = [6, 182, 212];

        self.tray_enabled = false;
        self.hotkeys_enabled = true;

        // Очищаем историю, но оставляем
        // self.history.clear();
    }

    // Обработка горячих клавиш
    fn handle_hotkeys(&mut self, ctx: &egui::Context) {
        // Получаем информацию о нажатых клавишах
        ctx.input(|input| {
            // Проверяем комбинации Ctrl + цифры
            if input.modifiers.ctrl {
                // Ctrl+1: Cloudflare DNS
                if input.key_pressed(egui::Key::Num1) {
                    match self.set_dns("1.1.1.1", "1.0.0.1") {
                        Ok(_) => self.status = "🎉 Cloudflare DNS activated (Ctrl+1)!".to_string(),
                        Err(e) => self.status = format!("❌ Hotkey failed: {}", e),
                    }
                    ctx.request_repaint();
                }
                // Ctrl+2: Google DNS
                else if input.key_pressed(egui::Key::Num2) {
                    match self.set_dns("8.8.8.8", "8.8.4.4") {
                        Ok(_) => self.status = "🎉 Google DNS activated (Ctrl+2)!".to_string(),
                        Err(e) => self.status = format!("❌ Hotkey failed: {}", e),
                    }
                    ctx.request_repaint();
                }
                // Ctrl+3: Quad9 DNS
                else if input.key_pressed(egui::Key::Num3) {
                    match self.set_dns("9.9.9.9", "149.112.112.112") {
                        Ok(_) => self.status = "🎉 Quad9 DNS activated (Ctrl+3)!".to_string(),
                        Err(e) => self.status = format!("❌ Hotkey failed: {}", e),
                    }
                    ctx.request_repaint();
                }
                // Ctrl+4: OpenDNS
                else if input.key_pressed(egui::Key::Num4) {
                    match self.set_dns("208.67.222.222", "208.67.220.220") {
                        Ok(_) => self.status = "🎉 OpenDNS activated (Ctrl+4)!".to_string(),
                        Err(e) => self.status = format!("❌ Hotkey failed: {}", e),
                    }
                    ctx.request_repaint();
                }
                // Ctrl+5: AdGuard DNS
                else if input.key_pressed(egui::Key::Num5) {
                    match self.set_dns("94.140.14.14", "94.140.15.15") {
                        Ok(_) => self.status = "🎉 AdGuard DNS activated (Ctrl+5)!".to_string(),
                        Err(e) => self.status = format!("❌ Hotkey failed: {}", e),
                    }
                    ctx.request_repaint();
                }
                // Ctrl+6: CleanBrowsing
                else if input.key_pressed(egui::Key::Num6) {
                    match self.set_dns("185.228.168.9", "185.228.169.9") {
                        Ok(_) => self.status = "🎉 CleanBrowsing activated (Ctrl+6)!".to_string(),
                        Err(e) => self.status = format!("❌ Hotkey failed: {}", e),
                    }
                    ctx.request_repaint();
                }
                // Ctrl+0: Сброс на DHCP
                else if input.key_pressed(egui::Key::Num0) {
                    match self.reset_dns() {
                        Ok(_) => self.status = "🔄 DHCP reset activated (Ctrl+0)!".to_string(),
                        Err(e) => self.status = format!("❌ Hotkey failed: {}", e),
                    }
                    ctx.request_repaint();
                }
            }
            // F5: Обновить статус
            else if input.key_pressed(egui::Key::F5) {
                match dns::providers::get_current_dns() {
                    Ok(dns) => self.status = format!("🔄 Status updated: {}", dns),
                    Err(e) => self.status = format!("❌ Status update failed: {}", e),
                }
                ctx.request_repaint();
            }
        });
    }

    fn show_custom_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.heading("🔧 Кастомные DNS серверы");
        });
        ui.separator();

        ui.label("⭐ Создайте свой собственный DNS профиль:");
        ui.add_space(15.0);

        // Название DNS профиля
        ui.horizontal(|ui| {
            ui.label("🏷️ Название:");
            ui.text_edit_singleline(&mut self.custom_dns_name);
        });

        // Описание
        ui.horizontal(|ui| {
            ui.label("📝 Описание:");
            ui.text_edit_singleline(&mut self.custom_dns_description);
        });

                ui.add_space(10.0);

        // Первичный DNS
        ui.horizontal(|ui| {
            ui.label("🔵 Первичный DNS:");
            ui.text_edit_singleline(&mut self.custom_primary);
        });

        // Вторичный DNS
        ui.horizontal(|ui| {
            ui.label("🔵 Вторичный DNS:");
            ui.text_edit_singleline(&mut self.custom_secondary);
        });

                ui.add_space(20.0);

        // Валидация и применение
        let primary_valid = !self.custom_primary.is_empty() && dns::providers::validate_ip_address(&self.custom_primary);
        let secondary_valid = self.custom_secondary.is_empty() ||
                             dns::providers::validate_ip_address(&self.custom_secondary);

        let can_apply = primary_valid && secondary_valid && !self.custom_dns_name.is_empty();

        // Статус валидации
        if !self.custom_primary.is_empty() && !primary_valid {
            ui.colored_label(egui::Color32::RED, "❌ Первичный DNS: неверный формат IP адреса");
        }
        if !self.custom_secondary.is_empty() && !secondary_valid {
            ui.colored_label(egui::Color32::RED, "❌ Вторичный DNS: неверный формат IP адреса");
        }

        ui.add_space(10.0);

        // Кнопка применения
        if ui.add_sized([ui.available_width(), 45.0],
            egui::Button::new("🚀 Применить кастомный DNS")).clicked() && can_apply {

            // Копируем значения перед вызовом mutable метода
            let primary = self.custom_primary.clone();
            let secondary = self.custom_secondary.clone();
            let name = self.custom_dns_name.clone();

            match self.set_dns(&primary, &secondary) {
                Ok(_) => {
                    self.status = format!("🎉 Кастомный DNS '{}' применен: {} {}",
                                         name,
                                         primary,
                                         if secondary.is_empty() { "".to_string() } else { format!(", {}", secondary) });
                }
                Err(e) => {
                    self.status = format!("💥 Ошибка применения кастомного DNS: {}", e);
                }
            }
            ctx.request_repaint();
        }

        // Предварительный просмотр
        ui.add_space(20.0);
        ui.label("👀 Предварительный просмотр:");
        ui.add_space(5.0);

        if !self.custom_dns_name.is_empty() {
            ui.label(format!("🏷️ **{}**", self.custom_dns_name));
            if !self.custom_dns_description.is_empty() {
                ui.label(format!("📝 {}", self.custom_dns_description));
            }
            ui.label(format!("🔵 Первичный: {}", self.custom_primary));
            if !self.custom_secondary.is_empty() {
                ui.label(format!("🔵 Вторичный: {}", self.custom_secondary));
            }
                } else {
            ui.label("💡 Заполните поля выше для просмотра");
        }

        ui.add_space(20.0);
        ui.label("💡 IP адреса должны быть в формате: xxx.xxx.xxx.xxx (например: 8.8.8.8)");
        ui.label("💡 Вторичный DNS можно оставить пустым");
    }

    // Монитор пинга
    fn show_monitor_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("🏓 Монитор сети");
        });
        ui.separator();

        ui.label("📊 Реал-тайм мониторинг сетевых параметров");
        ui.add_space(15.0);

        // Основная информация о пинге
        ui.label("🏠 **Пинг до роутера:**");
        ui.add_space(10.0);

        // Измеряем пинг в реальном времени
        let current_ping = self.measure_current_ping();

        // Отображаем пинг с цветовой индикацией
        let (ping_text, ping_color) = match current_ping {
            Some(p) if p < 30.0 => (format!("🟢 {:.0}ms (отлично)", p), egui::Color32::GREEN),
            Some(p) if p < 80.0 => (format!("🟡 {:.0}ms (нормально)", p), egui::Color32::YELLOW),
            Some(p) => (format!("🔴 {:.0}ms (плохо)", p), egui::Color32::RED),
            None => ("⚪ Нет данных".to_string(), egui::Color32::GRAY),
        };

        ui.colored_label(ping_color, ping_text);

        ui.add_space(20.0);

        // Легенда
        ui.label("📋 **Легенда:**");
        ui.add_space(5.0);
        ui.small("• 🟢 Зеленый: < 30ms (отлично)");
        ui.small("• 🟡 Желтый: 30-80ms (нормально)");
        ui.small("• 🔴 Красный: > 80ms (плохо)");
        ui.small("• ⚪ Серый: нет данных/ошибка");

        ui.add_space(15.0);

        // Информация об обновлении
        ui.label("🔄 **Обновление:** каждые 0.5 секунды");
        ui.add_space(10.0);

        // Кнопка принудительного обновления
        if ui.button("🔄 Обновить сейчас").clicked() {
            // Принудительное измерение
            let _ = self.measure_current_ping();
        }

        ui.add_space(20.0);

        // Дополнительная информация
        ui.collapsing("ℹ️ Дополнительная информация", |ui| {
            ui.label("🌐 **Что измеряется:**");
            ui.small("• Пинг до шлюза (роутера) вашей сети");
            ui.small("• Используется первый доступный сетевой адаптер");
            ui.small("• Измерения производятся локально, без нагрузки на интернет");

            ui.add_space(10.0);

            ui.label("💡 **Для чего это нужно:**");
            ui.small("• Контроль качества интернет-соединения");
            ui.small("• Диагностика сетевых проблем");
            ui.small("• Мониторинг производительности сети");
            ui.small("• Быстрое обнаружение проблем с подключением");
        });
    }

    fn measure_current_ping(&mut self) -> Option<f64> {
        // Не измеряем пинг в фоновом режиме для экономии ресурсов
        if self.is_background_mode {
            return None;
        }

        // Измеряем пинг до первого доступного адаптера
        for adapter in &self.network_adapters {
            if !adapter.gateway.is_empty() {
                let command = format!("ping {}", adapter.gateway);
                if let Some(ping) = dns::providers::ping_dns_server(&adapter.gateway) {
                    // Логируем успешный результат
                    self.log_command_result(
                        &command,
                        true,
                        &format!("{:.1}ms", ping),
                        None
                    );
                    return Some(ping);
                }
            }
        }

        // Если не удалось измерить ни для одного адаптера
        self.log_command_result(
            "ping_gateway",
            false,
            "no_adapters",
            Some("Нет доступных адаптеров с шлюзом")
        );

        None
    }

    fn check_app_status(&self) -> bool {
        // Проверяем, запущено ли приложение
        // В будущем можно добавить более сложную логику
        true // Пока просто возвращаем true, приложение работает
    }

    fn log_command_result(&mut self, command: &str, success: bool, result: &str, error: Option<&str>) {
        use chrono::Local;

        let entry = CommandResult {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            command: command.to_string(),
            success,
            result: result.to_string(),
            error_message: error.map(|e| e.to_string()),
        };

        self.command_results.push(entry);

        // Ограничиваем историю результатов (последние 50)
        if self.command_results.len() > 50 {
            self.command_results.remove(0);
        }

        // Логируем в консоль только в debug режиме
        #[cfg(debug_assertions)]
        {
            let status = if success { "✅ SUCCESS" } else { "❌ FAILED" };
            println!("{} Command: {} | Result: {}", status, command, result);

            if let Some(err) = error {
                println!("   Error: {}", err);
            }
        }
    }

    fn get_last_command_status(&self) -> Option<&CommandResult> {
        self.command_results.last()
    }
}
