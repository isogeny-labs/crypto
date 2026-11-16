#!/usr/bin/env sage
"""Frozen trace +10 screen: degree-seven B=H^2 or Qi*H^2, H quadratic."""
import hashlib
import itertools
import json
import sys
from pathlib import Path

F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
assert len(sys.argv)==2
bmask=int(sys.argv[1]);assert bmask in (12,151)
b=sum(K((bmask//3^i)%3)*a^i for i in range(5))
R=PolynomialRing(K,'x');x=R.gen();L=x+2
f=x^11+x^7+2*x^5+2*x^4+x^3+x^2+1
e=x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K,[0,1,0,0,b]).cardinality()==234
Q=[p for p,n in f.factor() if p.degree()==2 for _ in range(n)]
assert len(Q)==5 and L*prod(Q)==f
r0=e.roots(multiplicities=False);assert len(r0)==1;r0=R(r0[0])
roots=[]
for i,q in enumerate(Q):
 ext=K.extension(q,names=('w%d'%i,))
 vals=sorted([v for v,n in e.change_ring(ext).roots() for _ in range(n)],key=str)
 assert len(vals)==3 and len(set(vals))==3
 roots.append([R(v.lift().list()) for v in vals])
U=PolynomialRing(K,'u');u=U.gen();V=PolynomialRing(U,'v');v=V.gen();S=PolynomialRing(K,'v')
shapes=('Hquadratic','QiHquadratic')
counts={s:{'logical_assignments':0,'systems':0,'resultant_fallback_systems':0,'specialized_fallback_u':0,'parameter_solutions':0,'numerator_degree_seven':0,'coprime':0,'f_divides':0,'full_equation_hits':0} for s in shapes}
hits=[]

def choices_for(indices):
 moduli=[L]+[Q[j] for j in indices];M=prod(moduli)
 for choices in itertools.product(range(3),repeat=len(indices)):
  h=CRT_list([r0]+[roots[j][v] for j,v in zip(indices,choices)],moduli)
  yield choices,h,M

def check(kind,u0,v0,base,choices,h,M,poles):
 H=x^2+u0*x+v0
 if gcd(H,f)!=1 or H.discriminant()==0:return
 B=base*H^2
 A=(B*h)%M
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
 hits.append({'shape':kind,'A':str(A),'B':str(B),'C':str(C),'H':str(H),'choices':list(choices),'poles':list(poles)})

def equations(base,h,M):
 rems=[(base*x^i*h)%M for i in range(5)]
 return [V(rems[4][d])+V(2*rems[3][d])*u+V(rems[2][d])*(u^2+2*v)+V(2*rems[1][d])*u*v+V(rems[0][d])*v^2 for d in range(8,M.degree())]

allowed_H=K.cardinality()^2-K.cardinality()-5-(K.cardinality()-1)
assert allowed_H==58559
for choices,h,M in choices_for(tuple(range(5))):
 kind='Hquadratic';counts[kind]['systems']+=1;counts[kind]['logical_assignments']+=allowed_H
 eqs=equations(R(1),h,M);assert len(eqs)==3
 resultants=[eqs[i].resultant(eqs[j]) for i,j in ((0,1),(0,2),(1,2))]
 first=next((r for r in resultants if r!=0),None)
 if first is None:
  counts[kind]['resultant_fallback_systems']+=1;us=list(K)
 else:us=U(first).roots(multiplicities=False)
 for u0 in us:
  spec=[S([K(t(u0)) for t in F.list()]) for F in eqs]
  g=gcd(spec[0],gcd(spec[1],spec[2]))
  if g==0:counts[kind]['specialized_fallback_u']+=1;vs=list(K)
  else:vs=g.roots(multiplicities=False)
  for v0 in vs:
   if not all(F(v0)==0 for F in spec):continue
   counts[kind]['parameter_solutions']+=1
   check(kind,u0,v0,R(1),choices,h,M,[])

for j in range(5):
 indices=tuple(i for i in range(5) if i!=j)
 for choices,h,M in choices_for(indices):
  kind='QiHquadratic';counts[kind]['systems']+=1;counts[kind]['logical_assignments']+=allowed_H
  eqs=equations(Q[j],h,M);assert len(eqs)==1
  F=eqs[0]
  for u0 in K:
   g=S([K(t(u0)) for t in F.list()])
   if g==0:counts[kind]['specialized_fallback_u']+=1;vs=list(K)
   else:vs=g.roots(multiplicities=False)
   for v0 in vs:
    if g(v0)!=0:continue
    counts[kind]['parameter_solutions']+=1
    check(kind,u0,v0,Q[j],choices,h,M,[j])

assert counts['Hquadratic']['systems']==243 and counts['QiHquadratic']['systems']==405
print(json.dumps({'schema':'ghs.odd5.trace10-degree7-quadratic-square/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),'field':'F3[a]/(a^5+2a+1)','carrier':'x^11+x^7+2*x^5+2*x^4+x^3+x^2+1','bmask':bmask,'target_order':234,'allowed_H':allowed_H,'counts':counts,'hits':hits},sort_keys=True,default=int))
