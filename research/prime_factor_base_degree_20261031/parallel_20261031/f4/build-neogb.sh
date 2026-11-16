#!/usr/bin/env bash
set -euo pipefail
source_dir=${1:?usage: build-neogb.sh PINNED_MSOLVE_SOURCE OUTPUT_DIR}
output_dir=${2:?usage: build-neogb.sh PINNED_MSOLVE_SOURCE OUTPUT_DIR}
study_dir=$(cd -- "$(dirname -- "$0")" && pwd)
expected=3672113ee2c696a2ac9c637636e0f7214f500ba1
test "$(git -C "$source_dir" rev-parse HEAD)" = "$expected"
git -C "$source_dir" diff --exit-code -- src/neogb src/msolve/streams.h >/dev/null
mkdir -p "$output_dir"
gcc -std=gnu11 -O3 -march=native -DHAVE_AVX2 -mavx2 -fopenmp \
    -I"$source_dir/src/neogb" "$study_dir/neogb_driver.c" \
    "$source_dir/src/neogb/gb.c" -lgmp -lm -o "$output_dir/neogb-driver"
sha256sum "$output_dir/neogb-driver"
