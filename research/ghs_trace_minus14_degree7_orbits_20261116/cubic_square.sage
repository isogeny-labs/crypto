#!/usr/bin/env sage
"""Exact K-rational degree-seven B=H^2 screen, H monic cubic."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

assert len(sys.argv) == 3
bmask = int(sys.argv[1])
fmask = int(sys.argv[2])
assert bmask in (11,202,38,80)
assert fmask in (9735,17371)
wall0 = time.perf_counter()
cpu0 = time.process_time()

F3 = GF(3)
Z = PolynomialRing(F3, 'z'); z = Z.gen()
K = GF(3^5, name='a', modulus=z^5 + 2*z + 1); a = K.gen()
b = sum(K((bmask // (3^i)) % 3)*a^i for i in range(5))
R = PolynomialRing(K, 'x'); x = R.gen()
f = x^11+sum(K((fmask//3^i)%3)*x^i for i in range(11))
e = x^3 + x^2 + b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K, [0, 1, 0, 0, b]).cardinality() == 258
fac = [p for p,n in f.factor() for _ in range(n)]
Q = [p for p in fac if p.degree() == 2]
linear = [p for p in fac if p.degree() == 1]
assert len(linear) == 1 and len(Q) == 5
L = linear[0]
assert L*prod(Q) == f
r0 = e.roots(multiplicities=False)
assert len(r0) == 1
r0 = R(r0[0])
roots = []
for i, q in enumerate(Q):
    ext = K.extension(q, names=('w%d' % i,))
    vals = sorted([v for v, n in e.change_ring(ext).roots() for _ in range(n)], key=str)
    assert len(vals) == 3 and len(set(vals)) == 3
    roots.append([R(v.lift().list()) for v in vals])

P = PolynomialRing(K, names=('s', 't', 'w'), order='degrevlex')
s, t, w = P.gens()
hc = [w^2, 2*t*w, t^2+2*s*w, 2*s*t+2*w, s^2+2*t, 2*s, P(1)]
counts = {'systems': 0, 'zero_dimensional': 0, 'positive_dimensional': 0,
          'constant_branch_assignments': 0,
          'censored_systems': 0,
          'parameter_solutions': 0, 'allowed_cubics': 0,
          'numerator_degree_seven': 0, 'coprime': 0,
          'f_divides': 0, 'full_equation_hits': 0}
censored = []
hits = []
S = PolynomialRing(K, 'S0'); S0 = S.gen()

def recover_positive_dim(I, eqs):
    """Use the checked Frobenius relation to reduce to 243 quadratic systems."""
    gb = I.groebner_basis()
    relation = next((G for G in gb if G.degree(s) == 0 and
                     G.degree(t) == 3 and G.degree(w) == 3), None)
    if relation is None:
        raise ValueError('no Frobenius relation in positive-dimensional ideal')
    d = K(relation(0, 0, 0))
    c = K(relation(0, 0, 1) - d)
    if relation != t^3 + c*w^3 + d:
        raise ValueError('unexpected positive-dimensional relation')
    tc = (-c)^81
    td = (-d)^81
    assert tc^3 == -c and td^3 == -d
    out = []
    for wv in K:
        tv = tc*wv + td
        polys = [F(S0, tv, wv) for F in eqs]
        first = next((p for p in polys if p != 0), None)
        ss = K if first is None else first.roots(multiplicities=False)
        out.extend((sv, tv, wv) for sv in ss
                   if all(p(sv) == 0 for p in polys))
    return out

for choices in itertools.product(range(3), repeat=5):
    counts['systems'] += 1
    h = CRT_list([r0] + [roots[j][v] for j, v in enumerate(choices)], [L] + Q)
    rems = [(x^i*h) % f for i in range(7)]
    eqs = [sum(hc[i]*P(rems[i][d]) for i in range(7)) for d in (8, 9, 10)]
    I = P.ideal(eqs)
    try:
        if I.dimension() != 0:
            if h.degree() == 0:
                assert all(F == 0 for F in eqs)
                counts['constant_branch_assignments'] += 1
                continue
            counts['positive_dimensional'] += 1
            solutions = recover_positive_dim(I, eqs)
        else:
            counts['zero_dimensional'] += 1
            solutions = [(K(sol[s]), K(sol[t]), K(sol[w])) for sol in I.variety(ring=K)]
    except Exception as exc:
        censored.append({'choices': list(choices), 'reason': type(exc).__name__,
                         'detail': str(exc)[:300]})
        counts['censored_systems'] += 1
        continue
    for sv, tv, wv in solutions:
        assert all(F(sv, tv, wv) == 0 for F in eqs)
        counts['parameter_solutions'] += 1
        H = x^3 + sv*x^2 + tv*x + wv
        if gcd(H, f) != 1:
            continue
        counts['allowed_cubics'] += 1
        B = H^2
        A = (B*h) % f
        if A.degree() != 7:
            continue
        counts['numerator_degree_seven'] += 1
        if gcd(A, B) != 1:
            continue
        counts['coprime'] += 1
        N = B*(A^3 + A^2*B + b*B^3)
        C2, rem = N.quo_rem(f)
        if rem != 0:
            continue
        counts['f_divides'] += 1
        if not C2.is_square():
            continue
        C = C2.sqrt()
        assert f*C^2 == N and gcd(A, B) == 1
        counts['full_equation_hits'] += 1
        hits.append({'choices': list(choices), 'H': str(H),
                     'A': str(A), 'B': str(B), 'C': str(C)})

source = Path(__file__).with_suffix('')
print(json.dumps({'schema': 'ghs.degree7.orbit-cubic-square/v1',
                  'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                  'field': 'F3[a]/(a^5+2a+1)', 'carrier': str(f),
                  'bmask': bmask, 'carrier_mask': fmask, 'target_order': 258, 'counts': counts,
                  'censored': censored, 'hits': hits,
                  'wall_seconds': time.perf_counter()-wall0,
                  'cpu_seconds': time.process_time()-cpu0}, sort_keys=True, default=int))
