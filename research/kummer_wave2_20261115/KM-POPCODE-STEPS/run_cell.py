import sys, json, numpy as np
sys.path.insert(0,'.')
from sigma import *
# usage: run_cell.py KIND W DRAW p D smax out   (KIND: gen|f1|n1|n2)
kind,W,draw,p,D,smax,out=sys.argv[1],int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4]),int(sys.argv[5]),int(sys.argv[6]),sys.argv[7]
seed=0x504B4D54+100*W+draw
rng=np.random.default_rng(seed)
pk="irr"
if kind=="f1":
    coef=[[3,5678],[1234,9012]]   # coef[a][b]: c00=3,c01=5678,c10=1234,c11=9012
elif kind=="n2":
    coef=[[1,1],[1,1]]; pk="lines"
else:
    while True:
        coef=[[int(rng.integers(1,p)) for b in range(W+1)] for a in range(W+1)]
        if kind=="n1":
            coef[0][0]=(-(coef[0][1]+coef[1][0]+coef[1][1]))%p
        if W>1 or (coef[0][0]*coef[1][1]-coef[1][0]*coef[0][1])%p!=0: break
log("coef",coef,"seed",seed)
with open(out,"a") as fh: fh.write(json.dumps(dict(meta=True,kind=kind,W=W,draw=draw,p=p,seed=seed,coef=coef,ptkind=pk))+"\n")
L=Ladder(W,coef,p,D,smax,rng,pk,tag=f"{kind}-W{W}-d{draw}-p{p}",out=out)
L.run()
