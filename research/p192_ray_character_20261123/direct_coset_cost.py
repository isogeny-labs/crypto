"""Arithmetic-only cost certificate for the first remaining P-192 y coset.

Run: python3 research/p192_ray_character_20261123/direct_coset_cost.py
This does not evaluate the point product or test its sign exactness.
"""

from decimal import Decimal, localcontext
from math import gcd


R = 6277101735386680763835789423176059013767194773182842284081
Q = 9564682313913860059195669
M = 8 * Q
N = (R - 1) // 2
T = N // M
SEARCH_LIMIT = 100_000
PI = Decimal("3.14159265358979323846264338327950288419716939937510")


def naf_cost(scalar):
    """Return doublings and additions in a binary nonadjacent form chain."""
    digits = []
    while scalar:
        digit = 2 - scalar % 4 if scalar & 1 else 0
        scalar = (scalar - digit) // 2
        digits.append(digit)
    return len(digits) - 1, sum(digit != 0 for digit in digits) - 1


assert N % M == 0 and T % 2 == 1
h = pow(3, T, R)
assert pow(h, M, R) == R - 1
assert all(pow(h, M // prime, R) not in (1, R - 1) for prime in (2, Q))

best = None
residue = 1
for exponent in range(1, SEARCH_LIMIT + 1):
    residue = residue * h % R
    if gcd(exponent, M) != 1:
        continue
    scalar = min(residue, R - residue)
    doubles, additions = naf_cost(scalar)
    candidate = (doubles + additions, exponent, scalar, doubles, additions)
    if best is None or candidate < best:
        best = candidate

assert best == (
    231,
    45273,
    1658197927360780859070010465224078895503129499163741689,
    180,
    51,
)
operations, exponent, scalar, doubles, additions = best
assert pow(scalar, M, R) == R - 1
assert all(pow(scalar, M // prime, R) not in (1, R - 1) for prime in (2, Q))

with localcontext() as context:
    context.prec = 55
    rho_reference = (PI * Decimal(R) / 4).sqrt()
    cold_point_operations = 2 * (M - 1) * operations
    ratio = Decimal(cold_point_operations) / rho_reference
    per_factor_allowance = rho_reference / Decimal(2 * M)

print(f"m={M} index={T} search_limit={SEARCH_LIMIT}")
print(f"generator_exponent={exponent} scalar={scalar}")
print(f"doubles={doubles} additions={additions} operations={operations}")
print(f"cold_point_operations={cold_point_operations}")
print(f"rho_ratio={ratio:.12f} full_per_factor_allowance={per_factor_allowance:.12f}")
