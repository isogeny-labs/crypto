#!/bin/sh
# frozen command of PROTOCOL.md
genus_ladder run --primes 32771,65537,131101 --genera 3 --curves 3 --seeds 2 --arms c --sge 8,3 \
  --excess 0.2,0.85 --excess 0.5,0.7 --excess 1.0,0.5 --cc 2.0 --max-h 8 --threads 4 \
  --max-trials 4e8 --seed-search 600 --order-method bsgs1 --out grid.jsonl
