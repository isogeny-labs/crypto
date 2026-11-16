#!/bin/sh
# usage: KS_BIN=./ks_recompute_bin ./run.sh <.../research/isogeny-class-rho/runs/run-...> > ks_recompute.tsv
# Thin orchestration: jq extracts the recorded values, the native binary recomputes.
set -eu
BIN=${KS_BIN:-./ks_recompute_bin}
scratch=$(mktemp -d "${TMPDIR:-/tmp}/ks-recompute.XXXXXX")
trap 'rm -rf "$scratch"' 0
trap 'exit 1' HUP INT TERM
for r in "$1"/*/*/results.json; do
  d=$(dirname "$r"); lbl=$(basename "$(dirname "$d")")/$(basename "$d")
  jq -r '.arms[] | select(.tests.ks_d != null) | [.arm, .tests.ks_d, .tests.ks_p] | @tsv' "$r" > "$scratch/recorded.tsv"
  # A pipeline ending in awk hides a failed native replay in POSIX sh.
  # Capture it first so set -e propagates its status before publishing rows.
  "$BIN" "$d/trials.csv" "$scratch/recorded.tsv" > "$scratch/recomputed.tsv"
  awk -v l="$lbl" '{print l "\t" $0}' "$scratch/recomputed.tsv"
done
