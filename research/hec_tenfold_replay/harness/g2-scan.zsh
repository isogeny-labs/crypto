#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
harness=$PWD/harness
stage=${1:-character}
case "$stage" in
  original|optimized|character|crossover) ;;
  *) echo 'expected original, optimized, character or crossover' >&2; exit 2 ;;
esac
build=$(mktemp -d)
out="$build/$stage"
mkdir "$out"
echo "$out"
case "$(uname -s)" in Darwin) runtime=c++ ;; *) runtime=stdc++ ;; esac
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -C codegen-units=1 -A warnings -o "$build/libhec_native_ic.rlib"
cd variants/g2
seeds=(2026111920)
case "$stage" in
  original) gen=g2-scan-completion; mode=prefix ;;
  optimized) gen=g2-scan-completion; mode=reuse ;;
  character) gen=g2-character-base; mode=character; seeds+=(2026111921) ;;
  crossover) gen=g2-character-base; mode=crossover; seeds=(2026111922) ;;
esac
ruby "$harness/$gen.rb" "$mode" | c++ -O3 -std=c++17 -x c++ - -c -o "$build/kernel.o"
case "$stage" in
  crossover) ruby "$harness/g2-character-driver.rb" ;;
  character) sed 's/if g!=2 {continue;}/if g!=2 || p>257 {continue;}/' main.rs ;;
  *) cat main.rs ;;
esac | rustc - --edition=2021 --crate-name hec_bench --extern "hec_native_ic=$build/libhec_native_ic.rlib" -C opt-level=3 -C codegen-units=1 -C lto=fat -A warnings -C "link-arg=$build/kernel.o" -l "$runtime" -o "$build/bench"
for seed in "${seeds[@]}"; do
  for order in forward reverse; do
    args=()
    [[ "$order" == reverse ]] && args+=(reverse)
    HEC_PAIR_SEED=$seed "$build/bench" "${args[@]}" > "$out/$seed-$order.log" 2>"$out/$seed-$order.audit"
  done
done
args=()
[[ "$stage" == crossover ]] && args+=(--crossover)
ruby "$harness/g2-scan-medians.rb" "${args[@]}" "$out"/*.log > "$out/medians.txt"
