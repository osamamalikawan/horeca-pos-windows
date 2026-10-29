# Builds the web app and the Windows installer, signs it, and prepares the update files.
# Run from C:\dev\horeca-pos-windows after raising "version" in tauri.conf.json:
#   .\release.ps1 -Notes "What changed"
param(
  [string]$Notes = "Update",
  [string]$WebDir = "..\horeca-pos-web",
  [string]$KeyFile = "C:\dev\keys\horeca-updater.key"
)
$ErrorActionPreference = 'Stop'
if (-not (Test-Path $KeyFile)) { throw "Signing key not found: $KeyFile (see README, step 1)" }

Write-Host "1/3 Building the web app..."
Push-Location $WebDir; npm install; if ($LASTEXITCODE) { throw "npm install failed" }
npm run build; if ($LASTEXITCODE) { throw "web build failed" }; Pop-Location

Write-Host "2/3 Building and signing the installer (the first build is slow)..."
$env:TAURI_SIGNING_PRIVATE_KEY = $KeyFile
if (-not $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD) {
  $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = Read-Host "Signing key password (empty if none)"
}
npx @tauri-apps/cli@latest build; if ($LASTEXITCODE) { throw "installer build failed" }

Write-Host "3/3 Preparing the update files..."
node scripts/make-latest-json.mjs $Notes; if ($LASTEXITCODE) { throw "could not prepare release files" }
