#!/usr/bin/env bash
# Diagnostic (not study data): per-step F4 traces of the two m = 3 systems whose counts matched to 0.03%
# (XB-M3-A2-t5-tower, XB-M3-A23-n4-tower). Same arguments and seed; --trace 1 adds the step log, which is
# free in the counted multiplications, so each total must equal the committed one exactly.
set -uo pipefail
cd /work; export RAYON_NUM_THREADS=4; mkdir -p runs/trace
sha256sum pkm_odd_tower_f4 > runs/trace/binary.sha256
A="--p 1399680001 --m 3 --control tower --kind refute --draws 1 --seed 20261114 --budget 86400 --dmax 64 --trace 1"
./pkm_odd_tower_f4 $A --degrees 2,2,2,2,2 --cell TR-M3-A2-t5 --out runs/trace/TR-M3-A2-t5.jsonl > /dev/null 2> runs/trace/a.err &
./pkm_odd_tower_f4 $A --degrees 2,3,2,3 --cell TR-M3-A23-n4 --out runs/trace/TR-M3-A23-n4.jsonl > /dev/null 2> runs/trace/b.err &
wait
echo "trace end $(date -u +%FT%TZ)" >> runs/trace/progress.txt
