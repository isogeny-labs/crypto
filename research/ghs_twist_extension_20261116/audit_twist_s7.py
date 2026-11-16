"""Independent reconstruction of the saved s=7 twist subgroup transfer."""
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing, ZZ, sage_eval

HERE = Path(__file__).resolve().parent
OLD = HERE.parent / 'ghs_odd5_carrier_20261115'
d = json.loads((HERE/'twist_s7_transfer.json').read_text())
assert d['source_sha256'] == hashlib.sha256((HERE/'verify_twist_s7.sage').read_bytes()).hexdigest()
m = json.loads((OLD/'degree7_subgroup_map_30.json').read_text())['maps'][0]
F3 = GF(3)
Z = PolynomialRing(F3, 'z')
z = Z.gen()
K = GF(3**5, 'a', modulus=z**5+2*z+1)
L = GF(3**35, 'w', modulus=Z(sage_eval(d['field_modulus'], locals={'x':z}, preparse=True)))
w = L.gen()
def v(cs):
    return sum((L(c)*w**i for i,c in enumerate(cs)), L(0))
def vec(t):
    return [int(c) for c in t.polynomial().list()]
e = next(f for f in K.embeddings(L) if vec(f(K.gen())) == d['a_embedding'])
a = e(K.gen())
R = PolynomialRing(L, 'x')
x = R.gen()
f0 = x**11+x**9+2*x**8+x**7+2*x**5+x**3+x**2+x+1
f = 2*f0
def poly(cs):
    return R([e(K(c)) for c in cs])
A,B,C = (poly(m[name]) for name in ('A','B','C'))
assert f*C**2 == B*((2*A)**3+(2*A)**2*B+2*(a+a**3)*B**3)
Kx = PolynomialRing(K, 'X')
X = Kx.gen()
fK = X**11+X**9+2*X**8+X**7+2*X**5+X**3+X**2+X+1
Qi = R([e(c) for c in [q for q,n in fK.factor() if q.degree()==2][m['pole_quadratic']].list()])
J = HyperellipticCurve(f).jacobian()
O = J(0)
D0 = J(Qi,R(0))
assert D0 != O and 2*D0 == O
E = EllipticCurve(L,[0,1,0,0,2*(a+a**3)])
P = E(v(d['point_P'][0]),v(d['point_P'][1]))
r = ZZ(d['r'])
assert E.cardinality() == d['source_order'] and r*P == E(0) and P != E(0)
def div(vs):
    return J(R([v(c) for c in vs[0]]),R([v(c) for c in vs[1]]))
def pull(Pt):
    U = (2*A-Pt[0]*B).monic()
    V = (Pt[1]*B**2*C.inverse_mod(U))%U
    assert U.degree()==7 and (V**2-f)%U==0
    return J(U,V)-D0
def frob(D,i):
    e = 3**(7*i)
    return J(R([c**e for c in D[0].list()]),R([c**e for c in D[1].list()]))
D = pull(P)
T = sum((frob(D,i) for i in range(5)),O)
assert D == div(d['pullback_P']) and T == div(d['trace_P'])
assert T != O and r*T == O and frob(T,1)==T
assert pull(37*P)==37*D
assert sum((frob(pull(37*P),i) for i in range(5)),O)==37*T
print(json.dumps({'schema':'ghs.odd5.twist-s7-audit/v1',
                  'cover_rebuilt':True,'source_point_rebuilt':True,
                  'pullback_and_trace_match':True,'order_r_image':True,
                  'scalar_37_match':True,'r':int(r)},sort_keys=True))
