#!/bin/bash
# Native polyclose vs legacy lfd_small.closure on small emitted systems (all modes).
cd "$(dirname "$0")"
out=out/crosscheck.jsonl; : > $out
run() { local D=$1; shift; echo "{\"args\": \"$*\", \"D\": $D}" >> $out; python3 emit_lift.py "$@" 2>>$out | python3 crosscheck.py $D >> $out; }
for s in 7 9 10; do for D in 2 3 4; do run $D orig --n 7 --seed $s; done; for D in 2 3; do run $D bil --n 7 --seed $s; run $D bil --n 7 --seed $s --S 0 1; done; done
for s in 9 16; do for D in 2 3; do run $D orig --n 9 --seed $s; run $D bil --n 9 --seed $s; run $D bil --n 9 --seed $s --S 0 2; done; done
for r in 1 2 3; do for D in 2 3; do run $D rand --n 7 --rseed $r; run $D randbil --n 7 --rseed $r; done; done
for s in 7 9; do run 2 ext --n 7 --seed $s --D 3; run 1 ext --n 7 --seed $s --D 3; done
