#!/usr/bin/env bash
# Same per-run command and arm order as run_v5.sh, for part of a grid (addendum to PROTOCOL.md,
# 2026-11-16: q = 24001 hit the 2 h background limit after 34 of 40 runs, so the rest of that q
# and every seed of q = 32003 run as separate launches).
#   BIN=<frozen copy> BIN_SHA256_PREFIX=<prefix> PRIMES=<q> SEEDS="<s...>" K_FROM=<k> \
#     RESUMED=<0|1> ./run_v5_part.sh <dir>
# Appends to <dir>/results/v5.jsonl; for the first seed listed, starts at protocol position K_FROM.
set -euo pipefail
OUT_DIR=${1:?usage: run_v5_part.sh <dir>}
BIN=${BIN:?BIN must name the frozen binary}
sha256sum "$BIN" | grep -q "^${BIN_SHA256_PREFIX:?}" || { echo "binary $BIN does not match $BIN_SHA256_PREFIX" >&2; exit 2; }
mkdir -p "$OUT_DIR/results"
OUT="$OUT_DIR/results/v5.jsonl"
ERR="$OUT_DIR/results/v5.stderr"
ARMS=(iota lp:0.5 dlpf:0.5 dlpf:0.5 dlpf:0.5 lp:0.5 dlpx:0.5 dlpx:0.5)
SOLV=(l1 w2 w2 w1 l1 l1 l1 l1)
TAGS=(pre_l1 lp50_w2 dlpf50_w2 dlpf50_w1 dlpf50_l1 lp50_l1 dlpx50_l1 dlpx50_l1_aa)
N=${#ARMS[@]}
EXTRA=""; [ "${RESUMED:-0}" = 1 ] && EXTRA="\"resumed\":true,"
p=${PRIMES:?}
first=1
for seed in ${SEEDS:?}; do
  tw=$(( seed % 2 == 1 ? 1 : -1 ))
  k0=0; [ $first = 1 ] && k0=${K_FROM:-0}; first=0
  for k in $(seq "$k0" $((N - 1))); do
    i=$(( (seed + k) % N ))
    skip=1; [ "${TAGS[$i]}" = pre_l1 ] && skip=0
    line=$(HUMBERT_LA_PRECHECK=1 HUMBERT_LA_SOLVER=${SOLV[$i]} HUMBERT_SKIP_RHO=$skip "$BIN" attack "$p" "$tw" "$seed" "${ARMS[$i]}" 2>> "$ERR")
    echo "{\"tag\":\"${TAGS[$i]}\",\"order\":$k,${EXTRA}${line#\{}" >> "$OUT"
  done
done
