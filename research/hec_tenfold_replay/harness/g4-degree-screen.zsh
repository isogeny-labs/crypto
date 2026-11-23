#!/bin/zsh
cd "$(dirname "$0")/.."
set -euo pipefail
build=$(mktemp -d)
echo "$build"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
ruby harness/g4-degree-screen.rb "${1:-mask}" | c++ -std=c++17 -O3 -x c++ - -c -o "$build/four.o"
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$build/libhec_native_ic.rlib"
cd variants/g4
sed -e 's/let answer=candidate::solve(c,\&d,\&target,ell,i==2,audit);/extern "C" {fn select_screen(on:i32);} unsafe{select_screen((i==2) as i32);} let answer=candidate::solve(c,\&d,\&target,ell,true,audit);/' main.rs | rustc - --edition=2021 --crate-name hec_bench --extern "hec_native_ic=$build/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$build/four.o" -l "$runtime" -o "$build/bench"
seeds=(2026111927 2026111928); pairs=8
if [[ ${2:-} == expanded ]]; then seeds=(2026111929); pairs=32; fi
for seed in "${seeds[@]}"; do
for order in forward reverse; do
  args=()
  if [[ "$order" = reverse ]]; then args+=(reverse); fi
  HEC_PAIRS=$pairs HEC_PAIR_SEED=$seed "$build/bench" "${args[@]}" > "$build/$seed-$order.log" 2>"$build/$seed-$order.audit"
done
done
