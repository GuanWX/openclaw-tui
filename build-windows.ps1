# AI Token Monitor - Windows Build Script
# Run this in PowerShell as Administrator

Write-Host "🤖 AI Token Monitor - Windows Build" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan

# Check prerequisites
$errors = @()

# Check Node.js
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    $errors += "Node.js not found. Install from https://nodejs.org"
} else {
    $nodeVersion = node --version
    Write-Host "✅ Node.js: $nodeVersion" -ForegroundColor Green
}

# Check Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    $errors += "Rust not found. Install from https://rustup.rs"
} else {
    $rustVersion = rustc --version
    Write-Host "✅ Rust: $rustVersion" -ForegroundColor Green
}

# Check Visual Studio Build Tools
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vsWhere) {
    $vsPath = & $vsWhere -latest -property installationPath 2>$null
    if ($vsPath) {
        Write-Host "✅ Visual Studio: $vsPath" -ForegroundColor Green
    }
} else {
    $errors += "Visual Studio Build Tools not found. Install from https://visualstudio.microsoft.com/visual-cpp-build-tools/"
}

if ($errors.Count -gt 0) {
    Write-Host "`n❌ Missing prerequisites:" -ForegroundColor Red
    $errors | ForEach-Object { Write-Host "  - $_" -ForegroundColor Yellow }
    Write-Host "`nPlease install missing components and run again." -ForegroundColor Yellow
    exit 1
}

Write-Host "`n📦 Installing dependencies..." -ForegroundColor Cyan
npm install

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ npm install failed" -ForegroundColor Red
    exit 1
}

Write-Host "`n🔨 Building Tauri app..." -ForegroundColor Cyan
npm run tauri build

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ Build successful!" -ForegroundColor Green

    $msiPath = Get-ChildItem -Path "src-tauri\target\release\bundle\msi\*.msi" -ErrorAction SilentlyContinue | Select-Object -First 1
    $exePath = Get-ChildItem -Path "src-tauri\target\release\bundle\nsis\*.exe" -ErrorAction SilentlyContinue | Select-Object -First 1

    if ($msiPath) {
        Write-Host "📦 MSI installer: $($msiPath.FullName)" -ForegroundColor Cyan
    }
    if ($exePath) {
        Write-Host "📦 NSIS installer: $($exePath.FullName)" -ForegroundColor Cyan
    }

    Write-Host "`n📂 Opening output folder..." -ForegroundColor Cyan
    explorer.exe "src-tauri\target\release\bundle"
} else {
    Write-Host "`n❌ Build failed" -ForegroundColor Red
    exit 1
}