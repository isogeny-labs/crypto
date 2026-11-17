import sys, json, time, math
from cnt import *
dmax=int(sys.argv[1]); out=[]
t=time.time()
for d in range(1,dmax+1):
    best=None
    for J,k in valid_params(d):
        e,deg,L=count_poly(d,J,k)
        if best is None or e<best[0]: best=(e,deg,J,k,L)
    # also the minimum-degree candidate
    out.append((d,)+best)
    if d%250==0: print(d,best,round(time.time()-t),flush=True)
json.dump(out,open("scan_rows.json","w"))
