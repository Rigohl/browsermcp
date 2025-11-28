# 🚀 DEPLOY BROWSERMCP TO GOOGLE CLOUD RUN
# Author: Kimberly
# Date: 2025-11-28

Write-Host "🔐 GOOGLE CLOUD RUN DEPLOYMENT SCRIPT" -ForegroundColor Cyan
Write-Host "════════════════════════════════════" -ForegroundColor Cyan

# Step 1: Check if gcloud is installed
Write-Host "`n1️⃣ Checking gcloud CLI..." -ForegroundColor Yellow
$gcloudPath = Get-Command gcloud -ErrorAction SilentlyContinue
if (-not $gcloudPath) {
    Write-Host "❌ gcloud CLI not found!" -ForegroundColor Red
    Write-Host "📥 Installing Google Cloud SDK..." -ForegroundColor Yellow
    
    # Download and install Google Cloud SDK
    $url = "https://dl.google.com/dl/cloudsdk/channels/rapid/GoogleCloudSDKInstaller.exe"
    $installer = "$env:TEMP\GoogleCloudSDKInstaller.exe"
    
    Write-Host "📥 Downloading Google Cloud SDK..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri $url -OutFile $installer
    
    Write-Host "📦 Installing..." -ForegroundColor Cyan
    & $installer
    
    Write-Host "✅ Google Cloud SDK installed!" -ForegroundColor Green
    Write-Host "⚠️  Please close this terminal and open a new one" -ForegroundColor Yellow
    exit
}

Write-Host "✅ gcloud CLI found" -ForegroundColor Green

# Step 2: Check authentication
Write-Host "`n2️⃣ Checking authentication..." -ForegroundColor Yellow
$auth = gcloud auth list --format="table(account)" 2>&1

if ($auth -like "*ACTIVE*" -or $auth -like "*@gmail.com*") {
    Write-Host "✅ Already authenticated" -ForegroundColor Green
    Write-Host "   Account: $($auth | Select-Object -First 1)" -ForegroundColor Gray
} else {
    Write-Host "🔑 Authenticating with Kimberly account..." -ForegroundColor Cyan
    gcloud auth login
}

# Step 3: List projects
Write-Host "`n3️⃣ Available projects:" -ForegroundColor Yellow
$projects = gcloud projects list --format="table(projectId)" 2>&1
Write-Host $projects

Write-Host "`n📝 Enter your GCP Project ID:" -ForegroundColor Cyan
$projectId = Read-Host "Project ID"

if ([string]::IsNullOrEmpty($projectId)) {
    Write-Host "❌ Project ID cannot be empty!" -ForegroundColor Red
    exit 1
}

# Step 4: Set project
Write-Host "`n4️⃣ Setting project to: $projectId" -ForegroundColor Yellow
gcloud config set project $projectId

# Step 5: Enable APIs
Write-Host "`n5️⃣ Enabling required APIs..." -ForegroundColor Yellow
gcloud services enable run.googleapis.com --quiet
gcloud services enable build.googleapis.com --quiet
gcloud services enable cloudbuild.googleapis.com --quiet

# Step 6: Build Docker image
Write-Host "`n6️⃣ Building Docker image..." -ForegroundColor Yellow
Write-Host "📦 Image name: browsermcp-server" -ForegroundColor Cyan

# Use Dockerfile.production
if (-not (Test-Path "Dockerfile.production")) {
    Write-Host "⚠️  Dockerfile.production not found, using Dockerfile" -ForegroundColor Yellow
    $dockerfile = "Dockerfile"
} else {
    $dockerfile = "Dockerfile.production"
}

docker build -f $dockerfile -t gcr.io/$projectId/browsermcp-server:latest .

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Docker build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Docker image built successfully" -ForegroundColor Green

# Step 7: Push to Container Registry
Write-Host "`n7️⃣ Pushing image to Google Container Registry..." -ForegroundColor Yellow

# Configure Docker auth
Write-Host "🔑 Configuring Docker authentication..." -ForegroundColor Cyan
gcloud auth configure-docker gcr.io --quiet

# Push image
docker push gcr.io/$projectId/browsermcp-server:latest

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Docker push failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Image pushed to GCR successfully" -ForegroundColor Green

# Step 8: Deploy to Cloud Run
Write-Host "`n8️⃣ Deploying to Google Cloud Run..." -ForegroundColor Yellow

$serviceName = "browsermcp-server"
$region = "us-central1"

Write-Host "🚀 Service: $serviceName" -ForegroundColor Cyan
Write-Host "📍 Region: $region" -ForegroundColor Cyan

gcloud run deploy $serviceName `
    --image gcr.io/$projectId/$serviceName:latest `
    --platform managed `
    --region $region `
    --allow-unauthenticated `
    --memory 512Mi `
    --cpu 1 `
    --timeout 3600 `
    --max-instances 100

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Deployment successful!" -ForegroundColor Green
    
    # Get service URL
    $serviceUrl = gcloud run services describe $serviceName --platform managed --region $region --format 'value(status.url)'
    
    Write-Host "`n🎉 BrowserMCP is LIVE!" -ForegroundColor Green
    Write-Host "📍 URL: $serviceUrl" -ForegroundColor Cyan
    Write-Host "`n🔗 Test endpoints:" -ForegroundColor Yellow
    Write-Host "   Health: $serviceUrl/health" -ForegroundColor Gray
    Write-Host "   MCP: $serviceUrl/mcp" -ForegroundColor Gray
    
} else {
    Write-Host "❌ Deployment failed!" -ForegroundColor Red
    exit 1
}

Write-Host "`n✨ Deployment complete!" -ForegroundColor Green
Write-Host "📊 View logs: gcloud run logs read $serviceName --region $region -n 50" -ForegroundColor Yellow
