#!/usr/bin/env sage
"""Independent degree-six replay: field scan and coefficient square roots.

This does not import the producer or use resultants or Polynomial.is_square.
"""
import hashlib
import itertools
import json
import sys
from pathlib import Path

F = GF(3)
T = PolynomialRing(F, 't')
t = T.gen()
K = GF(3^5, name='a', modulus=t^5+2*t+1)
a = K.gen()
R = PolynomialRing(K, 'x')
x = R.gen()
V = PolynomialRing(K, 'v')
MASKS = (3, 27, 30, 103, 139, 167, 182, 194, 230, 233)
assert len(sys.argv) == 2
mask = int(sys.argv[1])
assert mask in MASKS
b = sum(K((mask // (3^j)) % 3)*a^j for j in range(5))
f = x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
L = x+1
Q = [q for q, multiplicity in f.factor() if q.degree() == 2]
assert len(Q) == 5 and f == L*prod(Q)
roots = []
for i, q in enumerate(Q):
    ext = K.extension(q, names=('w%d' % i,))
    vals = [R(z.lift().list()) for z, multiplicity in
            (x^3+2*x^2+b).change_ring(ext).roots()]
    assert len(vals) == 3
    roots.append(vals)

SHAPES = ('L', 'L3', 'LQi', 'Ldouble', 'L5', 'L3Qi',
          'L3double', 'LQiQj', 'LQidouble', 'LHsquare')
counts = {s: {'branch_assignments': 0, 'degree_six': 0,
              'coprime': 0, 'divisible': 0, 'squares': 0}
          for s in SHAPES}
hits = []


def assignments(indices):
    moduli = [Q[i] for i in indices]
    M = prod(moduli)
    for choice in itertools.product(range(3), repeat=len(indices)):
        h = CRT_list([roots[i][k] for i, k in zip(indices, choice)], moduli)
        yield choice, h, M


def square_by_coeff(g):
    if g == 0:
        return True, R(0)
    if g.degree() % 2:
        return False, None
    n = g.degree() // 2
    if not g[2*n].is_square():
        return False, None
    c = [K(0) for _ in range(n+1)]
    c[n] = g[2*n].sqrt()
    for k in range(n-1, -1, -1):
        known = sum((c[i]*c[j] for i in range(k+1, n+1)
                     for j in range(k+1, n+1) if i+j == n+k), K(0))
        c[k] = (g[n+k]-known)/(2*c[n])
    root = R(c)
    return root^2 == g, root


def test(kind, A, B):
    if A.degree() != 6:
        return
    counts[kind]['degree_six'] += 1
    if gcd(A, B) != 1:
        return
    counts[kind]['coprime'] += 1
    N = B*(A^3+2*A^2*B+b*B^3)
    quotient, remainder = N.quo_rem(f)
    if remainder:
        return
    counts[kind]['divisible'] += 1
    square, C = square_by_coeff(quotient)
    if square:
        assert f*C^2 == N
        counts[kind]['squares'] += 1
        hits.append({'shape': kind, 'A': str(A), 'B': str(B), 'C': str(C)})


def fixed(kind, B, collection):
    for choice, h, M in collection:
        counts[kind]['branch_assignments'] += 1
        residue = (B*h) % M
        if M.degree() == 6:
            assert kind == 'LQiQj'
            for scalar in K:
                if scalar:
                    test(kind, residue+scalar*M, B)
        else:
            assert M.degree() >= 8
            test(kind, residue, B)


full = list(assignments(tuple(range(5))))
assert len(full) == 243
for kind, B in (('L', L), ('L3', L^3), ('L5', L^5)):
    fixed(kind, B, full)
for i in range(5):
    short = list(assignments(tuple(j for j in range(5) if j != i)))
    fixed('LQi', L*Q[i], short)
    fixed('L3Qi', L^3*Q[i], short)
    for c in K:
        if c != -1:
            fixed('LQidouble', L*Q[i]*(x-c)^2, short)
for c in K:
    if c != -1:
        fixed('Ldouble', L*(x-c)^2, full)
        fixed('L3double', L^3*(x-c)^2, full)
for i, j in itertools.combinations(range(5), 2):
    short = assignments(tuple(k for k in range(5) if k not in (i, j)))
    fixed('LQiQj', L*Q[i]*Q[j], short)

# Scan u in K and solve the first nonzero quadratic in v; test all three
# high-coefficient equations. No resultant projection or fallback choice.
Qprod = prod(Q)
for choice, h, M in full:
    assert M == Qprod
    counts['LHsquare']['branch_assignments'] += K.cardinality()^2-K.cardinality()-5
    remainders = [(L*x^j*h) % Qprod for j in range(5)]
    for u in K:
        polys = []
        for degree in (7, 8, 9):
            r = remainders
            polys.append(V([r[4][degree]+2*u*r[3][degree]+u^2*r[2][degree],
                            2*r[2][degree]+2*u*r[1][degree],
                            r[0][degree]]))
        first = next((p for p in polys if p), None)
        values = list(K) if first is None else first.roots(multiplicities=False)
        for v in values:
            if not all(p(v) == 0 for p in polys):
                continue
            H = x^2+u*x+v
            if gcd(H, f) != 1:
                continue
            B = L*H^2
            A = (B*h) % Qprod
            test('LHsquare', A, B)

expected = {'L': 243, 'L3': 243, 'LQi': 405, 'Ldouble': 58806,
            'L5': 243, 'L3Qi': 405, 'L3double': 58806, 'LQiQj': 270,
            'LQidouble': 98010, 'LHsquare': 14288643}
assert {k: v['branch_assignments'] for k, v in counts.items()} == expected
print(json.dumps({
    'schema': 'ghs.odd5.degree6-direct-replay/v1',
    'source_sha256': hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
    'mask': mask, 'counts': counts, 'hits': hits,
}, sort_keys=True, default=int))
