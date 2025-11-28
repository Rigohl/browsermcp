# 🚀 GUÍA DE DEPLOYMENT - BrowserMCP → Google Cloud Run

**Fecha:** 2025-11-28  
**Proyecto:** BrowserMCP v3.0  
**Tamaño:** 13.6 MB (código limpio)

---

## ⚠️ REQUISITOS PREVIOS

- ✅ Google Cloud SDK instalado
- ✅ Docker Desktop corriendo
- ✅ Cuenta Kimberly con Google Cloud activa
- ✅ Tarjeta registrada en Google Play (para verificación)

---

## 🎯 PASOS PARA DEPLOYMENT

### **1. Instalar Google Cloud SDK** (si no lo tienes)

```powershell
# Descargar e instalar
$url = "https://dl.google.com/dl/cloudsdk/channels/rapid/GoogleCloudSDKInstaller.exe"
Invoke-WebRequest -Uri $url -OutFile "$env:TEMP\GoogleCloudSDKInstaller.exe"
& "$env:TEMP\GoogleCloudSDKInstaller.exe"

# Reiniciar terminal después de instalar
```

---

### **2. Autenticarse con Kimberly**

```powershell
# Abrir navegador para autenticación
gcloud auth login

# Seleccionar tu cuenta Kimberly en Google
# Google pedirá verificar con tarjeta de Google Play
```

---

### **3. Crear Proyecto en Google Cloud (OPCIONAL si ya existe)**

```powershell
# Listar proyectos
gcloud projects list

# Si necesitas crear uno nuevo:
gcloud projects create browsermcp-2025 --set-as-default
```

---

### **4. Configurar Proyecto**

```powershell
# Establecer proyecto
$PROJECT_ID = "tu-proyecto-id"  # Reemplaza con tu ID
gcloud config set project $PROJECT_ID

# Habilitar APIs necesarias
gcloud services enable run.googleapis.com
gcloud services enable cloudbuild.googleapis.com
gcloud services enable containerregistry.googleapis.com
```

---

### **5. Verificar que Docker está corriendo**

```powershell
docker ps  # Debe mostrar información sin errores
```

---

### **6. Compilar Docker Image**

```powershell
cd C:\Users\DELL\Desktop\PROYECTOS\browsermcp

# Compilar con Dockerfile.production
docker build -f Dockerfile.production -t gcr.io/$PROJECT_ID/browsermcp-server:latest .
```

---

### **7. Autenticar Docker con Google**

```powershell
gcloud auth configure-docker gcr.io --quiet
```

---

### **8. Subir Imagen a Container Registry**

```powershell
docker push gcr.io/$PROJECT_ID/browsermcp-server:latest

# Verificar que se subió
gcloud container images list --repository=gcr.io/$PROJECT_ID
```

---

### **9. Deployar a Cloud Run**

```powershell
$SERVICE_NAME = "browsermcp-server"
$REGION = "us-central1"

gcloud run deploy $SERVICE_NAME `
    --image gcr.io/$PROJECT_ID/browsermcp-server:latest `
    --platform managed `
    --region $REGION `
    --allow-unauthenticated `
    --memory 512Mi `
    --cpu 1 `
    --timeout 3600 `
    --max-instances 100
```

---

### **10. Obtener URL del Servicio**

```powershell
$SERVICE_URL = gcloud run services describe $SERVICE_NAME `
    --platform managed `
    --region $REGION `
    --format 'value(status.url)'

Write-Host "✅ BrowserMCP está LIVE en: $SERVICE_URL" -ForegroundColor Green
```

---

## 🔗 PROBAR EL DEPLOYMENT

### **Health Check**
```bash
curl https://[tu-cloud-run-url]/health
```

### **Test MCP Tools**
```bash
curl -X POST https://[tu-cloud-run-url]/mcp \
  -H "Content-Type: application/json" \
  -d '{"tool": "search_web", "query": "test"}'
```

---

## 📊 VERIFICAR LOGS

```powershell
$SERVICE_NAME = "browsermcp-server"
$REGION = "us-central1"

# Ver logs en tiempo real
gcloud run logs read $SERVICE_NAME --region $REGION -n 50 --follow

# Ver métricas
gcloud run describe $SERVICE_NAME --region $REGION --platform managed
```

---

## 💰 COSTOS ESTIMADOS

**Google Cloud Run Free Tier (ALWAYS FREE):**
- ✅ 2 millones de requests/mes
- ✅ 400,000 GB-segundos de compute
- ✅ Transfer ilimitado dentro de Google Cloud
- 🎉 **COSTO: $0/mes**

---

## ⚙️ CONFIGURACIÓN CLOUD RUN

| Parámetro | Valor | Razón |
|-----------|-------|-------|
| Memory | 512 MB | Suficiente para BrowserMCP |
| CPU | 1 | Adecuado para web server |
| Timeout | 3600s | 1 hora máximo |
| Max instances | 100 | Auto-scaling |
| Always free | ✅ | Dentro de límites |

---

## 🆘 TROUBLESHOOTING

### Error: "gcloud command not found"
```powershell
# Instala Google Cloud SDK desde:
# https://cloud.google.com/sdk/docs/install
```

### Error: "Docker daemon not running"
```powershell
# Abre Docker Desktop
# Espera a que termine de inicializar
docker ps  # Debe funcionar
```

### Error: "Authentication failed"
```powershell
# Vuelve a autenticar
gcloud auth login
```

### Error: "Quota exceeded"
```powershell
# Cloud Run tiene límites de 100 instancias simultáneas
# Dentro del free tier no deberías alcanzarlo
```

---

## 📝 COMANDOS ÚTILES

```powershell
# Ver todos los servicios Cloud Run
gcloud run services list --region us-central1

# Actualizar servicio
gcloud run deploy browsermcp-server `
    --image gcr.io/$PROJECT_ID/browsermcp-server:latest `
    --region us-central1

# Eliminar servicio
gcloud run services delete browsermcp-server --region us-central1

# Escalar a 0 (desactivar)
gcloud run services update-traffic browsermcp-server --to-revisions LATEST=0

# Ver costos
gcloud billing budgets create --billing-account=ACCOUNT_ID --display-name="BrowserMCP Budget"
```

---

## ✨ RESULTADO FINAL

Después de seguir estos pasos:

✅ BrowserMCP corriendo en Google Cloud Run  
✅ URL pública con HTTPS  
✅ Auto-scaling de 0 a N instancias  
✅ Logs en Google Cloud Console  
✅ $0/mes en costos  

---

**Duración esperada:** 10-15 minutos  
**Dificultad:** ⭐⭐ (Intermedia)  
**Soporte:** Google Cloud Documentation
