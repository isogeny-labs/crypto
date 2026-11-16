#!/usr/bin/env sage -python
"""Independent monomial point counts and geometry checks for all class matches."""
from pathlib import Path
from sage.all import GF, PolynomialRing, ProjectiveSpace, Curve
import hashlib
import json
import numpy as np

here=Path(__file__).parent
F3=GF(3)
R=PolynomialRing(F3,names=("x","y","z"));x,y,z=R.gens()
P=ProjectiveSpace(F3,2,names=("x","y","z"))
mons=[x**i*y**j*z**(6-i-j) for i in range(7) for j in range(7-i)]
K=GF(3**5,name="zeta");zeta=K.gen()
records=[]
for case in (0,1):
    data=json.loads((here/('FULL_%d.json'%case)).read_text())
    p=tuple(sum((K(a)*zeta**j for j,a in enumerate(w)),K(0)) for w in data["point"])
    for index,hit in enumerate(data["hits"]):
        f=R(hit["sextic"])
        C=Curve(P.subscheme([f]))
        assert C.is_irreducible()
        assert C.arithmetic_genus()==10 and C.geometric_genus()==5
        # Geometric reducibility of an F3-irreducible sextic has 2, 3, or 6
        # conjugate components; F9 or F27 factoring detects every possibility.
        factor_counts={}
        for r in (2,3):
            fac=f.change_ring(GF(3**r,name="a")).factor()
            assert len(fac)==1 and fac[0][1]==1
            factor_counts[str(r)]=len(fac)
        q=f.change_ring(K)
        assert q(*p)==0
        assert f.derivative(y).change_ring(K)(*p)==0
        assert f.derivative(z).change_ring(K)(*p)==0
        a=K(2)*f.derivative(y).derivative(y).change_ring(K)(*p)
        b=f.derivative(y).derivative(z).change_ring(K)(*p)
        cc=K(2)*f.derivative(z).derivative(z).change_ring(K)(*p)
        disc=b*b-a*cc
        assert disc!=0
        split=disc**((3**5-1)//2)==1
        assert bool(split)==hit["split"]
        records.append({"case":case,"hit_index":index,"trace_t":hit["trace_t"],
                        "coefficients":hit["coefficients"],"f":f,
                        "monomial_coefficients":[int(f.monomial_coefficient(m)) for m in mons],
                        "factor_counts":factor_counts,"geometric_genus":int(C.geometric_genus()),
                        "split":bool(split),"expected_N5":hit["N5"]})

coeff=np.array([rec["monomial_coefficients"] for rec in records],dtype=np.int16)
assert len(records)==62
for r in (1,2,3,4,5):
    L=GF(3**r,name="u");es=list(L);one=L(1);zero=L(0)
    pts=[(one,a,b) for a in es for b in es]
    pts.extend((zero,one,a) for a in es)
    pts.append((zero,zero,one))
    M=np.empty((28,len(pts),r),dtype=np.int16)
    for i,m in enumerate(mons):
        q=m.change_ring(L)
        for j,p in enumerate(pts):
            a=q(*p)
            w=[int(a)] if r==1 else [int(v) for v in a.polynomial()]
            M[i,j,:]=w+[0]*(r-len(w))
    V=(coeff@M.reshape(28,-1))%3
    plane=np.all(V.reshape(len(records),len(pts),r)==0,axis=2).sum(axis=1)
    for rec,n in zip(records,plane):
        normalized=int(n)+(5 if rec["split"] else -5) if r==5 else int(n)
        rec.setdefault("point_counts",{})[str(r)]={"plane":int(n),"normalized":normalized}
        expected=244-5*rec["trace_t"] if r==5 else 3**r+1
        assert normalized==expected,(rec["case"],rec["hit_index"],r,normalized,expected)
        if r==5:assert normalized==rec["expected_N5"]

for rec in records:
    del rec["f"]
print(json.dumps({"schema":"ghs.trace-minus14.quintic-node-hits-verification/v1",
                  "source_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  "inputs_sha256":{str(c):hashlib.sha256((here/('FULL_%d.json'%c)).read_bytes()).hexdigest()
                                   for c in (0,1)},
                  "verified_count":len(records),"records":records},sort_keys=True))
