#!/usr/bin/env bash

set -e

if [ -z "$1" ]; then
	echo "Please provide pallet name"
	exit 1
fi

if [ -z "$2" ]; then
	echo "Please provide folder name on pallets"
	exit 1
fi

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

PALLET=$1
FOLDER=$2

cargo +nightly run --release --locked --features=runtime-benchmarks -- benchmark pallet \
  --chain=dev \
  --execution=wasm \
  --wasm-execution=compiled \
  --pallet="$PALLET" \
  --extrinsic="*" \
  --steps=50 \
  --repeat=20 \
  --heap-pages=4096 \
  --template="./.maintain/pallet-weight-template.hbs" \
  --output="./pallets/${FOLDER}/src/weights.rs"

popd
