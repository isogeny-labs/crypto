#!/bin/sh
# Sibling-points Stage 0. Run from the repository root after
# `cargo build --release --bin nc2_stage0`. Seed 20261108 (default), n = 2000.
set -e
B=./target/release/nc2_stage0
O=research/curve25519_autolab_20261104/siblings/raw
mkdir -p $O
$B --r 131203,131317,131111 --c 1,2 --n 2000 --seed 20261108 --siblings 1,2,4,8,16 > $O/k20.txt
$B --r 2097349,2097559,2097523 --c 1,2 --n 2000 --seed 20261108 --siblings 1,2,4,8 > $O/k24.txt
$B --r 33558163,33554761,33558059 --c 1,2 --n 2000 --seed 20261108 --siblings 1,2,4,8 > $O/k28.txt
