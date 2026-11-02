#!/usr/bin/env python3
"""Linear Kneser/Vosper check: dim(V.V) over random l-dim F_2-subspaces of F_{2^n}, n prime.
Hou-Leung-Xiang / Bachoc-Serra-Zemor: dim(V.V) >= 2l-1 when V.V != K and no intermediate field (n prime);
equality only for geometric progressions c<1,g,...,g^{l-1}> (linear Vosper, prime degree).
Also: brute-force search by single-bit mutations (the idea's mutation operator) from random V towards low dim V.V."""
import sys, random, json
sys.path.insert(0,'.')
from dstar_vs_V import GF, f2rank, VV_dim
n=int(sys.argv[1]); l=int(sys.argv[2]); N=int(sys.argv[3]); seed=int(sys.argv[4])
rng=random.Random(seed); F=GF(n)
def randV():
    while True:
        V=[rng.randrange(1,1<<n) for _ in range(l)]
        if f2rank(V)==l: return V
hist={}
mn=10**9
for _ in range(N):
    d=VV_dim(F,randV()); hist[d]=hist.get(d,0)+1; mn=min(mn,d)
# geometric progressions
gp=[VV_dim(F,[F.mul(c,F.pw(g,j)) for j in range(l)]) for c,g in [(rng.randrange(1,1<<n),rng.randrange(2,1<<n)) for _ in range(50)]]
# hill-climb with single-bit flips on the basis (idea's mutation), 20 restarts x 400 steps, objective dim VV
climbs=[]
for r in range(20):
    V=randV(); best=VV_dim(F,V)
    for step in range(400):
        W=list(V); i=rng.randrange(l); W[i]^=1<<rng.randrange(n)
        if W[i]==0 or f2rank(W)<l: continue
        d=VV_dim(F,W)
        if d<=best: V,best=W,d
    climbs.append(best)
print(json.dumps(dict(n=n,l=l,samples=N,dimVV_hist=dict(sorted(hist.items())),min_random=mn,kneser_bound=2*l-1,
      geometric_progressions_dimVV=sorted(set(gp)),hillclimb_best_dimVV=sorted(climbs),
      bsym_dim=(l+1)*(l+2)//2,poly_form_dim=3*l)))
