#!/usr/bin/env sage -python
"""Direct monomial recount of one carrier from every unmodeled eligible class."""
from pathlib import Path
from sage.all import GF, PolynomialRing, ProjectiveSpace, Curve
import hashlib
import json
import numpy as np

here=Path(__file__).parent
input_path=here/'ANALYSIS.json'
data=json.loads(input_path.read_text())
selected=[s for s in data['class_summary']
          if s['source_elliptic_possible'] and s['LMFDB_model_count']==0]
assert len(selected)==data['eligible_unmodeled_classes']
F3=GF(3)
R=PolynomialRing(F3,names=("x","y","z"));x,y,z=R.gens()
P=ProjectiveSpace(F3,2,names=("x","y","z"))
mons=[x**i*y**j*z**(6-i-j) for i in range(7) for j in range(7-i)]
K=GF(243,name="u");u=K.gen()
records=[]
for s in selected:
    f=R(s['representative']['sextic'])
    C=Curve(P.subscheme([f]))
    assert C.is_irreducible() and C.geometric_genus()==5
    assert len(f.change_ring(GF(9,name="a")).factor())==1
    assert len(f.change_ring(GF(27,name="b")).factor())==1
    case=s['representative']['case']
    node=(K(1),u**(127 if case==0 else 218),u**(143 if case==0 else 72))
    a=K(2)*f.derivative(y).derivative(y).change_ring(K)(*node)
    b=f.derivative(y).derivative(z).change_ring(K)(*node)
    c=K(2)*f.derivative(z).derivative(z).change_ring(K)(*node)
    disc=b*b-a*c
    assert disc!=0
    split=disc**121==1
    records.append({'trace_t':s['trace_t'],'LMFDB_label':s['LMFDB_label'],
                    'case':case,'index':s['representative']['index'],
                    'split':bool(split),'sextic':str(f),
                    'monomial_coefficients':[int(f.monomial_coefficient(m)) for m in mons]})

C=np.array([rec['monomial_coefficients'] for rec in records],dtype=np.int16)
for r in (1,2,3,4,5):
    L=GF(3**r,name="v");es=list(L);one=L(1);zero=L(0)
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
    V=(C@M.reshape(28,-1))%3
    counts=np.all(V.reshape(len(records),len(pts),r)==0,axis=2).sum(axis=1)
    for rec,plane in zip(records,counts):
        n=int(plane)+(5 if rec['split'] else -5) if r==5 else int(plane)
        rec.setdefault('normalized_counts',[]).append(n)
        assert n==(244-5*rec['trace_t'] if r==5 else 3**r+1)

print(json.dumps({'schema':'ghs.quintic-node-unmodeled-replay/v1',
                  'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'input_sha256':hashlib.sha256(input_path.read_bytes()).hexdigest(),
                  'checked':len(records),'records':records},sort_keys=True))
