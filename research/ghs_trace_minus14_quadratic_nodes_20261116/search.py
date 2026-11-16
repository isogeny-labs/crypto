#!/usr/bin/env sage -python
"""Exact point-count filter for Kudo--Harashita (1,1,1,2) node positions."""
from sage.all import GF, PolynomialRing, matrix
from pathlib import Path
import hashlib
import json
import numpy as np
import sys
import time

wall0 = time.perf_counter()
cpu0 = time.process_time()
case = int(sys.argv[1])
limit = int(sys.argv[2]) if len(sys.argv) > 2 else None
assert 0 <= case < 6
F3 = GF(3)
K = GF(9, name="zeta")
zeta = K.gen()
assert zeta.multiplicative_order() == 8
R = PolynomialRing(F3, names=("x", "y", "z"))
x, y, z = R.gens()
variables = (x, y, z)
position = (
    ((1, zeta**5, zeta**7), (1, zeta**7, 1), (1, zeta**2, zeta**2)),
    ((1, zeta**5, zeta**7), (1, 1, zeta**7), (1, zeta**2, zeta**2)),
)
dependent = case >= 3
index = case % 3
rational = ((1,0,0),(0,1,0),(1,1,0)) if dependent else ((1,0,0),(0,1,0),(0,0,1))
p4 = tuple(K(v) for v in position[int(dependent)][index])
p5 = tuple(v**3 for v in p4)
nodes = tuple(tuple(K(v) for v in p) for p in rational) + (p4,p5)
mons = [x**i*y**j*z**(6-i-j) for i in range(7) for j in range(7-i)]

def coords(a):
    w = list(K(a).polynomial())
    return [int(v) for v in w] + [0]*(2-len(w))

rows = []
for p in nodes[:3] + (p4,):
    k = next(i for i,v in enumerate(p) if v)
    free = [j for j in range(3) if j != k]
    for values in ([m(*p) for m in mons],
                   *[[m.derivative(variables[j])(*p) for m in mons] for j in free]):
        expansions = [coords(v) for v in values]
        rows.append([v[0] for v in expansions])
        if p == p4:
            rows.append([v[1] for v in expansions])
constraint = matrix(F3, rows)
basis = [sum((v[i]*mons[i] for i in range(28)),R(0)) for v in constraint.right_kernel().basis()]
d = len(basis)
assert constraint.rank() == 15 and d == 13
for p in nodes:
    assert all(P(*p)==0 and all(P.derivative(v)(*p)==0 for v in variables) for P in basis)

tangent = []
for P in basis:
    row = []
    for p in nodes[:4]:
        k = next(i for i,v in enumerate(p) if v)
        u,v = [variables[i] for i in range(3) if i != k]
        for a in (K(2)*P.derivative(u).derivative(u)(*p),
                  P.derivative(u).derivative(v)(*p),
                  K(2)*P.derivative(v).derivative(v)(*p)):
            row.extend(coords(a))
    tangent.append(row)
tangent = np.array(tangent,dtype=np.int16)
elements = [K(a)+K(b)*zeta for b in range(3) for a in range(3)]
assert len(set(elements)) == 9
index_of = {v:i for i,v in enumerate(elements)}
mul = np.array([[index_of[a*b] for b in elements] for a in elements],dtype=np.int16)
sub = np.array([[index_of[a-b] for b in elements] for a in elements],dtype=np.int16)
square_nonzero = np.array([a != 0 and a**4 == 1 for a in elements],dtype=bool)

def evaluation(r):
    L = GF(3**r,name="u")
    es = list(L)
    one,zero = L(1),L(0)
    pts = [(one,u,v) for u in es for v in es]
    pts.extend((zero,one,u) for u in es)
    pts.append((zero,zero,one))
    M = np.empty((d,len(pts),r),dtype=np.int16)
    for j,P in enumerate(basis):
        Q = P.change_ring(L)
        for k,p in enumerate(pts):
            a = Q(*p)
            w = [int(a)] if r == 1 else [int(v) for v in a.polynomial()]
            M[j,k,:] = w + [0]*(r-len(w))
    return M.reshape(d,-1),len(pts),r

