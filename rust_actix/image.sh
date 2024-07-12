#!/bin/bash

# Define variables
IMAGE_NAME="actix-imdb"
PROJECT_ID="network-optimization-428815"
TAG="latest" # Replace with your specific tag if needed

# Navigate to the directory containing the Dockerfile
# cd /path/to/your/dockerfile

# Build the Docker image
docker build -t $IMAGE_NAME .

# Tag the Docker image for GCR
docker tag $IMAGE_NAME gcr.io/$PROJECT_ID/$IMAGE_NAME:$TAG

# Push the image to Google Container Registry
docker push gcr.io/$PROJECT_ID/$IMAGE_NAME:$TAG

echo "Docker image $IMAGE_NAME:$TAG pushed to GCR successfully."