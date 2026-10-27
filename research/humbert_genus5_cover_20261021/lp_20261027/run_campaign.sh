#!/usr/bin/env bash
# Thin orchestration of the frozen grid in PROTOCOL.md. Appends JSON lines.
# Usage: PRIMES="2003 3001" ./run_campaign.sh
set -euo pipefail
cd "$(dirname "$0")"
BIN=${BIN:-../target/release/humbert}
[ -n "${BIN_FROZEN:-}" ] || (cd .. && cargo build --release --quiet)
mkdir -p results
OUT=results/campaign.jsonl
ARMS=(iota iota lp:0.5 lp:0.35)
TAGS=(baseline aa lp50 lp35)
for p in ${PRIMES:-2003 3001 4001 6007 8009 12007}; do
  for seed in 1 2 3 4 5; do
    tw=$(( seed % 2 == 1 ? 1 : -1 ))
    for k in 0 1 2 3; do
      i=$(( (seed + k) % 4 ))
      arm=${ARMS[$i]}; tag=${TAGS[$i]}
      skip=1; [ "$tag" = baseline ] && skip=0
      line=$(HUMBERT_SKIP_RHO=$skip "$BIN" attack "$p" "$tw" "$seed" "$arm" 2>> results/campaign.stderr)
      echo "{\"tag\":\"$tag\",\"order\":$k,${line#\{}" >> "$OUT"
    done
  done
done
