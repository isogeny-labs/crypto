#!/usr/bin/env sage
"""Rebuild the s=7 transfer from saved coordinates and the original cover."""
import hashlib
import json
from pathlib import Path

here=Path(__file__).resolve().parent
receipt_path=here/'s7_transfer.json'
cover_path=here.parent/'ghs_odd5_carrier_20261115'/'degree7_quadratic_square_30.json'
d=json.loads(receipt_path.read_text())
hit=json.loads(cover_path.read_text())['hits'][0]
assert d['source_sha256']==hashlib.sha256((here/'s7_transfer.sage').read_bytes()).hexdigest()
assert d['cover_receipt_sha256']==hashlib.sha256(cover_path.read_bytes()).hexdigest()
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,'a',modulus=z^5+2*z+1)
assert EllipticCurve(K,[0,2,0,0,K.gen()+K.gen()^3]).cardinality()==218
t0,t1=ZZ(2),ZZ(26)
for _ in range(2,8):t0,t1=t1,26*t1-243*t0
assert 3^35+1-t1==d['source_order']
L=GF(3^35,'w',modulus=Z(d['field_modulus']))
def elem(v,F):return F(v)
a=elem(d['a_embedding'],L)
assert a^5+2*a+1==0 and a^243==a and a^3!=a
k=L.subfield(7);small_embed=k.embeddings(L)[0];back=small_embed.section()
assert [int(c) for c in k.modulus().list()]==d['small_field_modulus']
assert [int(c) for c in small_embed(k.gen()).polynomial().list()]==d['small_field_embedding']
R=PolynomialRing(L,'x');x=R.gen()
f=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
b=a+a^3
A=R(sage_eval(hit['A'],locals={'a':a,'x':x},preparse=True))
B=R(sage_eval(hit['B'],locals={'a':a,'x':x},preparse=True))
C=R(sage_eval(hit['C'],locals={'a':a,'x':x},preparse=True))
assert f*C^2==B*(A^3+2*A^2*B+b*B^3)
Jbig=HyperellipticCurve(f).jacobian();Obig=Jbig(0)
Kx=PolynomialRing(K,'x');xx=Kx.gen()
fk=xx^11+xx^9+2*xx^8+xx^7+2*xx^5+xx^3+xx^2+xx+1
quadratics=[q for q,m in fk.factor() if q.degree()==2 for _ in range(m)]
embed=K.embeddings(L)[0]
assert embed(K.gen())==a
Qi=R([embed(c) for c in quadratics[hit['poles'][0]].list()])
D0=Jbig(Qi,R(0))
Rk=PolynomialRing(k,'x');xk=Rk.gen()
Y=HyperellipticCurve(xk^11+xk^9+2*xk^8+xk^7+2*xk^5+xk^3+xk^2+xk+1)
J=Y.jacobian();O=J(0)
E=EllipticCurve(L,[0,2,0,0,b])
P=E(elem(d['P'][0],L),elem(d['P'][1],L))
Q=E(elem(d['Q'][0],L),elem(d['Q'][1],L))
r=ZZ(d['r']);secret=ZZ(d['secret'])
assert is_prime(r) and P!=E(0) and r*P==E(0) and Q==secret*P

def lift_saved(item):
 U,V=item[0]
 return J(Rk([elem(c,k) for c in U]),Rk([elem(c,k) for c in V]))
def transfer(T):
 U=(A-T[0]*B).monic()
 V=(T[1]*B^2*C.inverse_mod(U))%U
 assert (V^2-f)%U==0
 D=Jbig(U,V)-D0
 total=sum((Jbig(R([c^(3^(7*j)) for c in D[0].list()]),
                 R([c^(3^(7*j)) for c in D[1].list()])) for j in range(5)),Obig)
 assert Jbig(R([c^(3^7) for c in total[0].list()]),
             R([c^(3^7) for c in total[1].list()]))==total
 return J(Rk([back(c) for c in total[0].list()]),
          Rk([back(c) for c in total[1].list()]))
G=transfer(P);H=transfer(Q);G37=transfer(37*P)
assert G==lift_saved(d['G']) and H==lift_saved(d['H'])
assert G37==lift_saved(d['G37'])
assert G!=O and r*G==O and H==secret*G and G37==37*G

out={'schema':'ghs.odd5.s7-transfer-audit/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'input_sha256':hashlib.sha256(receipt_path.read_bytes()).hexdigest(),
     'base_order':int(218),'derived_s7_order':int(3^35+1-t1),
     'saved_images_reconstructed':True,'source_and_target_scalar_checks':True}
target=here/'s7_transfer_audit.json'
assert not target.exists(), 'refusing to overwrite receipt'
target.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
print(json.dumps(out,sort_keys=True))
