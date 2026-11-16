#!/usr/bin/env bash
# Thin orchestration of the frozen grid in PROTOCOL.md (v1).
#   ./run_campaign.sh <new-output-dir>      writes <dir>/results/campaign.jsonl
# The output directory must not exist: a reproduction never appends to the
# committed evidence in ./results. BIN defaults to the crate's release build
# (built here); set BIN to a frozen copy, and BIN_SHA256_PREFIX to check it.
# PRIMES overrides the grid's q list. Then: ../target/release/humbert summarize-lp <dir>
set -euo pipefail
OUT_DIR=${1:?usage: run_campaign.sh <new-output-dir>}
[ -e "$OUT_DIR" ] && { echo "refusing to write into existing $OUT_DIR" >&2; exit 2; }
HERE="$(cd "$(dirname "$0")" && pwd)"
if [ -z "${BIN:-}" ]; then
  (cd "$HERE/.." && cargo build --release --quiet)
  BIN="$HERE/../target/release/humbert"
fi
if [ -n "${BIN_SHA256_PREFIX:-}" ]; then
  sha256sum "$BIN" | grep -q "^$BIN_SHA256_PREFIX" || { echo "binary $BIN does not match $BIN_SHA256_PREFIX" >&2; exit 2; }
fi
mkdir -p "$OUT_DIR/results"
OUT="$OUT_DIR/results/campaign.jsonl"
ERR="$OUT_DIR/results/campaign.stderr"
ARMS=(iota iota lp:0.5 lp:0.35)
TAGS=(baseline aa lp50 lp35)
for p in ${PRIMES:-2003 3001 4001 6007 8009 12007}; do
  for seed in 1 2 3 4 5; do
    tw=$(( seed % 2 == 1 ? 1 : -1 ))
    for k in 0 1 2 3; do
      i=$(( (seed + k) % 4 ))
      arm=${ARMS[$i]}; tag=${TAGS[$i]}
      skip=1; [ "$tag" = baseline ] && skip=0
      line=$(HUMBERT_SKIP_RHO=$skip "$BIN" attack "$p" "$tw" "$seed" "$arm" 2>> "$ERR")
      echo "{\"tag\":\"$tag\",\"order\":$k,${line#\{}" >> "$OUT"
    done
  done
done
