# ============================================================================
# EXTREME BROWSER MCP - DOCKER DEPLOYMENT SCRIPT V2 🚀
# PowerShell version for Windows con validación completa
# ============================================================================

Write-Host "🚀 Extreme Browser MCP - Docker Deployment V2" -ForegroundColor Cyan
Write-Host "===============================================" -ForegroundColor Cyan

# Función para verificar si Docker está corriendo
function Test-DockerRunning {
    try {
        docker version | Out-Null
        return $true
    }
    catch {
        return $false
    }
}

# Verificar Docker
Write-Host "🔍 Verificando Docker..." -ForegroundColor Yellow
if (-not (Test-DockerRunning)) {
    Write-Host "❌ Error: Docker no está corriendo. Por favor inicia Docker Desktop." -ForegroundColor Red
    exit 1
}

# Limpiar contenedores anteriores
Write-Host "🧹 Limpiando contenedores anteriores..." -ForegroundColor Yellow
docker stop extreme-browser-mcp 2>$null
docker rm extreme-browser-mcp 2>$null

# Limpiar builds anteriores
Write-Host "🧹 Limpiando builds anteriores..." -ForegroundColor Yellow
cargo clean

# Compilar en modo release
Write-Host "⚙️  Compilando aplicación (puede tardar 3-5 minutos)..." -ForegroundColor Yellow
cargo build --release

# Verificar que el binario se creó
$binaryPath = ""
if (Test-Path "target/release/browsermcp-server.exe") {
    $binaryPath = "target/release/browsermcp-server.exe"
} elseif (Test-Path "target/release/browsermcp-server") {
    $binaryPath = "target/release/browsermcp-server"
} elseif (Test-Path "target/release/extreme-browser-mcp.exe") {
    $binaryPath = "target/release/extreme-browser-mcp.exe"
} elseif (Test-Path "target/release/extreme-browser-mcp") {
    $binaryPath = "target/release/extreme-browser-mcp"
}

if ($binaryPath) {
    Write-Host "✅ Compilación exitosa: $binaryPath" -ForegroundColor Green
    
    # Copiar binario para Docker
    Copy-Item $binaryPath "browsermcp-server"
    Write-Host "📋 Binario copiado para Docker" -ForegroundColor Green
} else {
    Write-Host "❌ Error: No se pudo compilar el binario" -ForegroundColor Red
    Write-Host "Archivos encontrados en target/release:" -ForegroundColor Yellow
    Get-ChildItem "target/release" -ErrorAction SilentlyContinue | ForEach-Object { Write-Host "  - $($_.Name)" }
    exit 1
}

# Construir imagen Docker
Write-Host "🐳 Construyendo imagen Docker..." -ForegroundColor Yellow
docker build -f Dockerfile.production -t extreme-browser-mcp:latest .

# Verificar que la imagen se creó
$imageExists = docker images extreme-browser-mcp:latest --format "table {{.Repository}}" | Select-String "extreme-browser-mcp"
if ($imageExists) {
    Write-Host "✅ Imagen Docker creada exitosamente" -ForegroundColor Green
    
    # Ejecutar contenedor
    Write-Host "🚀 Iniciando contenedor..." -ForegroundColor Yellow
    $containerId = docker run -d -p 8080:8080 --name extreme-browser-mcp extreme-browser-mcp:latest
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Contenedor iniciado: $containerId" -ForegroundColor Green
        
        # Esperar un momento y verificar estado
        Start-Sleep -Seconds 3
        $containerStatus = docker ps --filter "name=extreme-browser-mcp" --format "table {{.Status}}"
        
        if ($containerStatus -match "Up") {
            Write-Host "🎉 ¡Deployment completado exitosamente!" -ForegroundColor Green
            Write-Host ""
            Write-Host "📊 Información del deployment:" -ForegroundColor Cyan
            docker ps --filter "name=extreme-browser-mcp"
            Write-Host ""
            Write-Host "🌐 Servidor disponible en:" -ForegroundColor Cyan
            Write-Host "   http://localhost:8080" -ForegroundColor White
            Write-Host ""
            Write-Host "🔍 Comandos útiles:" -ForegroundColor Cyan
            Write-Host "   Ver logs: docker logs -f extreme-browser-mcp" -ForegroundColor White
            Write-Host "   Parar: docker stop extreme-browser-mcp" -ForegroundColor White
            Write-Host "   Reiniciar: docker restart extreme-browser-mcp" -ForegroundColor White
        } else {
            Write-Host "⚠️  Contenedor iniciado pero puede tener problemas" -ForegroundColor Yellow
            Write-Host "Ver logs: docker logs extreme-browser-mcp" -ForegroundColor White
        }
    } else {
        Write-Host "❌ Error al iniciar el contenedor" -ForegroundColor Red
        exit 1
    }
    
} else {
    Write-Host "❌ Error: No se pudo crear la imagen Docker" -ForegroundColor Red
    exit 1
}