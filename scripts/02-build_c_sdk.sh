#!/bin/bash

function do_loudly() {
    echo "+ $*"
    $*
}

set -e

SDK_PATH=$(realpath tinysys_c_sdk)

mkdir -p target/tinysys_build
OUTDIR=$(realpath target/tinysys_build)
pushd $OUTDIR >/dev/null

echo "[~] SDK_PATH"
echo "    + $SDK_PATH"
echo "[~] OUTDIR"
echo "    + $OUTDIR"

# Print sorted file list so we feel like hackers 😎
echo "[~] Compiling C files"
for file in $(find $SDK_PATH/SDK -type f -name "*.c" | sort); do
    if [[ -f "$file" ]]; then
        echo "    + ${file#$SDK_PATH/}"
    fi
done

echo "+ riscv64-unknown-elf-g++ ..."
# NOTE: Keep these in sync with the flags in build.rs
riscv64-unknown-elf-g++   \
    -I $SDK_PATH/SDK                \
    $SDK_PATH/SDK/*.c               \
    -g -O0 -Wall                    \
    -mcmodel=medany -std=c++20      \
    --param "min-pagesize=0"        \
    --param "l1-cache-line-size=64" \
    --param "l1-cache-size=16"      \
    -march=rv32im_zicsr_zifencei_zfinx \
    -mabi=ilp32                     \
    -Wl,-gc-sections                \
    -c

echo "+ riscv64-unknown-elf-ar ..."
riscv64-unknown-elf-ar      -rc $SDK_PATH/libtinysys_sdk.a *.o

riscv64-unknown-elf-objdump -tC $SDK_PATH/libtinysys_sdk.a > $SDK_PATH/objdump.txt
