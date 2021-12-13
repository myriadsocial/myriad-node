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

cargo +nightly run --release --locked --features=runtime-benchmarks -- benchmark \
  --chain=dev \
  --execution=wasm \
  --wasm-execution=compiled \
  --pallet="$PALLET" \
  --extrinsic="*" \
  --steps=20 \
  --repeat=10 \
  --heap-pages=4096 \
  --raw \
  --template="./.maintain/pallet-weight-template.hbs" \
  --output="./pallets/${FOLDER}/src/weights.rs"

popd
