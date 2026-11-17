#!/usr/bin/env bash
# Frozen run (PROTOCOL.md). Usage: ECBENCH=target/release/ecbench bash research/rho_frob_brent_20261106/run.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/rho_frob_brent_20261106"
ECBENCH="${ECBENCH:-$ROOT/target/release/ecbench}"
mkdir -p "$HERE/sessions" "$HERE/audits" "$HERE/tables" "$HERE/logs"
for spec in "$HERE"/specs/rho-frob-brent-*.json; do
  name=$(basename "$spec" .json); name=${name#rho-frob-brent-}
  out="$HERE/sessions/$name"
  [ -f "$out/session.json" ] && { echo "skip $name"; continue; }
  "$ECBENCH" run --wait --spec "$spec" --out "$out" --cpus none --allow-busy > "$HERE/logs/run-$name.log" 2>&1
  "$ECBENCH" verify --dir "$out" --replay-all --out "$HERE/audits/audit-$name.json" > "$HERE/logs/verify-$name.log" 2>&1
  "$ECBENCH" table --dir "$out" > "$HERE/tables/table-$name.md"
  for pair in "rho-frob-v2 rho-frob" "rho-frob-aa rho-frob"; do
    set -- $pair
    "$ECBENCH" compare --dir "$out" --a "$1" --b "$2" --save > "$HERE/tables/compare-$name-$2-vs-$1.txt"
  done
  echo "done $name"
done
echo RUN-DONE
