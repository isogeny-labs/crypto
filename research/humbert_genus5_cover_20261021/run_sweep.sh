#!/usr/bin/env bash
# Thin orchestration: build once, run the frozen grid, append JSON lines.
# Usage: ./run_sweep.sh verify|attack|classify
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release --quiet
BIN=./target/release/humbert
mkdir -p results
case "${1:-}" in
  classify)
    "$BIN" classify > results/classify.jsonl ;;
  verify)
    out=results/verify.jsonl; : > "$out"
    for p in 17 19 23; do
      for tw in 1 -1; do
        for seed in 1 2; do
          "$BIN" verify "$p" "$tw" "$seed" >> "$out"
        done
      done
    done ;;
  attack)
    out=results/attack.jsonl
    for p in ${PRIMES:-61 101 151 211 307 401 503 701 1009 1511 2003}; do
      for seed in 1 2 3; do
        tw=$(( seed % 2 == 1 ? 1 : -1 ))
        for arm in full iota; do
          "$BIN" attack "$p" "$tw" "$seed" "$arm" >> "$out" 2>> results/attack.stderr
        done
      done
    done ;;
  *) echo "usage: $0 verify|attack|classify" >&2; exit 2 ;;
esac
