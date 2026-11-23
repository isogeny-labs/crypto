#!/bin/sh
set -eu

variant=${1:-g2}
order=${2:-forward}
case "$variant" in g2|g2-registered|g3|g4) ;; *) echo "unknown variant $variant" >&2; exit 2 ;; esac
case "$order" in forward|reverse) ;; *) echo "unknown order $order" >&2; exit 2 ;; esac

cd "$(dirname "$0")"
build=$(mktemp -d "${TMPDIR:-/tmp}/hec-replay.XXXXXXXX")
trap 'rm -rf "$build"' EXIT
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib \
    -C opt-level=3 -C codegen-units=1 -A warnings -o "$build/libhec_native_ic.rlib"
"${CXX:-c++}" -std=c++17 -O3 -c "variants/$variant/kernel.cpp" -o "$build/kernel.o"
rustc "variants/$variant/main.rs" --edition=2021 --crate-name hec_bench \
    --extern "hec_native_ic=$build/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 \
    -C lto=fat -A warnings -C "link-arg=$build/kernel.o" -l "$runtime" -o "$build/bench"
if [ "$order" = reverse ]; then "$build/bench" reverse; else "$build/bench"; fi
