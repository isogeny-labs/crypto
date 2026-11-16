#!/usr/bin/env sage
"""Direct c-by-c CRT replay, independent of symbolic root elimination."""
import hashlib
import itertools
import json
import sys
from pathlib import Path

F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
assert len(sys.argv)==2
bmask=int(sys.argv[1]);assert bmask in (3,27,30,103,139,167,182,194,230,233)
b=sum(K((bmask//3^i)%3)*a^i for i in range(5))
R=PolynomialRing(K,'x');x=R.gen();L=x+1
f=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
e=x^3+2*x^2+b
Q=[p for p,n in f.factor() if p.degree()==2 for _ in range(n)]
assert len(Q)==5 and L*prod(Q)==f
r0=e.roots(multiplicities=False);assert len(r0)==1;r0=R(r0[0])
roots=[]
for i,q in enumerate(Q):
 ext=K.extension(q,names=('w%d'%i,))
 vals=sorted([v for v,n in e.change_ring(ext).roots() for _ in range(n)],key=str)
 assert len(vals)==3 and len(set(vals))==3
 roots.append([R(v.lift().list()) for v in vals])
shapes=('H2','H4','H6','QiH2','QiH4')
counts={s:{'logical_assignments':0,'numerator_degree_seven':0,'coprime':0,'f_divides':0,'full_equation_hits':0} for s in shapes}
hits=[]
for pole in [None]+list(range(5)):
 indices=tuple(j for j in range(5) if j!=pole)
 moduli=[L]+[Q[j] for j in indices]
 for choices in itertools.product(range(3),repeat=len(indices)):
  vals=[r0]+[roots[j][choice] for j,choice in zip(indices,choices)]
  for c0 in K:
   if c0==-1:continue
   for m in ((2,4,6) if pole is None else (2,4)):
    kind=('H%d'%m) if pole is None else ('QiH%d'%m)
    counts[kind]['logical_assignments']+=1
    B=(x-c0)^m if pole is None else Q[pole]*(x-c0)^m
    A=CRT_list([B*v for v in vals],moduli)
    if A.degree()!=7:continue
    counts[kind]['numerator_degree_seven']+=1
    if gcd(A,B)!=1:continue
    counts[kind]['coprime']+=1
    N=B*(A^3+2*A^2*B+b*B^3)
    C2,rem=N.quo_rem(f)
    if rem!=0:continue
    counts[kind]['f_divides']+=1
    if not C2.is_square():continue
    C=C2.sqrt();assert f*C^2==N
    counts[kind]['full_equation_hits']+=1
    hits.append({'shape':kind,'A':str(A),'B':str(B),'C':str(C),'c':str(c0),'choices':list(choices),'pole':pole})
print(json.dumps({'schema':'ghs.odd5.degree7-linear-square-replay/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),'bmask':bmask,'counts':counts,'hits':hits},sort_keys=True,default=int))
