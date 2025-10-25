// Модуль для Toast уведомлений Windows
// Компилируется только на Windows

#[cfg(target_os = "windows")]
use winrt_notification::{Toast, Duration as ToastDuration};

pub struct NotificationManager {
    silent_mode: bool,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            silent_mode: false,
        }
    }

    pub fn set_silent_mode(&mut self, silent: bool) {
        self.silent_mode = silent;
    }

    pub fn is_silent(&self) -> bool {
        self.silent_mode
    }

    #[cfg(target_os = "windows")]
    pub fn send_dns_change_notification(&self, provider_name: &str, primary: &str, secondary: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.silent_mode {
            return Ok(()); // Тихий режим - не показываем уведомления
        }

        let message = if secondary.is_empty() {
            format!("Установлен: {} ({})", provider_name, primary)
        } else {
            format!("Установлен: {} ({}, {})", provider_name, primary, secondary)
        };

        Toast::new(Toast::POWERSHELL_APP_ID)
            .title("DNS изменен")
            .text1(&message)
            .duration(ToastDuration::Short)
            .show()?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn send_dns_change_notification(&self, _provider_name: &str, _primary: &str, _secondary: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Заглушка для не-Windows платформ
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn send_scheduler_notification(&self, provider_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.silent_mode {
            return Ok(());
        }

        Toast::new(Toast::POWERSHELL_APP_ID)
            .title("Планировщик DNS")
            .text1(&format!("Автопереключение на: {}", provider_name))
            .duration(ToastDuration::Short)
            .show()?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn send_scheduler_notification(&self, _provider_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn send_speed_test_complete_notification(&self, fastest_provider: &str, avg_ping: f64) -> Result<(), Box<dyn std::error::Error>> {
        if self.silent_mode {
            return Ok(());
        }

        Toast::new(Toast::POWERSHELL_APP_ID)
            .title("Тест скорости завершен")
            .text1(&format!("Быстрейший: {} ({:.1}ms)", fastest_provider, avg_ping))
            .duration(ToastDuration::Short)
            .show()?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn send_speed_test_complete_notification(&self, _fastest_provider: &str, _avg_ping: f64) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}
