#!/usr/bin/env bash
# Runs every arm of PROTOCOL.md; outputs JSONL under data/.
set -euo pipefail
cd "$(dirname "$0")"
B=tool/target/release/ic-weakpoints
mkdir -p data
PS="31 61 127 251 509 1021 4093 65521 1048573 16777213 1073741789"
( for p in $PS; do e=""; [ $p -le 127 ] && e="--enum"; $B algebra --n 3 --p $p --inst 8 $e; done > data/algebra_n3.jsonl ) &
( for p in $PS; do e=""; [ $p -le 127 ] && e="--enum"; $B algebra --n 2 --p $p --inst 8 $e; done > data/algebra_n2.jsonl
  for p in 61 127 251 509; do $B yield --p $p --curves 3 --targets 2000; done > data/yield.jsonl ) &
( for p in 31 61 127 251 509; do $B pipeline --p $p --curves 3 --rho 30; done > data/pipeline.jsonl ) &
wait
echo done
