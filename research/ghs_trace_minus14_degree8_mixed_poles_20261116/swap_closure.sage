#!/usr/bin/env sage
"""Degree-eight mixed branch/nonbranch pole cover search on trace minus14."""
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
U=PolynomialRing(K,'c');cvar=U.gen()
cs=[v for v in K if L(v)!=0];assert len(cs)==242
M=prod(Q);assert M.degree()==10
counts={'branch_assignments':0,'logical_candidates':0,'parameter_roots':0,
        'coefficient_fallbacks':0,'degree_eight':0,'coprime':0,
        'f_divides':0,'full_equation_hits':0}
hits=[]
for choice in itertools.product(range(3),repeat=5):
 counts['branch_assignments']+=1
 h=CRT_list([roots[i][v] for i,v in enumerate(choice)],Q)
 rems=[(L^3*x^i*h)%M for i in range(5)]
 F=sum((U(K(binomial(4,i))*rems[i][9])*(-cvar)^(4-i)
        for i in range(5)),U(0))
 if F==0:
  counts['coefficient_fallbacks']+=1; candidate_cs=cs
 else:candidate_cs=F.roots(multiplicities=False)
 counts['logical_candidates']+=242
 for c0 in candidate_cs:
  if L(c0)==0:continue
  counts['parameter_roots']+=1
  B=L^3*(x-c0)^4;A=(B*h)%M
  if A.degree()!=8:continue
  counts['degree_eight']+=1
  if gcd(A,B)!=1:continue
  counts['coprime']+=1
  N=B*(A^3+A^2*B+b*B^3)
  C2,rem=N.quo_rem(f)
  if rem!=0:continue
  counts['f_divides']+=1
  if not C2.is_square():continue
  C=C2.sqrt()
  assert f*C^2==N and gcd(A,B)==1
  counts['full_equation_hits']+=1
  hits.append({'choice':list(choice),'c':str(c0),
               'A':str(A),'B':str(B),'C':str(C)})
assert counts['branch_assignments']==243
assert counts['logical_candidates']==243*242
source=Path(__file__).with_suffix('')
print(json.dumps({'schema':'ghs.trace-minus14.degree8-swap-closure/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'field':'F3[a]/(a^5+2a+1)','fmask':fmask,'carrier':str(f),
                  'bmask':bmask,'target_order':258,'counts':counts,'hits':hits,
                  'wall_seconds':time.perf_counter()-wall0,
                  'cpu_seconds':time.process_time()-cpu0},sort_keys=True,default=int))
