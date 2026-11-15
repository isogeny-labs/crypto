#!/bin/bash
# Usage: run_cell.sh TAG TYPE N SEED D [BATCH]   (instance line from out/instances.jsonl or out/scan.jsonl)
HERE=$(cd "$(dirname "$0")" && pwd); cd "$HERE"
TAG=$1; T=$2; NN=$3; S=$4; D=$5; BS=${6:-8192}
line=$(python3 -c "
import json
for f in ('out/instances.jsonl','out/instances_T.jsonl','out/scan.jsonl','out/extra_instances.jsonl'):
    for l in open(f):
        d=json.loads(l)
        if (d['type'],d['n'],d['seed'])==('$T',$NN,$S): print(d['sols']); print(d['line']); raise SystemExit")
sols=$(echo "$line" | sed -n 1p); inst=$(echo "$line" | sed -n 2p)
[ -z "$inst" ] && { echo "no instance $T $NN $S" >&2; exit 2; }
log=out/log/${TAG}_${T}${NN}s${S}_D${D}.log
res=$(echo "$inst" | RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 timeout 1200 ./${EXE:-fallclose} "$D" "$BS" 1 2> "$log") || res='{"error": "timeout_or_fail"}'
echo "$res" | python3 -c "
import json,sys
d=json.loads(sys.stdin.read()); d.update(exe='${EXE:-fallclose}', tag='$TAG', type='$T', seed=$S, sols=$sols, Dreq=$D, line='$inst')
print(json.dumps(d))" >> out/cells.jsonl
tail -1 out/cells.jsonl | cut -c1-260
