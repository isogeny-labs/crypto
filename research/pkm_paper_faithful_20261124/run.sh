#!/usr/bin/env bash
# Exact commands for research/pkm_paper_faithful_20261124/README.md.
# Binary: ic SHA-256 06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59
set -euo pipefail
IC=${IC:-/Volumes/SSD990-2/crypto/target/release/ic}
OUT=$(cd "$(dirname "$0")" && pwd)/results

run() { # bits t m linalg targets max_trials name
  "$IC" bench --bits "$1" --family generic \
    --factor-base "prime-isogeny:t=$2,seed=0,aux=ordinary" \
    --oracle "pkm-semaev:m=$3,engine=f4,degree=8,mode=chain" \
    --targets "$5" --linalg "$4" \
    --max-trials "$6" --solver-budget-seconds 30 --json > "$OUT/$7.json"
}

# |V| = 2^t = sqrt(p) on the roster curves (research_bench.rs bench_curves).
run 10 5 2 incremental-gauss-full-rank-checked walk  600  r10t5
run 12 6 2 incremental-gauss-full-rank-checked walk 2000  r12t6
run 14 7 2 incremental-gauss-full-rank-checked walk  400  r14t7
run 14 7 2 incremental-gauss                 random 2000 r14t7_random
run 16 8 2 incremental-gauss                 walk  1000  r16t8_pinned
run 16 8 2 incremental-gauss-full-rank-checked walk 1000 r16t8_fullrank
run 18 9 2 incremental-gauss                 walk  1200  r18t9
run 12 4 3 incremental-gauss                 walk  2000  r12t4_m3

# The construction dump for verify.py (u64 pilot, same isogeny_tower code):
#   pkm_tower_pilot --p 100003 --kinds isogeny --m 2 --t-min 5 --max-t 5 \
#     --planted 1 --random 0 --budget 20 --seed 1 --out pilot_iso.json
