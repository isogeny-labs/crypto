#!/usr/bin/env bash
set -euo pipefail

root=$(git rev-parse --show-toplevel)
study=research/isogeny-volcano-projected-base_20261101
source_study=research/isogeny-volcano-cost-v2_20261017
manifest=$root/$study/specs/manifest.json
vs=${1:-$root/target/release/volcano-stats}
eb=${2:-$root/target/release/ecbench}
out=$root/$study/analysis
mkdir -p "$out/comparisons" "$out/bands"

records=()
while IFS= read -r cell; do
  session=$root/$study/sessions/$cell
  status=$(jq -er '.status' "$session/session.json")
  [[ $status == complete ]] || { echo "$cell: session status $status" >&2; exit 1; }
  planned=$(jq -er '.executions_planned' "$session/session.json")
  written=$(jq -er '.records_written' "$session/session.json")
  [[ $planned == "$written" ]] || { echo "$cell: $written/$planned records" >&2; exit 1; }
  record=$session/records.jsonl
  [[ -f $record ]] || record=$record.gz
  [[ -f $record ]] || { echo "$cell: records absent" >&2; exit 1; }
  records+=("$record")
  "$eb" compare --dir "$session" --a ic-raw --b ic-projected --resamples 4000 --seed 20261101 --json \
    > "$out/comparisons/$cell.json"
  for weight in 0 0.1 1; do
    "$eb" compare --dir "$session" --a ic-raw --b ic-projected --resamples 4000 --seed 20261101 \
      --weight "$weight" --json > "$out/comparisons/$cell-weight-$weight.json"
  done
  "$eb" band --dir "$session" --weights 0,0.1,1 --json "$out/bands/$cell.json" > /dev/null
done < <(jq -r '.cells[].cell' "$manifest")

curves=()
while IFS= read -r class; do
  curves+=("$root/$source_study/classes/$class/curves.jsonl")
done < <(jq -r '.cells[].class' "$manifest" | sort -u)

join_by_comma() { local IFS=,; echo "$*"; }
"$vs" --records "$(join_by_comma "${records[@]}")" --curves "$(join_by_comma "${curves[@]}")" \
  --vertical stratified --alternative less --margin 0.03 --boot 2000 --perm 10000 \
  --pair ic-projected:ic-raw \
  --out "$out/projected-volcano.json" 2> "$out/projected-volcano.log"
"$vs" --records "$(join_by_comma "${records[@]}")" --curves "$(join_by_comma "${curves[@]}")" \
  --vertical stratified --alternative less --margin 0.03 --boot 2000 --perm 10000 \
  --plant-level 3:0.95 --out "$out/positive-control-floor-095.json" \
  2> "$out/positive-control-floor-095.log"

echo "analysis complete: $out"
