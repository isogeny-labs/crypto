#!/usr/bin/env bash
set -euo pipefail
norm_core_root=$(git rev-parse --show-toplevel)
norm_core_prefix=$(realpath "${1:?usage: build.sh PREFIX OUTDIR}")
norm_core_build=$(realpath -m "${2:?output directory required}")
bash "$norm_core_root/research/shared_norm_algebraic_20261115/build.sh" "$norm_core_prefix" "$norm_core_build"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
    -I"$norm_core_prefix/usr/include" \
    "$norm_core_root/research/norm_core_elimination_20261115/core.cpp" \
    -L"$norm_core_prefix/usr/lib/x86_64-linux-gnu" \
    -Wl,-rpath,"$norm_core_prefix/usr/lib/x86_64-linux-gnu" \
    -lflint -lgmp -o "$norm_core_build/core"
sha256sum "$norm_core_root/research/norm_core_elimination_20261115/core.cpp" \
    "$norm_core_root/research/shared_norm_algebraic_20261115/"*.cpp \
    "$norm_core_build/core" "$norm_core_prefix/usr/bin/msolve" > "$norm_core_build/core-source-binary.sha256"
