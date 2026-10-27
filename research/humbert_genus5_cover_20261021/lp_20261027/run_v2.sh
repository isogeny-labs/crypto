#!/usr/bin/env bash
# Thin orchestration of PROTOCOL_v2.md with the frozen binary. Appends JSON lines.
set -euo pipefail
cd "$(dirname "$0")"
BIN=bin/humbert-v2
mkdir -p results
OUT=results/v2.jsonl
ARMS=(iota iota iota lp:0.5)
TAGS=(v1 pre pre_aa lp50pre)
PRE=(0 1 1 1)
for p in ${PRIMES:-4001 6007 8009 12007}; do
  for seed in 1 2 3 4 5; do
    tw=$(( seed % 2 == 1 ? 1 : -1 ))
    for k in 0 1 2 3; do
      i=$(( (seed + k) % 4 ))
      skip=1; [ "${TAGS[$i]}" = v1 ] && skip=0
      line=$(HUMBERT_LA_PRECHECK=${PRE[$i]} HUMBERT_SKIP_RHO=$skip "$BIN" attack "$p" "$tw" "$seed" "${ARMS[$i]}" 2>> results/v2.stderr)
      echo "{\"tag\":\"${TAGS[$i]}\",\"order\":$k,${line#\{}" >> "$OUT"
    done
  done
done
