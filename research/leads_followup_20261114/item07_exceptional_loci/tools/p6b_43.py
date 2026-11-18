import sys,json,random
sys.path.insert(0,'.')
from census import *
p=43; rng=random.Random(4344)
R=[json.loads(l) for w in (0,1) for l in open(f'../data/plane43_t1_w{w}.jsonl')]
dep=[r for r in R if r.get('dim6') in (599,600,602,603,607,794) and r['unit7']]
rows=[];agree=0
for r in dep:
    for _ in range(20):
        t=rng.randrange(2,p); a,b=r['alpha']*t*t%p,r['beta']*pow(t,3,p)%p
        if admissible(p,a,b,t): break
    else: continue
    d,_o=closure_dim(p,a,b,t,6); ok=d==r['dim6']; agree+=ok
    rows.append(dict(alpha=r['alpha'],beta=r['beta'],t=t,dim=d,plane_dim=r['dim6'],agree=ok))
json.dump(rows,open('../data/p6b_43.json','w')); print('P6b departing groups',agree,'/',len(rows))
