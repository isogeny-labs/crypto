#!/usr/bin/env bash
# m-sweep: Kummer tower (split presentation), m summands, hky2 closures up to degree 9.
# One pilot invocation at --t-min T --max-t T emits depth T and T + 1 (observed locally: N = m*depth).
# usage: msweep_job.sh "M1 M2 ..." "T1 T2 ..." [MAXCOLS [TAG [BUDGET [DRAWS]]]]   (defaults 3000000, no tag, 2400 s, 2)
set -uo pipefail
MAXCOLS=${3:-3000000}; TAG=${4:-}; BUDGET=${5:-2400}; DRAWS=${6:-2}
W=/workspace; O=$W/out/msweep; B=$W/in/pilot_candidate; mkdir -p $O
exec > >(tee -a $O/msweep.log) 2>&1
date -u +%FT%TZ; nproc; free -g | head -2; uptime
echo "47d6c281ecf9cc85ac338fae9ee3ddb4012f88d159ed996cdb855ee0fdeda543  $B" | sha256sum -c - || exit 9
chmod +x $B
for T in $2; do for M in $1; do
  echo "=== m=$M T=$T start $(date -u +%T)"; uptime
  $B --p 2013265921 --ladder-t none --planted 0 --random $DRAWS --engine hky2 --hky-cmax 9 --budget $BUDGET \
     --hky-max-cols $MAXCOLS --hky-max-entries 14000000000 --kinds kummer --m $M --controls tower \
     --t-min $T --max-t $T --hky-progress $O/m$M-T$T$TAG.progress.jsonl --out $O/m$M-T$T$TAG.jsonl > $O/m$M-T$T$TAG.log 2>&1
  echo "rc=$? end $(date -u +%T)"
  python3 - $O/m$M-T$T$TAG.jsonl <<'P'
import json,sys
for l in open(sys.argv[1]):
    d=json.loads(l)
    if 'closures' not in d: continue
    print(' N',d.get('N'),'falls',d.get('falls'),'censored_at',d.get('censored_at'),[(c['c'],c['cols'],c['one'],c['censored'],round(c['ms']/1000,1)) for c in d['closures']])
P
done; done
echo "=== ALL DONE"; date -u +%FT%TZ
