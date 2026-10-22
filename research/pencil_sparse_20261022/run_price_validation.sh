#!/usr/bin/env bash
set -euo pipefail
root=research/pencil_sparse_20261022
old=research/four_followups_20261021/results/pencil-pilot
inputs=()
for p in 739 1151 1187; do
  for order in identity reverse affine peel; do
    inputs+=("$root/results/p${p}-${order}.jsonl")
  done
done
for p in 3371 3527 3931 11731 11887 12143; do
  inputs+=("$old/f${p}-cohort.jsonl" "$root/results/p${p}-peel.jsonl")
done
for order in identity reverse affine peel; do
  inputs+=("$root/results/p48383-${order}-t0.jsonl")
done
for p in 48823 49451; do
  inputs+=("$root/results/p${p}-identity-t0.jsonl")
done
for order in identity peel; do
  inputs+=("$root/results/p186379-${order}-t0.jsonl")
done
cap25="$root/results/p186379-peel-cap25m-t0.jsonl"
if [[ -f "$cap25" ]] && [[ "$(wc -l < "$cap25")" -ge 2 ]]; then
  inputs+=("$cap25")
fi
"$root/target/release/price-sparse" "${inputs[@]}" > /dev/null
for p in 739 1151 1187; do
  "$root/target/release/compare-sparse" \
    "$root/results/p${p}-identity.jsonl" "$root/results/p${p}-peel.jsonl" \
    "p${p}-peel-vs-online" > /dev/null
done
for p in 3371 3527 3931 11731 11887 12143; do
  "$root/target/release/compare-sparse" \
    "$old/f${p}-cohort.jsonl" "$root/results/p${p}-peel.jsonl" \
    "p${p}-peel-vs-online" > /dev/null
done
for order in reverse affine peel; do
  stem="p48383-${order}-vs-identity"
  if [[ "$order" == peel ]]; then stem=p48383-peel-vs-online; fi
  "$root/target/release/compare-sparse" \
    "$root/results/p48383-identity-t0.jsonl" \
    "$root/results/p48383-${order}-t0.jsonl" "$stem" > /dev/null
done
"$root/target/release/validate-sparse" "${inputs[@]}" > /dev/null
