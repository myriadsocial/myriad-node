#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ] ; then
   time rustup update nightly
   time rustup update stable
fi

time rustup target add wasm32-unknown-unknown --toolchain nightly
