#!/bin/bash
# usage: run_price.sh BIN OUTDIR TAG ; runs the three rule-v3 curves, pinned, one thread
S=/tmp/claude-0/-home-user/ec543291-1d7e-50da-a093-c11bce9240eb/scratchpad
cd /home/user/wt/perf2
mkdir -p "$2"
for n in k0n41 k0n53 k0n61; do
  RAYON_NUM_THREADS=1 taskset -c 3 timeout 900 "$1" price --params research/ic_tool_program/rule/v3/runs/$n/T01.params.json \
    --single-target --rho-seed 1 --repeats 3 > "$2/$n-$3.json" 2> "$2/$n-$3.err" || echo "FAIL $n $3 $(tail -1 "$2/$n-$3.err")"
done
