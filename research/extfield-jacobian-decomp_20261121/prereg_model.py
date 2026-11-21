#!/usr/bin/env python3
"""Preregistration model (written before any run): predicted total S and crossover q* per arm,
from round-1 constants.  Constants: p=0.0402 decomposition prob/trial, M=4.52e5 solver mulmods/trial,
rho S(w) ~ sqrt(ell)*0.99*(2+4w) + 9.5e3 with ell ~ q^4/8.5, 1 add per trial plus verification adds ~ 1.7/relation."""
import math
p=0.0402; M=4.52e5
def pge2(f): return 6*f*f*(1-f)**2+4*f**3*(1-f)+f**4
def ic(q,alpha,c,w):
    cls=q/2
    nf=cls if alpha==1 else min(cls,c*q**alpha)
    f=nf/cls
    if alpha==1 or nf>=cls: rel=cls+10; pr=p
    else:
        rel=1.1*(cls-nf)+nf+10; pr=p*pge2(f)
    T=rel/pr
    n=nf+1; nnz=rel*3.5
    la=4*n*nnz  # sequential Wiedemann on normal equations, 2n steps x 2nnz
    return T*(1+w*M)+ w*la
def rho(q,w): return (q**2/2.9)*0.99*(2+4*w)+9.5e3
for w in (0.1,1.0):
    for (a,c) in ((1,1),(0.75,1),(0.75,2),(0.6,1),(0.6,2)):
        qs=None
        for e in range(2,16):
            for m in (1,2,5):
                q=m*10**e
                if ic(q,a,c,w)<rho(q,w): qs=q;break
            if qs:break
        print(f"w={w} alpha={a} c={c} S(2039)={ic(2039,a,c,w):.3g} ratio={ic(2039,a,c,w)/rho(2039,w):.3g} qstar~{qs}")
