#!/bin/bash
# 🚀 DEPLOY BROWSERMCP TO GOOGLE CLOUD RUN (Docker Method)
# Este script usa Docker en lugar de gcloud local

echo "🚀 DEPLOYING BROWSERMCP TO GOOGLE CLOUD RUN"
echo "==========================================="
echo ""

# Variables
PROJECT_ID="${1:-browsermcp-2025}"
SERVICE_NAME="browsermcp-server"
REGION="us-central1"
IMAGE_NAME="gcr.io/${PROJECT_ID}/${SERVICE_NAME}:latest"

echo "📋 Configuration:"
echo "  Project ID: $PROJECT_ID"
echo "  Service: $SERVICE_NAME"
echo "  Region: $REGION"
echo "  Image: $IMAGE_NAME"
echo ""

# Step 1: Build Docker image
echo "1️⃣  Building Docker image..."
docker build -f Dockerfile.production -t $IMAGE_NAME .

if [ $? -ne 0 ]; then
    echo "❌ Docker build failed!"
    exit 1
fi

echo "✅ Docker image built successfully!"
echo ""

# Step 2: Configure Docker auth (requires gcloud)
echo "2️⃣  Authenticating Docker with Google Cloud..."
gcloud auth configure-docker gcr.io --quiet

if [ $? -ne 0 ]; then
    echo "❌ Docker authentication failed!"
    echo "Please run: gcloud auth configure-docker gcr.io"
    exit 1
fi

echo "✅ Docker authenticated!"
echo ""

# Step 3: Push image to GCR
echo "3️⃣  Pushing image to Google Container Registry..."
docker push $IMAGE_NAME

if [ $? -ne 0 ]; then
    echo "❌ Docker push failed!"
    exit 1
fi

echo "✅ Image pushed to GCR!"
echo ""

# Step 4: Deploy to Cloud Run
echo "4️⃣  Deploying to Google Cloud Run..."
gcloud run deploy $SERVICE_NAME \
    --image $IMAGE_NAME \
    --platform managed \
    --region $REGION \
    --allow-unauthenticated \
    --memory 512Mi \
    --cpu 1 \
    --timeout 3600 \
    --max-instances 100

if [ $? -ne 0 ]; then
    echo "❌ Cloud Run deployment failed!"
    exit 1
fi

echo "✅ Deployment successful!"
echo ""

# Step 5: Get service URL
echo "5️⃣  Getting service URL..."
SERVICE_URL=$(gcloud run services describe $SERVICE_NAME \
    --platform managed \
    --region $REGION \
    --format 'value(status.url)')

echo ""
echo "🎉 SUCCESS! BrowserMCP is LIVE!"
echo "======================================"
echo "Service URL: $SERVICE_URL"
echo ""
echo "Test endpoints:"
echo "  Health: $SERVICE_URL/health"
echo "  MCP: $SERVICE_URL/mcp"
echo ""
echo "View logs:"
echo "  gcloud run logs read $SERVICE_NAME --region $REGION -n 50 --follow"
echo ""
