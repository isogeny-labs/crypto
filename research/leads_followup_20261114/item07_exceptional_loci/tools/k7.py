import sys,json,random,collections,time
sys.path.insert(0,'.')
from census import *
p=19; rng=random.Random(7)
R=[json.loads(l) for l in open('../data/census19.jsonl')]
by=collections.defaultdict(list)
for r in R: by[r['dim6']].append(r)
rows=[]; t0=time.time()
for c,n in ((602,6),(603,6),(599,2),(600,2),(607,2),(794,6),(605,6)):
    for r in rng.sample(by[c],n):
        d7,o7=closure_dim(p,r['a'],r['b'],r['t'],7); rows.append(dict(a=r['a'],b=r['b'],t=r['t'],dim6=c,dim7=d7,one7=o7))
json.dump(rows,open('../data/k7_19.json','w')); print(time.time()-t0)
print(collections.Counter((r['dim6'],r['dim7'],r['one7']) for r in rows))
