#!/usr/bin/env bash
# Thin orchestration of PROTOCOL.md (levers: fast relations, threaded LA, safe clique removal).
#   BIN=<frozen copy> BIN_SHA256_PREFIX=<prefix> PRIMES="<q>" [SEEDS="1 2 3 4 5"] ./run_v6.sh <dir>
# appends to <dir>/results/v6.jsonl (creates it if missing). One q per launch.
set -euo pipefail
OUT_DIR=${1:?usage: run_v6.sh <output-dir>}
BIN=${BIN:?BIN must name the frozen binary}
sha256sum "$BIN" | grep -q "^${BIN_SHA256_PREFIX:?}" || { echo "binary $BIN does not match $BIN_SHA256_PREFIX" >&2; exit 2; }
mkdir -p "$OUT_DIR/results"
OUT="$OUT_DIR/results/v6.jsonl"; ERR="$OUT_DIR/results/v6.stderr"
#        tag        arm       extra environment (on top of PRECHECK=1 SOLVER=l1 SKIP_RHO=1)
TAGS=(base        rel                 la4                 comb                              comb_aa                           lp_comb                           dlpxs10                                                        dlpxs05                                                        dlpxs02)
ARMS=(dlpf:0.5    dlpf:0.5            dlpf:0.5            dlpf:0.5                          dlpf:0.5                          lp:0.5                            dlpx:0.5                                                       dlpx:0.5                                                       dlpx:0.5)
ENVS=("" "HUMBERT_REL_FAST=1" "HUMBERT_LA_THREADS=4" "HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4" "HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4" "HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4" "HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4 HUMBERT_CLIQUE_SAFE=1 HUMBERT_EXCESS=0.10" "HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4 HUMBERT_CLIQUE_SAFE=1 HUMBERT_EXCESS=0.05" "HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4 HUMBERT_CLIQUE_SAFE=1 HUMBERT_EXCESS=0.02")
N=${#TAGS[@]}
for p in ${PRIMES:?}; do
  for seed in ${SEEDS:-1 2 3 4 5}; do
    tw=$(( seed % 2 == 1 ? 1 : -1 ))
    for k in $(seq 0 $((N - 1))); do
      i=$(( (seed + k) % N ))
      line=$(env HUMBERT_LA_PRECHECK=1 HUMBERT_LA_SOLVER=l1 HUMBERT_SKIP_RHO=1 ${ENVS[$i]} "$BIN" attack "$p" "$tw" "$seed" "${ARMS[$i]}" 2>> "$ERR")
      echo "{\"tag\":\"${TAGS[$i]}\",\"order\":$k,\"load1\":$(cut -d' ' -f1 /proc/loadavg),${line#\{}" >> "$OUT"
    done
  done
done
