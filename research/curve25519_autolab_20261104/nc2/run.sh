#!/bin/sh
# Exact command lines of the Stage 0 run (binary: cargo build --release --bin nc2_stage0).
# Same seed everywhere; D3 budget fixed before the run: 12 restarts x 50 steps, 1000 training samples.
B=../../../target/release/nc2_stage0
COMMON="--c 1,2 --n 2000 --seed 20261108 --train 1000 --restarts 12 --steps 50"
$B --r 131203,131317,131111 --d1 0.5,4 $COMMON > out-k20.txt
$B --r 2097349,2097559,2097523 --d1 1.0,16 $COMMON > out-k24.txt
$B --r 33558163,33554761,33558059 --d1 1.0,8 $COMMON > out-k28.txt
# K = 1 cross-check (C = 0) and 20000-sample control precision, no search
for r in 131203 2097349 33558163; do
  $B --r $r --c 0,1,2 --n 20000 --seed 20261108 --no-d3 > out-controls-$r.txt
done
