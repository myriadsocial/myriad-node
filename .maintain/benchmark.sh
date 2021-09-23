#!/usr/bin/env bash

set -e

if [ "$#" -ne 1 ]; then
	echo "Please provide pallet name"
	exit 1
fi


pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

PALLET=$1

./target/release/myriad benchmark \
  --chain=dev \
  --execution=wasm \
  --wasm-execution=compiled \
  --pallet="$PALLET" \
  --extrinsic="*" \
  --steps=20 \
  --repeat=10 \
  --heap-pages=4096 \
  --raw \
  --output="./runtime/src/weights/${PALLET/-/_}.rs"

popd
