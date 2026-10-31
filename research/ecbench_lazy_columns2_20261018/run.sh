#!/usr/bin/env bash
# Frozen run (PROTOCOL.md §4). Usage: bash research/ecbench_lazy_columns2_20261018/run.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/ecbench_lazy_columns2_20261018"
ECBENCH="${ECBENCH:-$ROOT/target/release/ecbench}"
mkdir -p "$HERE/sessions" "$HERE/audits" "$HERE/tables" "$HERE/logs"
for spec in "$ROOT"/docs/ecbench/specs/claim-koblitz-lazy-c2-*.json; do
  name=$(basename "$spec" .json); name=${name#claim-koblitz-lazy-c2-}
  out="$HERE/sessions/$name"
  [ -f "$out/session.json" ] && { echo "skip $name"; continue; }
  "$ECBENCH" run --wait --spec "$spec" --out "$out" --cpus none --allow-busy > "$HERE/logs/run-$name.log" 2>&1
  "$ECBENCH" verify --dir "$out" --replay-all --out "$HERE/audits/audit-$name.json" > "$HERE/logs/verify-$name.log" 2>&1
  "$ECBENCH" table --dir "$out" > "$HERE/tables/table-$name.md"
  base=$(jq -r '.arms[] | select(.name|test("^ic-lazy-c[0-9]+$") and . != "ic-lazy-c2") | .name' "$spec")
  set -- "ic-lazy-c2" "$base"  "rho-strong" "ic-lazy-c2"  "rho-strong" "$base"  "rho-frob" "rho-frob-aa"
  while [ $# -gt 0 ]; do
    "$ECBENCH" compare --dir "$out" --a "$1" --b "$2" --save > "$HERE/tables/compare-$name-$2-vs-$1.txt"
    shift 2
  done
  echo "done $name"
done
echo RUN-DONE
