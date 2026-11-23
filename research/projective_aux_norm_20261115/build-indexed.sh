#!/usr/bin/env bash
set -euo pipefail
idx_repo=$(git rev-parse --show-toplevel)
idx_prefix=$(realpath "${1:?usage: build-indexed.sh PREFIX NEW_BUILD}")
idx_build=$(realpath -m "${2:?fresh output directory}")
mkdir "$idx_build"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror -I"$idx_prefix/usr/include" \
 "$idx_repo/research/projective_aux_norm_20261115/indexed.cpp" \
 -L"$idx_prefix/usr/lib/x86_64-linux-gnu" -Wl,-rpath,"$idx_prefix/usr/lib/x86_64-linux-gnu" \
 -lflint -lgmp -o "$idx_build/produce"
sha256sum "$idx_repo/research/projective_aux_norm_20261115/indexed.cpp" \
 "$idx_build/produce" > "$idx_build/build-source-binary.sha256"
g++ --version > "$idx_build/compiler.txt"
