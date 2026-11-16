#!/usr/bin/env bash
set -euo pipefail
mca_repo=$(git rev-parse --show-toplevel)
mca_prefix=$(realpath "${1:?usage: build.sh PREFIX OUTDIR}")
mca_build=$(realpath -m "${2:?output required}")
mkdir -p "$mca_build"
for mca_program in produce verify compare; do
 g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
  -I"$mca_prefix/usr/include" "$mca_repo/research/macaulay_provenance_20261115/$mca_program.cpp" \
  -L"$mca_prefix/usr/lib/x86_64-linux-gnu" -Wl,-rpath,"$mca_prefix/usr/lib/x86_64-linux-gnu" \
  -lflint -lgmp -o "$mca_build/$mca_program"
done
sha256sum "$mca_repo/research/macaulay_provenance_20261115/"*.cpp \
 "$mca_repo/research/macaulay_provenance_20261115/"*.hpp \
 "$mca_build/produce" "$mca_build/verify" "$mca_build/compare" > "$mca_build/source-binary.sha256"
g++ --version > "$mca_build/compiler.txt"
