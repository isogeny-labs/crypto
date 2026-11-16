#!/usr/bin/env sage
"""Replay the 2-isogeny using fixed rational formulas, without Sage isogeny enumeration."""
import hashlib
import json
import time
from pathlib import Path

t0=time.process_time()
HERE=Path(__file__).resolve().parent
SRC=HERE.parent/'ghs_twist_extension_20261116'/'twist_s7_transfer.json'
ATTACK=HERE.parent/'ghs_twist_s7_attack_20261116'/'s7_full_attack.json'
RHO=HERE.parent/'ghs_twist_s7_attack_20261116'/'rho_s7_result.json'
CONSTRUCT=HERE/'construct.json'
src=json.loads(SRC.read_text())
atk=json.loads(ATTACK.read_text())
rho=json.loads(RHO.read_text())
rec=json.loads(CONSTRUCT.read_text())
assert rec['source_sha256']==hashlib.sha256((HERE/'construct.sage').read_bytes()).hexdigest()
assert rec['source_receipt_sha256']==hashlib.sha256(SRC.read_bytes()).hexdigest()
assert rec['attack_receipt_sha256']==hashlib.sha256(ATTACK.read_bytes()).hexdigest()

K=GF(3^5,'a',modulus=x^5+2*x+1)
a=K.gen()
KX=FractionField(PolynomialRing(K,'X'))
X=KX.gen()
b=a+a^3
e4=2*a^3+2*a^2+1
e6=a^4+a^3+1
E_K=EllipticCurve(K,[0,1,0,0,2*b])
E2_K=EllipticCurve(K,[0,1,0,e4,e6])
assert E_K.order()==E2_K.order()==270
assert E2_K.j_invariant()==2*a^3+a^2+2
assert E_K.j_invariant()!=E2_K.j_invariant()

# Fixed forward and reverse formulas from the K-rational degree-two maps.
c=-a^3-a^2+1
fX=(X^2+c*X+c)/(X+c)
fYm=X^2+(a^3+a^2-1)*X+(a^4-a^3+a+1)
fYd=X^2+(a^3+a^2-1)*X+(a^4+a^3-a^2+a-1)
fY=fYm/fYd
assert fY^2*(X^3+X^2+2*b)==fX^3+fX^2+e4*fX+e6

d=-a^3-a^2-1
gX=(X^2+d*X+(a^3+a^2))/(X+d)
gYm=X^2+(a^3+a^2+1)*X+(a^4+a^3-a^2+a-1)
gYd=X^2+(a^3+a^2+1)*X+(a^4-a^3+a-1)
gY=gYm/gYd
# The reverse codomain has a4=1, a6=2b; its isomorphism to E is x -> x-1.
assert gY^2*(X^3+X^2+e4*X+e6)==gX^3+gX^2+gX+2*b
assert (gX-1)^3+(gX-1)^2+2*b==gX^3+gX^2+gX+2*b

F3=GF(3)
Z=PolynomialRing(F3,'z');z=Z.gen()
L=GF(3^35,'w',modulus=Z(sage_eval(src['field_modulus'],locals={'x':z},preparse=True)))
w=L.gen()
def unpack(cs): return sum((L(k)*w^i for i,k in enumerate(cs)),L(0))
def coords(P): return [[int(k) for k in P[i].polynomial().list()] for i in (0,1)]
A=unpack(src['a_embedding'])
assert A^5+2*A+1==0
B=A+A^3
E=EllipticCurve(L,[0,1,0,0,2*B])
E2=EllipticCurve(L,[0,1,0,2*A^3+2*A^2+1,A^4+A^3+1])
P=E(unpack(atk['source_P'][0]),unpack(atk['source_P'][1]))
Q=E(unpack(atk['source_Q'][0]),unpack(atk['source_Q'][1]))
P2=E2(unpack(rec['P2'][0]),unpack(rec['P2'][1]))
Q2=E2(unpack(rec['Q2'][0]),unpack(rec['Q2'][1]))
E167=EllipticCurve(L,[0,1,0,0,A^4+2*A+1])
P167=E167(unpack(rec['P167'][0]),unpack(rec['P167'][1]))
Q167=E167(unpack(rec['Q167'][0]),unpack(rec['Q167'][1]))
shift=2*A^3+2*A^2+1
assert P2==E2(P167[0]+shift,P167[1])
assert Q2==E2(Q167[0]+shift,Q167[1])

def manual_forward(R):
    xx,yy=R[0],R[1]
    cc=-A^3-A^2+1
    fx=(xx^2+cc*xx+cc)/(xx+cc)
    fm=xx^2+(A^3+A^2-1)*xx+(A^4-A^3+A+1)
    fd=xx^2+(A^3+A^2-1)*xx+(A^4+A^3-A^2+A-1)
    return E2(fx,yy*fm/fd)
def manual_reverse(R):
    xx,yy=R[0],R[1]
    dd=-A^3-A^2-1
    gx=(xx^2+dd*xx+(A^3+A^2))/(xx+dd)
    gm=xx^2+(A^3+A^2+1)*xx+(A^4+A^3-A^2+A-1)
    gd=xx^2+(A^3+A^2+1)*xx+(A^4-A^3+A-1)
    return E(gx-1,-yy*gm/gd)

r=ZZ(atk['r'])
assert is_prime(r) and r*P2==E2(0) and P2!=E2(0)
assert r*P167==E167(0) and P167!=E167(0)
assert Q2==12345*P2
assert Q167==12345*P167
assert manual_forward(int(inverse_mod(2,r))*P)==P2
assert manual_reverse(P2)==P
assert manual_reverse(Q2)==Q
assert coords(manual_reverse(P2))==rec['Pback']
assert coords(manual_reverse(Q2))==rec['Qback']
assert rec['reverse_sign']==-1
assert rho['status']=='verified_solve' and rho['r']==int(r)
assert rho['source_P']==atk['source_P'] and rho['source_Q']==atk['source_Q']
assert rho['solution']['scalar']==12345
c0,c1=rho['solution']['collision_coefficients']
assert c0[0]*P2+c0[1]*Q2==c1[0]*P2+c1[1]*Q2

out={'schema':'ghs.odd5.isogenous2-manual-audit/v1',
     'status':'PASS','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'construct_sha256':hashlib.sha256(CONSTRUCT.read_bytes()).hexdigest(),
     'rho_sha256':hashlib.sha256(RHO.read_bytes()).hexdigest(),
     'r':int(r),'E2_order_K':int(E2_K.order()),
     'curve_equations_verified':True,'point_transport_verified':True,
     'normalized_target_verified':True,
     'rho_collision_transferred':True,
     'cpu_seconds':time.process_time()-t0}
(HERE/'audit_manual.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps(out,indent=2,sort_keys=True,default=int))
