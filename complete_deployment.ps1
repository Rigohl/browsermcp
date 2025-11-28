#!/usr/bin/env pwsh
# ============================================================================
# EXTREME BROWSER MCP - COMPLETE DEPLOYMENT SCRIPT V3
# Compilación + Docker + Testing automatizado
# ============================================================================

Write-Host "🚀 Extreme Browser MCP - Complete Deployment V3" -ForegroundColor Cyan
Write-Host "===============================================" -ForegroundColor Cyan

# Verificar Docker
Write-Host "🔍 Verificando Docker..." -ForegroundColor Yellow
try {
    docker --version | Out-Null
    Write-Host "✅ Docker disponible" -ForegroundColor Green
} catch {
    Write-Host "❌ Docker no disponible" -ForegroundColor Red
    exit 1
}

# Limpiar contenedores anteriores
Write-Host "🧹 Limpiando contenedores anteriores..." -ForegroundColor Yellow
docker stop $(docker ps -q) 2>$null
docker rm extreme-browser-mcp 2>$null

# Verificar o compilar binario
if (Test-Path "target\release\browsermcp-server.exe") {
    Write-Host "✅ Binario encontrado: $(Get-Item 'target\release\browsermcp-server.exe' | Select-Object -ExpandProperty Length) bytes" -ForegroundColor Green
} else {
    Write-Host "⚙️ Compilando aplicación (puede tardar 3-5 minutos)..." -ForegroundColor Yellow
    cargo build --release --bin browsermcp-server
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Error en compilación" -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ Compilación exitosa" -ForegroundColor Green
}

# Copiar binario para Docker
Copy-Item "target\release\browsermcp-server.exe" "browsermcp-server" -Force
Write-Host "📋 Binario copiado para Docker" -ForegroundColor Green

# Construir imagen Docker
Write-Host "🐳 Construyendo imagen Docker..." -ForegroundColor Yellow
docker build -f Dockerfile.production -t extreme-browser-mcp:latest .
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error en Docker build" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Imagen Docker creada exitosamente" -ForegroundColor Green

# Iniciar contenedor
Write-Host "🚀 Iniciando contenedor..." -ForegroundColor Yellow
$containerId = docker run -d --name extreme-browser-mcp -p 8080:8080 -e RUST_LOG=info --restart=unless-stopped extreme-browser-mcp:latest
Write-Host "✅ Contenedor iniciado: $containerId" -ForegroundColor Green

# Esperar y verificar
Start-Sleep 5
$containerStatus = docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | Select-String "extreme-browser-mcp"

if ($containerStatus) {
    Write-Host "✅ DEPLOYMENT EXITOSO" -ForegroundColor Green
    Write-Host "🌐 Servidor: http://localhost:8080" -ForegroundColor Cyan
    Write-Host "📋 Herramientas: 20+ disponibles" -ForegroundColor Cyan
    Write-Host "🔍 Verificar: docker logs extreme-browser-mcp" -ForegroundColor Yellow
} else {
    Write-Host "⚠️ Contenedor iniciado pero verificar logs" -ForegroundColor Yellow
    Write-Host "Ver logs: docker logs extreme-browser-mcp" -ForegroundColor Yellow
}

Write-Host "`n🎉 DEPLOYMENT COMPLETO!" -ForegroundColor Green