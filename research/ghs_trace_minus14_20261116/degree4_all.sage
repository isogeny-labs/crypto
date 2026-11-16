#!/usr/bin/env sage
"""Exhaust the degree-four denominator shapes in DEGREE4_ALL_PROTOCOL.md."""
import hashlib
import itertools
import json
from pathlib import Path

F3 = GF(3)
Pz = PolynomialRing(F3, 'z')
z = Pz.gen()
K = GF(3^5, name='a', modulus=z^5 + 2*z + 1)
a = K.gen()
R = PolynomialRing(K, 'x')
x = R.gen()
f = x^11+x^8+x^7+x^6+x^5+x^2+2*x
b = a^2+2
e = x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
fac = [p for p, m in f.factor() for _ in range(m)]
linear = [p for p in fac if p.degree() == 1]
quads = [p for p in fac if p.degree() == 2]
assert linear == [x] and len(quads) == 5
Qprod = prod(quads)

root_reps = []
for i, q in enumerate(quads):
    L = K.extension(q, names=('u%d' % i,))
    rr = [v for v, m in e.change_ring(L).roots() for _ in range(m)]
    assert len(rr) == 3 and len(set(rr)) == 3
    rr.sort(key=lambda v: str(v))
    root_reps.append([R(v.lift().list()) for v in rr])

counts = {kind: {'assignments': 0, 'degree_four_crt': 0,
                 'coprime_crt': 0, 'polynomial_quotient': 0,
                 'square_quotient': 0} for kind in
          ('linear_plus_quadratic', 'linear_plus_double_nonbranch',
           'triple_linear', 'linear_only')}
survivors = []
hits = []

def check(kind, B, included, choices, h, modulus, parameter):
    counts[kind]['assignments'] += 1
    A = (B*h) % modulus
    if A.degree() != 4:
        return
    counts[kind]['degree_four_crt'] += 1
    survivors.append({'kind': kind, 'parameter': parameter,
                      'assignment': list(choices), 'A': str(A), 'B': str(B)})
    if gcd(A, B) != 1:
        return
    counts[kind]['coprime_crt'] += 1
    H = A^3 + A^2*B + b*B^3
    C2, rem = (B*H).quo_rem(f)
    if rem != 0:
        return
    counts[kind]['polynomial_quotient'] += 1
    if not C2.is_square():
        return
    counts[kind]['square_quotient'] += 1
    C = C2.sqrt()
    assert f*C^2 == B*H
    hits.append({'kind': kind, 'parameter': parameter,
                 'assignment': list(choices), 'A': str(A),
                 'B': str(B), 'C': str(C)})

# Five degree-three denominators with three finite branch poles.
for pole in range(5):
    remaining = [j for j in range(5) if j != pole]
    moduli = [quads[j] for j in remaining]
    modulus = prod(moduli)
    B = x*quads[pole]
    for choices in itertools.product(range(3), repeat=4):
        h = CRT_list([root_reps[j][k] for j, k in zip(remaining, choices)], moduli)
        check('linear_plus_quadratic', B, remaining, choices, h, modulus, pole)

# Every other allowed denominator has all five quadratic branch pairs finite.
full_assignments = []
for choices in itertools.product(range(3), repeat=5):
    h = CRT_list([root_reps[j][k] for j, k in enumerate(choices)], quads)
    full_assignments.append((choices, h))

for c in K:
    if c == 0:
        continue
    B = x*(x-c)^2
    for choices, h in full_assignments:
        check('linear_plus_double_nonbranch', B, range(5), choices, h,
              Qprod, str(c))

for kind, B in [('triple_linear', x^3), ('linear_only', x+1)]:
    for choices, h in full_assignments:
        check(kind, B, range(5), choices, h, Qprod, None)

assert sum(c['assignments'] for c in counts.values()) == 59697
print(json.dumps({'schema': 'ghs.odd5.degree4-all/v1',
                  'source_sha256': hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
                  'field': 'F3[a]/(a^5+2a+1)', 'trace': -14,
                  'counts': counts, 'survivors': survivors,
                  'hits': hits}, sort_keys=True, default=int))
