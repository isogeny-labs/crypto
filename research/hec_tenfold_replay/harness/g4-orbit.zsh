#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
b=$(mktemp -d)
echo "$b"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$b/libhec_native_ic.rlib"
rustc harness/g4-orbit-check.rs --edition=2021 --extern "hec_native_ic=$b/libhec_native_ic.rlib" -o "$b/check"
"$b/check"
ruby harness/g4-hermite-solve.rb fixtures carry | awk '$1==37 && $2==0' > "$b/curves"
for mode in packed order; do
  o="$b/$mode"
  mkdir "$o"
  ruby harness/g4-orbit.rb "$mode" | c++ -std=c++17 -O3 -x c++ - -c -o "$o/kernel.o"
  (cd variants/g4
   ruby ../../harness/g4-hermite-solve.rb driver "$b/curves" carry | rustc - --edition=2021 --crate-name orbit --extern "hec_native_ic=$b/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$o/kernel.o" -l "$runtime" -o "$o/bench")
  seeds=(2026111978 2026111979)
  if [[ $mode == order ]]; then seeds=(2026111981 2026111982); fi
  for seed in "${seeds[@]}"; do
    for order in forward reverse; do
      args=()
      if [[ $order == reverse ]]; then args=(reverse); fi
      HEC_PAIR_SEED=$seed HEC_PAIRS=32 "$o/bench" "${args[@]}" > "$o/$seed-$order.log" 2>"$o/$seed-$order.audit"
    done
  done
done
