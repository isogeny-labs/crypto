#!/usr/bin/env bash
# Thin orchestration of PROTOCOL.md (filtered double-large-prime matrices).
#   BIN=<frozen copy> BIN_SHA256_PREFIX=<prefix> ./run_v4.sh <new-output-dir>
# writes <dir>/results/v4.jsonl. The output directory must not exist. PRIMES overrides the q list.
set -euo pipefail
OUT_DIR=${1:?usage: run_v4.sh <new-output-dir>}
[ -e "$OUT_DIR" ] && { echo "refusing to write into existing $OUT_DIR" >&2; exit 2; }
BIN=${BIN:?BIN must name the frozen binary}
if [ -n "${BIN_SHA256_PREFIX:-}" ]; then
  sha256sum "$BIN" | grep -q "^$BIN_SHA256_PREFIX" || { echo "binary $BIN does not match $BIN_SHA256_PREFIX" >&2; exit 2; }
fi
mkdir -p "$OUT_DIR/results"
OUT="$OUT_DIR/results/v4.jsonl"
ERR="$OUT_DIR/results/v4.stderr"
ARMS=(iota lp:0.5 dlp:0.5 dlpm:0.5 dlpf:0.5 dlpf:0.5)
TAGS=(pre lp50 dlp50 dlpm50 dlpf50 dlpf50_aa)
for p in ${PRIMES:-8009 12007 16001 24001}; do
  for seed in 1 2 3 4 5; do
    tw=$(( seed % 2 == 1 ? 1 : -1 ))
    for k in 0 1 2 3 4 5; do
      i=$(( (seed + k) % 6 ))
      skip=1; [ "${TAGS[$i]}" = pre ] && skip=0
      line=$(HUMBERT_LA_PRECHECK=1 HUMBERT_SKIP_RHO=$skip "$BIN" attack "$p" "$tw" "$seed" "${ARMS[$i]}" 2>> "$ERR")
      echo "{\"tag\":\"${TAGS[$i]}\",\"order\":$k,${line#\{}" >> "$OUT"
    done
  done
done
