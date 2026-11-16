#!/usr/bin/env sage
"""Degree-eight L*H^m and L*Qi*H^m, linear H, on trace +22."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

assert len(sys.argv)==3
fmask=int(sys.argv[1]);bmask=int(sys.argv[2])
assert fmask in (32280,35349) and bmask in (68,84)
wall0=time.perf_counter();cpu0=time.process_time()
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
b=sum(K((bmask//(3^i))%3)*a^i for i in range(5))
R=PolynomialRing(K,'x');x=R.gen()
f=x^11+sum(K((fmask//(3^i))%3)*x^i for i in range(11))
e=x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K,[0,1,0,0,b]).cardinality()==222
L=x
Q=[p for p,n in f.factor() if p.degree()==2 for _ in range(n)]
assert len(Q)==5 and L*prod(Q)==f
roots=[]
for i,q in enumerate(Q):
 ext=K.extension(q,names=('w%d'%i,))
 vals=sorted([v for v,n in e.change_ring(ext).roots() for _ in range(n)],key=str)
 assert len(vals)==3 and len(set(vals))==3
 roots.append([R(v.lift().list()) for v in vals])
U=PolynomialRing(K,'c');cvar=U.gen()
cs=[v for v in K if v!=0];assert len(cs)==242
points=[K(v) for v in (1,2,a,a+1,a+2,a^2,a^2+1,a^2+2,a^3,a^4)]
assert len(set(points))==10 and all(z!=0 and f(z)!=0 for z in points)
squares={v^2 for v in K}
lambdas=[v for v in K if v!=0 and v in squares]
assert len(lambdas)==121
shapes=('LH2','LH4','LH6','LQiH2','LQiH4')
counts={s:{'assignments':0,'logical_candidates':0,'parameter_roots':0,
           'coefficient_fallbacks':0,'point_survivors':0,
           'point_filter_controls':0,'degree_eight':0,'coprime':0,
           'f_divides':0,'full_equation_hits':0} for s in shapes}
hits=[]

def check(kind,A,B,choices,c0,pole=None,lam=None):
 if A.degree()!=8:return
 counts[kind]['degree_eight']+=1
 if gcd(A,B)!=1:return
 counts[kind]['coprime']+=1
 N=B*(A^3+A^2*B+b*B^3)
 C2,rem=N.quo_rem(f)
 if rem!=0:return
 counts[kind]['f_divides']+=1
 if not C2.is_square():return
 C=C2.sqrt();assert f*C^2==N and gcd(A,B)==1
 counts[kind]['full_equation_hits']+=1
 hits.append({'shape':kind,'choices':list(choices),'pole':pole,
              'c':str(c0),'lambda':None if lam is None else str(lam),
              'A':str(A),'B':str(B),'C':str(C)})

def assignments(indices):
 moduli=[Q[i] for i in indices];M=prod(moduli)
 for choices in itertools.product(range(3),repeat=len(indices)):
  h=CRT_list([roots[i][v] for i,v in zip(indices,choices)],moduli)
  yield choices,h,M

for choices,h,M in assignments(tuple(range(5))):
 assert M.degree()==10
 for m in (2,4,6):
  kind='LH%d'%m;counts[kind]['assignments']+=1
  counts[kind]['logical_candidates']+=242
  rems=[(L*x^i*h)%M for i in range(m+1)]
  F=sum((U(K(binomial(m,i))*rems[i][9])*(-cvar)^(m-i)
         for i in range(m+1)),U(0))
  if F==0:
   counts[kind]['coefficient_fallbacks']+=1;candidate_cs=list(K)
  else:candidate_cs=F.roots(multiplicities=False)
  for c0 in candidate_cs:
   if c0==0:continue
   counts[kind]['parameter_roots']+=1
   B=L*(x-c0)^m;A=(B*h)%M
   counts[kind]['point_survivors']+=1
   check(kind,A,B,choices,c0)

for j in range(5):
 indices=tuple(i for i in range(5) if i!=j)
 M=prod(Q[i] for i in indices);assert M.degree()==8
 mvals=[M(z) for z in points]
 base=L*Q[j]
 basevals=[base(z) for z in points]
 allowed=[{v/(f(z)*bv) for v in squares}
          for z,bv in zip(points,basevals)]
 assert all(mv!=0 and bv!=0 for mv,bv in zip(mvals,basevals))
 for choices,h,M in assignments(indices):
  for m in (2,4):
   kind='LQiH%d'%m;counts[kind]['assignments']+=1
   counts[kind]['logical_candidates']+=242*121
   for c0 in cs:
    B=base*(x-c0)^m
    residue=(B*h)%M
    rvals=[residue(z) for z in points]
    bvals=[bv*(z-c0)^m for bv,z in zip(basevals,points)]
    for z,rv,mv,bv,ok in zip(points,rvals,mvals,bvals,allowed):
     if z==c0:continue
     av=rv+lambdas[0]*mv
     tv=av^3+av^2*bv+b*bv^3
     assert (tv in ok)==((bv*tv/f(z)) in squares)
     counts[kind]['point_filter_controls']+=1
    for lam in lambdas:
     for z,rv,mv,bv,ok in zip(points,rvals,mvals,bvals,allowed):
      if z==c0:continue
      av=rv+lam*mv
      if av^3+av^2*bv+b*bv^3 not in ok:break
     else:
      counts[kind]['point_survivors']+=1
      check(kind,residue+lam*M,B,choices,c0,j,lam)
 sys.stderr.write('quadratic_pole=%d candidates=%d cpu=%.3f\n'%
                  (j,sum(v['logical_candidates'] for v in counts.values()),
                   time.process_time()-cpu0))
 sys.stderr.flush()

assert [counts[s]['assignments'] for s in shapes]==[243]*3+[405]*2
assert [counts[s]['logical_candidates'] for s in shapes]==[243*242]*3+[405*242*121]*2
source=Path(__file__).with_suffix('')
print(json.dumps({'schema':'ghs.trace22.degree8-linear-square/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'field':'F3[a]/(a^5+2a+1)','fmask':fmask,'carrier':str(f),
                  'bmask':bmask,'target_order':222,'counts':counts,'hits':hits,
                  'wall_seconds':time.perf_counter()-wall0,
                  'cpu_seconds':time.process_time()-cpu0},sort_keys=True,default=int))
