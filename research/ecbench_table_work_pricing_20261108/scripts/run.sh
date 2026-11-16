#!/usr/bin/env bash
# Band outputs under both readings, pooled per study and size, over the
# committed sessions of the two F = 1 studies: `band-<study>-k<k>` is the
# pre-2026-11-08 reading (`--no-price-table-work`, formerly the default) and
# `band-<study>-k<k>-table-work` the default since 2026-11-08 (table work
# priced, formerly `--price-table-work`).  Analysis of committed records
# only: nothing is measured, so the benchrec hook is told so.
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
    "$ECBENCH" band --dir "${dirs[@]}" --ranks --no-price-table-work \
      --json "$HERE/data/band-$tag-k$k.json" > "$HERE/data/band-$tag-k$k.txt"
    "$ECBENCH" band --dir "${dirs[@]}" --ranks \
      --json "$HERE/data/band-$tag-k$k-table-work.json" > "$HERE/data/band-$tag-k$k-table-work.txt"
  done
done
sha256sum "$ECBENCH" > "$HERE/data/binary.sha256"
# Run before committing, this names the parent of the commit that lands the
# outputs (a file cannot carry its own commit's hash).
(cd "$ROOT" && echo "$(git rev-parse HEAD) (parent); the binary was built from the working tree of the commit that lands this file") > "$HERE/data/source-commit.txt"
