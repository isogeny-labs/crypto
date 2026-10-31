#!/bin/bash
# Frozen measurements for K (COMMANDS.md), run sequentially, then the replay check.
cd /home/user/crypto
R=research/isogeny-class-rho/runs/run-20261018T0857Z
B=target/release/isoclass-rho
run() { out=$1; shift; mkdir -p $out; echo "START $(date -u +%FT%TZ) $*" >> $R/measure-k.progress; "$@" > $out.log 2>&1; echo "EXIT $? $(date -u +%FT%TZ) $out" >> $R/measure-k.progress; }
run $R/K/class      $B measure --walk $R/K/walk.json --mode class      --trials 100 --controls 3 --plants --folds --seed 20261018 --out $R/K/class
run $R/K/generators $B measure --walk $R/K/walk.json --mode generators --generators 64 --max-vertices 3 --trials 25 --controls 0 --seed 20261018 --out $R/K/generators
run $R/K/shift      $B measure --walk $R/K/walk.json --mode shift      --trials 25 --seed 20261018 --out $R/K/shift
run $R/K/replay     $B measure --walk $R/K/walk.json --mode class      --trials 5 --controls 3 --plants --folds --seed 20261018 --out $R/K/replay
echo "ALL DONE $(date -u +%FT%TZ)" >> $R/measure-k.progress
