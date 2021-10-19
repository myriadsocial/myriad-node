#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

IMAGE_NAME=myriadsocial/myriad-node

# Copy binary
echo "Copiying binary"
time cp ./target/release/myriad .

# Build the image
echo "Building ${IMAGE_NAME}:latest docker image, hang on!"
time docker build -f ./.maintain/docker/Dockerfile -t ${IMAGE_NAME}:latest .

# Remove binary
echo "Removing binary"
time rm ./myriad

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${IMAGE_NAME}

popd
