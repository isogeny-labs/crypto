#!/bin/sh
# usage: KS_BIN=./ks_recompute_bin ./run.sh <.../research/isogeny-class-rho/runs/run-...> > ks_recompute.tsv
# Thin orchestration: jq extracts the recorded values, the native binary recomputes.
set -e
BIN=${KS_BIN:-./ks_recompute_bin}
for r in "$1"/*/*/results.json; do
  d=$(dirname "$r"); lbl=$(basename "$(dirname "$d")")/$(basename "$d")
  jq -r '.arms[] | select(.tests.ks_d != null) | [.arm, .tests.ks_d, .tests.ks_p] | @tsv' "$r" > /tmp/ks_rec.$$.tsv
  $BIN "$d/trials.csv" /tmp/ks_rec.$$.tsv | awk -v l="$lbl" '{print l "\t" $0}'
done
rm -f /tmp/ks_rec.$$.tsv
