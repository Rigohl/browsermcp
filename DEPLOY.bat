@echo off
REM =============================================================================
REM GOOGLE CLOUD SDK INSTALLER FOR BROWSERMCP DEPLOYMENT
REM =============================================================================

setlocal enabledelayedexpansion

echo.
echo ========================================
echo   GOOGLE CLOUD SDK INSTALLATION
echo ========================================
echo.

REM Check if gcloud is already installed
gcloud --version >nul 2>&1
if %errorlevel% equ 0 (
    echo [OK] Google Cloud SDK already installed
    gcloud --version | findstr /R "Google"
    goto auth
)

echo [*] Downloading Google Cloud SDK...
cd /d %TEMP%

REM Download the installer
powershell -Command "Invoke-WebRequest -Uri 'https://dl.google.com/dl/cloudsdk/channels/rapid/GoogleCloudSDKInstaller.exe' -OutFile 'GoogleCloudSDKInstaller.exe'" >nul 2>&1

if %errorlevel% neq 0 (
    echo [ERROR] Failed to download Google Cloud SDK
    echo Please download manually from: https://cloud.google.com/sdk/docs/install
    pause
    exit /b 1
)

echo [*] Running installer...
GoogleCloudSDKInstaller.exe

if %errorlevel% neq 0 (
    echo [ERROR] Installation failed
    pause
    exit /b 1
)

echo [OK] Google Cloud SDK installed successfully

:auth
echo.
echo ========================================
echo   AUTHENTICATING WITH KIMBERLY
echo ========================================
echo.

gcloud auth login

if %errorlevel% neq 0 (
    echo [ERROR] Authentication failed
    pause
    exit /b 1
)

echo [OK] Authenticated successfully!

:config
echo.
echo ========================================
echo   CONFIGURING PROJECT
echo ========================================
echo.

echo [*] Available projects:
gcloud projects list --format="table(projectId)"

set /p PROJECT_ID="Enter your GCP Project ID: "

if "!PROJECT_ID!"=="" (
    echo [ERROR] Project ID cannot be empty
    pause
    exit /b 1
)

gcloud config set project !PROJECT_ID!

if %errorlevel% neq 0 (
    echo [ERROR] Failed to set project
    pause
    exit /b 1
)

echo [OK] Project set to: !PROJECT_ID!

:enable_apis
echo.
echo ========================================
echo   ENABLING REQUIRED APIs
echo ========================================
echo.

echo [*] Enabling Cloud Run API...
gcloud services enable run.googleapis.com --quiet

echo [*] Enabling Cloud Build API...
gcloud services enable cloudbuild.googleapis.com --quiet

echo [*] Enabling Container Registry API...
gcloud services enable containerregistry.googleapis.com --quiet

echo [OK] All APIs enabled!

:build_image
echo.
echo ========================================
echo   BUILDING DOCKER IMAGE
echo ========================================
echo.

cd /d C:\Users\DELL\Desktop\PROYECTOS\browsermcp

echo [*] Building image: gcr.io/!PROJECT_ID!/browsermcp-server:latest
docker build -f Dockerfile.production -t gcr.io/!PROJECT_ID!/browsermcp-server:latest .

if %errorlevel% neq 0 (
    echo [ERROR] Docker build failed
    pause
    exit /b 1
)

echo [OK] Docker image built successfully!

:push_image
echo.
echo ========================================
echo   PUSHING IMAGE TO GCR
echo ========================================
echo.

echo [*] Configuring Docker authentication...
gcloud auth configure-docker gcr.io --quiet

echo [*] Pushing image to Google Container Registry...
docker push gcr.io/!PROJECT_ID!/browsermcp-server:latest

if %errorlevel% neq 0 (
    echo [ERROR] Docker push failed
    pause
    exit /b 1
)

echo [OK] Image pushed to GCR!

:deploy
echo.
echo ========================================
echo   DEPLOYING TO GOOGLE CLOUD RUN
echo ========================================
echo.

set SERVICE_NAME=browsermcp-server
set REGION=us-central1

echo [*] Service: !SERVICE_NAME!
echo [*] Region: !REGION!
echo [*] Image: gcr.io/!PROJECT_ID!/!SERVICE_NAME!:latest

gcloud run deploy !SERVICE_NAME! ^
    --image gcr.io/!PROJECT_ID!/!SERVICE_NAME!:latest ^
    --platform managed ^
    --region !REGION! ^
    --allow-unauthenticated ^
    --memory 512Mi ^
    --cpu 1 ^
    --timeout 3600 ^
    --max-instances 100

if %errorlevel% neq 0 (
    echo [ERROR] Deployment failed
    pause
    exit /b 1
)

echo [OK] Deployment successful!

:get_url
echo.
echo ========================================
echo   DEPLOYMENT COMPLETE
echo ========================================
echo.

for /f "delims=" %%i in ('gcloud run services describe !SERVICE_NAME! --platform managed --region !REGION! --format "value(status.url)"') do set SERVICE_URL=%%i

echo [OK] BrowserMCP is LIVE!
echo.
echo Service URL: !SERVICE_URL!
echo.
echo Health check: !SERVICE_URL!/health
echo MCP endpoint: !SERVICE_URL!/mcp
echo.
echo View logs: gcloud run logs read !SERVICE_NAME! --region !REGION! -n 50
echo.

pause
