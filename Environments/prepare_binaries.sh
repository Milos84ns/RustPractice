#!/usr/bin/env bash

set -e
set -x
binary_name=environments
dist_dir=../target/dist
cargo build --release --bin "$binary_name"
rm -rf "$dist_dir" || 0
mkdir "$dist_dir"
cp ../target/release/"$binary_name" "$dist_dir"
cp ./exampleini.ini "$dist_dir"
