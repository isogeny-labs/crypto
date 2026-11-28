#!/bin/sh
cd "$(dirname "$0")"
for p in 251 509 1021 2039; do "$BIN" yield $p 3 20261121 results/yield_$p.jsonl "$GIT" > logs/yield_$p.log 2>&1; done
