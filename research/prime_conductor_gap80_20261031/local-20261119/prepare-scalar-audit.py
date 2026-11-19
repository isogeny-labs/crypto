#!/usr/bin/env python3
"""Serialize native scalar observations for independent PARI; no curve math."""
from pathlib import Path
import json
s=Path(__file__).resolve().parent;d=s/'results/scalar-l0';p=json.loads((d/'plan.json').read_text());rows=[json.loads(l) for l in (d/'samples.jsonl').read_text().splitlines()]
work={w['workload_id']:w for w in p['workloads']};curves=[];indices={};defs=[]
for w in p['workloads']:
 c=w['curve_spec'];key=json.dumps(c,sort_keys=True)
 if key not in indices:
  indices[key]=len(curves)+1;curves.append(c);defs.append([c['p'],c['a'],c['b'],c['r']])
def gp(v):
 if isinstance(v,list):return '['+','.join(gp(x) for x in v)+']'
 if isinstance(v,int):return str(v)
 return json.dumps(v)
# Keep decimal/hex as strings; PARI performs all numeric interpretation.
lines=[gp(defs)]
for r in rows:
 w=work[r['workload_id']];i=indices[json.dumps(w['curve_spec'],sort_keys=True)]
 lines.append(gp([i,r['scalar'],w['target'],r['result']]))
(s/'scalar-audit.gpvec').write_text('\n'.join(lines)+'\n')
