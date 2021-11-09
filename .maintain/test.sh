#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo "Check Code"
cargo +nightly check --all
echo "Test Code"
cargo +nightly test --all
echo "Check Lint"
cargo +nightly clippy --all -- -D warnings
echo "Check Format"
cargo +nightly fmt --all -- --check

popd
