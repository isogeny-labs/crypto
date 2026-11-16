#!/usr/bin/env sage
"""Complete K-defined degree-six cover screen under DEGREE6_PROTOCOL.md."""
import hashlib
import itertools
import json
import sys
from pathlib import Path

F3 = GF(3)
Z = PolynomialRing(F3, 'z')
z = Z.gen()
K = GF(3^5, name='a', modulus=z^5+2*z+1)
a = K.gen()
assert len(sys.argv) == 2
bmask = int(sys.argv[1])
assert bmask == 11
b = sum(K((bmask // (3^i)) % 3)*a^i for i in range(5))
R = PolynomialRing(K, 'x')
x = R.gen()
L = x
f = x^11+x^8+x^7+x^6+x^5+x^2+2*x
e = x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K, [0, 1, 0, 0, b]).cardinality() == 258
fac = [p for p, m in f.factor() for _ in range(m)]
Q = [p for p in fac if p.degree() == 2]
assert [p for p in fac if p.degree() == 1] == [L] and len(Q) == 5
Qprod = prod(Q)
assert L*Qprod == f

root_reps = []
for i, q in enumerate(Q):
    ext = K.extension(q, names=('w%d' % i,))
    vals = [v for v, m in e.change_ring(ext).roots() for _ in range(m)]
    assert len(vals) == 3 and len(set(vals)) == 3
    vals.sort(key=str)
    root_reps.append([R(v.lift().list()) for v in vals])

shapes = ('L', 'L3', 'LQi', 'Ldouble', 'L5', 'L3Qi', 'L3double',
          'LQiQj', 'LQidouble', 'LHsquare')
counts = {s: {'branch_assignments': 0, 'numerator_degree_six': 0,
              'coprime': 0, 'f_divides': 0, 'full_equation_hits': 0}
          for s in shapes}
hits = []


def choices_for(indices):
    moduli = [Q[j] for j in indices]
    M = prod(moduli)
    for choices in itertools.product(range(3), repeat=len(indices)):
        targets = [root_reps[j][k] for j, k in zip(indices, choices)]
        yield choices, CRT_list(targets, moduli), M


def check_A(kind, A, B, choices, parameter):
    if A.degree() != 6:
        return
    counts[kind]['numerator_degree_six'] += 1
    if gcd(A, B) != 1:
        return
    counts[kind]['coprime'] += 1
    N = B*(A^3+A^2*B+b*B^3)
    C2, rem = N.quo_rem(f)
    if rem != 0:
        return
    counts[kind]['f_divides'] += 1
    if not C2.is_square():
        return
    C = C2.sqrt()
    assert f*C^2 == N and gcd(A, B) == 1
    counts[kind]['full_equation_hits'] += 1
    hits.append({'shape': kind, 'A': str(A), 'B': str(B), 'C': str(C),
                 'choices': list(choices), 'parameter': parameter})


def check_fixed(kind, B, choices, h, M, parameter):
    counts[kind]['branch_assignments'] += 1
    residue = (B*h) % M
    if M.degree() == 6:
        assert kind == 'LQiQj'
        for scalar in K:
            if scalar != 0:
                check_A(kind, residue+scalar*M, B, choices,
                        {'poles': parameter, 'lambda': str(scalar)})
    else:
        assert M.degree() >= 8
        check_A(kind, residue, B, choices, parameter)


full = list(choices_for(tuple(range(5))))
assert len(full) == 243 and all(M == Qprod for _, _, M in full)
for choices, h, M in full:
    check_fixed('L', L, choices, h, M, None)
    check_fixed('L3', L^3, choices, h, M, None)
    check_fixed('L5', L^5, choices, h, M, None)

for i in range(5):
    indices = tuple(j for j in range(5) if j != i)
    for choices, h, M in choices_for(indices):
        check_fixed('LQi', L*Q[i], choices, h, M, {'quadratic': i})
        check_fixed('L3Qi', L^3*Q[i], choices, h, M, {'quadratic': i})

for c in K:
    if c == 0:
        continue
    d = (x-c)^2
    for choices, h, M in full:
        check_fixed('Ldouble', L*d, choices, h, M, {'c': str(c)})
        check_fixed('L3double', L^3*d, choices, h, M, {'c': str(c)})

for i, j in itertools.combinations(range(5), 2):
    indices = tuple(k for k in range(5) if k not in (i, j))
    for choices, h, M in choices_for(indices):
        check_fixed('LQiQj', L*Q[i]*Q[j], choices, h, M,
                    {'quadratics': [i, j]})

for i in range(5):
    indices = tuple(j for j in range(5) if j != i)
    assignments = list(choices_for(indices))
    assert len(assignments) == 81
    for c in K:
        if c == 0:
            continue
        B = L*Q[i]*(x-c)^2
        for choices, h, M in assignments:
            check_fixed('LQidouble', B, choices, h, M,
                        {'quadratic': i, 'c': str(c)})

# The dominant shape B=L*(x²+u*x+v)² is solved by exact resultants.
U = PolynomialRing(K, 'u')
u = U.gen()
V = PolynomialRing(U, 'v')
v = V.gen()
S = PolynomialRing(K, 'v')
allowed_H_count = K.cardinality()^2-K.cardinality()-5
assert allowed_H_count == 58801
resultant_choice_count = 0
fallback_choice_count = 0
fallback_u_count = 0
high_system_solutions = 0
for choices, h, M in full:
    counts['LHsquare']['branch_assignments'] += allowed_H_count
    rems = [((L*x^k*h) % Qprod) for k in range(5)]
    equations = []
    for d in (7, 8, 9):
        equations.append(V(rems[4][d]) + V(2*rems[3][d])*u +
                         V(rems[2][d])*(u^2+2*v) +
                         V(2*rems[1][d])*u*v + V(rems[0][d])*v^2)
    resultants = [equations[i].resultant(equations[j])
                  for i, j in ((0, 1), (0, 2), (1, 2))]
    first = next((r for r in resultants if r != 0), None)
    if first is None:
        fallback_choice_count += 1
        us = list(K)
    else:
        resultant_choice_count += 1
        us = U(first).roots(multiplicities=False)
    for u0 in us:
        if first is None:
            fallback_u_count += 1
        specialized = [S([K(coeff(u0)) for coeff in F.list()])
                       for F in equations]
        common = gcd(specialized[0], gcd(specialized[1], specialized[2]))
        vs = list(K) if common == 0 else common.roots(multiplicities=False)
        for v0 in vs:
            if not all(F(v0) == 0 for F in specialized):
                continue
            high_system_solutions += 1
            H = x^2+u0*x+v0
            if gcd(H, f) != 1:
                continue
            B = L*H^2
            A = (B*h) % Qprod
            check_A('LHsquare', A, B, choices,
                    {'H': str(H), 'resultant_fallback': first is None})

expected = {'L': 243, 'L3': 243, 'LQi': 405, 'Ldouble': 58806,
            'L5': 243, 'L3Qi': 405, 'L3double': 58806,
            'LQiQj': 270, 'LQidouble': 98010, 'LHsquare': 14288643}
for shape, n in expected.items():
    assert counts[shape]['branch_assignments'] == n, (shape, counts[shape], n)
assert sum(expected.values()) == 14506074
print(json.dumps({
    'schema': 'ghs.odd5.degree6-screen/v1',
    'source_sha256': hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
    'field': 'F3[a]/(a^5+2a+1)', 'a6_mask': bmask, 'a6': str(b),
    'target_order': 258, 'trace': -14,
    'denominator_count': 60518, 'branch_assignment_count': sum(expected.values()),
    'LQiQj_lambda_values_per_assignment': 242,
    'resultant_choice_count': resultant_choice_count,
    'fallback_choice_count': fallback_choice_count,
    'fallback_u_count': fallback_u_count,
    'high_system_solutions': high_system_solutions,
    'counts': counts, 'hits': hits,
}, sort_keys=True, default=int))
