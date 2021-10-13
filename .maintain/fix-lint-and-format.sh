#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo "Fix Lint"
cargo +nightly-2021-06-29 clippy --all
echo "Fix Format"
cargo +nightly-2021-06-29 fmt --all

popd
