#!/usr/bin/bash

cd tools/generate-ast
cargo run --release ../../src
cd ../..