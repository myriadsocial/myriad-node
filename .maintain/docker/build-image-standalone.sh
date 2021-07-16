#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

IMAGE_NAME=myriadsocial/myriad-node
VCS_REF=`git rev-parse --short HEAD`
VERSION=`grep -m 1 "^version" ./nodes/standalone/Cargo.toml | egrep -o "([0-9\.]+)"`
BUILD_DATE=`date -u +"%Y%m%d"`

# Copy binary
echo "Copiying binary standalone"
time cp ./target/release/myriad .

# Build the image
echo "Building ${IMAGE_NAME}:latest docker image, hang on!"
time docker build -f ./.maintain/docker/node-standalone.Dockerfile --build-arg VCS_REF=${VCS_REF} --build-arg BUILD_DATE=${BUILD_DATE} -t ${IMAGE_NAME}:latest .
# Build verison
time docker tag ${IMAGE_NAME}:latest ${IMAGE_NAME}:${VERSION}

# Remove binary
echo "Removing binary standalone"
time rm ./myriad

popd
