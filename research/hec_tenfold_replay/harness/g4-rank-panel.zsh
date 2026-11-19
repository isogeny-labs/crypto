#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
build=$(mktemp -d)
echo "$build"
gp -q harness/g4-rank-panel.gp > "$build/curves"
cmp "$build/curves" results/g4-rank-panel-curves.txt
awk '$3=="selected"{print $1,$2,$5}' "$build/curves" > "$build/selected"
for mode in census extend screen; do
  options=()
  case "$mode" in extend) options=(extend);; screen) options=(extend screen);; esac
  ruby harness/g4-rank-panel.rb "${options[@]}" | c++ -std=c++17 -O3 -x c++ - -o "$build/$mode"
  "$build/$mode" < "$build/selected" > "$build/$mode.out"
done
cmp "$build/census.out" results/g4-rank-panel.txt
for mode in extend screen; do
  awk '$1=="VALUES"' "$build/$mode.out" | cmp - results/g4-rank-panel-values.txt
  variant=rank
  [[ $mode == screen ]] && variant=screen
  awk '$1=="EXT"' "$build/$mode.out" | cmp - "results/g4-two-point-$variant.txt"
done
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -A warnings -o "$build/libhec_native_ic.rlib"
rustc harness/g4-rank-verify.rs --edition=2021 -C opt-level=3 --extern "hec_native_ic=$build/libhec_native_ic.rlib" -o "$build/verify"
"$build/verify" "$build/curves" results/g4-rank-panel-values.txt
ruby harness/g4-two-point-time.rb "$build/extend" "$build/screen" > "$build/time"
