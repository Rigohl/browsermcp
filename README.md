# 🚀 Extreme Browser MCP

**Browser Data Extraction & Social Media Intelligence Platform**

[![Rust](https://img.shields.io/badge/rust-1.81+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com)

> **Plataforma avanzada de extracción de datos de navegador con inteligencia artificial para análisis de redes sociales, detección de vulnerabilidades y autenticación biométrica.**

## 🌟 Características Principales

### 🔍 **Browser Data Extraction**
- **SQLite3 Real**: Lee bases de datos de Chrome, Edge, Firefox
- **Cookies & Passwords**: Extracción segura con encriptación AES-256
- **Historial de navegación**: Análisis completo de actividad web
- **Extensiones instaladas**: Detección y análisis de plugins

### 🧠 **Social Media Intelligence**  
- **Análisis de Sentimientos**: NLP avanzado sin dependencias externas
- **Detección de Campañas**: Identificación de patrones coordinados
- **Multi-plataforma**: Twitter, LinkedIn, Instagram
- **Influencer Analytics**: Scoring automático de influencia

### 🛡️ **Windows Hello Integration**
- **Autenticación Biométrica**: Fingerprint, Face ID, Iris
- **PIN Management**: Gestión segura de credenciales
- **Device Credentials**: Extracción de datos de dispositivo

### ⚡ **High-Performance Computing**
- **MEMORY_P Integration**: 1M+ tareas en paralelo
- **NUCLEAR_CRAWLER**: Escaneo masivo de vulnerabilidades
- **WASM Runtime**: Ejecución segura en sandbox
- **Async/Await**: Procesamiento no-bloqueante

---

## 🚀 Quick Start

### Local Development
```bash
cd browsermcp
cargo build --release
./target/release/browsermcp-server
```

Server en: **http://127.0.0.1:3001**

### ☁️ Cloud Deployment (Google Cloud Run)

**Always Free Tier: 2M requests/month at $0**

1. **Create GCP Service Account** (one-time setup):
```bash
# In Google Cloud Console:
# 1. Create new service account
# 2. Grant roles: roles/run.admin + roles/artifactregistry.admin
# 3. Generate JSON key
# 4. Encode key: cat key.json | base64 -w 0
```

2. **Add GitHub Secrets** to repository:
   - `GCP_PROJECT_ID`: Your GCP project ID
   - `GCP_SA_KEY`: Base64-encoded service account key

3. **Deploy**:
```bash
# Automatic on every push to master (via GitHub Actions)
# Or manual trigger:
git commit --allow-empty -m "Deploy to Cloud Run"
git push origin master
```

4. **Monitor Deployment**:
   - GitHub Actions: https://github.com/Rigohl/browsermcp/actions
   - Cloud Run Console: https://console.cloud.google.com/run
   - Service URL will be printed in GitHub Actions logs

5. **Test Cloud Run Endpoint**:
```bash
curl https://browsermcp-[PROJECT_ID].run.app/health
```

---

## 📡 API Ejemplos

### Web Scraping

```bash
curl -X POST http://127.0.0.1:3001/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "web_scrape",
      "arguments": {
        "url": "https://example.com",
        "selectors": ["title", "h1", ".email"]
      }
    }
  }'
```

**Respuesta:**
```json
{
  "status": "success",
  "title": "Example Domain",
  "emails": ["contact@example.com"],
  "phones": ["+1-555-123-4567"],
  "links": [{"href": "https://example.com", "text": "Home"}]
}
```

### Workflow Orchestrator

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "workflow_orchestrator",
    "arguments": {
      "action": "create",
      "workflow": {
        "name": "Monitoring",
        "steps": [
          {"action": "scrape", "url": "https://target.com"},
          {"action": "analyze"},
          {"action": "store"},
          {"action": "alert"}
        ],
        "schedule": "hourly"
      }
    }
  }
}
```

### Vulnerability Scanner

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "vulnerability_scanner",
    "arguments": {
      "target": "https://webapp.com",
      "scan_type": "full"
    }
  }
}
```

---

## ☁️ Cloud Run Features

| Feature | Details |
|---------|---------|
| **Deployment** | Automated via GitHub Actions |
| **Memory** | 512 MB per instance |
| **CPU** | 1 vCPU per instance |
| **Timeout** | 3600 seconds (1 hour) |
| **Max Instances** | 100 (auto-scaling) |
| **Free Tier** | 2M requests/month |
| **Cost** | $0/month for free tier usage |
| **Region** | us-central1 |
| **HTTPS** | Automatic, always encrypted |

---

## 📊 Arquitectura

```
Claude AI
    │
    ▼
BROWSERMCP (Port 3001)
    │
    ├─ Browser Layer
    ├─ Scraping Layer
    ├─ Analysis Layer
    └─ Data Layer
```

---

## 📈 Performance

| Métrica | Valor |
|---------|-------|
| Compilación | 3 minutos |
| Binary | 12.9 MB (pre-compiled) |
| Memory | 50-200 MB |
| Warnings | 0 |
| **Cloud Run Deployment** | 5-10 minutos |
| **Docker Image Size** | ~500 MB |
| **Requests/Month** | 2,000,000 FREE |

---

## 🔧 Configuración

```bash
export RUST_LOG=debug
export SCRAPER_TIMEOUT_SECS=30
export RATE_LIMIT_PER_MINUTE=60
```

---

## 🧪 Testing

```bash
cargo test
RUST_LOG=debug cargo run --release
```

---

## 📞 Soporte

- 🐛 Issues: https://github.com/tu-usuario/browsermcp/issues
- 📖 Docs: https://docs.browsermcp.local

---

**Última actualización:** 28 Nov 2025
**Versión:** 3.0.0-PRO
**Status:** ✅ Production Ready
**Deployment:** 🚀 Cloud Run + GitHub Actions

---

## 📚 Recursos

- **GitHub Repo**: https://github.com/Rigohl/browsermcp
- **GitHub Actions**: https://github.com/Rigohl/browsermcp/actions
- **Deployment Guide**: See `DEPLOY_GUIA.md`
- **Cloud Options**: See `CLOUD_DEPLOYMENT_OPTIONS_2025.md`
- **Docker**: Optimized production Dockerfile included

```
╔════════════════════════════════════════╗
║  🚀 BrowserMCP PRO v3.0.0-PRO 🚀      ║
║  20+ Tools • Zero Warnings • Secure   ║
║  Cloud-Ready • GitHub Actions CI/CD   ║
║  Web Automation & Scraping            ║
╚════════════════════════════════════════╝
```
