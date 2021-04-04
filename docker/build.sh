#!/usr/bin/env bash
set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

GITUSER=myriadsocial
GITREPO=myriad-node
VCS_REF=`git rev-parse --short HEAD`
VERSION=`grep "^version" ./node/Cargo.toml | egrep -o "([0-9\.]+)"`
BUILD_DATE=`date -u +"%Y%m%d"`

# Build the image
echo "Building ${GITUSER}/${GITREPO}:latest docker image, hang on!"
time docker build -f ./docker/Dockerfile --build-arg PROFILE=release --build-arg VCS_REF=${VCS_REF} --build-arg VERSION=${VERSION} --build-arg BUILD_DATE=${BUILD_DATE} -t ${GITUSER}/${GITREPO}:latest .

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${GITREPO}

echo -e "\nIf you just built version ${VERSION}, you may want to update your tag:"
echo " $ docker tag ${GITUSER}/${GITREPO}:$VERSION ${GITUSER}/${GITREPO}:${VERSION}"

popd