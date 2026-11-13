#!/usr/bin/env bash
# Thin orchestration of PROTOCOL.md (double large primes).
#   ./run_v3.sh <new-output-dir>      writes <dir>/results/v3.jsonl
# The output directory must not exist. BIN defaults to the crate's release build
# (built here); the recorded campaign used a frozen copy with sha256 prefix
# a8ff53874ff869a9 (BIN=<copy> BIN_SHA256_PREFIX=a8ff53874ff869a9).
# PRIMES overrides the q list.
set -euo pipefail
OUT_DIR=${1:?usage: run_v3.sh <new-output-dir>}
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
OUT="$OUT_DIR/results/v3.jsonl"
ERR="$OUT_DIR/results/v3.stderr"
ARMS=(iota iota lp:0.5 dlp:0.5 dlp:0.35)
TAGS=(pre pre_aa lp50 dlp50 dlp35)
for p in ${PRIMES:-4001 8009 12007 16001}; do
  for seed in 1 2 3 4 5; do
    tw=$(( seed % 2 == 1 ? 1 : -1 ))
    for k in 0 1 2 3 4; do
      i=$(( (seed + k) % 5 ))
      skip=1; [ "${TAGS[$i]}" = pre ] && skip=0
      line=$(HUMBERT_LA_PRECHECK=1 HUMBERT_SKIP_RHO=$skip "$BIN" attack "$p" "$tw" "$seed" "${ARMS[$i]}" 2>> "$ERR")
      echo "{\"tag\":\"${TAGS[$i]}\",\"order\":$k,${line#\{}" >> "$OUT"
    done
  done
done
