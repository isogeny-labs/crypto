#!/usr/bin/env bash
# Thin orchestration of PROTOCOL.md (slope_20261120).
#   BIN=<frozen copy> BIN_SHA256_PREFIX=<prefix> PRIMES="<q>" [SEEDS="1 2 3 4 5"] ./run_v8.sh <dir>
set -euo pipefail
OUT_DIR=${1:?usage: run_v8.sh <output-dir>}
BIN=${BIN:?}
sha256sum "$BIN" | grep -q "^${BIN_SHA256_PREFIX:?}" || { echo "binary $BIN does not match $BIN_SHA256_PREFIX" >&2; exit 2; }
mkdir -p "$OUT_DIR/results"; OUT="$OUT_DIR/results/v8.jsonl"; ERR="$OUT_DIR/results/v8.stderr"
IC="HUMBERT_LA_PRECHECK=1 HUMBERT_LA_SOLVER=l1 HUMBERT_SKIP_RHO=1 HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4 HUMBERT_SETUP_FAST=1"
R7="HUMBERT_RHO_ONLY=1 HUMBERT_RHO_THREADS=4 HUMBERT_RHO_BATCH=64 HUMBERT_RHO_FASTMUL=1 HUMBERT_RHO_NEG=1"
TAGS=(best7 best8 best8_aa d25_8 d50_8 lp50_8 rho_m7 rho_m8)
ARMS=(dlpf:0.35 dlpf:0.35 dlpf:0.35 dlpf:0.25 dlpf:0.5 lp:0.5 iota iota)
ENVS=("$IC" "$IC HUMBERT_LA_FASTMOD=1" "$IC HUMBERT_LA_FASTMOD=1" "$IC HUMBERT_LA_FASTMOD=1" "$IC HUMBERT_LA_FASTMOD=1" "$IC HUMBERT_LA_FASTMOD=1"
      "$R7" "HUMBERT_RHO_ONLY=1 HUMBERT_RHO_THREADS=4 HUMBERT_RHO_BATCH=256 HUMBERT_RHO_FASTMUL=1 HUMBERT_RHO_NEG=1 HUMBERT_RHO_SIMD=1")
N=${#TAGS[@]}
for p in ${PRIMES:?}; do
  for seed in ${SEEDS:-1 2 3 4 5}; do
    tw=$(( seed % 2 == 1 ? 1 : -1 ))
    for k in $(seq 0 $((N - 1))); do
      i=$(( (seed + k) % N ))
      line=$(env ${ENVS[$i]} "$BIN" attack "$p" "$tw" "$seed" "${ARMS[$i]}" 2>> "$ERR")
      echo "{\"tag\":\"${TAGS[$i]}\",\"order\":$k,\"load1\":$(cut -d' ' -f1 /proc/loadavg),${line#\{}" >> "$OUT"
    done
  done
done
