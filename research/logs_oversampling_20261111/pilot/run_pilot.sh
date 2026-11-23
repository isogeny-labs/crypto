#!/usr/bin/env bash
# 5-seed local pilot of the relation-oversampling protocol (DRAFT).
# Owner-requested local route; shared container, no isolation receipt:
# wall times are indicative only. Counted units (trials, row_ops) are exact.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
D=research/logs_oversampling_20261111
OUT=$D/pilot/rows
IC=./target/release/ic
mkdir -p "$OUT"
export RAYON_NUM_THREADS=1
COMMON=(--seeds 5 --fractions 0,0.1,0.25,0.5 --aa-control --logs-arm --json)
run() { local id=$1; shift; echo "== $id $(date -u +%FT%TZ)"; "$IC" oversample "$@" "${COMMON[@]}" --rows-out "$OUT/$id.jsonl" > "$OUT/$id.out.json"; }
run C1 --degree 23 --curve-a 0 --summands 2 --factor-index 0
run C2 --degree 37 --curve-a 0 --summands 3 --factor-base $D/fb37_ic_search_selected.json
run C3 --degree 37 --curve-a 0 --summands 3 --factor-base $D/fb37_subgroup_orbits_7400.json
run C4 --degree 37 --curve-a 0 --summands 3 --factor-base $D/fb37_subgroup_orbits_7400.json --filter-strategy legacy
echo "== done $(date -u +%FT%TZ)"