def count_plane(C,ev):
    M,n,r = ev
    if len(C)==0:return np.empty(0,dtype=np.int32)
    V = (C@M)%3
    return np.all(V.reshape(len(C),n,r)==0,axis=2).sum(axis=1)

evals = {1:evaluation(1),2:evaluation(2)}
counts = {k:0 for k in ("raw_vectors","projective_vectors","ordinary_nodes","N1","N2","N3","N4","N5")}
near=[];n4_candidates=[];hits=[];controls=0
total = 3**d-1 if limit is None else min(3**d-1,limit)
powers = np.array([3**i for i in range(d)],dtype=np.int64)
for start in range(1,total+1,4096):
    ids = np.arange(start,min(total+1,start+4096),dtype=np.int64)
    C = ((ids[:,None]//powers[None,:])%3).astype(np.int16)
    counts["raw_vectors"] += len(C)
    first = np.argmax(C!=0,axis=1)
    C = C[C[np.arange(len(C)),first]==1]
    counts["projective_vectors"] += len(C)
    if len(C)==0:continue
    T = ((C@tangent)%3).reshape(len(C),4,3,2)
    code = T[:,:,:,0]+3*T[:,:,:,1]
    a,b,c = code[:,:,0],code[:,:,1],code[:,:,2]
    disc = sub[mul[b,b],mul[a,c]]
    assert np.all((disc[:,:3]==0)|(disc[:,:3]==1)|(disc[:,:3]==2))
    keep = np.all(disc!=0,axis=1)
    C=C[keep];disc=disc[keep]
    counts["ordinary_nodes"] += len(C)
    if len(C)==0:continue
    if controls==0:
        f = sum((F3(int(C[0,i]))*basis[i] for i in range(d)),R(0))
        for r in (1,2):
            L=GF(3**r,name="v");es=list(L);one=L(1);zero=L(0)
            pts=[(one,u,v) for u in es for v in es]
            pts.extend((zero,one,u) for u in es)
            pts.append((zero,zero,one))
            assert sum(f.change_ring(L)(*p)==0 for p in pts)==int(count_plane(C[:1],evals[r])[0])
        controls+=1
    splitr = np.sum(disc[:,:3]==1,axis=1)
    splitq = square_nonzero[disc[:,3]].astype(np.int16)
    keep = count_plane(C,evals[1])+2*splitr-3 == 4
    C=C[keep];splitr=splitr[keep];splitq=splitq[keep]
    counts["N1"] += len(C)
    if len(C)==0:continue
    keep = count_plane(C,evals[2])+1+4*splitq == 10
    C=C[keep];splitr=splitr[keep];splitq=splitq[keep]
    counts["N2"] += len(C)
    near.extend([[int(v) for v in row] for row in C])
    if len(C)==0:continue
    for r,target in ((3,28),(4,82),(5,314)):
        if r not in evals:evals[r]=evaluation(r)
        correction = 1+4*splitq if r%2==0 else 2*splitr-3
        observed = count_plane(C,evals[r])+correction
        if r==5:
            n4_candidates.extend({"coefficients":[int(v) for v in row],"N5":int(n)}
                                 for row,n in zip(C,observed))
        keep = observed == target
        C=C[keep];splitr=splitr[keep];splitq=splitq[keep]
        counts["N%d"%r] += len(C)
        if len(C)==0:break
    else:
        for row in C:
            coeff=[int(v) for v in row]
            f=sum((F3(coeff[i])*basis[i] for i in range(d)),R(0))
            hits.append({"coefficients":coeff,"sextic":str(f)})

print(json.dumps({"schema":"ghs.trace-minus14.quadratic-nodes/v1",
                  "source_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  "case":case,"dependent":dependent,
                  "p4":[coords(v) for v in p4],"p5":[coords(v) for v in p5],
                  "basis_dimension":d,"full_nonzero_vectors":3**d-1,"max_id":total,
                  "counts":counts,"direct_point_controls":controls,"near":near,
                  "n4_candidates":n4_candidates,"hits":hits,
                  "cpu_seconds":time.process_time()-cpu0,
                  "wall_seconds":time.perf_counter()-wall0},sort_keys=True))
