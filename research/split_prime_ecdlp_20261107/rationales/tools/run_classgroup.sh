#!/usr/bin/env bash
# Exact class groups of the P-192 Frobenius fields with the library's native
# relation method (crypto::isogeny::class_group_subexp, binary `classgroup`).
# Usage: tools/run_classgroup.sh [label ...]   (default: the five P-192 census curves)
# Output: data/cl/rust_<label>.json, one JSON object each; stderr log beside it.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../.." && pwd)"
BIN="${CLASSGROUP_BIN:-$ROOT/target/release/classgroup}"
INPUTS="$ROOT/research/nist_skip_criterion_20261025/inputs.tsv"
OUT="$HERE/../data/cl"
mkdir -p "$OUT"
labels=("$@")
if [ ${#labels[@]} -eq 0 ]; then labels=(p192-139 p192-2198 p192-5228 p192-5405 p192-6013); fi
for lab in "${labels[@]}"; do
  line="$(grep -P "^${lab}\t" "$INPUTS")"
  p="$(printf '%s' "$line" | cut -f6)"
  n="$(printf '%s' "$line" | cut -f8)"
  "$BIN" --label "$lab" --p "$p" --n "$n" --threads "${THREADS:-4}" --seed "${SEED:-1}" \
    ${BOUND:+--bound "$BOUND"} ${HALF:+--half-length "$HALF"} ${SUPPORT:+--support "$SUPPORT"} \
    --sub-bounds 50,163,691,2000 --out "$OUT/rust_${lab}.json" 2> "$OUT/rust_${lab}.log"
  echo "$lab: $(python3 -I -c "import json,sys; d=json.load(open('$OUT/rust_${lab}.json')); print('h_bits', d['h_bits'], 'ratio', d['ratio_to_analytic'], 'accepted', d['accepted'], 'wall', d['wall_s']['total'])")"
done
