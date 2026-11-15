import sys,json,random
sys.path.insert(0,'.')
from census import *
p=43; rng=random.Random(4343)
R=[json.loads(l) for w in (0,1) for l in open(f'../data/plane43_t1_w{w}.jsonl')]
plane={(r['alpha'],r['beta']):r for r in R if 'dim6' in r and r['unit7']}
keys=sorted(plane); rng.shuffle(keys)
n=0; agree=0; rows=[]
for al,be in keys:
    t=rng.randrange(2,p)
    a,b=al*t*t%p,be*pow(t,3,p)%p
    if not admissible(p,a,b,t): continue
    d,one=closure_dim(p,a,b,t,6); n+=1; ok=(d==plane[(al,be)]['dim6']); agree+=ok
    rows.append(dict(alpha=al,beta=be,t=t,a=a,b=b,dim=d,plane_dim=plane[(al,be)]['dim6'],agree=ok))
    if n>=150: break
json.dump(rows,open('../data/p6_43.json','w'))
print('P6',agree,'/',n)
