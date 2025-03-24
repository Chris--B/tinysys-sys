#!/bin/bash

function do_loudly() {
    echo "+ $*"
    $*
}

set -e

# Our build script generates the rs files
riscv64-unknown-elf-gcc --version
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(riscv64-unknown-elf-gcc -print-sysroot)"
do_loudly cargo build --features=bindgen $*
do_loudly cargo fmt
