# ☁️ Cloud Run Deployment Checklist

## Status: ✅ READY FOR DEPLOYMENT

Your BrowserMCP is configured and ready to deploy to Google Cloud Run with automated GitHub Actions CI/CD.

---

## 📋 Step-by-Step Deployment Guide

### Step 1: Create Google Cloud Project (if you don't have one)

```bash
# Option A: Via Google Cloud Console
# 1. Go to https://console.cloud.google.com
# 2. Click "Select a Project" → "New Project"
# 3. Name: browsermcp
# 4. Click Create
```

### Step 2: Create Service Account

**In Google Cloud Console:**

1. Navigate to: **IAM & Admin** → **Service Accounts**
2. Click **Create Service Account**
3. Service account name: `browsermcp-github`
4. Click **Create and Continue**
5. Grant these roles:
   - `Cloud Run Admin` (roles/run.admin)
   - `Artifact Registry Service Agent` (roles/artifactregistry.serviceagent)
   - `Storage Admin` (roles/storage.admin) - for artifact registry
6. Click **Continue** → **Done**

### Step 3: Create & Export Service Account Key

**In the same console:**

1. Find your new service account: `browsermcp-github@[PROJECT-ID].iam.gserviceaccount.com`
2. Click on it
3. Go to **Keys** tab
4. Click **Add Key** → **Create new key**
5. Choose **JSON**
6. A file will download: `[project-id]-[randomid].json`
7. Keep this file safe ✓

### Step 4: Encode Service Account Key to Base64

**On your Windows machine (PowerShell):**

```powershell
# Replace with your downloaded file path
$keyPath = "C:\Path\To\Your\[project-id]-[randomid].json"
$base64Key = [Convert]::ToBase64String([System.IO.File]::ReadAllBytes($keyPath))
$base64Key | Set-Clipboard
Write-Host "✅ Base64 key copied to clipboard!"
```

Or manually:
- Open the JSON file
- Copy entire content
- Use online base64 encoder: https://www.base64encode.org
- Copy the encoded result

### Step 5: Create Artifact Registry (one-time)

**In Google Cloud Console:**

1. Enable Artifact Registry API:
   ```
   APIs & Services → Enable APIs and Services
   Search: "Artifact Registry API"
   Click Enable
   ```

2. Create repository:
   ```
   Artifact Registry → Repositories → Create Repository
   - Name: browsermcp-server
   - Format: Docker
   - Location: us-central1
   - Click Create
   ```

### Step 6: Add GitHub Secrets

**In your GitHub repository:**

1. Go: https://github.com/Rigohl/browsermcp/settings/secrets/actions
2. Click **New repository secret**

**Add Secret 1: GCP_PROJECT_ID**
- Name: `GCP_PROJECT_ID`
- Value: Your GCP project ID (e.g., `my-project-123456`)
- Click **Add secret**

**Add Secret 2: GCP_SA_KEY**
- Name: `GCP_SA_KEY`
- Value: The base64-encoded key you created in Step 4
- Click **Add secret**

✅ Both secrets are now added!

### Step 7: Trigger First Deployment

**Option A: Via Git Commit**

```bash
cd C:\Users\DELL\Desktop\PROYECTOS\browsermcp
git commit --allow-empty -m "🚀 Trigger Cloud Run deployment"
git push origin master
```

**Option B: Via GitHub Actions UI**

1. Go: https://github.com/Rigohl/browsermcp/actions
2. Select workflow: `Deploy to Cloud Run`
3. Click **Run workflow**
4. Branch: `master`
5. Click **Run workflow**

---

## 🔍 Monitor Deployment

### Watch GitHub Actions

1. Go: https://github.com/Rigohl/browsermcp/actions
2. Click the running workflow
3. Look for job: `deploy`
4. Wait for completion (5-10 minutes)

**Success output:**
```
✓ Docker image built and pushed
✓ Cloud Run service deployed
✓ Service URL: https://browsermcp-xxxxx.run.app
```

### Check Cloud Run Console

1. Go: https://console.cloud.google.com/run
2. Find service: `browsermcp-server`
3. Status should be: ✅ Green (Running)
4. Click service name to see details
5. Copy **Service URL** from top

---

## ✅ Verify Deployment

### Test Health Endpoint

```bash
# Replace with your actual service URL
$url = "https://browsermcp-xxxxx.run.app/health"
curl -Uri $url
```

Expected response:
```json
{"status": "healthy"}
```

### Test MCP Endpoint

```bash
$url = "https://browsermcp-xxxxx.run.app/mcp"
$body = @{
    jsonrpc = "2.0"
    id = 1
    method = "tools/list"
} | ConvertTo-Json

curl -Uri $url -Method Post -Body $body -ContentType "application/json"
```

---

## 🚀 Your Cloud Run Service is Now Live!

### Endpoint
```
https://browsermcp-[PROJECT_ID].run.app
```

### Features
- ✅ Auto-scales 0→100 instances
- ✅ HTTPS automatic (always encrypted)
- ✅ 2M requests/month FREE
- ✅ Automatic deployments on git push
- ✅ Logs in Cloud Run console

---

## 📊 Cost Breakdown (Always Free Tier)

| Service | Free Tier | Cost |
|---------|-----------|------|
| Cloud Run | 2M requests/month | $0 |
| Cloud Build | 120 build-minutes/day | $0 |
| Artifact Registry | 0.5 GB storage | $0 |
| **Total** | - | **$0/month** |

---

## 🐛 Troubleshooting

### GitHub Actions Fails
- ❌ Check GitHub Secrets are added correctly
- ❌ Verify `GCP_SA_KEY` is base64-encoded
- ❌ Confirm service account has correct roles

### Cloud Run Deployment Fails
- ❌ Check Artifact Registry exists
- ❌ Verify service account has `run.admin` role
- ❌ Check Cloud Run API is enabled

### Service Returns 500 Error
- ❌ Check Cloud Run logs: https://console.cloud.google.com/run → Service → Logs
- ❌ Verify binary is working locally first
- ❌ Check memory isn't exceeded

---

## 📞 Need Help?

1. **GitHub Actions Logs**: https://github.com/Rigohl/browsermcp/actions
2. **Cloud Run Console**: https://console.cloud.google.com/run
3. **Cloud Logs**: https://console.cloud.google.com/logs
4. **Google Cloud Docs**: https://cloud.google.com/run/docs

---

## ✨ Next Steps After Deployment

1. ✅ Test all MCP tools against live endpoint
2. ✅ Monitor costs in Cloud Run console
3. ✅ Set up Cloud Monitoring alerts (optional)
4. ✅ Configure custom domain (optional)
5. ✅ Integrate Nuclear Crawler MCP for enhanced capabilities

---

**Last Updated**: 28 Nov 2025  
**Status**: ✅ Ready for Deployment  
**Version**: 3.0.0-PRO
