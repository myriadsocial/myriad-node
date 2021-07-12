#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo "Starting node..."
./target/debug/myriad-appchain \
-d .local/appchain \
--dev
