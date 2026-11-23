import json,sys,collections,random,time
sys.path.insert(0,'/home/user/crypto/research/leads_chase_20261112/tools')
from unit_test import resultant_unit
p=19
R=[json.loads(l) for l in open('../data/census19.jsonl')]
rng=random.Random(5)
by=collections.defaultdict(list)
for r in R:
    ti=pow(r['t'],17,p); by[(r['a']*ti*ti%p,r['b']*pow(ti,3,p)%p)].append(r)
sel=[]
for k,v in by.items():
    c=v[0]['dim6']
    if c==794: sel+=rng.sample(v,min(3,len(v)))
    elif c==605 and rng.random()<0.2: sel+=rng.sample(v,1)
    elif c in (602,603): sel+=rng.sample(v,1)
out=open('../data/hi_unit19.jsonl','w'); t0=time.time()
for r in sel:
    res={}
    for k in (8,9,10):
        res[k]=resultant_unit(p,r['a'],r['b'],r['t'],k)['gcd_degree']
    out.write(json.dumps(dict(r,gcd=res))+'\n'); out.flush()
print('done',len(sel),time.time()-t0)
