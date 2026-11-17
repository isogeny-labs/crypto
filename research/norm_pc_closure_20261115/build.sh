#!/usr/bin/env bash
set -euo pipefail
pc_repo=$(git rev-parse --show-toplevel)
pc_prefix=$(realpath "${1:?usage: build.sh PREFIX OUTDIR}")
pc_build=$(realpath -m "${2:?output required}")
mkdir -p "$pc_build"
for pc_program in produce verify; do
 g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
  -I"$pc_prefix/usr/include" "$pc_repo/research/norm_pc_closure_20261115/$pc_program.cpp" \
  -L"$pc_prefix/usr/lib/x86_64-linux-gnu" -Wl,-rpath,"$pc_prefix/usr/lib/x86_64-linux-gnu" \
  -lflint -lgmp -o "$pc_build/$pc_program"
done
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 -I"$pc_prefix/usr/include" "$pc_repo/research/macaulay_provenance_20261115/compare.cpp" \
 -L"$pc_prefix/usr/lib/x86_64-linux-gnu" -Wl,-rpath,"$pc_prefix/usr/lib/x86_64-linux-gnu" \
 -lflint -lgmp -o "$pc_build/compare"
sha256sum "$pc_repo/research/norm_pc_closure_20261115/"*.cpp \
 "$pc_repo/research/norm_pc_closure_20261115/"*.hpp "$pc_build/"{produce,verify,compare} > "$pc_build/source-binary.sha256"
g++ --version > "$pc_build/compiler.txt"
