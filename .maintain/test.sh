#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo "Check Code"
cargo check --all
echo "Test Code"
cargo test --all
echo "Check Lint"
cargo +nightly-2021-06-29 clippy --all -- -D warnings
echo "Check Format"
cargo +nightly-2021-06-29 fmt --all -- --check

popd
