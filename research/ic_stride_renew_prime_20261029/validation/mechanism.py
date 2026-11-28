#!/usr/bin/env python3
"""Validator: relation-count and trial-count decomposition of walk vs srenew (own code; two-stage cluster bootstrap seed 29102026). Usage: python3 -I mechanism.py <session_dir>"""
import json,sys,random,math
from collections import defaultdict
sdir=sys.argv[1]
recs=[json.loads(l) for l in open(sdir+'/records.jsonl')]
M=defaultdict(dict)
for x in recs:
    if x['warmup'] or x['arm'] not in ('walk','srenew'): continue
    rel=[p for p in x['phases'] if p['name']=='relations'][0]
    la=[p for p in x['phases'] if p['name']=='linear_algebra'][0]
    M[x['arm']][(x['workload']['workload_id'],x['round'])]=dict(trials=rel['native']['trials'],relations=rel['native']['relations'],
        dep=x['counters'].get('matrix_dependent',0),rank=x['counters']['matrix_rank'],rows=x['counters']['matrix_rows'],
        rep=rel['native']['repeated_targets_skipped'],cols=la['native'].get('columns'),
        muls=rel['scalar_mults'],dbl=rel['doubles'],G=x['cost']['total_gae'])
keys=sorted(M['walk'])
def tot(arm,f,ks): return sum(M[arm][k][f] for k in ks)
out={}
for f in ('trials','relations','dep','rank','rows','rep','G'):
    out[f]={'walk_mean':tot('walk',f,keys)/len(keys),'srenew_mean':tot('srenew',f,keys)/len(keys)}
    if tot('walk',f,keys)>0:
        out[f]['ratio']=tot('srenew',f,keys)/tot('walk',f,keys)
        cl=defaultdict(list)
        for k in keys: cl[k[0]].append(k)
        ids=sorted(cl); rng=random.Random(29102026); vals=[]
        for _ in range(4000):
            a=b=0
            for _ in ids:
                mem=cl[ids[rng.randrange(len(ids))]]
                for _ in mem:
                    k=mem[rng.randrange(len(mem))]; a+=M['walk'][k][f]; b+=M['srenew'][k][f]
            vals.append(b/a if a else float('nan'))
        vals.sort(); out[f]['ci95_two_stage']=[vals[100],vals[3899]]
out['cols']=sorted({M['walk'][k]['cols'] for k in keys})
print(json.dumps(out,indent=1))
