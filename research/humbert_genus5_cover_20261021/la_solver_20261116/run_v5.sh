#!/usr/bin/env bash
# Thin orchestration of PROTOCOL.md (single-solve linear algebra, Lanczos, excess + clique removal).
#   BIN=<frozen copy> BIN_SHA256_PREFIX=<prefix> PRIMES="<q>" ./run_v5.sh <new-output-dir>
# writes <dir>/results/v5.jsonl. The output directory must not exist. One q per launch keeps
# each launch under the session's 2 h background limit.
set -euo pipefail
OUT_DIR=${1:?usage: run_v5.sh <new-output-dir>}
[ -e "$OUT_DIR" ] && { echo "refusing to write into existing $OUT_DIR" >&2; exit 2; }
BIN=${BIN:?BIN must name the frozen binary}
if [ -n "${BIN_SHA256_PREFIX:-}" ]; then
  sha256sum "$BIN" | grep -q "^$BIN_SHA256_PREFIX" || { echo "binary $BIN does not match $BIN_SHA256_PREFIX" >&2; exit 2; }
fi
mkdir -p "$OUT_DIR/results"
OUT="$OUT_DIR/results/v5.jsonl"
ERR="$OUT_DIR/results/v5.stderr"
ARMS=(iota lp:0.5 dlpf:0.5 dlpf:0.5 dlpf:0.5 lp:0.5 dlpx:0.5 dlpx:0.5)
SOLV=(l1 w2 w2 w1 l1 l1 l1 l1)
TAGS=(pre_l1 lp50_w2 dlpf50_w2 dlpf50_w1 dlpf50_l1 lp50_l1 dlpx50_l1 dlpx50_l1_aa)
N=${#ARMS[@]}
for p in ${PRIMES:?PRIMES must name the q values}; do
  for seed in 1 2 3 4 5; do
    tw=$(( seed % 2 == 1 ? 1 : -1 ))
    for k in $(seq 0 $((N - 1))); do
      i=$(( (seed + k) % N ))
      skip=1; [ "${TAGS[$i]}" = pre_l1 ] && skip=0
      line=$(HUMBERT_LA_PRECHECK=1 HUMBERT_LA_SOLVER=${SOLV[$i]} HUMBERT_SKIP_RHO=$skip "$BIN" attack "$p" "$tw" "$seed" "${ARMS[$i]}" 2>> "$ERR")
      echo "{\"tag\":\"${TAGS[$i]}\",\"order\":$k,${line#\{}" >> "$OUT"
    done
  done
done
