#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
mkdir -p build results
flint_prefix=${FLINT_PREFIX:-/home/sage/sage/local}
if (( $# )); then sources=("$@"); else sources=(construction_test rr_test); fi
for source in "${sources[@]}"; do
  case "$source" in construction_test|rr_test|sections_test|transfer_test) ;; *) echo 'unsupported native build target' >&2; exit 2 ;; esac
  "${CXX:-g++}" -std=c++17 -O2 -Wall -Wextra -Werror \
    -isystem "$flint_prefix/include" -I../native_followup/vendor "$source.cpp" \
    -L"$flint_prefix/lib" -Wl,-rpath,"$flint_prefix/lib" -lflint -lgmp -ldl -o "build/$source"
done
