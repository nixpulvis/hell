#!/bin/bash

cd 2/take5/
cargo test
cd ../take5_cli/
cargo test
cd ../../3/take5_ext
cargo test
cd ../../4/remote
cargo test
cd ../evolution
cargo test
