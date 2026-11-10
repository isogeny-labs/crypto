#!/bin/sh
# SNC-2 Stage 0 exact commands (run from the repository root; seed 20261108 everywhere).
B=target/release/nc2_stage0
O=research/curve25519_autolab_20261104/snc/raw
$B --chained --r 131203,131317,131111      --n 2000 --seed 20261108 > $O/snc2-k20.txt
$B --chained --r 2097349,2097559,2097523   --n 2000 --seed 20261108 > $O/snc2-k24.txt
$B --chained --r 33558163,33554761,33558059 --n 2000 --seed 20261108 > $O/snc2-k28.txt
# D0i control at n = 20000 (record: 1.662 +- 0.01), first r of each size
for r in 131203 2097349 33558163; do
  $B --chained --r $r --n 20000 --seed 20261108 > $O/snc2-ctrl-$r.txt
done
