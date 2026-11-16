"""Rebuild both isogenies and the Jacobian trace from saved coefficient vectors."""
import json
from pathlib import Path
from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing, ZZ

PTH=Path(__file__).resolve().parent
d=json.loads((PTH/'primitive_cover_s11.json').read_text())
m=json.loads((PTH/'degree7_subgroup_map_30.json').read_text())['maps'][0]
F3=GF(3);Z=PolynomialRing(F3,'z')
K=GF(3**5,'a',modulus=Z([1,2,0,0,0,1]))
L=GF(3**55,'w',modulus=Z(d['field_modulus']))
emb=K.embeddings(L)[0];w=L.gen();a=emb(K.gen())
def e(v):return sum((L(c)*w**i for i,c in enumerate(v)),L(0))
def point(E,p):return E(e(p[0]),e(p[1]))
R=PolynomialRing(L,'x');x=R.gen()
def pvec(v):return R([emb(K(c)) for c in v])
E=EllipticCurve(L,[e(v) for v in d['source_ainvs']])
C=EllipticCurve(L,[e(v) for v in d['primitive_ainvs']])
assert C.j_invariant()==e(d['primitive_j']) and C.j_invariant().minpoly().degree()==55
P=point(E,d['P']);Q=point(C,d['Q']);r=ZZ(d['r'])
assert r*P==E(0) and r*Q==C(0) and not Q.is_zero()
forward=E.isogeny(R([e(c) for c in d['forward_kernel_polynomial']]))
assert forward.codomain().ainvs()==C.ainvs() and forward(P)==Q
reverse=C.isogeny(R([e(c) for c in d['reverse_kernel_polynomial']]))
assert reverse.codomain().ainvs()==tuple(e(v) for v in d['reverse_codomain_ainvs'])
iso=reverse.codomain().isomorphism_to(E)
back=d['reverse_sign']*iso(reverse(Q))
assert back==23*P and back==point(E,d['reverse_Q'])
A=pvec(m['A']);B=pvec(m['B']);Cc=pvec(m['C'])
f=x**11+x**9+2*x**8+x**7+2*x**5+x**3+x**2+x+1
assert f*Cc**2==B*(A**3+2*A**2*B+(a+a**3)*B**3)
Kx=PolynomialRing(K,'x');xx=Kx.gen()
fk=xx**11+xx**9+2*xx**8+xx**7+2*xx**5+xx**3+xx**2+xx+1
Qi=pvec([[int(c) for c in u.polynomial().list()] for u in [q for q,n in fk.factor() if q.degree()==2][m['pole_quadratic']].list()])
J=HyperellipticCurve(f).jacobian();O=J(0);D0=J(Qi,R(0))
def pull(Qp):
 U=(A-Qp[0]*B).monic();V=(Qp[1]*B**2*Cc.inverse_mod(U))%U
 assert U.degree()==7 and (V**2-f)%U==0
 return J(U,V)-D0
def frob(D,i):return J(R([c**(3**(11*i)) for c in D[0].list()]),R([c**(3**(11*i)) for c in D[1].list()]))
def trace(D):return sum((frob(D,i) for i in range(5)),O)
def saved_div(v):return J(R([e(c) for c in v[0]]),R([e(c) for c in v[1]]))
T=trace(pull(back));T37=trace(pull(d['reverse_sign']*iso(reverse(37*Q))))
assert T!=O and r*T==O and frob(T,1)==T
assert T==saved_div(d['trace_Q']) and T37==saved_div(d['trace_37Q']) and T37==37*T
print(json.dumps({'schema':'ghs.odd5.primitive-cover-s11-replay/v1','j_degree':55,
                  'forward_kernel_rebuilt':True,'reverse_kernel_rebuilt':True,
                  'saved_trace_rebuilt':True,'scalar_preserving':True},sort_keys=True))
