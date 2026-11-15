#!/usr/bin/env sage
"""Complete degree-five interpolation search under DEGREE5_PROTOCOL.md."""
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
assert f.is_squarefree() and e.is_squarefree()
fac = [p for p, m in f.factor() for _ in range(m)]
linear = [p for p in fac if p.degree() == 1]
quads = [p for p in fac if p.degree() == 2]
assert linear == [x+1] and len(quads) == 5
Qprod = prod(quads)
assert (x+1)*Qprod == f
rational_roots = e.roots()
assert len(rational_roots) == 1 and rational_roots[0][1] == 1
rational_root = R(rational_roots[0][0])

root_reps = []
for i, q in enumerate(quads):
    L = K.extension(q, names=('u%d' % i,))
    rr = [v for v, m in e.change_ring(L).roots() for _ in range(m)]
    assert len(rr) == 3 and len(set(rr)) == 3
    rr.sort(key=str)
    root_reps.append([R(v.lift().list()) for v in rr])

shapes = ('constant', 'quadratic_branch', 'double_linear',
          'two_quadratic_branches', 'quadratic_branch_double_linear',
          'square_quadratic_nonbranch')
counts = {k: {'logical_assignments': 0, 'interpolation_degree_five': 0,
              'coprime': 0, 'full_equation_hits': 0} for k in shapes}
survivors = []
hits = []


def interpolation(kind, B, h, modulus, choices, parameter):
    counts[kind]['logical_assignments'] += 1
    A = (B*h) % modulus
    if A.degree() != 5:
        return
    counts[kind]['interpolation_degree_five'] += 1
    record = {'shape': kind, 'parameter': parameter,
              'choices': list(choices), 'A': str(A), 'B': str(B)}
    survivors.append(record)
    if gcd(A, B) != 1:
        return
    counts[kind]['coprime'] += 1
    Hpoly = A^3 + 2*A^2*B + a*B^3
    C2, rem = (B*Hpoly).quo_rem(f)
    if rem != 0 or not C2.is_square():
        return
    C = C2.sqrt()
    assert f*C^2 == B*Hpoly
    assert A.degree() == 5 and gcd(A, B) == 1
    counts[kind]['full_equation_hits'] += 1
    hits.append(dict(record, C=str(C)))


def crt_choices(included):
    moduli = [x+1] + [quads[j] for j in included]
    modulus = prod(moduli)
    for choices in itertools.product(range(3), repeat=len(included)):
        residues = [rational_root] + [root_reps[j][k]
                                      for j, k in zip(included, choices)]
        yield choices, CRT_list(residues, moduli), modulus

# e_infinity=5, B=1.
for choices, h, modulus in crt_choices(range(5)):
    interpolation('constant', R(1), h, modulus, choices, None)

# e_infinity=3, B=Qi or (x-c)^2.
for i in range(5):
    included = [j for j in range(5) if j != i]
    for choices, h, modulus in crt_choices(included):
        interpolation('quadratic_branch', quads[i], h, modulus,
                      choices, {'quadratic': i})

full = list(crt_choices(range(5)))
assert len(full) == 243
for c in K:
    if c == -1:
        continue
    B = (x-c)^2
    for choices, h, modulus in full:
        interpolation('double_linear', B, h, modulus, choices, str(c))

# e_infinity=1, B=Qi Qj or Qi (x-c)^2.
for i, j in itertools.combinations(range(5), 2):
    included = [k for k in range(5) if k not in (i, j)]
    for choices, h, modulus in crt_choices(included):
        interpolation('two_quadratic_branches', quads[i]*quads[j], h,
                      modulus, choices, {'quadratics': [i, j]})

for i in range(5):
    included = [j for j in range(5) if j != i]
    branch_assignments = list(crt_choices(included))
    assert len(branch_assignments) == 81
    for c in K:
        if c == -1:
            continue
        B = quads[i]*(x-c)^2
        for choices, h, modulus in branch_assignments:
            interpolation('quadratic_branch_double_linear', B, h,
                          modulus, choices, {'quadratic': i, 'c': str(c)})

# e_infinity=1, B=H^2 with H monic quadratic coprime to f.
# For each assignment, the five high-degree coefficients of B*h mod f
# give a 5x4 linear system in the four lower coefficients of B.
rank_hist = {str(k): 0 for k in range(5)}
consistent_rank_deficient = 0
fallback_H_checks = 0
allowed_H_count = K.cardinality()^2 - K.cardinality() - 5
assert allowed_H_count == 58801
for choices, h, modulus in full:
    assert modulus == f
    rems = [(x^k*h) % f for k in range(5)]
    M = matrix(K, 5, 4, [[rems[k][d] for k in range(4)]
                         for d in range(6, 11)])
    rhs = vector(K, [-rems[4][d] for d in range(6, 11)])
    rank = M.rank()
    rank_hist[str(rank)] += 1
    if M.augment(matrix(K, 5, 1, list(rhs))).rank() != rank:
        counts['square_quadratic_nonbranch']['logical_assignments'] += allowed_H_count
        continue
    if rank == 4:
        coeff = M.solve_right(rhs)
        u = coeff[3] / K(2)
        v = (coeff[2] - u^2) / K(2)
        if coeff[1] == 2*u*v and coeff[0] == v^2:
            H = x^2 + u*x + v
            if gcd(H, f) == 1:
                B = H^2
                interpolation('square_quadratic_nonbranch', B, h, f,
                              choices, {'H': str(H), 'rank': rank})
                counts['square_quadratic_nonbranch']['logical_assignments'] += allowed_H_count - 1
                continue
        counts['square_quadratic_nonbranch']['logical_assignments'] += allowed_H_count
        continue
    consistent_rank_deficient += 1
    actual = 0
    for u in K:
        for v in K:
            H = x^2 + u*x + v
            if gcd(H, f) != 1:
                continue
            actual += 1
            fallback_H_checks += 1
            interpolation('square_quadratic_nonbranch', H^2, h, f,
                          choices, {'H': str(H), 'rank': rank})
    assert actual == allowed_H_count

expected = {
    'constant': 243,
    'quadratic_branch': 5*81,
    'double_linear': 242*243,
    'two_quadratic_branches': 10*27,
    'quadratic_branch_double_linear': 5*242*81,
    'square_quadratic_nonbranch': 58801*243,
}
for shape, n in expected.items():
    assert counts[shape]['logical_assignments'] == n, (shape, counts[shape], n)
assert sum(expected.values()) == 14446377
assert sum(rank_hist.values()) == 243
print(json.dumps({
    'schema': 'ghs.odd5.degree5-screen/v1',
    'source_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    'field': 'F3[a]/(a^5+2a+1)', 'trace': 26,
    'denominator_count': 60269,
    'logical_assignment_count': sum(expected.values()),
    'counts': counts, 'square_quadratic_rank_hist': rank_hist,
    'consistent_rank_deficient': consistent_rank_deficient,
    'fallback_H_checks': fallback_H_checks,
    'survivors': survivors, 'hits': hits,
}, sort_keys=True, default=int))
