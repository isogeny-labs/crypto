import sys, json, time
from cnt import *
dmax=int(sys.argv[1]); out=[]; t=time.time()
for d in range(1,dmax+1):
    J,k=valid_params(d)[0]
    e,deg,L=count_poly(d,J,k)
    out.append((d,e,deg,J,k,L))
    if d%500==0: print(d,out[-1],round(time.time()-t),flush=True)
json.dump(out,open("scanA_rows.json","w"))
