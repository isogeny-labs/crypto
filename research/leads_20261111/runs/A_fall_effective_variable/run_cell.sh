#!/bin/bash
# Run the degree-D closure on one selected instance.  Usage: run_cell.sh TYPE N K SEED D FULL TAG
# Appends the JSON result (with cell tag, seed, sols, max RSS) to out/cells.jsonl; progress in out/log/.
HERE=$(cd "$(dirname "$0")" && pwd); cd "$HERE"; mkdir -p out/log
T=$1; NN=$2; K=$3; S=$4; D=$5; FULL=$6; TAG=$7
rec=$(python3 -c "
import json,sys
for l in open('out/instances.jsonl'):
    d=json.loads(l)
    if (d['type'],d['n'],d['k'],d['seed'])==('$T',$NN,$K,$S): print(d['sols']); print(d['line']); break")
sols=$(echo "$rec" | sed -n 1p); line=$(echo "$rec" | sed -n 2p)
[ -z "$line" ] && { echo "instance not found" >&2; exit 2; }
log=out/log/${TAG}_${T}${NN}k${K}s${S}_D${D}.log
res=$(echo "$line" | RAYON_NUM_THREADS=1 ./fallclose "$D" 8192 "$FULL" 2> "$log")
echo "$res" | python3 -c "
import json,sys
d=json.loads(sys.stdin.read()); d.update(tag='$TAG', type='$T', seed=$S, sols=$sols, full=$FULL)
print(json.dumps(d))" >> out/cells.jsonl
tail -1 out/cells.jsonl
