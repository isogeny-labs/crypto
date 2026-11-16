#!/usr/bin/env sage
"""Degree-eight branch-supported cover search on trace-minus14 orbit representatives."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

assert len(sys.argv)==3
fmask=int(sys.argv[1]);bmask=int(sys.argv[2])
assert fmask in (9735,17371) and bmask in (11,202,38,80)
wall0=time.perf_counter();cpu0=time.process_time()
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
b=sum(K((bmask//(3^i))%3)*a^i for i in range(5))
R=PolynomialRing(K,'x');x=R.gen()
f=x^11+sum(K((fmask//(3^i))%3)*x^i for i in range(11))
e=x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K,[0,1,0,0,b]).cardinality()==258
fac=[p for p,n in f.factor() for _ in range(n)]
L=next((p for p in fac if p.degree()==1),None)
Q=[p for p in fac if p.degree()==2]
assert L is not None and len(Q)==5 and L*prod(Q)==f
roots=[]
for i,q in enumerate(Q):
 ext=K.extension(q,names=('w%d'%i,))
 vals=sorted([v for v,n in e.change_ring(ext).roots() for _ in range(n)],key=str)
 assert len(vals)==3 and len(set(vals))==3
 roots.append([R(v.lift().list()) for v in vals])
points=[v for v in K if v!=0 and f(v)!=0][:10]
assert len(set(points))==10 and all(v!=0 and f(v)!=0 for v in points)
squares={v^2 for v in K}
lambdas=[v for v in K if v!=0 and v in squares]
assert len(lambdas)==121
shapes=('L1','L3','L5','L7','LQi','L3Qi','L5Qi','LQi3')
counts={s:{'assignments':0,'logical_candidates':0,'point_survivors':0,
           'degree_eight':0,'coprime':0,'f_divides':0,
           'full_equation_hits':0,'point_filter_controls':0} for s in shapes}
hits=[]

def check(kind,A,B,choices,pole=None,lam=None):
 if A.degree()!=8:return
 counts[kind]['degree_eight']+=1
 if gcd(A,B)!=1:return
 counts[kind]['coprime']+=1
 T=A^3+A^2*B+b*B^3
 N=B*T
 C2,rem=N.quo_rem(f)
 if rem!=0:return
 counts[kind]['f_divides']+=1
 if not C2.is_square():return
 C=C2.sqrt()
 assert f*C^2==N and gcd(A,B)==1
 counts[kind]['full_equation_hits']+=1
 hits.append({'shape':kind,'A':str(A),'B':str(B),'C':str(C),
              'choices':list(choices),'pole':pole,
              'lambda':None if lam is None else str(lam)})

def assignments(indices):
 moduli=[Q[i] for i in indices];M=prod(moduli)
 for choices in itertools.product(range(3),repeat=len(indices)):
  h=CRT_list([roots[i][v] for i,v in zip(indices,choices)],moduli)
  yield choices,h,M

for choices,h,M in assignments(tuple(range(5))):
 assert M.degree()==10
 for m in (1,3,5,7):
  kind='L%d'%m;counts[kind]['assignments']+=1
  counts[kind]['logical_candidates']+=1
  B=L^m;A=(B*h)%M
  counts[kind]['point_survivors']+=1
  check(kind,A,B,choices)

for j in range(5):
 indices=tuple(i for i in range(5) if i!=j)
 M=prod(Q[i] for i in indices)
 assert M.degree()==8
 mvals=[M(z) for z in points]
 prepared={}
 for kind,ml,mq in (('LQi',1,1),('L3Qi',3,1),
                    ('L5Qi',5,1),('LQi3',1,3)):
  B=L^ml*Q[j]^mq
  bvals=[B(z) for z in points]
  allowed=[{v/(f(z)*bv) for v in squares}
           for z,bv in zip(points,bvals)]
  assert all(mv!=0 and bv!=0 for mv,bv in zip(mvals,bvals))
  prepared[kind]=(B,bvals,allowed)
 for choices,h,M in assignments(indices):
  assert M.degree()==8
  for kind,(B,bvals,allowed) in prepared.items():
   counts[kind]['assignments']+=1
   residue=(B*h)%M
   rvals=[residue(z) for z in points]
   for z,rv,mv,bv,ok in zip(points,rvals,mvals,bvals,allowed):
    av=rv+lambdas[0]*mv
    tv=av^3+av^2*bv+b*bv^3
    assert (tv in ok)==((bv*tv/f(z)) in squares)
    counts[kind]['point_filter_controls']+=1
   for lam in lambdas:
    counts[kind]['logical_candidates']+=1
    for rv,mv,bv,ok in zip(rvals,mvals,bvals,allowed):
     av=rv+lam*mv
     if av^3+av^2*bv+b*bv^3 not in ok:break
    else:
     counts[kind]['point_survivors']+=1
     check(kind,residue+lam*M,B,choices,j,lam)

assert [counts[s]['assignments'] for s in shapes]==[243]*4+[405]*4
assert [counts[s]['logical_candidates'] for s in shapes]==[243]*4+[405*121]*4
assert [counts[s]['point_filter_controls'] for s in shapes]==[0]*4+[405*10]*4
source=Path(__file__).with_suffix('')
print(json.dumps({'schema':'ghs.trace-minus14.degree8-branch-sparse/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'field':'F3[a]/(a^5+2a+1)','fmask':fmask,'carrier':str(f),
                  'bmask':bmask,'target_order':258,'trace':-14,'counts':counts,'hits':hits,
                  'wall_seconds':time.perf_counter()-wall0,
                  'cpu_seconds':time.process_time()-cpu0},sort_keys=True,default=int))
