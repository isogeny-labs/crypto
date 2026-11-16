#!/usr/bin/env sage
"""Construct the exact 2-isogenous source and replay its saved target transfer."""
import hashlib
import json
import time
from pathlib import Path

t0 = time.process_time()
HERE = Path(__file__).resolve().parent
SRC = HERE.parent / 'ghs_twist_extension_20261116' / 'twist_s7_transfer.json'
ATTACK = HERE.parent / 'ghs_twist_s7_attack_20261116' / 's7_full_attack.json'
src = json.loads(SRC.read_text())
atk = json.loads(ATTACK.read_text())

F3 = GF(3)
Z = PolynomialRing(F3, 'z')
z = Z.gen()
L = GF(3^35, 'w', modulus=Z(sage_eval(src['field_modulus'], locals={'x':z}, preparse=True)))
w = L.gen()
def field_vec(x):
    return [int(c) for c in x.polynomial().list()]
def field_from_vec(cs):
    return sum((L(c)*w^i for i,c in enumerate(cs)), L(0))
def point_vec(P):
    return [field_vec(P[0]),field_vec(P[1])]

a = field_from_vec(src['a_embedding'])
assert a^5+2*a+1 == 0
b = a+a^3
E = EllipticCurve(L, [0,1,0,0,2*b])
P = E(field_from_vec(atk['source_P'][0]), field_from_vec(atk['source_P'][1]))
Q = E(field_from_vec(atk['source_Q'][0]), field_from_vec(atk['source_Q'][1]))
r = ZZ(atk['r'])
assert r == 185302017921931 and is_prime(r)
assert r*P == E(0) and P != E(0) and Q == 12345*P

forward = E.isogenies_prime_degree(2)
assert len(forward) == 1
phi = forward[0]
E2 = phi.codomain()
assert phi.degree() == 2
assert list(E2.a_invariants()) == [L(0),L(1),L(0),2*a^3+2*a^2+1,a^4+a^3+1]
reverse = E2.isogenies_prime_degree(2)
assert len(reverse) == 1
rev = reverse[0]
assert rev.degree() == 2 and rev.codomain().is_isomorphic(E)
iso = rev.codomain().isomorphism_to(E)
psi0 = lambda R: iso(rev(R))
image = psi0(phi(P))
assert image == 2*P or image == -2*P
sign = 1 if image == 2*P else -1
psi = lambda R: sign*psi0(R)

u = inverse_mod(2,r)
P2 = phi(int(u)*P)
Q2 = 12345*P2
shift = 2*a^3+2*a^2+1
E167 = EllipticCurve(L,[0,1,0,0,a^4+2*a+1])
P167 = E167(P2[0]-shift,P2[1])
Q167 = E167(Q2[0]-shift,Q2[1])
Pback = psi(P2)
Qback = psi(Q2)
assert P2 != E2(0) and r*P2 == E2(0)
assert Pback == P and Qback == Q
assert r*P167==E167(0) and Q167==12345*P167
assert E2.j_invariant() != E.j_invariant()
assert E2.j_invariant()^(3^5) == E2.j_invariant()
elapsed = time.process_time()-t0

out = {
    'schema':'ghs.odd5.isogenous2/v1',
    'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
    'source_receipt_sha256':hashlib.sha256(SRC.read_bytes()).hexdigest(),
    'attack_receipt_sha256':hashlib.sha256(ATTACK.read_bytes()).hexdigest(),
    'field_modulus':src['field_modulus'],
    'a_embedding':src['a_embedding'],
    'E_invariants':[field_vec(c) for c in E.a_invariants()],
    'E2_invariants_in_a':[[0],[1],[0],field_vec(2*a^3+2*a^2+1),field_vec(a^4+a^3+1)],
    'E2_invariants':[field_vec(c) for c in E2.a_invariants()],
    'E167_invariants':[field_vec(c) for c in E167.a_invariants()],
    'E_j':field_vec(E.j_invariant()),
    'E2_j':field_vec(E2.j_invariant()),
    'phi_kernel_polynomial':str(phi.kernel_polynomial()),
    'reverse_sign':sign,
    'r':int(r),
    'P2':point_vec(P2),
    'Q2':point_vec(Q2),
    'P167':point_vec(P167),
    'Q167':point_vec(Q167),
    'Pback':point_vec(Pback),
    'Qback':point_vec(Qback),
    'checks':{'source_order_r':True,'codomain_order_r':True,
              'forward_reverse_composition_2':True,
              'transport_P':True,'transport_Q':True,
              'distinct_j':True,'codomain_over_K':True,
              'normalized_mask167_target':True},
    'cpu_seconds':elapsed,
}
(HERE/'construct.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({k:v for k,v in out.items() if k not in ('P2','Q2','Pback','Qback','E_invariants','E2_invariants','E2_invariants_in_a','a_embedding','field_modulus')},indent=2,sort_keys=True,default=int))
