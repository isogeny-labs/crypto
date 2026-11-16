#!/bin/sh
# PROTOCOL.md sections 5-9, thin orchestration only: every figure is computed
# by examples/coincidence_ladder.rs. Run from anywhere; THREADS sets workers.
# Raw per-curve outputs (main/*.tsv) are large and are not committed; the run
# is a pure function of the labels in the source, so this script regenerates
# them byte for byte (main/SHA256SUMS records their hashes).
set -eu
cd "$(dirname "$0")"
SRC=../../examples/coincidence_ladder.rs
BIN=${CL_BIN:-bin/coincidence_ladder}
THREADS=${THREADS:-6}
RUNGS="16 20 24 28 32"
PILOT=8000000
MAIN=8100000
REP=8200000
mkdir -p bin pilot main audits analysis
[ -x "$BIN" ] || rustc -O -o "$BIN" "$SRC"
shasum -a 256 "$BIN" "$SRC" > BINARY.sha256
"$BIN" selftest > audits/selftest.txt
for b in $RUNGS; do
  "$BIN" census --bits "$b" --block "$PILOT" --n30 40 --n35 40 --threads "$THREADS" --out "pilot/pilot-$b.tsv"
done
"$BIN" design pilot/pilot-*.tsv > design.json
census() { # block outdir suffix
  for b in $RUNGS; do
    n30=$(jq -r --argjson b "$b" '.[] | select(.bits == $b and .theta == 0.3) | .n_main' design.json)
    n35=$(jq -r --argjson b "$b" '.[] | select(.bits == $b and .theta == 0.35) | .n_main' design.json)
    "$BIN" census --bits "$b" --block "$1" --n30 "$n30" --n35 "$n35" --threads "$THREADS" \
      --verify-every 97 --out "$2/$3-$b.tsv"
  done
}
census "$MAIN" main main
# determinism: the 24-bit rung again with a different worker count
n30=$(jq -r '.[] | select(.bits == 24 and .theta == 0.3) | .n_main' design.json)
n35=$(jq -r '.[] | select(.bits == 24 and .theta == 0.35) | .n_main' design.json)
"$BIN" census --bits 24 --block "$MAIN" --n30 "$n30" --n35 "$n35" --threads 2 --out audits/rerun-24.tsv
cmp main/main-24.tsv audits/rerun-24.tsv
echo "rerun-24 identical to main-24 ($(shasum -a 256 < main/main-24.tsv | cut -d' ' -f1))" > audits/determinism.txt
rm audits/rerun-24.tsv
shasum -a 256 main/*.tsv > main/SHA256SUMS
"$BIN" analyze main/main-*.tsv > analysis/analysis.json
"$BIN" stage1 stage1/ca5e01-cells.tsv > analysis/stage1.json
if [ "$(jq -r .reading analysis/analysis.json)" = ALIVE ]; then
  mkdir -p rep
  census "$REP" rep rep
  shasum -a 256 rep/*.tsv > rep/SHA256SUMS
  "$BIN" analyze rep/rep-*.tsv > analysis/analysis-rep.json
fi
