#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
b=$(mktemp -d)
echo "$b"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
ruby harness/g3-transfer-kernels.rb results/g3-transfer-curves.txt > "$b/kernels.cpp"
patch "$b/kernels.cpp" < harness/g3-projection.patch
c++ -std=c++17 -O3 -c "$b/kernels.cpp" -o "$b/kernels.o"
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$b/libhec_native_ic.rlib"
cd variants/g3
ruby ../../harness/g3-transfer-driver.rb ../../results/g3-transfer-curves.txt | rustc - --edition=2021 --crate-name projected --extern "hec_native_ic=$b/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$b/kernels.o" -l "$runtime" -o "$b/bench"
for seed in 2026111956 2026111957; do
  out="$b/$seed"
  mkdir "$out"
  for stage in crossover panel; do
    if [[ "$stage" == crossover ]]; then export HEC_CROSSOVER=1; else unset HEC_CROSSOVER; fi
    for order in forward reverse; do
      args=()
      if [[ "$order" == reverse ]]; then args+=(reverse); fi
      HEC_PAIR_SEED=$seed HEC_PAIRS=8 "$b/bench" "${args[@]}" > "$out/$stage-$order.log" 2> "$out/$stage-$order.audit"
    done
  done
  ruby ../../harness/g3-transfer-medians.rb "$out"/*.log > "$out/medians.txt"
done
