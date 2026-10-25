#!/bin/bash

cargo bundle
echo "== Bundling Done =="
rustc bundle.rs
echo "== Compiled Bundle =="
echo "== Running Binary =="
cargo run main
