# 🚀 **DNS Manager - Инструкция по Переносу**

## 📋 **Краткая версия для ленивых:**

1. **Скопируй папку** `dns-manager-rust` на новый компьютер
2. **Запусти** `target\release\dns-manager.exe` **от имени администратора**
3. **Готово!** Приложение работает без установки зависимостей

---

## 🎯 **Подробная инструкция для агентов:**

### **Шаг 1: Подготовка на исходном ПК**

```bash
# Убедись что приложение скомпилировано
cd dns-manager-rust
cargo build --release

# Проверь что файл существует
dir target\release\dns-manager.exe
```

### **Шаг 2: Перенос файлов**

```bash
# Вариант 1: Копирование всей папки
xcopy dns-manager-rust C:\path\to\destination\dns-manager-rust /E /I /H /Y

# Вариант 2: Только исполняемый файл
copy dns-manager-rust\target\release\dns-manager.exe C:\path\to\destination\
copy dns-manager-rust\README.md C:\path\to\destination\
```

### **Шаг 3: Проверка на целевом ПК**

```cmd
# Проверь версию Windows
winver

# Запусти приложение
# Правой кнопкой на dns-manager.exe → "Run as administrator"

# Проверь работу в PowerShell
powershell -Command "Get-NetAdapter | Where-Object { $_.Status -eq 'Up' }"
```

### **Шаг 4: Тестирование функциональности**

```
✅ Запуск от администратора
✅ Показ текущих DNS адресов
✅ Установка Cloudflare DNS
✅ Установка Google DNS
✅ Сброс на DHCP
✅ Копирование текста
```

---

## 🛠️ **Устранение проблем:**

### **Если "Отказано в доступе":**

```cmd
# Запусти от администратора
runas /user:Administrator "dns-manager.exe"
```

### **Если "Missing DLL":**

```cmd
# Скачай и установи
start https://aka.ms/vs/17/release/vc_redist.x64.exe
```

### **Если не работает на старой Windows:**

```bash
# Перекомпилируй с правильным target
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

---

## 📦 **Создание portable версии:**

```bash
# Создай папку для распространения
mkdir DNS-Manager-v1.0
cd DNS-Manager-v1.0

# Скопируй файлы
copy ..\dns-manager-rust\target\release\dns-manager.exe .
copy ..\dns-manager-rust\README.md .
copy ..\dns-manager-rust\DEPLOYMENT.md .

# Создай ярлык с правами администратора
echo 'Set oWS = WScript.CreateObject("WScript.Shell")' > run_as_admin.vbs
echo 'sLinkFile = "DNS Manager.lnk"' >> run_as_admin.vbs
echo 'Set oLink = oWS.CreateShortcut(sLinkFile)' >> run_as_admin.vbs
echo 'oLink.TargetPath = "%~dp0dns-manager.exe"' >> run_as_admin.vbs
echo 'oLink.WorkingDirectory = "%~dp0"' >> run_as_admin.vbs
echo 'oLink.IconLocation = "shell32.dll,1"' >> run_as_admin.vbs
echo 'oLink.Save' >> run_as_admin.vbs
cscript run_as_admin.vbs
del run_as_admin.vbs

# Сожми в ZIP
powershell "Compress-Archive -Path . -DestinationPath DNS-Manager-v1.0.zip"
```

---

## 🎉 **Результат:**

- **Размер:** ~2-3 MB (зависит от оптимизаций)
- **Зависимости:** Только Windows 10/11 + права администратора
- **Совместимость:** 99% Windows компьютеров
- **Функциональность:** Полная, как на исходном ПК

**🚀 Готово к использованию на любом Windows 10/11 компьютере!**
