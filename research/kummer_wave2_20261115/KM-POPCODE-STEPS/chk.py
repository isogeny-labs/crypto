import sys,numpy as np
sys.path.insert(0,'.')
from sigma import *
# certificate: full evaluation rank = (N+1)^2-(N+1-W)^2 for s=3..4 ; points count
p=40961
for W,seed in ((1,1),(2,2),(3,3)):
    rng=np.random.default_rng(seed)
    coef=[[int(rng.integers(1,p)) for b in range(W+1)] for a in range(W+1)]
    X,Y=curve_points(W,coef,p,W*2**13,rng)
    print("W",W,"points",len(X))
    for s in (3,4):
        N=2**s-1
        lev=Level(s,2*s)  # all monomials (pop<=2s)
        Fc=W*2**(s+1)
        M=eval_matrix(lev,X[:Fc],Y[:Fc],p)
        r=rank_rows(M,p); print(" s",s,"rank",r,"expected",(N+1)**2-(N+1-W)**2)
