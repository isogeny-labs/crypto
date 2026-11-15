import numpy as np, sys
sys.path.insert(0,'.')
from sigma import *
rng=np.random.default_rng(1)
def naive_rank(A,p):
    A=A.astype(np.int64)%p; r=0
    m,n=A.shape
    for c in range(n):
        piv=None
        for i in range(r,m):
            if A[i,c]: piv=i;break
        if piv is None: continue
        A[[r,piv]]=A[[piv,r]]
        A[r]=A[r]*pow(int(A[r,c]),-1,p)%p
        for i in range(m):
            if i!=r and A[i,c]: A[i]=(A[i]-A[i,c]*A[r])%p
        r+=1
        if r==m:break
    return r
for p in (40961,12289,786433):
  for (m,n,rk) in [(30,40,10),(120,100,60),(150,300,70),(300,280,200),(40,40,40),(330,600,200)]:
    B=rng.integers(0,p,(m,rk));C=rng.integers(0,p,(rk,n))
    A=(B@C)%p
    r0=naive_rank(A,p)
    M=A.astype(np.int32).copy()
    free,K=nullspace(M,p,b=32)
    ok=np.all((A@K.astype(np.int64))%p==0) if K.shape[1] else True
    print(p,(m,n,rk),'naive',r0,'nullity',len(free),'rank',n-len(free),'Kok',ok, 'rank_rows',rank_rows(A.astype(np.int32).copy(),p))
    assert n-len(free)==r0 and ok
print('ok')
