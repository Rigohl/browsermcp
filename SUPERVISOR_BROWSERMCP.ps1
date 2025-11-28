# ============================================================================
# SUPERVISOR - BROWSERMCP PERSISTENTE
# Mantiene el servidor UP 24/7, reinicia automáticamente si se cae
# ============================================================================

$ServerPath = "c:\Users\DELL\Desktop\PROYECTOS\browsermcp"
$ServerBinary = "$ServerPath\target\release\browsermcp-server.exe"
$Port = 3001
$LogFile = "$ServerPath\supervisor_browsermcp.log"

function Log-Message {
    param([string]$Message)
    $Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $LogEntry = "[$Timestamp] $Message"
    Write-Host $LogEntry -ForegroundColor Cyan
    Add-Content -Path $LogFile -Value $LogEntry
}

function Check-ServerHealth {
    try {
        $Response = Invoke-WebRequest -Uri "http://127.0.0.1:$Port/mcp" `
            -Method POST `
            -Body '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":1}' `
            -ContentType "application/json" `
            -TimeoutSec 5 `
            -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

function Start-Server {
    Log-Message "🚀 Iniciando BROWSERMCP en puerto $Port..."
    Push-Location $ServerPath
    & $ServerBinary | Out-Null &
    Pop-Location
    Start-Sleep -Seconds 3
}

function Kill-Server {
    Log-Message "⛔ Terminando proceso anterior..."
    Get-Process | Where-Object { $_.ProcessName -eq "browsermcp-server" } | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 2
}

# ============================================================================
# BUCLE PRINCIPAL DE SUPERVISIÓN
# ============================================================================

Log-Message "════════════════════════════════════════════════════════════"
Log-Message "🔒 SUPERVISOR INICIADO - BROWSERMCP PERSISTENTE"
Log-Message "════════════════════════════════════════════════════════════"
Log-Message "Verificando binario: $ServerBinary"

if (-not (Test-Path $ServerBinary)) {
    Log-Message "❌ ERROR: Binario no encontrado. Compila primero: cargo build --release"
    exit 1
}

Log-Message "✅ Binario verificado"

# Iniciar servidor por primera vez
Start-Server

$RestartCount = 0
$HealthCheckInterval = 10  # Verificar cada 10 segundos
$MaxConsecutiveFailures = 3

while ($true) {
    Start-Sleep -Seconds $HealthCheckInterval
    
    if (Check-ServerHealth) {
        # Servidor está OK
        $RestartCount = 0
        Write-Host "." -NoNewline -ForegroundColor Green
    } else {
        $RestartCount++
        Log-Message "⚠️  Health check falló ($RestartCount/$MaxConsecutiveFailures)"
        
        if ($RestartCount -ge $MaxConsecutiveFailures) {
            Log-Message "🔴 CRITICO: Servidor muerto después de $MaxConsecutiveFailures intentos"
            Log-Message "Reiniciando automáticamente..."
            Kill-Server
            Start-Server
            $RestartCount = 0
        }
    }
}
