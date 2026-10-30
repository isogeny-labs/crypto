#!/usr/bin/env bash
# Thin orchestration of PROTOCOL_v2.md.
#   ./run_v2.sh <new-output-dir>      writes <dir>/results/v2.jsonl
# The output directory must not exist: a reproduction never appends to the
# committed evidence in ./results. BIN defaults to the crate's release build
# (built here). The recorded campaign used a frozen copy of that build with
# sha256 prefix 8251f2c37883cea8: to reproduce exactly, build at the recorded
# commit, copy the binary, and pass BIN=<copy> BIN_SHA256_PREFIX=8251f2c37883cea8.
# PRIMES overrides the grid's q list. Then: ../target/release/humbert summarize-lp <dir>
set -euo pipefail
OUT_DIR=${1:?usage: run_v2.sh <new-output-dir>}
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
OUT="$OUT_DIR/results/v2.jsonl"
ERR="$OUT_DIR/results/v2.stderr"
ARMS=(iota iota iota lp:0.5)
TAGS=(v1 pre pre_aa lp50pre)
PRE=(0 1 1 1)
for p in ${PRIMES:-4001 6007 8009 12007}; do
  for seed in 1 2 3 4 5; do
    tw=$(( seed % 2 == 1 ? 1 : -1 ))
    for k in 0 1 2 3; do
      i=$(( (seed + k) % 4 ))
      skip=1; [ "${TAGS[$i]}" = v1 ] && skip=0
      line=$(HUMBERT_LA_PRECHECK=${PRE[$i]} HUMBERT_SKIP_RHO=$skip "$BIN" attack "$p" "$tw" "$seed" "${ARMS[$i]}" 2>> "$ERR")
      echo "{\"tag\":\"${TAGS[$i]}\",\"order\":$k,${line#\{}" >> "$OUT"
    done
  done
done
