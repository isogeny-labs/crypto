#!/bin/bash
# Skeptic runs: own descent (out/*.mysys) + own closure vclose, 1 thread, column seed 5, block 16384.
cd "$(dirname "$0")"; B=$1
for t in T35k17s1 T35k17s2 R36k18s1 R36k18s2 T33k17s2 R35k18s6; do
  r=$(timeout 1150 "$B" 4 out/$t.mysys 5 16384 0 2> out/log_$t.txt)
  echo "$r" | python3 -c "import json,sys; d=json.loads(sys.stdin.read()); d['instance']='$t'; print(json.dumps(d))" >> out/vclose.jsonl
done
# reproduce the producer's tool on the cheapest fall instance with an unused column seed and FIFO
OMP_NUM_THREADS=1 timeout 1150 "$2" 4 ../sys/T35k17s1.sys 8192 0 13 1 > out/iclose_rerun_T35k17s1.json 2> out/iclose_rerun_T35k17s1.log
