#!/bin/sh

rm -rf src
svd2rust --target riscv < svd/CH56Xxx.svd
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
