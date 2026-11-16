#!/usr/bin/env sage
"""Base-change a degree-seven cover and test its 69-bit Jacobian trace."""
import hashlib
import json
import sys
from pathlib import Path

PTH=Path(__file__).resolve().parent
assert len(sys.argv)==2
bmask=int(sys.argv[1]);assert bmask in (30,103,139,194,233)
small=json.loads((PTH/('degree7_quadratic_square_%d.json'%bmask)).read_text())
assert small['bmask']==bmask and len(small['hits'])==1
hit=small['hits'][0];pole=hit['poles'][0]
primitive=json.loads((PTH/'primitive_isogeny_s11.json').read_text())
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1)
L55=GF(3^55,name='w',modulus=Z(primitive['field_modulus']))
embed=K.embeddings(L55)[0]
a=embed(K.gen())
assert [int(c) for c in a.polynomial().list()]==primitive['a_embedding']
R=PolynomialRing(L55,'x');x=R.gen()
f=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
b=sum(L55((bmask//3^i)%3)*a^i for i in range(5))
A=R(sage_eval(hit['A'],locals={'a':a,'x':x},preparse=True))
B=R(sage_eval(hit['B'],locals={'a':a,'x':x},preparse=True))
C=R(sage_eval(hit['C'],locals={'a':a,'x':x},preparse=True))
assert A.degree()==7 and B.degree()==6 and gcd(A,B)==1
assert f*C^2==B*(A^3+2*A^2*B+b*B^3)
Kx=PolynomialRing(K,'x');kx=Kx.gen();fk=kx^11+kx^9+2*kx^8+kx^7+2*kx^5+kx^3+kx^2+kx+1
Qk=[q for q,n in fk.factor() if q.degree()==2 for _ in range(n)]
Qi=R([embed(c) for c in Qk[pole].list()])
J=HyperellipticCurve(f).jacobian();O=J(0)
D0=J(Qi,R(0));assert D0!=O and 2*D0==O
E=EllipticCurve(L55,[0,2,0,0,b])
r=ZZ(333566377062870325133);N=ZZ(174449211009094048378706606);h=N//r
assert h*r==N and is_prime(r)
set_random_seed(20261116+bmask)
P=E(0)
for _ in range(30):
 candidate=h*E.random_point()
 if not candidate.is_zero():P=candidate;break
assert P!=E(0) and r*P==E(0)

def pullback(P):
 if P.is_zero():return O
 U=(A-P[0]*B).monic()
 assert U.degree()==7 and U.is_squarefree() and gcd(U,C)==1
 V=(P[1]*B^2*C.inverse_mod(U))%U
 assert (V^2-f)%U==0
 return J(U,V)-D0
def frob(D,i):
 return J(R([c^(3^(11*i)) for c in D[0].list()]),R([c^(3^(11*i)) for c in D[1].list()]))
def trace(D):return sum((frob(D,i) for i in range(5)),O)
def vec(t):return [int(c) for c in t.polynomial().list()]
def polyvec(g):return [vec(c) for c in g.list()]
def divvec(D):return [polyvec(D[0]),polyvec(D[1])]
D=pullback(P);assert D!=O and r*D==O
T=trace(D);assert r*T==O and frob(T,1)==T
k=37;P2=k*P;D2=pullback(P2);T2=trace(D2)
assert D2==k*D and T2==k*T
out={'schema':'ghs.odd5.degree7-s11-transfer/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),'bmask':bmask,'source_order':int(N),'r':int(r),'cofactor':int(h),'point_P':[vec(P[0]),vec(P[1])],'point_37P':[vec(P2[0]),vec(P2[1])],'pullback_P':divvec(D),'trace_P':divvec(T),'trace_37P':divvec(T2),'trace_nonzero':bool(T!=O),'checks':{'exact_cover_identity':True,'order_r_source':True,'order_r_pullback':True,'order_r_trace':True,'trace_fixed_by_3pow11':True,'pullback_37P_eq_37_pullback_P':True,'trace_37P_eq_37_trace_P':True}}
(PTH/('degree7_s11_transfer_%d.json'%bmask)).write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'schema':out['schema'],'bmask':bmask,'trace_nonzero':out['trace_nonzero'],'checks':out['checks']},sort_keys=True))
