#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

IMAGE_NAME=myriadsocial/myriad-node

echo "Start building"
time ./.maintain/docker/build-image-standalone.sh && ./.maintain/docker/build-image-standalone.sh && ./.maintain/docker/build-image-standalone.sh
echo "Finish build"

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${IMAGE_NAME}

popd
