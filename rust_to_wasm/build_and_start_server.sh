#!/bin/bash

set -x

rustc --target=wasm32-unknown-unknown -O -o output.wasm wasm_module.rs && python3 -m http.server
