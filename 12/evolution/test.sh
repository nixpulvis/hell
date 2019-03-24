#!/bin/bash

echo "Testing crate - evolution"
cargo test

cd evolution-wire
echo "Testing crate - evolution-wire"
cargo test
