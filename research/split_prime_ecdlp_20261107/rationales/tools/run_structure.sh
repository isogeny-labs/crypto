#!/usr/bin/env bash
# Structure stage for every landed P-192 class number: Sylow invariants and
# subgroup orders/indices from rust_<label>.json (h) via classgroup-structure.
# Idempotent: skips labels whose structure file exists.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../.." && pwd)"
BIN="${CLASSGROUP_STRUCTURE_BIN:-$ROOT/target/release/classgroup-structure}"
INPUTS="$ROOT/research/nist_skip_criterion_20261025/inputs.tsv"
OUT="$HERE/../data/cl"
for res in "$OUT"/rust_p*.json; do
  [ -s "$res" ] || continue
  lab="$(basename "$res" .json)"; lab="${lab#rust_}"
  [ -s "$OUT/structure_${lab}.json" ] && { echo "$lab: structure exists"; continue; }
  grep -q '"accepted": true' "$res" || { echo "$lab: class number not accepted, skipping"; continue; }
  h="$(grep -o '"h": [0-9]*' "$res" | head -1 | cut -d' ' -f2)"
  line="$(grep -P "^${lab}\t" "$INPUTS")"
  p="$(printf '%s' "$line" | cut -f6)"; n="$(printf '%s' "$line" | cut -f8)"
  "$BIN" --label "$lab" --p "$p" --n "$n" --h "$h" --bound 2000 --sub-bounds 50,163,691,2000 \
    --out "$OUT/structure_${lab}.json" > /dev/null 2>> "$OUT/structure_${lab}.log"
  echo "$lab: $(grep -o '"two_rank_genus": [0-9]*\|"smallest_split_prime_bound_generating": [0-9a-z]*' "$OUT/structure_${lab}.json" | tr '\n' ' ')"
done
