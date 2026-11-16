import sys, os, json, time
from l2lib import *
sys.path.insert(0, os.path.join(HERE,'..','..','MP-comm_alg-1'))
import tensor_closure as TC
from run import ARMS
out=[]; mism=0; tot=0
for n in (9,11,13):
    k=(n+1)//2
    for s in range(3):
        sy=system(n,k,s)
        for a in list(ARMS)+['RM']:
            for d in (5,):
                t=time.time(); ref=TC.closure(sy[a],2*k,d); tp=time.time()-t
                r=call5('closure',2*k,d,sy[a])
                mine=[r['dim'],r['low'],r['one']]; refv=[ref['dim'],ref['low'],int(ref['one'])]
                ok=mine==refv; tot+=1; mism+=not ok
                out.append(dict(n=n,k=k,s=s,arm=a,d=d,c=mine,py=refv,ok=ok,cpu_c=r['cpu'],t_py=round(tp,2)))
                print(json.dumps(out[-1]),flush=True)
print(json.dumps(dict(total=tot,mismatches=mism)))
