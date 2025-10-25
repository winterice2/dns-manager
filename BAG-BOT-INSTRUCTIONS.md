# 🚀 BAG BOT AUTONOMOUS AGENT - ИНСТРУКЦИИ

## 🎯 МИССИЯ
Автономная оптимизация DNS Manager для background режима работы 24/7

## ⚙️ НАСТРОЙКИ АГЕНТА
- **Автономность:** 100% - работать без вмешательства
- **Цель:** Фоновая работа с минимальным потреблением ресурсов
- **Приоритет:** Производительность > Функциональность > Безопасность

## 📋 ЗАДАЧИ ПО ЭТАПАМ

### ЭТАП 1: System Tray Integration (2 часа)
```rust
// ДОБАВИТЬ В Cargo.toml
tray-icon = "0.14"
winit = "0.29"
winrt-notification = "0.5"

// ЦЕЛЬ:
- Tray иконка в системном трее
- Контекстное меню: Показать/Выход/Настройки
- CPU <1% в фоне
- RAM <20MB в фоне
```

### ЭТАП 2: Minimize to Tray (1 час)
```rust
// ДОБАВИТЬ В DNSManager struct
minimize_to_tray: bool,
tray_icon: Option<TrayIcon>,

// ЦЕЛЬ:
- Полное скрытие окна window.set_visible(false)
- Восстановление по клику на tray
- Сохранение состояния между запусками
```

### ЭТАП 3: Background Notifications (1.5 часа)
```rust
// Toast уведомления Windows
Toast::new(Toast::POWERSHELL_APP_ID)
    .title("DNS изменен")
    .text(&format!("Установлен: {}", provider_name))
    .show()?;

// ЦЕЛЬ:
- Уведомления при автоматической смене DNS
- Настройки "Тихий режим"
- Не беспокоить пользователя без необходимости
```

### ЭТАП 4: Performance Optimization (2 часа)
```rust
// Адаптивный пинг интервал
fn get_ping_interval(is_background: bool) -> Duration {
    if is_background {
        Duration::from_secs(2)  // Фон: 2 сек
    } else {
        Duration::from_millis(500)  // UI: 0.5 сек
    }
}

// ЦЕЛЬ:
- CPU <1% в фоне
- RAM <25MB постоянно
- Lazy UI updates (обновление только при взаимодействии)
```

### ЭТАП 5: Settings Persistence (1 час)
```rust
#[derive(Serialize, Deserialize)]
struct TraySettings {
    minimize_to_tray: bool,
    silent_mode: bool,
    ping_interval: u64,
}

// ЦЕЛЬ:
- Сохранение настроек в settings.json
- Восстановление состояния при запуске
- Миграция настроек между версиями
```

### ЭТАП 6: Auto-startup (1 час)
```rust
// Создать ярлык в Startup папке
let startup = env::var("APPDATA")? +
    "\\Microsoft\\Windows\\Start Menu\\Programs\\Startup";

// ЦЕЛЬ:
- Автозапуск при загрузке Windows
- Регистрация в автозагрузке
- Возможность отключения
```

## 🧪 ТЕСТИРОВАНИЕ

### Автоматические тесты:
```rust
#[test]
fn test_tray_icon_creation() { /* ... */ }
#[test]
fn test_minimize_to_tray() { /* ... */ }
#[test]
fn test_background_resource_usage() { /* ... */ }
```

### Ручное тестирование:
1. **Запуск:** `./target/release/dns-manager.exe`
2. **Minimize to Tray:** Кнопка "В трей"
3. **Tray Click:** Клик по иконке → показать меню
4. **Notifications:** Сменить DNS → проверить уведомление
5. **Performance:** Task Manager → CPU <1%, RAM <25MB
6. **Auto-startup:** Перезагрузка → приложение в трее

## 🚨 КРИТИЧЕСКИЕ УСЛОВИЯ

### ОСТАНОВИТЬ И ОТКАТИТЬ:
- ❌ Компиляция ломается
- ❌ Приложение не запускается
- ❌ CPU >5% в фоне
- ❌ RAM >50MB постоянно
- ❌ Тесты не проходят

### ПРОДОЛЖИТЬ ОПТИМИЗАЦИЮ:
- ✅ Все компилируется
- ✅ Все тесты проходят
- ✅ Performance в пределах нормы
- ✅ Функциональность работает

## 📊 МЕТРИКИ УСПЕХА

| Метрика | Цель | Текущее | Статус |
|---------|------|---------|--------|
| CPU Usage | <1% | - | ❌ |
| RAM Usage | <20MB | - | ❌ |
| Tray Icon | Работает | - | ❌ |
| Notifications | Работают | - | ❌ |
| Auto-startup | Работает | - | ❌ |
| Settings Save | Работает | - | ❌ |

## 🔄 АВТОНОМНЫЙ РЕЖИМ

### Алгоритм работы:
1. **Анализ:** Проверить текущий код на проблемы
2. **Планирование:** Выбрать следующий этап оптимизации
3. **Реализация:** Написать код для выбранного этапа
4. **Тестирование:** Запустить тесты, проверить метрики
5. **Коммит:** Сохранить изменения если всё работает
6. **Повтор:** Перейти к следующему этапу

### Логика принятия решений:
```rust
if compilation_fails() { rollback_changes() }
else if tests_fail() { fix_tests() }
else if performance_bad() { optimize_further() }
else { commit_and_continue() }
```

## 🎯 ФИНАЛЬНЫЙ РЕЗУЛЬТАТ

DNS Manager должен работать как полноценный background agent:
- ✅ Системный трей с иконкой
- ✅ Минимальное потребление ресурсов
- ✅ Автоматические уведомления
- ✅ Сохранение настроек
- ✅ Автозапуск при загрузке
- ✅ Полная автономность

## 🚀 ЗАПУСК АГЕНТА

```bash
# В ветке autonomous-agent
git checkout autonomous-agent
cargo build --release
./target/release/dns-manager.exe --background
```

**Агент готов к автономной работе!** 🔥
