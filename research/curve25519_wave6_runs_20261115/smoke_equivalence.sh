#!/usr/bin/env bash
# Binary equivalence smoke: the new binary must reproduce the frozen dc2c30a4 binary's per-target field_ops
# on two tiny chain cells before any lead runs.
set -e
NEW=/home/user/ctarget-f1/release/curve25519_family; OLD=/home/user/bin-archive/curve25519_family-dc2c30a4
for args in "20 sub 3 0 mont 2 sym 2 20263003 60" "20 sub 9 0 mont 3 sym 1 20264009 120"; do
  a=$(RAYON_NUM_THREADS=1 $OLD f4sub $args | python3 -c "import json,sys;j=json.loads(sys.stdin.read().strip().splitlines()[-1]);print([ (r['target_x'],r['field_ops'],r['solving_degree_max']) for r in j['per_target']])")
  b=$(RAYON_NUM_THREADS=1 $NEW f4sub $args | python3 -c "import json,sys;j=json.loads(sys.stdin.read().strip().splitlines()[-1]);print([ (r['target_x'],r['field_ops'],r['solving_degree_max']) for r in j['per_target']])")
  echo "old $a"; echo "new $b"; [ "$a" = "$b" ] && echo "MATCH: $args" || { echo "MISMATCH: $args"; exit 1; }
done
