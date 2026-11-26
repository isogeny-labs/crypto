"""Exhaust the order-97 short-Weierstrass models over F_107.

Run: python3 research/p192_ray_character_20261123/maximal_cm_class_scan.py
The GP replay checks one representative of each j class independently.
"""

from collections import defaultdict

P = 107
R = 97
HALF = (R - 1) // 2
M = 16
H = pow(5, HALF // M, R)
FIELD_SIGN = [0 if a == 0 else (1 if pow(a, (P - 1) // 2, P) == 1 else -1)
              for a in range(P)]
SCALAR_SIGN = [0 if a == 0 else (1 if pow(a, (R - 1) // 2, R) == 1 else -1)
               for a in range(R)]
Y_ROOT = {y * y % P: y for y in range(P)}

indices = []
scalar = 1
for _ in range(M):
    indices.append(scalar if scalar % 2 else R - scalar)
    scalar = scalar * H % R
assert len({min(j, R - j) for j in indices}) == M


def add(left, right, a):
    if left is None:
        return right
    if right is None:
        return left
    x, y = left
    u, v = right
    if x == u and (y + v) % P == 0:
        return None
    slope = ((3 * x * x + a) * pow(2 * y, -1, P) if left == right
             else (v - y) * pow(u - x, -1, P)) % P
    x3 = (slope * slope - x - u) % P
    return x3, (slope * (x - x3) - y) % P


rows = defaultdict(lambda: {"models": 0, "mismatches": set()})
for a in range(P):
    for b in range(P):
        denominator = (4 * a**3 + 27 * b * b) % P
        if denominator == 0:
            continue
        values = [(x**3 + a * x + b) % P for x in range(P)]
        if P + 1 + sum(FIELD_SIGN[value] for value in values) != R:
            continue
        generator = next((x, Y_ROOT[value]) for x, value in enumerate(values)
                         if value and value in Y_ROOT)
        multiples = [None] * R
        current = None
        for d in range(1, R):
            current = add(current, generator, a)
            multiples[d] = current
        assert add(current, generator, a) is None

        def coset_sign(d):
            product = 1
            for j in indices:
                product = product * multiples[j * d % R][1] % P
            return FIELD_SIGN[product]

        base = coset_sign(1)
        mismatches = sum(coset_sign(d) * base != SCALAR_SIGN[d]
                         for d in range(1, HALF + 1))
        j_invariant = 1728 * 4 * a**3 * pow(denominator, -1, P) % P
        rows[j_invariant]["models"] += 1
        rows[j_invariant]["mismatches"].add(mismatches)

expected = {9: (53, {16, 32}), 51: (53, {16}), 99: (53, {0})}
assert {j: (entry["models"], entry["mismatches"])
        for j, entry in rows.items()} == expected
for j in sorted(rows):
    entry = rows[j]
    print(f"j={j} models={entry['models']} mismatches={sorted(entry['mismatches'])}")
print(f"total_models={sum(entry['models'] for entry in rows.values())}")
