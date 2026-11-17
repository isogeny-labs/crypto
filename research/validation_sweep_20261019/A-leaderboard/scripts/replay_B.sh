#!/bin/sh
# Native re-execution of the Round-5 prime and random-binary ladders with the frozen configuration
# (seed 123212651130, 3 repeats, the config block of the ledger) and comparison of every priced count.
# usage: replay_B.sh <ic-binary> <outdir>      (repo root)
IC="$1"; OUT="$2"
RAYON_NUM_THREADS=1 "$IC" boundary --regime prime  --prime-bits 10,12,14,16,18,20,22,24 --repeats 3 --seed 123212651130 --json --out "$OUT/prime.json"  > /dev/null 2> "$OUT/prime.err"
RAYON_NUM_THREADS=1 "$IC" boundary --regime char2  --char2-degrees 15,18,21,24,27 --repeats 3 --seed 123212651130 --json --out "$OUT/char2.json" > /dev/null 2> "$OUT/char2.err"
