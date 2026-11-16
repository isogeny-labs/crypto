#!/bin/bash
# Frozen command card for run-20261019-n45 (PROTOCOL.md section 3). Run from the repository root.
set -u
R=research/isogeny-class-rho-n45/runs/run-20261019-n45
B=target/release/isoclass-rho
mkdir -p $R/P2
echo "start $(date -u +%FT%TZ)" > $R/P2/progress.txt
$B walk --curve 46339417407469,4184642753251,42214793703111 --ells 2,3,5,7,11,13,17,19 --hard-ells 23,29,31,37,41,43,47,53,59,61 --hard-from 3 --radius 3 --max-vertices 5000 --per-stratum 3 --hard 4 --sources 2 --seed 20261018 --out $R/P2 > $R/P2/walk.log 2>&1 || { echo "walk failed $(date -u +%FT%TZ)" >> $R/P2/progress.txt; exit 1; }
echo "walk done $(date -u +%FT%TZ)" >> $R/P2/progress.txt
$B measure --walk $R/P2/walk.json --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/P2/class > $R/P2/class.log 2>&1
echo "class done $(date -u +%FT%TZ)" >> $R/P2/progress.txt
$B measure --walk $R/P2/walk.json --mode generators --generators 20 --max-vertices 2 --trials 10 --controls 0 --seed 20261018 --out $R/P2/generators > $R/P2/generators.log 2>&1
echo "generators done $(date -u +%FT%TZ)" >> $R/P2/progress.txt
$B measure --walk $R/P2/walk.json --mode class      --trials 5 --controls 3 --plants --folds --seed 20261018 --out $R/P2/replay > $R/P2/replay.log 2>&1
echo "replay done $(date -u +%FT%TZ)" >> $R/P2/progress.txt
