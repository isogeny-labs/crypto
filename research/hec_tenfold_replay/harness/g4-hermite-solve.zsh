#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
b=$(mktemp -d);echo "$b"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
ruby harness/g4-hermite-solve.rb fixtures "${1:-screen}" > "$b/fixtures"
ruby harness/g4-hermite-solve.rb kernel "${1:-screen}" | c++ -std=c++17 -O3 -x c++ - -c -o "$b/kernel.o"
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$b/libhec_native_ic.rlib"
cd variants/g4
ruby ../../harness/g4-hermite-solve.rb driver "$b/fixtures" "${1:-screen}" | rustc - --edition=2021 --crate-name hec_bench --extern "hec_native_ic=$b/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$b/kernel.o" -l "$runtime" -o "$b/bench"
for order in forward reverse; do
  args=()
  [[ $order == reverse ]] && args+=(reverse)
  HEC_PAIRS=${HEC_PAIRS:-4} HEC_PAIR_SEED=${HEC_PAIR_SEED:-2026111934} "$b/bench" "${args[@]}" > "$b/$order.log" 2>"$b/$order.audit"
done
