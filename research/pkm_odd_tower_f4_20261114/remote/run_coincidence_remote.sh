#!/usr/bin/env bash
# Diagnostic (not study data): is the m = 3 S = 32 / S = 36 F4-count match structural or chance?
# Same two shapes under a different seed (curve, g, target) at P1, and the original seed at P2.
set -uo pipefail
cd /work; export RAYON_NUM_THREADS=4; mkdir -p runs/coinc
sha256sum pkm_odd_tower_f4 > runs/coinc/binary.sha256
B="--m 3 --control tower --kind refute --draws 1 --budget 86400 --dmax 64"
./pkm_odd_tower_f4 $B --p 1399680001 --seed 77 --degrees 2,2,2,2,2 --cell CO-P1-s77-A2-t5 --out runs/coinc/CO-P1-s77-A2-t5.jsonl > /dev/null 2>&1 &
./pkm_odd_tower_f4 $B --p 1399680001 --seed 77 --degrees 2,3,2,3 --cell CO-P1-s77-A23-n4 --out runs/coinc/CO-P1-s77-A23-n4.jsonl > /dev/null 2>&1 &
./pkm_odd_tower_f4 $B --p 3265920001 --seed 20261114 --degrees 2,2,2,2,2 --cell CO-P2-A2-t5 --out runs/coinc/CO-P2-A2-t5.jsonl > /dev/null 2>&1 &
./pkm_odd_tower_f4 $B --p 3265920001 --seed 20261114 --degrees 2,3,2,3 --cell CO-P2-A23-n4 --out runs/coinc/CO-P2-A23-n4.jsonl > /dev/null 2>&1 &
wait
echo "coinc end $(date -u +%FT%TZ)" >> runs/coinc/progress.txt
