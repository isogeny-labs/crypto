#!/bin/zsh
cd "$(dirname "$0")/.."
set -euo pipefail
build=$(mktemp -d)
echo "$build"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
c++ -std=c++17 -O3 -Dquartic=original_quartic -c variants/g4/kernel.cpp -o "$build/original.o"
ruby harness/g4-fourth-pencil.rb library | c++ -std=c++17 -O3 -x c++ - -c -o "$build/four.o"
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$build/libhec_native_ic.rlib"
cd variants/g4
sed -e 's/let answer=candidate::solve(c,\&d,\&target,ell,i==2,audit);/extern "C" {fn select_fourth(on:i32);} unsafe{select_fourth((i==2) as i32);} let answer=candidate::solve(c,\&d,\&target,ell,true,audit);/' main.rs | rustc - --edition=2021 --crate-name hec_bench --extern "hec_native_ic=$build/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$build/original.o" -C "link-arg=$build/four.o" -l "$runtime" -o "$build/bench"
for order in forward reverse; do
  args=()
  if [[ "$order" = reverse ]]; then args+=(reverse); fi
  HEC_PAIRS=8 HEC_PAIR_SEED=2026111908 "$build/bench" "${args[@]}" > "$build/$order.log" 2>"$build/$order.audit"
done
