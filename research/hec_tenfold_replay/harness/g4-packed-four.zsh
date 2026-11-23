#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
build=$(mktemp -d)
echo "$build"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$build/libhec_native_ic.rlib"
cp results/g4-packed-four-curves.txt "$build/curves"
ruby harness/g4-packed-four.rb library "$build/curves" | c++ -std=c++17 -O3 -x c++ - -c -o "$build/kernel.o"
cd variants/g4
ruby ../../harness/g4-transfer.rb driver "$build/curves" | rustc - --edition=2021 --crate-name hec_bench --extern "hec_native_ic=$build/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$build/kernel.o" -l "$runtime" -o "$build/bench"
seed=2026111932; pairs=16
if [[ ${1:-} == expanded ]]; then seed=2026111933; pairs=32; fi
for order in forward reverse; do
  args=()
  [[ $order == reverse ]] && args+=(reverse)
  HEC_PAIRS=$pairs HEC_PAIR_SEED=$seed "$build/bench" "${args[@]}" > "$build/$order.log" 2>"$build/$order.audit"
done
