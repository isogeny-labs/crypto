#!/usr/bin/env sage
"""Independent direct-linear-system replay of the degree-four branch screen."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

assert len(sys.argv)==3
bmask=int(sys.argv[1]);assert bmask in (11,202,38,80)
fmask=int(sys.argv[2]);assert fmask in (9735,17371,21624,44227,64725,90937,92153,100118,124586,144548,156795,159256)
wall0=time.perf_counter();cpu0=time.process_time()
F = GF(3)
Z = PolynomialRing(F, 'z')
z = Z.gen()
K = GF(3^5, name='a', modulus=z^5+2*z+1)
a = K.gen()
R = PolynomialRing(K, 'x')
x = R.gen()
f = x^11+sum(K((fmask//(3^i))%3)*x^i for i in range(11))
elliptic_b=sum(K((bmask//(3^i))%3)*a^i for i in range(5))
e=x^3+x^2+elliptic_b
factors = [p for p, m in f.factor() for _ in range(m)]
L1 = [p for p in factors if p.degree() == 1]
Q = [p for p in factors if p.degree() == 2]
assert len(L1)==1 and len(Q)==5
Lbranch=L1[0]
assert Lbranch*prod(Q)==f
assert EllipticCurve(K,[0,1,0,0,elliptic_b]).cardinality()==258

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
    B = Lbranch*Q[pole]
    indices = [j for j in range(5) if j != pole]
    for choices in itertools.product(range(3), repeat=4):
        check('linear_plus_quadratic', B, indices, choices,
              partial[pole], pole)

indices = list(range(5))
for c in K:
    if Lbranch(c)==0:
        continue
    B = Lbranch*(x-c)^2
    for choices in itertools.product(range(3), repeat=5):
        check('linear_plus_double_nonbranch', B, indices,
              choices, complete, str(c))

for kind, B in [('triple_linear',Lbranch^3),('linear_only',Lbranch)]:
    for choices in itertools.product(range(3), repeat=5):
        check(kind, B, indices, choices, complete, None)

assert sum(v['assignments'] for v in counts.values()) == 59697
out={'schema':'ghs.trace-minus14.degree4-direct-linear-replay/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'trace':-14,'bmask':bmask,'fmask':fmask,
     'counts':counts,'solutions':solutions,
     'cpu_seconds':time.process_time()-cpu0,
     'wall_seconds':time.perf_counter()-wall0}
path=Path(__file__).resolve().parent/('linear_%d_%d.json'%(bmask,fmask))
assert not path.exists(), 'refusing to overwrite receipt'
path.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'bmask':bmask,'fmask':fmask,
                  'degree_four_solutions':sum(v['degree_exactly_four'] for v in counts.values()),
                  'cpu_seconds':out['cpu_seconds']},sort_keys=True,default=int),flush=True)
