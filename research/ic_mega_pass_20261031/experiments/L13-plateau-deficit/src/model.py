"""P1: pure model. n random triples (lam,mu,ell) of linear forms in N vars over GF(2); ideal sum_r (lam mu, lam ell, mu ell)
in A=F2[x]/(x_i^2) (top-degree algebra of the Boolean ring). Ranks of degree-3 and degree-4 parts vs formula."""
import sys, json, random, itertools
from math import comb as C
import numpy as np
def R4(n,N): return n*(3*C(N-3,2)+N-3)-9*C(n,2)
def rank_gf2(rows,ncols):
    M=np.array(rows,dtype=np.uint8)%2; r=0; M=M.copy()
    for c in range(ncols):
        p=np.nonzero(M[r:,c])[0]
        if len(p)==0: continue
        p=p[0]+r; M[[r,p]]=M[[p,r]]
        nz=np.nonzero(M[:,c])[0]; nz=nz[nz!=r]; M[nz]^=M[r]; r+=1
        if r==M.shape[0]: break
    return r
def mul(p,q):  # polys as dict-free sets of bitmasks; product in A (drop non-squarefree)
    out=set()
    for a in p:
        for b in q:
            if a&b==0: out^={a|b}
    return out
def run(n,N,seed):
    rng=random.Random(seed)
    lin=lambda: {1<<i for i in range(N) if rng.getrandbits(1)} or {1}
    quads=[]
    for _ in range(n):
        l,m,e=lin(),lin(),lin(); quads+= [mul(l,m),mul(l,e),mul(m,e)]
    out={}
    for d in (3,4):
        monos=[sum(1<<i for i in c) for c in itertools.combinations(range(N),d)]; idx={m:i for i,m in enumerate(monos)}
        mult=[sum(1<<i for i in c) for c in itertools.combinations(range(N),d-2)]
        rows=[]
        for q in quads:
            for m in mult:
                v=np.zeros(len(monos),np.uint8)
                for t in q:
                    if t&m==0: v[idx[t|m]]^=1
                rows.append(v)
        out[d]=rank_gf2(rows,len(monos))
    pred3=min(C(N,3),n*(3*N-8)); pred4=min(C(N,4),R4(n,N))
    return dict(n=n,N=N,seed=seed,rank2_quads=rank_gf2([[1 if sum(1<<i for i in c) in q else 0 for c in itertools.combinations(range(N),2)] for q in quads],C(N,2)),
                rank3=out[3],pred3=pred3,rank4=out[4],pred4=pred4,R4=R4(n,N),CN4=C(N,4),ok=(out[3]==pred3 and out[4]==pred4))
if __name__=='__main__':
    for a in sys.argv[1:]:
        n,N=map(int,a.split(',')); print(json.dumps(run(n,N,1000*n+N)),flush=True)
