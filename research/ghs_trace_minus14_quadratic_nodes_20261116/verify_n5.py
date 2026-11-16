#!/usr/bin/env sage -python
"""Recount every N4 survivor over F243 using monomial, not kernel-basis, evaluations."""
from pathlib import Path
from sage.all import GF, PolynomialRing, matrix
import hashlib
import json
import numpy as np

here = Path(__file__).parent
F3 = GF(3)
K = GF(9, name="zeta")
zeta = K.gen()
R = PolynomialRing(F3, names=("x", "y", "z"))
x,y,z = R.gens()
variables = (x,y,z)
mons = [x**i*y**j*z**(6-i-j) for i in range(7) for j in range(7-i)]
L = GF(3**5, name="u")
es = list(L)
one,zero = L(1),L(0)
pts = [(one,a,b) for a in es for b in es]
pts.extend((zero,one,a) for a in es)
pts.append((zero,zero,one))
M = np.empty((28,len(pts),5),dtype=np.int16)
for i,m in enumerate(mons):
    q = m.change_ring(L)
    for j,p in enumerate(pts):
        w = [int(v) for v in q(*p).polynomial()]
        M[i,j,:] = w+[0]*(5-len(w))
M = M.reshape(28,-1)

def field_coords(a):
    w = [int(v) for v in K(a).polynomial()]
    return w+[0]*(2-len(w))

results=[]
for case in range(6):
    input_path = here/('FULL_%d.json'%case)
    data = json.loads(input_path.read_text())
    rational = ((1,0,0),(0,1,0),(1,1,0)) if data["dependent"] else ((1,0,0),(0,1,0),(0,0,1))
    p4 = tuple(K(a)+K(b)*zeta for a,b in data["p4"])
    nodes = tuple(tuple(K(v) for v in p) for p in rational)+(p4,)
    rows=[]
    for p in nodes:
        k = next(i for i,a in enumerate(p) if a)
        for polys in ([m for m in mons],
                      *[[m.derivative(variables[j]) for m in mons] for j in range(3) if j != k]):
            values=[q(*p) for q in polys]
            expansions=[field_coords(v) for v in values]
            rows.append([w[0] for w in expansions])
            if p==p4:rows.append([w[1] for w in expansions])
    basis=[sum((v[i]*mons[i] for i in range(28)),R(0))
           for v in matrix(F3,rows).right_kernel().basis()]
    assert len(basis)==13
    coeffs=[];split=[]
    for candidate in data["n4_candidates"]:
        c=candidate["coefficients"]
        f=sum((F3(c[i])*basis[i] for i in range(13)),R(0))
        coeffs.append([int(f.monomial_coefficient(m)) for m in mons])
        s=0
        for p in nodes[:3]:
            k=next(i for i,a in enumerate(p) if a)
            u,v=[variables[i] for i in range(3) if i != k]
            a=K(2)*f.derivative(u).derivative(u)(*p)
            b=f.derivative(u).derivative(v)(*p)
            cc=K(2)*f.derivative(v).derivative(v)(*p)
            disc=b*b-a*cc
            assert disc in (K(1),K(2))
            s+=disc==K(1)
        split.append(int(s))
    if coeffs:
        C=np.array(coeffs,dtype=np.int16)
        ev=(C@M)%3
        planes=np.all(ev.reshape(len(C),len(pts),5)==0,axis=2).sum(axis=1)
    else:planes=[]
    for candidate,s,plane in zip(data["n4_candidates"],split,planes):
        n5=int(plane+2*s-3)
        assert n5==candidate["N5"],(case,candidate,n5)
        results.append({"case":case,"coefficients":candidate["coefficients"],
                        "plane_N5":int(plane),"split_rational":s,"N5":n5})

assert len(results)==52
print(json.dumps({"schema":"ghs.trace-minus14.quadratic-nodes-monomial-replay/v1",
                  "source_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  "inputs_sha256":{str(i):hashlib.sha256((here/('FULL_%d.json'%i)).read_bytes()).hexdigest()
                                   for i in range(6)},
                  "checked":len(results),"results":results},sort_keys=True))
