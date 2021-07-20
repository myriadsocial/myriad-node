#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo "Check Code"
time cargo check --all
echo "Test Code"
time cargo test --all
echo "Check Lint"
time cargo clippy --all -- -D warnings
echo "Check Format"
time cargo fmt --all -- --check
