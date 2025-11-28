# BrowserMCP Dual Server Setup

## 🎯 Overview
This setup provides two BrowserMCP servers running simultaneously on different ports:

- **MEMORY_P Server**: Port 9000 (http://127.0.0.1:9000/mcp)
- **NUCLEAR_CRAWLER Server**: Port 3000 (http://127.0.0.1:3000/mcp)

Both servers provide the same 20+ tools and are fully functional MCP 2025 compliant servers.

## 🚀 Quick Start

### Option 1: Start Both Servers (Recommended)
```powershell
.\START_BOTH_MCP_SERVERS.ps1
```

### Option 2: Start Individual Servers
```powershell
.\START_MEMORY_P.ps1      # Starts on port 9000
.\START_NUCLEAR_CRAWLER.ps1  # Starts on port 3000
```

## 📋 Available Scripts

| Script | Description | Port |
|--------|-------------|------|
| `START_BOTH_MCP_SERVERS.ps1` | Starts both servers in background | 9000 & 3000 |
| `START_MEMORY_P.ps1` | Starts MEMORY_P server | 9000 |
| `START_NUCLEAR_CRAWLER.ps1` | Starts NUCLEAR_CRAWLER server | 3000 |

## ✅ Verification

After starting the servers, you can verify they're running by checking the health endpoints:

```bash
# Test MEMORY_P Server
curl http://127.0.0.1:9000/health

# Test NUCLEAR_CRAWLER Server  
curl http://127.0.0.1:3000/health
```

## 🛑 Stopping Servers

### Stop All Servers
```powershell
Get-Process -Name 'powershell' | Stop-Process
```

### Stop Individual Servers
Use Task Manager or PowerShell to kill specific processes.

## 🔧 Configuration

Both servers use the same binary with different port arguments:
- Port 9000: `./target/release/browsermcp-server.exe 9000`
- Port 3000: `./target/release/browsermcp-server.exe 3000`

## 📡 MCP Endpoints

Both servers provide:
- **MCP Endpoint**: `http://127.0.0.1:[PORT]/mcp`
- **Health Check**: `http://127.0.0.1:[PORT]/health`

## 🛠 Available Tools (20+)

1. browser_automation - Automated browser control
2. form_filling - Intelligent form detection and filling
3. captcha_solving - CAPTCHA solving (reCAPTCHA, hCaptcha, AWS)
4. dom_extraction - DOM data extraction with CSS selectors
5. screenshot_capture - Page screenshots
6. stealth_browsing - Anti-detection browsing
7. browser_create - Browser instance management
8. browser_list - List browser instances
9. oauth_store_token - OAuth token storage
10. oauth_list - OAuth token listing
11. account_create - Account creation
12. account_list - Account listing
13. session_start - Session management
14. session_list - Active sessions
15. web_scrape - Web scraping with email/phone extraction
16. get_stealth_headers - Anti-detection headers
17. analyze_code - Source code analysis
18. analyze_url_code - URL code analysis
19. analyze_file - Local file analysis
20. analyze_project - Project analysis

## ⚡ Performance

- Zero warnings in production build
- Optimized release mode
- Ready for production use
- Both servers can run simultaneously without conflicts

## 🔍 Troubleshooting

1. **Port Already in Use**: Change ports in scripts or kill existing processes
2. **Permission Errors**: Run PowerShell as Administrator
3. **Firewall Issues**: Allow connections on ports 9000 and 3000
4. **Build Issues**: Run `cargo build --release` first

## 📝 Notes

- Servers run in background (hidden/minimized)
- Both servers are identical in functionality
- Designed for MCP 2025 protocol compliance
- Production-ready with full error handling