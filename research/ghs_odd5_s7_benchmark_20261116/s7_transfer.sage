#!/usr/bin/env sage
"""Exact s=7 mask-30 cover transfer, with immutable JSON receipt."""
import hashlib
import json
from pathlib import Path

here=Path(__file__).resolve().parent
source=here.parent/'ghs_odd5_carrier_20261115'/'degree7_quadratic_square_30.json'
hit=json.loads(source.read_text())['hits'][0]
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,'a',modulus=z^5+2*z+1)
L=GF(3^35,'w');embed=K.embeddings(L)[0];a=embed(K.gen())
k=L.subfield(7);small_embed=k.embeddings(L)[0];back=small_embed.section()
R=PolynomialRing(L,'x');x=R.gen()
f=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
b=a+a^3
A=R(sage_eval(hit['A'],locals={'a':a,'x':x},preparse=True))
B=R(sage_eval(hit['B'],locals={'a':a,'x':x},preparse=True))
C=R(sage_eval(hit['C'],locals={'a':a,'x':x},preparse=True))
assert A.degree()==7 and B.degree()==6 and gcd(A,B)==1
assert f*C^2==B*(A^3+2*A^2*B+b*B^3)

Kx=PolynomialRing(K,'x');xx=Kx.gen()
fk=xx^11+xx^9+2*xx^8+xx^7+2*xx^5+xx^3+xx^2+xx+1
Qi0=[q for q,n in fk.factor() if q.degree()==2 for _ in range(n)][hit['poles'][0]]
Qi=R([embed(c) for c in Qi0.list()])
JL=HyperellipticCurve(f).jacobian();OL=JL(0);D0=JL(Qi,R(0))
assert D0!=OL and 2*D0==OL
Rk=PolynomialRing(k,'x');xk=Rk.gen()
fk2=xk^11+xk^9+2*xk^8+xk^7+2*xk^5+xk^3+xk^2+xk+1
J=HyperellipticCurve(fk2).jacobian();O=J(0)
E=EllipticCurve(L,[0,2,0,0,b])
r=ZZ(1070455751);N=ZZ(50031545359078046);h=N//r
assert N==h*r and N==2*29*109*7393*r and is_prime(r)
set_random_seed(202611160730)
P=E(0)
for _ in range(30):
 candidate=h*E.random_point()
 if not candidate.is_zero():P=candidate;break
assert P!=E(0) and r*P==E(0)
secret=ZZ(123456789);Q=secret*P

def pull(Pt):
 if Pt.is_zero():return OL
 U=(A-Pt[0]*B).monic()
 assert U.degree()==7 and U.is_squarefree() and gcd(U,C)==1
 V=(Pt[1]*B^2*C.inverse_mod(U))%U
 assert (V^2-f)%U==0
 return JL(U,V)-D0
def frobL(D,i):
 return JL(R([c^(3^(7*i)) for c in D[0].list()]),
           R([c^(3^(7*i)) for c in D[1].list()]))
def transfer(Pt):
 D=pull(Pt);T=sum((frobL(D,i) for i in range(5)),OL)
 assert frobL(T,1)==T
 return J(Rk([back(c) for c in T[0].list()]),
          Rk([back(c) for c in T[1].list()]))
G=transfer(P);H=transfer(Q);G37=transfer(37*P)
assert G!=O and r*G==O and H==secret*G and G37==37*G

def vec(v):return [int(c) for c in v.polynomial().list()]
def divvec(D):return [[[vec(c) for c in poly.list()] for poly in (D[0],D[1])]]
out={'schema':'ghs.odd5.s7-transfer/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'cover_receipt_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
     'field_modulus':[int(c) for c in L.modulus().list()],
     'small_field_modulus':[int(c) for c in k.modulus().list()],
     'a_embedding':vec(a),'small_field_embedding':vec(small_embed(k.gen())),
     'source_order':int(N),'r':int(r),'cofactor':int(h),'secret':int(secret),
     'P':[vec(P[0]),vec(P[1])],'Q':[vec(Q[0]),vec(Q[1])],
     'G':divvec(G),'H':divvec(H),'G37':divvec(G37),
     'checks':{'cover_identity':True,'source_order_r':True,
               'image_order_r':True,'image_nonzero':True,
               'frobenius_fixed':True,'Q_scalar_preserved':True,
               'independent_37_scalar_preserved':True}}
target=here/'s7_transfer.json'
assert not target.exists(), 'refusing to overwrite receipt'
target.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
print(json.dumps({k:out[k] for k in ('schema','source_order','r','cofactor','secret','checks')},sort_keys=True))
