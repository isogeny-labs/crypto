#!/usr/bin/env bash
# Band outputs with and without `--price-table-work`, pooled per study and
# size, over the committed sessions of the two F = 1 studies.  Analysis of
# committed records only: nothing is measured, so the benchrec hook is told so.
set -euo pipefail
HERE="$(cd "$(dirname "$0")/.." && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
ECBENCH="${ECBENCH:-${CARGO_TARGET_DIR:-$ROOT/target}/release/ecbench}"
export BENCHREC_SKIP="band analysis of committed sessions, no new measurement"
mkdir -p "$HERE/data"
for study in curve25519_f1_targetwalk_20261108:targetwalk curve25519_f1_walk_20261107:walk; do
  dir="${study%%:*}"; tag="${study##*:}"
  for k in 20 24; do
    dirs=("$ROOT"/research/"$dir"/sessions/k${k}-s*)
    "$ECBENCH" band --dir "${dirs[@]}" --ranks \
      --json "$HERE/data/band-$tag-k$k.json" > "$HERE/data/band-$tag-k$k.txt"
    "$ECBENCH" band --dir "${dirs[@]}" --ranks --price-table-work \
      --json "$HERE/data/band-$tag-k$k-table-work.json" > "$HERE/data/band-$tag-k$k-table-work.txt"
  done
done
sha256sum "$ECBENCH" > "$HERE/data/binary.sha256"
(cd "$ROOT" && git rev-parse HEAD) > "$HERE/data/source-commit.txt"
