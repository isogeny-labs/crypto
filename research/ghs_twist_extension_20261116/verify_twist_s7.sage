#!/usr/bin/env sage
"""Check the quadratic-twist degree-seven cover and its s=7 subgroup image."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
OLD = ROOT.parent / 'ghs_odd5_carrier_20261115'
hit = json.loads((OLD / 'degree7_quadratic_square_30.json').read_text())['hits'][0]
F3 = GF(3)
Z = PolynomialRing(F3, 'z')
z = Z.gen()
K = GF(3**5, name='a', modulus=z**5 + 2*z + 1)
L = GF(3**35, name='w')
embed = K.embeddings(L)[0]
a = embed(K.gen())
R = PolynomialRing(L, 'x')
x = R.gen()
f = x**11+x**9+2*x**8+x**7+2*x**5+x**3+x**2+x+1
b = a+a**3
A = R(sage_eval(hit['A'], locals={'a':a,'x':x}, preparse=True))
B = R(sage_eval(hit['B'], locals={'a':a,'x':x}, preparse=True))
C = R(sage_eval(hit['C'], locals={'a':a,'x':x}, preparse=True))
assert f*C**2 == B*(A**3+2*A**2*B+b*B**3)
assert (2*f)*C**2 == B*((2*A)**3+(2*A)**2*B+2*b*B**3)
Kx = PolynomialRing(K, 'x')
kx = Kx.gen()
fk = kx**11+kx**9+2*kx**8+kx**7+2*kx**5+kx**3+kx**2+kx+1
Qk = [q for q, n in fk.factor() if q.degree()==2 for _ in range(n)]
Qi = R([embed(c) for c in Qk[hit['poles'][0]].list()])
J = HyperellipticCurve(2*f).jacobian()
O = J(0)
D0 = J(Qi, R(0))
assert D0 != O and 2*D0 == O
E = EllipticCurve(L, [0,1,0,0,2*b])
N = ZZ(50031544838921370)
r = ZZ(185302017921931)
assert N % r == 0 and is_prime(r)
assert E.cardinality() == N
set_random_seed(2026111607)
P = E(0)
for _ in range(30):
    P = (N//r)*E.random_point()
    if not P.is_zero():
        break
assert P != E(0) and r*P == E(0)

def pullback(point):
    if point.is_zero():
        return O
    U = (2*A-point[0]*B).monic()
    assert U.degree()==7 and U.is_squarefree() and gcd(U,C)==1
    V = (point[1]*B**2*C.inverse_mod(U)) % U
    assert (V**2-2*f) % U == 0
    return J(U,V)-D0

def frob(D,i):
    e = 3**(7*i)
    return J(R([c**e for c in D[0].list()]), R([c**e for c in D[1].list()]))

def trace(D):
    return sum((frob(D,i) for i in range(5)), O)

def vec(t):
    return [int(c) for c in t.polynomial().list()]

def polyvec(g):
    return [vec(c) for c in g.list()]

def divvec(D):
    return [polyvec(D[0]),polyvec(D[1])]

D = pullback(P)
T = trace(D)
assert D != O and r*D == O
assert T != O and r*T == O and frob(T,1) == T
P37 = 37*P
assert pullback(P37) == 37*D
assert trace(pullback(P37)) == 37*T
out = {
    'schema':'ghs.odd5.twist-s7-transfer/v1',
    'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
    'source_old_cover':'degree7_quadratic_square_30.json',
    'field_modulus':str(L.modulus()),
    'a_embedding':vec(a),
    'source_order':int(N),
    'r':int(r),
    'cofactor':int(N//r),
    'point_P':[vec(P[0]),vec(P[1])],
    'pullback_P':divvec(D),
    'trace_P':divvec(T),
    'checks':{'twist_identity':True,'source_order':True,'prime_order_source':True,
              'prime_order_pullback':True,'prime_order_trace':True,
              'trace_nonzero':True,'trace_fixed_by_3pow7':True,
              'pullback_37P':True,'trace_37P':True}
}
(ROOT/'twist_s7_transfer.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps({'r':int(r),'trace_nonzero':True,'checks':out['checks']},sort_keys=True))
