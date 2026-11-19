#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
mode=${1:-all}
case "$mode" in all) seed=2026111923 ;; positive-only) seed=2026111924 ;; *) exit 2 ;; esac
b=$(mktemp -d)
echo "$b"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$b/libhec_native_ic.rlib"
cp results/g3-transfer-curves.txt "$b/curves"
ruby harness/g3-character-solve.rb "$b/curves" "$mode" | c++ -std=c++17 -O3 -x c++ - -c -o "$b/kernels.o"
cd variants/g3
ruby ../../harness/g3-character-solve.rb driver "$b/curves" | rustc - --edition=2021 --crate-name hec_bench --extern "hec_native_ic=$b/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$b/kernels.o" -l "$runtime" -o "$b/bench"
for stage in panel crossover; do
  if [[ "$stage" == crossover ]]; then export HEC_CROSSOVER=1; else unset HEC_CROSSOVER; fi
  for order in forward reverse; do
    args=()
    if [[ "$order" == reverse ]]; then args+=(reverse); fi
    HEC_PAIR_SEED=$seed HEC_PAIRS=8 "$b/bench" "${args[@]}" > "$b/$stage-$order.log" 2>"$b/$stage-$order.audit"
  done
done
ruby ../../harness/g3-transfer-medians.rb "$b"/*.log > "$b/metrics"
