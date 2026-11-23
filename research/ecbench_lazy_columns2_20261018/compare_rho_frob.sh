#!/usr/bin/env bash
# Read-out added after the report, for the progress chart (docs/ic/progress-timeline.json):
# each IC arm against the fully priced signed-Frobenius rho (rho-frob) of the SAME recorded
# session, paired by target. A derived read-out of the recorded runs; no new measurement.
# Usage: bash research/ecbench_lazy_columns2_20261018/compare_rho_frob.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/ecbench_lazy_columns2_20261018"
ECBENCH="${ECBENCH:-$ROOT/target/release/ecbench}"
for out in "$HERE"/sessions/*/; do
  name=$(basename "$out")
  for ic in $(jq -r '.arms[] | select(.name|test("^ic-lazy-c[0-9]+$")) | .name' "$ROOT"/docs/ecbench/specs/claim-koblitz-lazy-c2-"$name".json); do
    "$ECBENCH" compare --dir "$out" --a rho-frob --b "$ic" --save > "$HERE/tables/compare-$name-$ic-vs-rho-frob.txt"
  done
done
echo COMPARE-DONE
