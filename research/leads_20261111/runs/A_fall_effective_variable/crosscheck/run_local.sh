#!/bin/bash
# Local runs of iclose (1 thread, column seed 1, LIFO). Usage: run_local.sh BIN ; appends to out/local.jsonl
cd "$(dirname "$0")"; BIN=$1
for spec in "T35k17s1 0" "T35k17s2 0" "R36k18s1 0" "R36k18s2 0" "T33k17s2 1" "R35k18s6 1"; do
  set -- $spec; inst=$1; full=$2
  res=$(OMP_NUM_THREADS=1 timeout 1150 "$BIN" 4 sys/$inst.sys 8192 $full 1 0 2> out/log/local_$inst.log)
  echo "$res" | python3 -c "import json,sys; d=json.loads(sys.stdin.read()); d.update(instance='$inst', host='local', full=$full); print(json.dumps(d))" >> out/local.jsonl
  tail -1 out/local.jsonl
done
