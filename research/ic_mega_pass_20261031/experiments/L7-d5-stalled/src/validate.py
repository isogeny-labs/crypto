import sys, os, json
from l2lib import *
sys.path.insert(0, os.path.join(HERE,'..','..','MP-comm_alg-1'))
import tensor_closure as TC
mism=0;tot=0
for n in (9,11):
    k=(n+1)//2
    for s in range(3):
        sy=system(n,k,s)
        for a in ('A','C1','D1'):
            ref=TC.closure(sy[a],2*k,5); r=call5('closure',2*k,5,sy[a])
            m=[r['dim'],r['low'],r['one']]; p=[ref['dim'],ref['low'],int(ref['one'])]
            tot+=1; mism+= m!=p
            print(json.dumps(dict(n=n,s=s,arm=a,c=m,py=p,ok=m==p)),flush=True)
print(json.dumps(dict(total=tot,mismatches=mism)))
