// Модуль для работы с системным треем Windows
// Компилируется только на Windows

#[cfg(target_os = "windows")]
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIcon, TrayIconBuilder, Icon,
};

#[cfg(target_os = "windows")]
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Clone)]
pub enum TrayEvent {
    Show,
    Hide,
    Quit,
}

#[cfg(target_os = "windows")]
pub struct TrayManager {
    _tray_icon: TrayIcon,
    event_receiver: Receiver<TrayEvent>,
}

#[cfg(target_os = "windows")]
impl TrayManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Создаем простую иконку 16x16 (белый круг на прозрачном фоне)
        let icon_rgba = Self::create_simple_icon();
        let icon = Icon::from_rgba(icon_rgba, 16, 16)?;

        // Создаем меню
        let (event_sender, event_receiver) = channel();
        let tray_menu = Self::create_menu(event_sender)?;

        // Создаем tray icon
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("DNS Manager")
            .with_icon(icon)
            .with_menu(Box::new(tray_menu))
            .build()?;

        Ok(Self {
            _tray_icon: tray_icon,
            event_receiver,
        })
    }

    fn create_simple_icon() -> Vec<u8> {
        // Создаем простую иконку 16x16 (синий круг)
        let mut rgba = vec![0u8; 16 * 16 * 4];
        
        for y in 0..16 {
            for x in 0..16 {
                let idx = (y * 16 + x) * 4;
                let dx = x as f32 - 8.0;
                let dy = y as f32 - 8.0;
                let dist = (dx * dx + dy * dy).sqrt();
                
                if dist < 6.0 {
                    // Синий круг (RGB: 59, 130, 246)
                    rgba[idx] = 59;      // R
                    rgba[idx + 1] = 130; // G
                    rgba[idx + 2] = 246; // B
                    rgba[idx + 3] = 255; // A (полностью непрозрачный)
                } else {
                    // Прозрачный фон
                    rgba[idx + 3] = 0;
                }
            }
        }
        
        rgba
    }

    fn create_menu(event_sender: Sender<TrayEvent>) -> Result<Menu, Box<dyn std::error::Error>> {
        let menu = Menu::new();

        let show_item = MenuItem::new("Показать", true, None);
        let hide_item = MenuItem::new("Скрыть", true, None);
        let quit_item = MenuItem::new("Выход", true, None);

        // Клонируем sender для каждого события
        let show_sender = event_sender.clone();
        let hide_sender = event_sender.clone();
        let quit_sender = event_sender;

        // Захватываем ID элементов меню (thread-safe)
        let show_id = show_item.id().clone();
        let hide_id = hide_item.id().clone();
        let quit_id = quit_item.id().clone();

        // Обработчики событий меню
        MenuEvent::set_event_handler(Some(move |event: tray_icon::menu::MenuEvent| {
            if event.id == show_id {
                let _ = show_sender.send(TrayEvent::Show);
            } else if event.id == hide_id {
                let _ = hide_sender.send(TrayEvent::Hide);
            } else if event.id == quit_id {
                let _ = quit_sender.send(TrayEvent::Quit);
            }
        }));

        menu.append(&show_item)?;
        menu.append(&hide_item)?;
        menu.append(&quit_item)?;

        Ok(menu)
    }

    pub fn poll_events(&self) -> Vec<TrayEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_receiver.try_recv() {
            events.push(event);
        }
        events
    }
}

// Заглушка для не-Windows платформ
#[cfg(not(target_os = "windows"))]
pub struct TrayManager;

#[cfg(not(target_os = "windows"))]
impl TrayManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Err("System tray is only supported on Windows".into())
    }

    pub fn poll_events(&self) -> Vec<TrayEvent> {
        Vec::new()
    }
}
