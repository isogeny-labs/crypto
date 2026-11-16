#!/usr/bin/env sage
"""Trace +22 exact one-variable degree-seven rational-square elimination."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

F3=GF(3)
wall0=time.perf_counter();cpu0=time.process_time()
Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
assert len(sys.argv)==2
bmask=int(sys.argv[1]);assert bmask == 11
b=sum(K((bmask//(3^i))%3)*a^i for i in range(5))
R=PolynomialRing(K,'x');x=R.gen();L=x
f=x^11+x^8+x^7+x^6+x^5+x^2+2*x
e=x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K,[0,1,0,0,b]).cardinality()==258
Q=[p for p,n in f.factor() if p.degree()==2 for _ in range(n)]
assert len(Q)==5 and L*prod(Q)==f
r0=e.roots(multiplicities=False);assert len(r0)==1;r0=R(r0[0])
roots=[]
for i,q in enumerate(Q):
 ext=K.extension(q,names=('w%d'%i,))
 vals=sorted([v for v,n in e.change_ring(ext).roots() for _ in range(n)],key=str)
 assert len(vals)==3 and len(set(vals))==3
 roots.append([R(v.lift().list()) for v in vals])
U=PolynomialRing(K,'c');cvar=U.gen()
shapes=('H2','H4','H6','QiH2','QiH4')
counts={s:{'logical_assignments':0,'root_candidates':0,'fallback_assignments':0,'numerator_degree_seven':0,'coprime':0,'f_divides':0,'full_equation_hits':0} for s in shapes}
hits=[]

def choices_for(indices):
 moduli=[L]+[Q[j] for j in indices]
 M=prod(moduli)
 for choices in itertools.product(range(3),repeat=len(indices)):
  h=CRT_list([r0]+[roots[j][v] for j,v in zip(indices,choices)],moduli)
  yield choices,h,M

def check(kind,A,B,choices,poles,c0):
 if A.degree()!=7:return
 counts[kind]['numerator_degree_seven']+=1
 if gcd(A,B)!=1:return
 counts[kind]['coprime']+=1
 N=B*(A^3+A^2*B+b*B^3)
 C2,rem=N.quo_rem(f)
 if rem!=0:return
 counts[kind]['f_divides']+=1
 if not C2.is_square():return
 C=C2.sqrt();assert f*C^2==N and gcd(A,B)==1
 counts[kind]['full_equation_hits']+=1
 hits.append({'shape':kind,'A':str(A),'B':str(B),'C':str(C),'choices':list(choices),'poles':list(poles),'c':str(c0)})

def run(kind,base,m,choices,h,M,poles):
 counts[kind]['logical_assignments']+=K.cardinality()-1
 rems=[(base*x^i*h)%M for i in range(m+1)]
 eqs=[]
 for d in range(8,M.degree()):
  F=sum((U(K(binomial(m,i))*rems[i][d])*(-cvar)^(m-i) for i in range(m+1)),U(0))
  eqs.append(F)
 common=gcd(eqs)
 if common==0:
  counts[kind]['fallback_assignments']+=1
  cs=list(K)
 else:cs=common.roots(multiplicities=False)
 for c0 in cs:
  if c0==0:continue
  counts[kind]['root_candidates']+=1
  B=base*(x-c0)^m
  A=(B*h)%M
  check(kind,A,B,choices,poles,c0)

for choices,h,M in choices_for(tuple(range(5))):
 assert M.degree()==11
 for m in (2,4,6):run('H%d'%m,R(1),m,choices,h,M,[])
for j in range(5):
 indices=tuple(i for i in range(5) if i!=j)
 for choices,h,M in choices_for(indices):
  assert M.degree()==9
  for m in (2,4):run('QiH%d'%m,Q[j],m,choices,h,M,[j])
assert [counts[s]['logical_assignments'] for s in shapes]==[58806,58806,58806,98010,98010]
print(json.dumps({'schema':'ghs.degree7.trace-minus14-linear-square/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),'field':'F3[a]/(a^5+2a+1)','bmask':bmask,'target_order':258,'counts':counts,'hits':hits,'wall_seconds':time.perf_counter()-wall0,'cpu_seconds':time.process_time()-cpu0},sort_keys=True,default=int))
