import json,sys
from collections import Counter
sys.path.insert(0,'.')
from kernel import kernel
for line in open(sys.argv[1]):
    d=json.loads(line); r,cc,ct=d['r'],d['cc'],d['ct']
    L=[]
    for ents,rhs in d['rows']:
        v=[0]*cc
        for c,x in ents: v[c]=x%r
        if not v[ct]: L.append(v)
    K=kernel([[v[c] for c in range(cc) if c!=ct] for v in L],r)
    w=[sum(1 for v in L if v[c]) for c in range(cc)]
    sups=[]
    for k in K:
        s=[i for i,x in enumerate(k) if x]
        sups.append((len(s),[w[i] for i in s][:6] if len(s)<10 else 'dense'))
    print(d['q'],d['seed'],d['attempt'],'dL',len(K),sorted(sups,key=lambda x:x[0]))
