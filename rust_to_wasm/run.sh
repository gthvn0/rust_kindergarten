#!/bin/bash

set -x
rustc --target=wasm32-unknown-unknown -O -o output.wasm wasm_module.rs && node test_it.js
