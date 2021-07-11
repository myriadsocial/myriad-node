#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo "Starting node..."
./target/debug/myriad-parachain \
-d .local/parachain \
--dev \
--force-authoring \
--parachain-id 2000 \
-- \
--execution wasm \
--chain rococo
