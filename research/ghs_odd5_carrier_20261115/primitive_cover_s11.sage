#!/usr/bin/env sage
"""Compose a primitive-j 23-isogeny with the degree-seven GHS cover trace."""
import hashlib
import json
from pathlib import Path

PTH=Path(__file__).resolve().parent
old=json.loads((PTH/'primitive_isogeny_s11.json').read_text())
hit=json.loads((PTH/'degree7_quadratic_square_30.json').read_text())['hits'][0]
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1)
L=GF(3^55,name='w',modulus=Z(old['field_modulus']))
embed=K.embeddings(L)[0];a=embed(K.gen())
assert [int(c) for c in a.polynomial().list()]==old['a_embedding']
b=a+a^3
E=EllipticCurve(L,[0,2,0,0,b]);N=ZZ(174449211009094048378706606)
r=ZZ(333566377062870325133);assert N%r==0 and is_prime(r)
R=PolynomialRing(L,'x');x=R.gen()
f=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
A=R(sage_eval(hit['A'],locals={'a':a,'x':x},preparse=True))
B=R(sage_eval(hit['B'],locals={'a':a,'x':x},preparse=True))
C=R(sage_eval(hit['C'],locals={'a':a,'x':x},preparse=True))
assert f*C^2==B*(A^3+2*A^2*B+b*B^3)
Kx=PolynomialRing(K,'x');kx=Kx.gen()
fk=kx^11+kx^9+2*kx^8+kx^7+2*kx^5+kx^3+kx^2+kx+1
Qi0=[q for q,n in fk.factor() if q.degree()==2 for _ in range(n)][hit['poles'][0]]
Qi=R([embed(c) for c in Qi0.list()])
J=HyperellipticCurve(f).jacobian();O=J(0);D0=J(Qi,R(0))
assert D0!=O and 2*D0==O
def pullback(P):
 if P.is_zero():return O
 U=(A-P[0]*B).monic();assert U.degree()==7 and U.is_squarefree() and gcd(U,C)==1
 V=(P[1]*B^2*C.inverse_mod(U))%U
 assert (V^2-f)%U==0
 return J(U,V)-D0
def frob(D,i):return J(R([c^(3^(11*i)) for c in D[0].list()]),R([c^(3^(11*i)) for c in D[1].list()]))
def trace(D):return sum((frob(D,i) for i in range(5)),O)
def vec(t):return [int(c) for c in t.polynomial().list()]
def pointvec(P):return [vec(P[0]),vec(P[1])]
def polyvec(p):return [vec(c) for c in p.list()]
def divvec(D):return [polyvec(D[0]),polyvec(D[1])]

isogenies=E.isogenies_prime_degree(23)
primitive=[g for g in isogenies if g.codomain().j_invariant().minpoly().degree()==55]
assert primitive
forward=primitive[0];Eprime=forward.codomain()
set_random_seed(20261116+30+23)
P=E(0)
for _ in range(30):
 candidate=(N//r)*E.random_point()
 if not candidate.is_zero():P=candidate;break
assert P!=E(0) and r*P==E(0)
Q=forward(P);assert Q!=Eprime(0) and r*Q==Eprime(0)
reverse=None;iso=None;sign=None
for g in Eprime.isogenies_prime_degree(23):
 if g.codomain().j_invariant()!=E.j_invariant():continue
 giso=g.codomain().isomorphism_to(E)
 for e in (1,-1):
  if e*giso(g(Q))==23*P:
   reverse,iso,sign=g,giso,e;break
 if reverse is not None:break
assert reverse is not None
Pback=sign*iso(reverse(Q))
T=trace(pullback(Pback));assert T!=O and r*T==O and frob(T,1)==T
Q37=37*Q;Pback37=sign*iso(reverse(Q37))
T37=trace(pullback(Pback37));assert Pback37==37*Pback and T37==37*T
out={'schema':'ghs.odd5.primitive-cover-s11/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
 'bmask':30,'field_modulus':old['field_modulus'],'a_embedding':old['a_embedding'],'r':int(r),'order':int(N),
 'source_ainvs':[vec(c) for c in E.ainvs()],'primitive_ainvs':[vec(c) for c in Eprime.ainvs()],
 'primitive_j':vec(Eprime.j_invariant()),'primitive_j_degree':int(Eprime.j_invariant().minpoly().degree()),
 'forward_kernel_polynomial':polyvec(forward.kernel_polynomial()),'reverse_kernel_polynomial':polyvec(reverse.kernel_polynomial()),
 'reverse_codomain_ainvs':[vec(c) for c in reverse.codomain().ainvs()],'reverse_sign':sign,
 'P':pointvec(P),'Q':pointvec(Q),'reverse_Q':pointvec(Pback),'trace_Q':divvec(T),'trace_37Q':divvec(T37),
 'checks':{'forward_nonzero':True,'reverse_eq_23P':True,'trace_nonzero':True,'trace_order_r':True,
           'trace_fixed_by_3pow11':True,'scalar_preserving_on_37Q':True}}
(PTH/'primitive_cover_s11.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'schema':out['schema'],'primitive_j_degree':out['primitive_j_degree'],'r':out['r'],'checks':out['checks']},sort_keys=True))
