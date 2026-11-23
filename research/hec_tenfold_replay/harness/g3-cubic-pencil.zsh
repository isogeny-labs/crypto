#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
stage=${1:-panel}
case "$stage" in panel|rule|crossover) ;; *) echo "use panel, rule or crossover" >&2; exit 2 ;; esac
kernel=$stage; if [[ "$stage" == crossover ]]; then kernel=rule; fi
build=$(mktemp -d)
echo "$build"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
c++ -std=c++17 -O3 -Dquartic=original_quartic -c variants/g3/kernel.cpp -o "$build/orig.o"
ruby harness/g3-cubic-pencil.rb "$kernel" | c++ -std=c++17 -O3 -x c++ - -c -o "$build/dual.o"
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$build/libhec_native_ic.rlib"
cd variants/g3
ruby ../../harness/g3-cubic-driver.rb "$stage" | rustc - --edition=2021 --crate-name hec_bench --extern "hec_native_ic=$build/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$build/orig.o" -C "link-arg=$build/dual.o" -l "$runtime" -o "$build/bench"
if [[ "$stage" == rule ]]; then seeds=(2026111914 2026111915 2026111916); elif [[ "$stage" == crossover ]]; then seeds=(2026111917); else seeds=(2026111912 2026111913 2026111906); fi
for seed in "${seeds[@]}"; do
  for order in forward reverse; do
    args=()
    if [[ "$order" == reverse ]]; then args+=(reverse); fi
    HEC_PAIR_SEED="$seed" HEC_PAIRS=8 "$build/bench" "${args[@]}" > "$build/$seed-$order.log" 2>"$build/$seed-$order.audit"
  done
done
