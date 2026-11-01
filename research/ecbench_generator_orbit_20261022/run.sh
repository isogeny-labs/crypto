#!/usr/bin/env bash
# Frozen claim run (PROTOCOL.md §4). Usage: bash research/ecbench_generator_orbit_20261022/run.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/ecbench_generator_orbit_20261022"
ECBENCH="${ECBENCH:-$ROOT/target/release/ecbench}"
mkdir -p "$HERE/sessions" "$HERE/audits" "$HERE/tables" "$HERE/logs"
for spec in "$ROOT"/docs/ecbench/specs/claim-koblitz-generator-orbit-*.json; do
  name=$(basename "$spec" .json); name=${name#claim-koblitz-generator-orbit-}
  out="$HERE/sessions/$name"
  [ -f "$out/session.json" ] && { echo "skip $name"; continue; }
  "$ECBENCH" run --wait --spec "$spec" --out "$out" --cpus none --allow-busy > "$HERE/logs/run-$name.log" 2>&1
  "$ECBENCH" verify --dir "$out" --replay-all --out "$HERE/audits/audit-$name.json" > "$HERE/logs/verify-$name.log" 2>&1
  "$ECBENCH" table --dir "$out" > "$HERE/tables/table-$name.md"
  "$ECBENCH" band --dir "$out" --json "$HERE/tables/band-$name.json" > "$HERE/tables/band-$name.md"
  lazy=$(jq -r '.arms[] | select(.name|test("^ic-lazy-c")) | .name' "$spec")
  layout=$(jq -r '.arms[] | select(.name|test("^ic-layout-c")) | .name' "$spec")
  gen=$(jq -r '.arms[] | select(.name|test("^ic-gen-layout-c")) | .name' "$spec")
  # pairs: --a A --b B, ratio B / A
  set -- "$layout" "$lazy"  "$gen" "$layout"  "$gen" "$lazy"  "rho-frob" "$gen"  "rho-strong" "$gen" \
         "rho-frob" "$lazy"  "rho-frob" "rho-frob-aa"
  while [ $# -gt 0 ]; do
    "$ECBENCH" compare --dir "$out" --a "$1" --b "$2" --save > "$HERE/tables/compare-$name-$2-vs-$1.txt"
    shift 2
  done
  echo "done $name"
done
echo RUN-DONE
