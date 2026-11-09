#!/bin/bash
# L05 pilot: m=3, b=12, small-x, bits-grouplaw, l=2,3,4, 3 curves x (5 planted + 5 uniform), kissat --conflicts cap.
cd /tmp/claude-501/L05
export CONFLICTS=${CONFLICTS:-100000}
BIN=./sweeper/target/release/prime_pdp_sat_sweep
run_variant () { # name rename sym
  export TMPDIR=/tmp/claude-501/L05/scratch/$1; mkdir -p $TMPDIR
  for seed in 1 2 3; do
    $BIN sweep --m 3 --bits 12 --seed $seed --two-adicity 1 --families small-x --encodings bits-grouplaw \
      --ls 2,3,4 --planted 5 --uniform 5 --timeout 300 --sym $3 --rename $2 --out results/$1/seed$seed > logs_$1_$seed.txt 2>&1
  done
}
mkdir -p results
run_variant ref none 1 &
run_variant nosym none 0 &
run_variant hi hi 1 &
run_variant lo lo 1 &
run_variant rand rand 1 &
wait
