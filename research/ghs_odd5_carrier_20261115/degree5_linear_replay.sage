#!/usr/bin/env sage
"""Direct coefficient-system replay under DEGREE5_REPLAY_PROTOCOL.md."""
import hashlib
import itertools
import json
from pathlib import Path

F = GF(3)
Z = PolynomialRing(F, 'z')
z = Z.gen()
K = GF(3^5, name='a', modulus=z^5+2*z+1)
a = K.gen()
R = PolynomialRing(K, 'x')
x = R.gen()
f = x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
e = x^3+2*x^2+a
factors = [p for p, m in f.factor() for _ in range(m)]
assert f.is_squarefree() and e.is_squarefree()
assert [p for p in factors if p.degree() == 1] == [x+1]
Q = [p for p in factors if p.degree() == 2]
assert len(Q) == 5
rational_roots = e.roots()
assert len(rational_roots) == 1 and rational_roots[0][1] == 1
rho = R(rational_roots[0][0])
roots = []
for j, q in enumerate(Q):
    L = K.extension(q, names=('v%d' % j,))
    vals = [t for t, multiplicity in e.change_ring(L).roots()
            for _ in range(multiplicity)]
    assert len(vals) == 3 and len(set(vals)) == 3
    roots.append([R(t.lift().list()) for t in vals])

shapes = ('constant', 'quadratic_branch', 'double_linear',
          'two_quadratic_branches', 'quadratic_branch_double_linear',
          'square_quadratic_nonbranch')
counts = {s: {'logical_assignments': 0, 'consistent_degree_at_most_five': 0,
              'degree_five': 0, 'full_equation_hits': 0} for s in shapes}
survivors = []
hits = []


def system(indices):
    moduli = [x+1] + [Q[j] for j in indices]
    rows = []
    for q in moduli:
        for d in range(q.degree()):
            rows.append([((x^i) % q)[d] for i in range(6)])
    mat = matrix(K, rows)
    assert mat.nrows() == sum(q.degree() for q in moduli)
    assert mat.rank() == 6
    return mat, mat.left_kernel().basis_matrix(), moduli

systems = {}
for indices in [tuple(range(5))] + [tuple(j for j in range(5) if j != i)
                                      for i in range(5)] + [tuple(j for j in range(5)
                                      if j not in ij)
                                      for ij in itertools.combinations(range(5), 2)]:
    systems[indices] = system(indices)


def values(indices, choices):
    return [rho] + [roots[j][k] for j, k in zip(indices, choices)]


def full_test(kind, A, B, choices, parameter):
    record = {'shape': kind, 'A': str(A), 'B': str(B),
              'choices': list(choices), 'parameter': parameter}
    survivors.append(record)
    assert A.degree() == 5 and gcd(A, B) == 1
    numerator = B*(A^3+2*A^2*B+a*B^3)
    quotient, remainder = numerator.quo_rem(f)
    record['equation_remainder_zero'] = (remainder == 0)
    record['quotient_square'] = bool(remainder == 0 and quotient.is_square())
    if record['quotient_square']:
        C = quotient.sqrt()
        assert f*C^2 == numerator
        counts[kind]['full_equation_hits'] += 1
        hits.append(dict(record, C=str(C)))


def check_fixed(kind, B, indices, choices, parameter, counted=True):
    if counted:
        counts[kind]['logical_assignments'] += 1
    mat, annihilator, moduli = systems[tuple(indices)]
    rhs = []
    for q, target in zip(moduli, values(indices, choices)):
        term = (B*target) % q
        rhs.extend(term[d] for d in range(q.degree()))
    rhs = vector(K, rhs)
    if any(annihilator*rhs):
        return
    counts[kind]['consistent_degree_at_most_five'] += 1
    coeff = mat.solve_right(rhs)
    if coeff[5] == 0:
        return
    counts[kind]['degree_five'] += 1
    A = sum(coeff[i]*x^i for i in range(6))
    for q, target in zip(moduli, values(indices, choices)):
        assert (A-B*target) % q == 0
    if gcd(A, B) != 1:
        return
    full_test(kind, A, B, choices, parameter)


all_indices = tuple(range(5))
full_choices = list(itertools.product(range(3), repeat=5))
for choice in full_choices:
    check_fixed('constant', R(1), all_indices, choice, None)
