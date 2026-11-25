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
SEARCH_LIMIT = 1_000_000
PI = Decimal("3.14159265358979323846264338327950288419716939937510")


def wnaf_cost(scalar, width):
    """Count a full Jacobian width-w NAF step, including its rebuilt table."""
    assert 2 <= width <= 7
    modulus = 1 << width
    half = modulus >> 1
    digits = weight = 0
    while scalar:
        digit = scalar & (modulus - 1) if scalar & 1 else 0
        if digit >= half:
            digit -= modulus
        scalar = (scalar - digit) >> 1
        digits += 1
        weight += digit != 0
    table_doubles = 1 if width > 2 else 0
    table_additions = (1 << (width - 2)) - 1 if width > 2 else 0
    return digits - 1 + table_doubles, weight - 1 + table_additions


assert N % M == 0 and T % 2 == 1
h = pow(3, T, R)
assert pow(h, M, R) == R - 1
assert all(pow(h, M // prime, R) not in (1, R - 1) for prime in (2, Q))

best_by_width = {width: None for width in range(2, 8)}
best_field_formula = None
residue = 1
for exponent in range(1, SEARCH_LIMIT + 1):
    residue = residue * h % R
    if gcd(exponent, M) != 1:
        continue
    scalar = min(residue, R - residue)
    for width in best_by_width:
        doubles, additions = wnaf_cost(scalar, width)
        multiplications = 3 * doubles + 11 * additions + 2
        squarings = 5 * (doubles + additions)
        candidate = (
            doubles + additions,
            exponent,
            scalar,
            doubles,
            additions,
            multiplications,
            squarings,
        )
        if best_by_width[width] is None or candidate < best_by_width[width]:
            best_by_width[width] = candidate
        field_candidate = (
            multiplications + squarings,
            exponent,
            width,
            scalar,
            doubles,
            additions,
            multiplications,
            squarings,
        )
        if best_field_formula is None or field_candidate < best_field_formula:
            best_field_formula = field_candidate

assert best_by_width[2][:5] == (
    228, 391601, 125650396604550144886539610233302508351526245526126689, 176, 52
)
assert [best_by_width[width][0] for width in best_by_width] == [228, 213, 208, 203, 208, 221]
best_width, best = min(best_by_width.items(), key=lambda item: (item[1], item[0]))
assert best_width == 5
assert best[:5] == (
    203, 474203, 6716193708113405591269066791027723138604200469306871, 170, 33
)
assert best_field_formula == (
    1890, 474203, 5, 6716193708113405591269066791027723138604200469306871,
    170, 33, 875, 1015
)
operations, exponent, scalar, doubles, additions, field_m, field_s = best
assert pow(scalar, M, R) == R - 1
assert all(pow(scalar, M // prime, R) not in (1, R - 1) for prime in (2, Q))

with localcontext() as context:
    context.prec = 55
    rho_reference = (PI * Decimal(R) / 4).sqrt()
    cold_point_operations = 2 * (M - 1) * operations
    ratio = Decimal(cold_point_operations) / rho_reference
    per_factor_allowance = rho_reference / Decimal(2 * M)

print(f"m={M} index={T} search_limit={SEARCH_LIMIT}")
print(f"generator_exponent={exponent} scalar={scalar} width={best_width}")
print(f"doubles={doubles} additions={additions} operations={operations} field={field_m}M+{field_s}S")
print(f"best_operations_by_width={[(w, row[0]) for w, row in best_by_width.items()]}")
print(f"cold_point_operations={cold_point_operations}")
print(f"rho_ratio={ratio:.12f} full_per_factor_allowance={per_factor_allowance:.12f}")
print(f"equal_weight_field_formula={best_field_formula[0]}M/S units")
