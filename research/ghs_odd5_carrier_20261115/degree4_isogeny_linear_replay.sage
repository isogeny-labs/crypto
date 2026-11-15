#!/usr/bin/env sage
"""Independent degree-four direct-linear replay for alternate targets."""
import hashlib
import itertools
import json
import sys
from pathlib import Path

F = GF(3)
Z = PolynomialRing(F, 'z')
z = Z.gen()
K = GF(3^5, name='a', modulus=z^5+2*z+1)
a = K.gen()
assert len(sys.argv) == 2
bmask = int(sys.argv[1])
assert bmask in (27, 30, 103, 139, 167, 182, 194, 230, 233)
target_b = sum(K((bmask // (3^i)) % 3)*a^i for i in range(5))
R = PolynomialRing(K, 'x')
x = R.gen()
f = x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
e = x^3+2*x^2+target_b
assert EllipticCurve(K, [0, 2, 0, 0, target_b]).cardinality() == 218
factors = [p for p, m in f.factor() for _ in range(m)]
L1 = [p for p in factors if p.degree() == 1]
Q = [p for p in factors if p.degree() == 2]
assert L1 == [x+1] and len(Q) == 5

values = []
for j, q in enumerate(Q):
    ext = K.extension(q, names=('w%d' % j,))
    roots = [v for v, m in e.change_ring(ext).roots() for _ in range(m)]
    assert len(roots) == 3
    values.append([R(v.lift().list()) for v in roots])

def linear_filter(indices):
    rows = []
    moduli = [Q[j] for j in indices]
    for q in moduli:
        for coefficient in (0, 1):
            rows.append([((x^k) % q)[coefficient] for k in range(5)])
    M = matrix(K, rows)
    assert M.rank() == 5
    annihilator = M.left_kernel().basis_matrix()
    assert annihilator.ncols() == M.nrows()
    return M, annihilator, moduli

partial = {pole: linear_filter([j for j in range(5) if j != pole])
           for pole in range(5)}
complete = linear_filter(list(range(5)))

counts = {kind: {'assignments': 0, 'consistent_degree_at_most_four': 0,
                 'degree_exactly_four': 0} for kind in
          ('linear_plus_quadratic', 'linear_plus_double_nonbranch',
           'triple_linear', 'linear_only')}
solutions = []

def check(kind, B, indices, choices, filter_data, parameter):
    counts[kind]['assignments'] += 1
    M, annihilator, moduli = filter_data
    rhs = []
    for j, choice, q in zip(indices, choices, moduli):
        residue = (B * values[j][choice]) % q
        rhs.extend((residue[0], residue[1]))
    b = vector(K, rhs)
    if any(annihilator * b):
        return
    counts[kind]['consistent_degree_at_most_four'] += 1
    coefficient = M.solve_right(b)
    if coefficient[4] == 0:
        return
    counts[kind]['degree_exactly_four'] += 1
    A = sum(coefficient[k]*x^k for k in range(5))
    assert all((A-B*values[j][choice]) % Q[j] == 0
               for j, choice in zip(indices, choices))
    solutions.append({'kind': kind, 'parameter': parameter,
                      'assignment': list(choices), 'A': str(A), 'B': str(B)})

for pole in range(5):
    B = (x+1)*Q[pole]
    indices = [j for j in range(5) if j != pole]
    for choices in itertools.product(range(3), repeat=4):
        check('linear_plus_quadratic', B, indices, choices,
              partial[pole], pole)

indices = list(range(5))
for c in K:
    if c == -1:
        continue
    B = (x+1)*(x-c)^2
    for choices in itertools.product(range(3), repeat=5):
        check('linear_plus_double_nonbranch', B, indices,
              choices, complete, str(c))

for kind, B in [('triple_linear',(x+1)^3),('linear_only',x+1)]:
    for choices in itertools.product(range(3), repeat=5):
        check(kind, B, indices, choices, complete, None)

assert sum(v['assignments'] for v in counts.values()) == 59697
print(json.dumps({'schema': 'ghs.odd5.degree4-isogeny-linear-replay/v1',
                  'source_sha256': hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
                  'trace': 26, 'a6_mask': bmask, 'a6': str(target_b),
                  'target_order': 218, 'counts': counts, 'solutions': solutions},
                 sort_keys=True, default=int))
