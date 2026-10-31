#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
mkdir -p build results
flint_prefix=${FLINT_PREFIX:-/home/sage/sage/local}
for source in obstruction projective_divisor source_replay normalization_probe normalization_family_audit; do
  "${CXX:-g++}" -std=c++17 -O2 -Wall -Wextra -Werror \
    -isystem "$flint_prefix/include" -Ivendor "$source.cpp" \
    -L"$flint_prefix/lib" -Wl,-rpath,"$flint_prefix/lib" -lflint -lgmp -o "build/$source"
done
