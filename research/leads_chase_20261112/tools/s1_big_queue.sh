#!/bin/bash
# big-engine cells at k = 10, D = 5 (PROTOCOL.md S1, amended): arm (a) control, arm (b') mod l, arms d1/d2 mod 12289.
# usage: s1_big_queue.sh <threads>; reads the instances line of s1/small.jsonl
set -u; T=${1:-2}; CSD=/home/user/ml-cryptanalysis/csd; O=s1/big; mkdir -p $O
run() { f=$1; shift; [ -s "$f" ] && return; SECONDS=0; python3 tools/s1_big.py --csd $CSD --out "$f" --threads $T "$@" > "$f.log" 2>&1; echo "$(date -u +%FT%TZ) $f rc=$? ${SECONDS}s" >> $O/queue.log; }
# (a) control: unit instance over 12289 (a=1, t=9362): D=5 must give (0,0,2,138,3163,18569)
run $O/a-p12289-a1-k10-D5.json --p 12289 --a 1 --b 0 --t 9362 --k 10 --D 5
# (b') one generator mod l, two instances per l
python3 - <<'PY' > $O/bprime.list
import json
inst = json.loads(open('s1/small.jsonl').readline())['mod_l']
for l, lst in inst.items():
    for ins in lst[:2]: print(l, ins['a'], ins['b'], ins['t'])
PY
while read l a b t; do run $O/b-p${l}-a${a}-b${b}-t${t}-k10-D5.json --p $l --a $a --b $b --t $t --k 10 --D 5; done < $O/bprime.list
# (d) two equations over 12289 at k = 10, D = 5: d1 {g, h(seed from the small run)}, d2 {g, sigma_257 g}; two instances
python3 - <<'PY' > $O/d.list
import json
rows = [json.loads(l) for l in open('s1/small.jsonl')]
inst = rows[0]['p12289'][:2]
hs = {}
for r in rows:
    if r.get('arm') == 'd1' and 'h_draw' in r: hs[r['a']] = r['h_draw']
import hashlib
for ins in inst:
    hd = hs.get(ins['a'], 1); seed = int(hashlib.sha256(f's1-h-{hd}'.encode()).hexdigest()[:8], 16)
    print(ins['a'], ins['t'], seed)
PY
while read a t seed; do
  run $O/d1-p12289-a${a}-k10-D5.json --p 12289 --a $a --b 0 --t $t --k 10 --D 5 --h-seed $seed
  run $O/d2-p12289-a${a}-k10-D5.json --p 12289 --a $a --b 0 --t $t --k 10 --D 5 --sigma 257
done < $O/d.list
echo "queue done $(date -u +%FT%TZ)" >> $O/queue.log
