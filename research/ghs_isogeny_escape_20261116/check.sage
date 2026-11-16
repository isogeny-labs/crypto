#!/usr/bin/env sage
"""Independent arithmetic and modular-polynomial spot checks for the escape bound."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
q = 243
rows = []
for t in [-26, 26]:
    u = [0, 1]
    for n in range(2, 8):
        u.append(t * u[-1] - q * u[-2])
    assert abs(u[7]) == 21156227
    assert is_prime(abs(u[7]))
    rows.append({'trace': t, 'lucas_u': [int(v) for v in u], 'u7_factorization': str(factor(u[7]))})

F = GF(3^5, 'a', modulus=x^5 + 2*x + 1)
a = F.gen()
R = PolynomialRing(F, 'Y')
Y = R.gen()
E = EllipticCurve(F, [0, 1, 0, 0, a^4 + 2*a + 1])
assert E.cardinality() == 270 and 243 + 1 - E.cardinality() == -26
j = E.j_invariant()
spot = {}
for ell in [7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61]:
    h = R(classical_modular_polynomial(ell)(j, Y))
    degrees = sorted([int(f.degree()) for f, e in h.factor() for _ in range(e)])
    assert 7 not in degrees
    spot[str(ell)] = degrees

out = {
    'schema': 'ghs.isogeny-escape/v1',
    'source_sha256': hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
    'base_q': int(q),
    'extension_degree': 7,
    'absolute_u7': int(21156227),
    'u7_is_prime': True,
    'rows': rows,
    'source_j': str(j),
    'modular_polynomial_spot_degrees': spot,
}
(HERE / 'check.json').write_text(json.dumps(out, indent=2, sort_keys=True, default=int) + '\n')
print(json.dumps({'status':'PASS','u7':out['absolute_u7'],'spot_primes':len(spot)},sort_keys=True,default=int))
