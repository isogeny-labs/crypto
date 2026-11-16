#!/usr/bin/env sage
"""Attacker-facing E2 target preprocessing; takes only public curve and points."""
import hashlib
import json
import time
from pathlib import Path

t0=time.process_time()
HERE=Path(__file__).resolve().parent
REC=HERE/'construct.json'
rec=json.loads(REC.read_text())
F3=GF(3)
Z=PolynomialRing(F3,'z');z=Z.gen()
L=GF(3^35,'w',modulus=Z(sage_eval(rec['field_modulus'],locals={'x':z},preparse=True)))
w=L.gen()
def unpack(cs):return sum((L(c)*w^i for i,c in enumerate(cs)),L(0))
def pack(x):return [int(c) for c in x.polynomial().list()]
def point_pack(P):return [pack(P[0]),pack(P[1])]
a=unpack(rec['a_embedding'])
b=a+a^3
E=EllipticCurve(L,[0,1,0,0,2*b])
E2=EllipticCurve(L,[0,1,0,2*a^3+2*a^2+1,a^4+a^3+1])
E167=EllipticCurve(L,[0,1,0,0,a^4+2*a+1])
P167=E167(unpack(rec['P167'][0]),unpack(rec['P167'][1]))
Q167=E167(unpack(rec['Q167'][0]),unpack(rec['Q167'][1]))
shift=2*a^3+2*a^2+1
P2=E2(P167[0]+shift,P167[1])
Q2=E2(Q167[0]+shift,Q167[1])
def psi(R):
    xx,yy=R[0],R[1]
    d=-a^3-a^2-1
    xg=(xx^2+d*xx+(a^3+a^2))/(xx+d)
    gm=xx^2+(a^3+a^2+1)*xx+(a^4+a^3-a^2+a-1)
    gd=xx^2+(a^3+a^2+1)*xx+(a^4-a^3+a-1)
    return E(xg-1,-yy*gm/gd)
P=psi(P2)
Q=psi(Q2)
elapsed=time.process_time()-t0
out={'schema':'ghs.odd5.isogenous2-preprocess/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'construct_sha256':hashlib.sha256(REC.read_bytes()).hexdigest(),
     'cpu_seconds':elapsed,'P':point_pack(P),'Q':point_pack(Q)}
(HERE/'preprocess.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'schema':out['schema'],'cpu_seconds':elapsed,'P_sha256':hashlib.sha256(json.dumps(out['P']).encode()).hexdigest(),'Q_sha256':hashlib.sha256(json.dumps(out['Q']).encode()).hexdigest()},sort_keys=True))
