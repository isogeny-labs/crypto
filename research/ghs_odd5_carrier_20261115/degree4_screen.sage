#!/usr/bin/env sage
"""Frozen 405-case branch-pole screen in DEGREE4_PROTOCOL.md."""
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
f = x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
e = x^3+2*x^2+a

assert f.is_squarefree()
factors = [p for p, mult in f.factor() for _ in range(mult)]
linear = [p for p in factors if p.degree() == 1]
quadratics = [p for p in factors if p.degree() == 2]
assert linear == [x+1] and len(quadratics) == 5
assert all(p.is_irreducible() for p in quadratics)
assert e.is_squarefree()

extensions = []
for i, q in enumerate(quadratics):
    L = K.extension(q, names=('u%d' % i,))
    u = L.gen()
    roots = [rt for rt, mult in e.change_ring(L).roots() for _ in range(mult)]
    assert len(roots) == 3 and len(set(roots)) == 3
    roots.sort(key=lambda v: str(v))
    extensions.append((L, u, roots))

counts = {'assignments': 0, 'degree_four_crt': 0,
          'coprime_crt': 0, 'polynomial_quotient': 0, 'square_quotient': 0}
hits = []
for pole_index, pole_q in enumerate(quadratics):
    B = (x+1)*pole_q
    remaining = [j for j in range(5) if j != pole_index]
    for assignment in itertools.product(range(3), repeat=4):
        counts['assignments'] += 1
        residues = []
        moduli = []
        for j, choice in zip(remaining, assignment):
            L, u, roots = extensions[j]
            target_value = B.change_ring(L)(u) * roots[choice]
            residues.append(R(target_value.lift().list()))
            moduli.append(quadratics[j])
        A = CRT_list(residues, moduli)
        assert A.degree() < 8
        if A.degree() != 4:
            continue
        counts['degree_four_crt'] += 1
        if gcd(A, B) != 1:
            continue
        counts['coprime_crt'] += 1
        H = A^3 + 2*A^2*B + a*B^3
        candidate, remainder = (B*H).quo_rem(f)
        if remainder != 0:
            continue
        counts['polynomial_quotient'] += 1
        if candidate.degree() != 4 or not candidate.is_square():
            continue
        counts['square_quotient'] += 1
        C = candidate.sqrt()
        assert f*C^2 == B*H
        hits.append({'pole_index': pole_index, 'assignment': list(assignment),
                     'A': str(A), 'B': str(B), 'C': str(C)})

assert counts['assignments'] == 405
print(json.dumps({'schema': 'ghs.odd5.degree4-branch-pole/v1',
                  'source_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'trace': 26, 'base_field': 'F3[a]/(a^5+2a+1)',
                  'factors': [str(p) for p in factors],
                  'counts': counts, 'hits': hits}, sort_keys=True))
