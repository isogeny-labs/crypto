#!/usr/bin/env python3
"""Serialize existing evidence only; all new algebra runs in native PARI/GP."""
from pathlib import Path
import hashlib,json,re
root=Path(__file__).resolve().parent
repo=root.parents[2]
base=repo/'research/prime_conductor_gap80_20261031'
files=[base/'wide-harness-20261118/paired-map-packets.json',base/'complete-pipeline/fixture-receipts.jsonl',base/'prepared-norm-20261118/results/attempt-a/blocked17-affine-baseline.json',base/'prepared-norm-20261118/results/attempt-b/small-explicit-affine-prepared.json']
packets=json.loads(files[0].read_text());receipts=[json.loads(l) for l in files[1].read_text().splitlines()]
def gp(v):
    if v is None:return '[0]'
    if isinstance(v,list):return '['+','.join(map(gp,v))+']'
    if isinstance(v,int):return str(v)
    if isinstance(v,str) and re.fullmatch(r'-?[0-9]+',v):return v
    return json.dumps(v)
rows=[]
for label,path in [('degree19',files[3]),('degree10009',files[2])]:
    d=json.loads(path.read_text())
    points=[[x['input'],x['image']] for x in d['images']]
    targets=[x for x in receipts if x.get('type')=='paired_known_answer_control' and x['p']==d['p']]
    summary=next(x for x in receipts if x.get('status')=='pass' and x['p']==d['p'])
    order={'71':'57','6666679':'6665994'}[d['p']]
    rows.append([label,d['p'],d['source'],d['ell'],d['kernel'],d['codomain'],order,summary['q'],targets[0]['source_target'],targets[0]['destination_target'],points,[[t['expected_scalar'],t['source_target'],t['destination_target']] for t in targets],1,'small'])
for d in packets:
    two=d['control_degree']==2
    points=[[d['generators']['source'],d['generators']['destination']]]+[[t['source'],t['destination']] for t in d['targets']]+[[d['kernel'],None],[None,None]]
    rows.append(['field202-degree2' if two else 'field96-degree5',d['p'],d['source'],d['control_degree'],d['kernel'],d['destination'],d['group_order'],d['r'],d['generators']['source'],d['generators']['destination'],points,[[t['scalar'],t['source'],t['destination']] for t in d['targets']],2 if two else 1,'cm2' if two else 'cm5'])
(root/'inputs.gpvec').write_text(''.join(gp(r)+'\n' for r in rows))
negative={}
r=json.loads(json.dumps(rows[1]));r[12]=r[3];negative['degree_is_not_gap']=r
r=json.loads(json.dumps(rows[0]));r[3]='21';negative['composite_degree']=r
r=json.loads(json.dumps(rows[1]));r[4]=r[8];negative['wrong_kernel']=r
r=json.loads(json.dumps(rows[2]));r[5]=['1','1'];negative['wrong_codomain']=r
r=json.loads(json.dumps(rows[2]));r[10][0][1]=['1','1'];negative['wrong_image']=r
# Early ordinary guard; no claim about the unused coordinates of this rejection.
r=['supersingular-guard','11',['1','0'],'2',['0','0'],['7','0'],'12','3',['0','0'],['0','0'],[],[],1,'small'];negative['supersingular']=r
for label,r in negative.items():(root/('negative-'+label+'.gpvec')).write_text(gp(r)+'\n')
(root/'inputs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(repo))+'\n' for p in files))
