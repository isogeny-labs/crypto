#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
variant=${1:-mask}
case "$variant" in scan|mask) ;; *) echo 'use scan or mask' >&2; exit 2 ;; esac
b=$(mktemp -d)
echo "$b"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$b/libhec_native_ic.rlib"
cp results/g3-transfer-curves.txt "$b/curves"
ruby harness/g3-pivot-exchange.rb "$b/curves" "$variant" | c++ -std=c++17 -O3 -x c++ - -c -o "$b/kernels.o"
ruby harness/g3-pivot-exchange.rb "$b/curves" "$variant" audit | c++ -std=c++17 -O3 -x c++ - -o "$b/ops"
"$b/ops" > "$b/ops.txt"
cd variants/g3
ruby ../../harness/g3-transfer-driver.rb "$b/curves" | rustc - --edition=2021 --crate-name hec_bench --extern "hec_native_ic=$b/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$b/kernels.o" -l "$runtime" -o "$b/bench"
for stage in panel crossover; do
  if [[ "$stage" == crossover ]]; then export HEC_CROSSOVER=1; else unset HEC_CROSSOVER; fi
  for order in forward reverse; do
    args=()
    if [[ "$order" == reverse ]]; then args+=(reverse); fi
    HEC_PAIR_SEED=2026111919 HEC_PAIRS=8 "$b/bench" "${args[@]}" > "$b/$stage-$order.log" 2>"$b/$stage-$order.audit"
  done
done
ruby ../../harness/g3-transfer-medians.rb "$b"/*.log > "$b/metrics"
