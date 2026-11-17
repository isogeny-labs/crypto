#!/bin/sh
# Generates specs/*.json for D, Ua, Ub, S, N, ctl on V5 and V6, and the manifest lines of those specs.
# Rounds per level: ceil(2000 / (workloads per round summed over D, Ua, Ub, S, N at that level)).
# Chunk size: the most curves per spec whose pessimistic wall estimate stays <= 27 min (cost model in scripts/costmodel.json).
# CTL_SUBSET=1 applies PLAN 3's reduction: V6 ctl restricted to a seeded random subset of 200 vertices per level with more than 200.
. "$(dirname "$0")/env.sh"
cd $STUDY; mkdir -p specs runs; : > $SP/t/manifest.lines
ICF=$(jq -c '.ic|{factor_base,oracle}' $CENSUS/FROZEN.json)
for V in V5 V6; do
 T=census/$V/transport
 MAXPASS=$(jq -r --arg V $V '.maxpass[$V]' scripts/costmodel.json)
 LC=$(for f in D floor-a floor-b crater-b D; do [ -f $T/$f.jsonl ] && cat $T/$f.jsonl; done | jq -s -c 'group_by(.level)|map({level:.[0].level,n:length})')
 for FAM in D Ua Ub S N ctl; do
  case $FAM in D) F=D;; Ua) F=floor-a;; Ub) F=floor-b;; S) F=crater-b;; N) F=D;; ctl) F=D;; esac
  [ -f $T/$F.jsonl ] || continue
  TS=20261123; [ $FAM = N ] && TS=20261125
  SRC=$T/$F.jsonl
  if [ "$FAM" = ctl ] && [ "$V" = V6 ] && [ "${CTL_SUBSET:-0}" = 1 ]; then
    # seeded random subset: per level with > 200 vertices keep the 200 with the least H('ctl-subset:V6:L<level>:<id>')
    python3 -I - $T/$F.jsonl > $SP/t/ctl_src.jsonl <<'PY'
import sys, json, hashlib
rows=[json.loads(l) for l in open(sys.argv[1])]
H=lambda s:int(hashlib.sha256(s.encode()).hexdigest()[:8],16)
by={}
for r in rows: by.setdefault(r["level"],[]).append(r)
keep=[]
for L,rs in sorted(by.items()):
    if len(rs)>200: rs=sorted(rs,key=lambda r:(H(f"ctl-subset:V6:L{L}:{r['id']}"),r["id"]))[:200]
    keep+=sorted(rs,key=lambda r:r["id"])
for r in keep: print(json.dumps(r,separators=(",",":")))
PY
    SRC=$SP/t/ctl_src.jsonl
  fi
  jq -s -c -L scripts --arg V $V --arg FAM $FAM --argjson ts $TS --argjson ic "$ICF" --argjson maxpass $MAXPASS --argjson lvlcount "$LC" -f scripts/gen_specs.jq $SRC > $SP/t/gen.lines
  while IFS= read -r line; do
    n=$(echo "$line" | jq -r .name)
    echo "$line" | jq '.spec' > specs/$n.json
    echo "$line" | jq -c 'del(.spec)' >> $SP/t/manifest.lines
  done < $SP/t/gen.lines
 done
done
cp $SP/t/manifest.lines runs/manifest_std.lines
ls specs | wc -l
