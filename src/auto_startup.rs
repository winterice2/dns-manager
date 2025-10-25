// Модуль для управления автозагрузкой Windows
// Компилируется только на Windows

use std::path::PathBuf;
use std::env;

pub struct AutoStartupManager;

impl AutoStartupManager {
    #[cfg(target_os = "windows")]
    pub fn get_startup_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let appdata = env::var("APPDATA")?;
        let mut path = PathBuf::from(appdata);
        path.push("Microsoft");
        path.push("Windows");
        path.push("Start Menu");
        path.push("Programs");
        path.push("Startup");
        Ok(path)
    }

    #[cfg(not(target_os = "windows"))]
    pub fn get_startup_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Err("Auto-startup is only supported on Windows".into())
    }

    #[cfg(target_os = "windows")]
    pub fn is_enabled() -> bool {
        if let Ok(startup_path) = Self::get_startup_path() {
            let shortcut = startup_path.join("DNS Manager.lnk");
            shortcut.exists()
        } else {
            false
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn is_enabled() -> bool {
        false
    }

    #[cfg(target_os = "windows")]
    pub fn enable() -> Result<(), Box<dyn std::error::Error>> {
        let startup_path = Self::get_startup_path()?;
        let shortcut_path = startup_path.join("DNS Manager.lnk");

        // Получаем путь к текущему исполняемому файлу
        let exe_path = env::current_exe()?;

        // Создаем ярлык с помощью PowerShell
        let ps_script = format!(
            "$ws = New-Object -ComObject WScript.Shell; \
             $s = $ws.CreateShortcut('{}'); \
             $s.TargetPath = '{}'; \
             $s.WorkingDirectory = '{}'; \
             $s.Description = 'DNS Manager - Background Agent'; \
             $s.Save()",
            shortcut_path.display(),
            exe_path.display(),
            exe_path.parent().unwrap_or(&exe_path).display()
        );

        let output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg(&ps_script)
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!("Failed to create startup shortcut: {:?}", 
                String::from_utf8_lossy(&output.stderr)).into())
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn enable() -> Result<(), Box<dyn std::error::Error>> {
        Err("Auto-startup is only supported on Windows".into())
    }

    #[cfg(target_os = "windows")]
    pub fn disable() -> Result<(), Box<dyn std::error::Error>> {
        let startup_path = Self::get_startup_path()?;
        let shortcut_path = startup_path.join("DNS Manager.lnk");

        if shortcut_path.exists() {
            std::fs::remove_file(shortcut_path)?;
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn disable() -> Result<(), Box<dyn std::error::Error>> {
        Err("Auto-startup is only supported on Windows".into())
    }
}
