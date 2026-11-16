import numpy as np, json, sys, math
from sympy import nextprime
D=np.load("toy19.npz"); enc,cenc,r=D["enc"],D["cenc"],int(D["r"])
s=int(nextprime(2<<19)); rng=np.random.default_rng(20261031)
def masks(L,alpha,beta):
    tau=int(round(L*s/(r-1)))
    out=[]
    for z in (enc,cenc):
        m=((alpha*z+beta)%s)<tau; m[0]=False; out.append(m)
    return out
def X_count(ms,R):
    # number of tuples (i_1..i_m) in F_1 x ... x F_m, nonzero, with sum == R mod r
    m=len(ms); cur=np.zeros(1,dtype=np.int64); w=np.ones(1,dtype=np.int64)
    sets=[np.nonzero(x)[0].astype(np.int64) for x in ms[:-1]]
    for st in sets:
        cur=(cur[:,None]+st[None,:]).ravel()%r
    part=(R-cur)%r
    return int(ms[-1][part].sum())
def run(m,L,T):
    res={0:[], 1:[]}
    for _ in range(T):
        al=[int(rng.integers(1,s)) for _ in range(m)]; be=[int(rng.integers(0,s)) for _ in range(m)]
        R=int(rng.integers(1,r))
        mk=[masks(L,al[i],be[i]) for i in range(m)]
        for a in (0,1): res[a].append(X_count([mk[i][a] for i in range(m)],R))
    return np.array(res[0]),np.array(res[1])
out=[]
for m,Ls,T in ((2,(90,128,256,362,512,724),30000),(3,(32,48,64,96),4000)):
    for L in Ls:
        u,f=run(m,L,T); pu,pf=(u>0).mean(),(f>0).mean()
        d=(f>0).astype(float)-(u>0).astype(float)   # paired difference
        se=d.std(ddof=1)/math.sqrt(T)
        row=dict(m=m,L=L,T=T,EX_unf=u.mean(),EX_fold=f.mean(),P_unf=pu,P_fold=pf,ratio=pf/pu,diff=pf-pu,diff_se=se,
                 VarX_unf=u.var(),VarX_fold=f.var(),varratio=f.var()/u.var(),expl=2*(n_:=19)-0) 
        row.pop("expl"); out.append(row); print(json.dumps(row),flush=True)
json.dump(out,open("mc_rows.json","w"))
