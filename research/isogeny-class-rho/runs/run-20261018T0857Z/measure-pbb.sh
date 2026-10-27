#!/bin/bash
# Frozen measurements for P1, P0, B1 (COMMANDS.md), run sequentially.
cd /home/user/crypto
R=research/isogeny-class-rho/runs/run-20261018T0857Z
B=target/release/isoclass-rho
run() { out=$1; shift; mkdir -p $out; echo "START $(date -u +%FT%TZ) $*" >> $R/measure-pbb.progress; "$@" > $out.log 2>&1; echo "EXIT $? $(date -u +%FT%TZ) $out" >> $R/measure-pbb.progress; }
run $R/P1/class      $B measure --walk $R/P1/walk.json --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/P1/class
run $R/P1/generators $B measure --walk $R/P1/walk.json --mode generators --generators 100 --max-vertices 6 --trials 25 --controls 0 --seed 20261018 --out $R/P1/generators
run $R/P1/shift      $B measure --walk $R/P1/walk.json --mode shift      --trials 25 --seed 20261018 --out $R/P1/shift
run $R/P0/class      $B measure --walk $R/P0/walk.json --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/P0/class
run $R/P0/generators $B measure --walk $R/P0/walk.json --mode generators --generators 100 --max-vertices 4 --trials 25 --controls 0 --seed 20261018 --out $R/P0/generators
run $R/P0/shift      $B measure --walk $R/P0/walk.json --mode shift      --trials 25 --seed 20261018 --out $R/P0/shift
run $R/B1/class      $B measure --walk $R/B1/walk.json --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/B1/class
run $R/B1/generators $B measure --walk $R/B1/walk.json --mode generators --generators 64 --max-vertices 6 --trials 25 --controls 0 --seed 20261018 --out $R/B1/generators
run $R/B1/shift      $B measure --walk $R/B1/walk.json --mode shift      --trials 25 --seed 20261018 --out $R/B1/shift
# replay checks: class mode at 5 seeds must reproduce the recorded iteration counts
for c in P1 P0 B1; do run $R/$c/replay $B measure --walk $R/$c/walk.json --mode class --trials 5 --controls 3 --plants --folds --seed 20261018 --out $R/$c/replay; done
echo "ALL DONE $(date -u +%FT%TZ)" >> $R/measure-pbb.progress
