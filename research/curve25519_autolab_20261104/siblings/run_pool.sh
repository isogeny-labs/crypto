#!/bin/sh
# Sibling pool amendment. Run from the repository root after
# `cargo build --release --bin nc2_stage0`. Seed 20261108, n = 2000.
set -e
B=./target/release/nc2_stage0
O=research/curve25519_autolab_20261104/siblings/raw
K20=131203,131317,131111; K24=2097349,2097559,2097523; K28=33558163,33554761,33558059
for k in 20 24 28; do
  eval R=\$K$k
  $B --r $R --c 1,2 --n 2000 --seed 20261108 --pool 64 --siblings 4,8 > $O/pool64-k$k.txt
  $B --r $R --c 1,2 --n 2000 --seed 20261108 --pool 256 --siblings 8,16 > $O/pool256-k$k.txt
done
