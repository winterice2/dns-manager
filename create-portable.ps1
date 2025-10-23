# DNS Manager - Создание Portable версии
# Запуск: .\create-portable.ps1

Write-Host "🚀 Создание portable версии DNS Manager..." -ForegroundColor Green

# Проверь что приложение скомпилировано
$exePath = ".\target\release\dns-manager.exe"
if (!(Test-Path $exePath)) {
    Write-Host "❌ Ошибка: Приложение не скомпилировано!" -ForegroundColor Red
    Write-Host "   Запусти: cargo build --release" -ForegroundColor Yellow
    exit 1
}

# Создай папку для portable версии
$portableDir = "DNS-Manager-Portable"
if (Test-Path $portableDir) {
    Remove-Item $portableDir -Recurse -Force
}
New-Item -ItemType Directory -Path $portableDir | Out-Null

# Скопируй файлы
Write-Host "📁 Копирование файлов..." -ForegroundColor Cyan
Copy-Item $exePath $portableDir\
Copy-Item "README.md" $portableDir\
Copy-Item "DEPLOYMENT.md" $portableDir\
Copy-Item "memory.md" $portableDir\

# Создай ярлык с правами администратора
Write-Host "🔗 Создание ярлыка..." -ForegroundColor Cyan
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$portableDir\DNS Manager.lnk")
$Shortcut.TargetPath = "cmd.exe"
$Shortcut.Arguments = "/c `"$PSScriptRoot\dns-manager.exe`" & pause"
$Shortcut.WorkingDirectory = "$PSScriptRoot"
$Shortcut.IconLocation = "shell32.dll,1"
$Shortcut.Save()

# Создай ZIP архив
Write-Host "📦 Создание ZIP архива..." -ForegroundColor Cyan
$zipPath = "DNS-Manager-Portable.zip"
if (Test-Path $zipPath) {
    Remove-Item $zipPath -Force
}
Compress-Archive -Path $portableDir -DestinationPath $zipPath

# Покажи результат
Write-Host "✅ Portable версия создана!" -ForegroundColor Green
Write-Host "📁 Папка: $portableDir" -ForegroundColor White
Write-Host "📦 Архив: $zipPath" -ForegroundColor White
Write-Host "📏 Размер: $([math]::Round((Get-Item $exePath).Length / 1MB), 2) MB" -ForegroundColor White

Write-Host ""
Write-Host "🎯 Инструкция по использованию:" -ForegroundColor Yellow
Write-Host "1. Скопируй папку '$portableDir' или архив '$zipPath' на другой компьютер"
Write-Host "2. Запусти 'DNS Manager.lnk' (ярлык автоматически запустит от имени администратора)"
Write-Host "3. Или запусти 'dns-manager.exe' правой кнопкой → 'Run as administrator'"

Write-Host ""
Write-Host "🌟 Готово! Приложение portable и не требует установки зависимостей." -ForegroundColor Green