for i in range(5):
    indices = tuple(j for j in range(5) if j != i)
    for choice in itertools.product(range(3), repeat=4):
        check_fixed('quadratic_branch', Q[i], indices, choice, {'quadratic': i})
for c in K:
    if c == -1:
        continue
    for choice in full_choices:
        check_fixed('double_linear', (x-c)^2, all_indices, choice, str(c))
for i, j in itertools.combinations(range(5), 2):
    indices = tuple(k for k in range(5) if k not in (i, j))
    for choice in itertools.product(range(3), repeat=3):
        check_fixed('two_quadratic_branches', Q[i]*Q[j], indices,
                    choice, {'quadratics': [i, j]})
for i in range(5):
    indices = tuple(j for j in range(5) if j != i)
    choices = list(itertools.product(range(3), repeat=4))
    for c in K:
        if c == -1:
            continue
        B = Q[i]*(x-c)^2
        for choice in choices:
            check_fixed('quadratic_branch_double_linear', B, indices,
                        choice, {'quadratic': i, 'c': str(c)})

# This system includes A and the four free lower coefficients of a monic B.
# It is built directly from quotient-ring coefficients, with no CRT residues.
allowed_H_count = K.cardinality()^2-K.cardinality()-5
assert allowed_H_count == 58801
rank_hist = {str(i): 0 for i in range(11)}
consistent_rank_deficient = 0
fallback_H_checks = 0
for choice in full_choices:
    counts['square_quadratic_nonbranch']['logical_assignments'] += allowed_H_count
    moduli = [x+1] + Q
    targets = values(all_indices, choice)
    rows = []
    rhs = []
    for q, target in zip(moduli, targets):
        a_res = [(x^i) % q for i in range(6)]
        b_res = [(-target*x^i) % q for i in range(4)]
        rhs_res = (target*x^4) % q
        for d in range(q.degree()):
            rows.append([u[d] for u in a_res+b_res])
            rhs.append(rhs_res[d])
    mat = matrix(K, rows)
    b = vector(K, rhs)
    rank = mat.rank()
    rank_hist[str(rank)] += 1
    if mat.augment(matrix(K, 11, 1, list(b))).rank() != rank:
        continue
    if rank == 10:
        solution = mat.solve_right(b)
        u = solution[9]/K(2)
        v = (solution[8]-u^2)/K(2)
        if solution[7] != 2*u*v or solution[6] != v^2:
            continue
        H = x^2+u*x+v
        if gcd(H, f) != 1:
            continue
        B = H^2
        A = sum(solution[i]*x^i for i in range(6))
        counts['square_quadratic_nonbranch']['consistent_degree_at_most_five'] += 1
        if A.degree() != 5:
            continue
        counts['square_quadratic_nonbranch']['degree_five'] += 1
        assert all((A-B*target) % q == 0 for q, target in zip(moduli, targets))
        full_test('square_quadratic_nonbranch', A, B, choice,
                  {'H': str(H), 'rank': rank})
        continue
    consistent_rank_deficient += 1
    actual = 0
    for u in K:
        for v in K:
            H = x^2+u*x+v
            if gcd(H, f) != 1:
                continue
            actual += 1
            fallback_H_checks += 1
            check_fixed('square_quadratic_nonbranch', H^2, all_indices,
                        choice, {'H': str(H), 'rank': rank}, counted=False)
    assert actual == allowed_H_count

expected = {
    'constant': 243, 'quadratic_branch': 5*81,
    'double_linear': 242*243, 'two_quadratic_branches': 10*27,
    'quadratic_branch_double_linear': 5*242*81,
    'square_quadratic_nonbranch': 58801*243,
}
for shape, n in expected.items():
    assert counts[shape]['logical_assignments'] == n, (shape, counts[shape], n)
assert sum(expected.values()) == 14446377
assert sum(rank_hist.values()) == 243
print(json.dumps({
    'schema': 'ghs.odd5.degree5-linear-replay/v1',
    'source_sha256': hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
    'field': 'F3[a]/(a^5+2a+1)', 'trace': 26,
    'denominator_count': 60269,
    'logical_assignment_count': sum(expected.values()),
    'counts': counts, 'direct_matrix_rank_hist': rank_hist,
    'consistent_rank_deficient': consistent_rank_deficient,
    'fallback_H_checks': fallback_H_checks,
    'survivors': survivors, 'hits': hits,
}, sort_keys=True, default=int))
