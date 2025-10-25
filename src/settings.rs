// Модуль для сохранения и загрузки настроек приложения
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppSettings {
    // Системный трей
    pub tray_enabled: bool,
    pub window_visible: bool,

    // Уведомления
    pub silent_mode: bool,

    // Планировщик
    pub scheduler_enabled: bool,
    pub scheduler_interval: u32,

    // Темы
    pub theme_dark: bool,
    pub theme_custom_colors: bool,
    pub theme_primary: [u8; 3],
    pub theme_secondary: [u8; 3],
    pub theme_accent: [u8; 3],

    // Горячие клавиши
    pub hotkeys_enabled: bool,

    // Автозапуск
    pub auto_startup_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            tray_enabled: false,
            window_visible: true,
            silent_mode: false,
            scheduler_enabled: false,
            scheduler_interval: 60,
            theme_dark: true,
            theme_custom_colors: false,
            theme_primary: [147, 51, 234],
            theme_secondary: [59, 130, 246],
            theme_accent: [6, 182, 212],
            hotkeys_enabled: true,
            auto_startup_enabled: false,
        }
    }
}

impl AppSettings {
    fn get_settings_path() -> PathBuf {
        // Используем домашнюю директорию пользователя
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".dns-manager");
        
        // Создаем директорию если её нет
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
        
        path.push("settings.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::get_settings_path();
        
        if path.exists() {
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str::<AppSettings>(&contents) {
                    return settings;
                }
            }
        }
        
        // Возвращаем настройки по умолчанию если не удалось загрузить
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_settings_path();
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}
