Write-Host "🚀 Starting BOTH MCP Servers in Background..."
Write-Host ""

# Start MEMORY_P Server on Port 9000
Write-Host "🔄 Starting MEMORY_P MCP Server on Port 9000..."
cd "C:\Users\DELL\Desktop\PROYECTOS\browsermcp"
Start-Process powershell.exe -ArgumentList "-Command", "Write-Host 'MEMORY_P Server (Port 9000):'; .\target\release\browsermcp-server.exe 9000" -WindowStyle Hidden
Start-Sleep -Seconds 3

# Start NUCLEAR_CRAWLER Server on Port 3000
Write-Host "🔄 Starting NUCLEAR_CRAWLER MCP Server on Port 3000..."
Start-Process powershell.exe -ArgumentList "-Command", "Write-Host 'NUCLEAR_CRAWLER Server (Port 3000):'; .\target\release\browsermcp-server.exe 3000" -WindowStyle Hidden
Start-Sleep -Seconds 3

Write-Host ""
Write-Host "✅ Both MCP Servers started successfully!"
Write-Host "📡 MEMORY_P Server: http://127.0.0.1:9000/mcp"
Write-Host "📡 NUCLEAR_CRAWLER Server: http://127.0.0.1:3000/mcp"
Write-Host ""
Write-Host "💡 Both servers are running in background processes"
Write-Host "🛑 To stop them, use: Get-Process -Name 'powershell' | Stop-Process"