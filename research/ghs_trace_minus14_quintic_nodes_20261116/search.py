#!/usr/bin/env sage -python
"""Exact coefficient screen for the two degree-five node-orbit positions."""
from sage.all import GF, PolynomialRing, matrix
from pathlib import Path
import hashlib
import json
import numpy as np
import sys
import time

wall0=time.perf_counter();cpu0=time.process_time()
case=int(sys.argv[1]);limit=int(sys.argv[2]) if len(sys.argv)>2 else None
assert case in (0,1)
F3=GF(3);K=GF(3**5,name="zeta");zeta=K.gen()
assert zeta.multiplicative_order()==3**5-1
p=(K(1),zeta**(127 if case==0 else 218),zeta**(143 if case==0 else 72))
R=PolynomialRing(F3,names=("x","y","z"));x,y,z=R.gens();vs=(x,y,z)
mons=[x**i*y**j*z**(6-i-j) for i in range(7) for j in range(7-i)]

def coords(a,r):
    w=[int(v) for v in a.polynomial()]
    return w+[0]*(r-len(w))

rows=[]
for polys in ([m for m in mons],
              [m.derivative(y) for m in mons],
              [m.derivative(z) for m in mons]):
    values=[q.change_ring(K)(*p) for q in polys]
    expansions=[coords(v,5) for v in values]
    rows.extend([[w[j] for w in expansions] for j in range(5)])
constraint=matrix(F3,rows)
basis=[sum((v[i]*mons[i] for i in range(28)),R(0)) for v in constraint.right_kernel().basis()]
d=len(basis)
assert d==13 and constraint.rank()==15
assert all(q.change_ring(K)(*p)==0 and
           q.derivative(y).change_ring(K)(*p)==0 and
           q.derivative(z).change_ring(K)(*p)==0 for q in basis)

def evaluation(r):
    L=GF(3**r,name="u");es=list(L);one=L(1);zero=L(0)
    pts=[(one,a,b) for a in es for b in es]
    pts.extend((zero,one,a) for a in es)
    pts.append((zero,zero,one))
    M=np.empty((d,len(pts),r),dtype=np.int16)
    for j,P in enumerate(basis):
        Q=P.change_ring(L)
        for k,pt in enumerate(pts):
            a=Q(*pt)
            w=[int(a)] if r==1 else coords(a,r)
            M[j,k,:]=w
    return M.reshape(d,-1),len(pts),r

def count_plane(C,ev):
    M,n,r=ev
    if len(C)==0:return np.empty(0,dtype=np.int32)
    chunk=4096 if r==1 else 512 if r==2 else 64 if r<=4 else 16
    result=np.empty(len(C),dtype=np.int32)
    for start in range(0,len(C),chunk):
        D=C[start:start+chunk]
        V=(D@M)%3
        result[start:start+len(D)]=np.all(V.reshape(len(D),n,r)==0,axis=2).sum(axis=1)
    return result

evals={1:evaluation(1),2:evaluation(2)}
counts={k:0 for k in ("raw_vectors","projective_vectors","N1","N2","N3","N4","ordinary_N4","eligible_N5")}
controls=0;n4_candidates=[];hits=[]
total=3**d-1 if limit is None else min(3**d-1,limit)
powers=np.array([3**i for i in range(d)],dtype=np.int64)
target_N5={244-5*t:t for t in (-26,-22,-14,-10,10,14,22,26)}
for start in range(1,total+1,4096):
    ids=np.arange(start,min(total+1,start+4096),dtype=np.int64)
    C=((ids[:,None]//powers[None,:])%3).astype(np.int16)
    counts["raw_vectors"]+=len(C)
    first=np.argmax(C!=0,axis=1)
    C=C[C[np.arange(len(C)),first]==1]
    counts["projective_vectors"]+=len(C)
    if len(C)==0:continue
    if controls==0:
        f=sum((F3(int(C[0,i]))*basis[i] for i in range(d)),R(0))
        for r in (1,2):
            L=GF(3**r,name="v");es=list(L);one=L(1);zero=L(0)
            pts=[(one,a,b) for a in es for b in es]
            pts.extend((zero,one,a) for a in es)
            pts.append((zero,zero,one))
            assert sum(f.change_ring(L)(*pt)==0 for pt in pts)==int(count_plane(C[:1],evals[r])[0])
        controls+=1
    for r,target in ((1,4),(2,10),(3,28),(4,82)):
        if r not in evals:evals[r]=evaluation(r)
        keep=count_plane(C,evals[r])==target
        C=C[keep]
        counts["N%d"%r]+=len(C)
        if len(C)==0:break
    if len(C)==0:continue
    if 5 not in evals:evals[5]=evaluation(5)
    plane5=count_plane(C,evals[5])
    for row,n in zip(C,plane5):
        coef=[int(v) for v in row]
        f=sum((F3(coef[i])*basis[i] for i in range(d)),R(0))
        a=K(2)*f.derivative(y).derivative(y).change_ring(K)(*p)
        b=f.derivative(y).derivative(z).change_ring(K)(*p)
        cc=K(2)*f.derivative(z).derivative(z).change_ring(K)(*p)
        disc=b*b-a*cc
        if disc==0:
            n4_candidates.append({"coefficients":coef,"plane_N5":int(n),"ordinary":False})
            continue
        counts["ordinary_N4"]+=1
        split=disc**((3**5-1)//2)==1
        n5=int(n)+(5 if split else -5)
        candidate={"coefficients":coef,"plane_N5":int(n),"split":bool(split),"N5":n5}
        n4_candidates.append(candidate)
        if n5 in target_N5:
            counts["eligible_N5"]+=1
            hits.append({**candidate,"trace_t":target_N5[n5],"sextic":str(f)})

print(json.dumps({"schema":"ghs.trace-minus14.quintic-node-orbit/v1",
                  "source_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  "case":case,"point":[coords(v,5) for v in p],"basis_dimension":d,
                  "full_nonzero_vectors":3**d-1,"max_id":total,"counts":counts,
                  "direct_point_controls":controls,"n4_candidates":n4_candidates,"hits":hits,
                  "cpu_seconds":time.process_time()-cpu0,"wall_seconds":time.perf_counter()-wall0},sort_keys=True))
