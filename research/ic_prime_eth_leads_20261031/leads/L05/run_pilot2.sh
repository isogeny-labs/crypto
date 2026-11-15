#!/bin/bash
# L05 pilot v2: m=3, b=8, small-x, bits-grouplaw, l=2,3,4; 3 curves (seeds 11,12,13) x (5 planted + 5 uniform); kissat --conflicts=150000
cd /tmp/claude-501/L05
export CONFLICTS=150000
run_one () { # name rename sym seed
  export TMPDIR=/tmp/claude-501/L05/scratch/$1_$4; mkdir -p $TMPDIR
  ./sweep2 sweep --m 3 --bits 8 --seed $4 --two-adicity 1 --families small-x --encodings bits-grouplaw \
    --ls 2,3,4 --planted 5 --uniform 5 --timeout 400 --sym $3 --rename $2 --out res_b8/$1/seed$4 > res_b8_log_$1_$4.txt 2>&1
}
mkdir -p res_b8
for seed in 11 12 13; do
  run_one ref none 1 $seed &
  run_one nosym none 0 $seed &
  run_one hi hi 1 $seed &
  run_one lo lo 1 $seed &
  run_one rand rand 1 $seed &
done
wait
echo ALLDONE
