"""Apply the explicit Vyugin--Shkredov shift-intersection theorem to P-192.

Run: python3 research/p192_ray_character_20261123/grid_intersection_gate.py

Theorem 1.1: https://www.mathnet.ru/links/38bb4daba4e7ddeb5ffe4d195be3adcf/sm7857_eng.pdf
Here Q=R in the paper's notation. If A+B lies in the lifted coset,
fix k+1 elements of A; B lies in k distinct shifts of that subgroup.
The script checks all three displayed theorem assumptions for each k.
"""

from decimal import Decimal, localcontext


SCALAR_PRIME = 6277101735386680763835789423176059013767194773182842284081
ODD_FACTOR = 9564682313913860059195669
COSET_CLASSES = 8 * ODD_FACTOR
LIFTED_SUBGROUP = 2 * COSET_CLASSES
GRID_SIDE = 6185364116659


with localcontext() as ctx:
    ctx.prec = 100
    subgroup = Decimal(LIFTED_SUBGROUP)
    prime = Decimal(SCALAR_PRIME)
    two = Decimal(2)
    valid = []
    for k in range(1, 100):
        first = LIFTED_SUBGROUP > k * 2 ** (2 * k + 4)
        q_root = (subgroup / k) ** (Decimal(1) / (2 * k))
        second = subgroup < (q_root - 1) ** (2 * k + 1)
        subgroup_root = subgroup ** (Decimal(1) / (2 * k + 1))
        third = prime >= 4 * k * subgroup * (subgroup_root + 1)
        if first and second and third:
            upper = 4 * (k + 1) * (subgroup_root + 1) ** (k + 1)
            valid.append((upper, k))

    assert [k for _, k in valid] == list(range(1, 10))
    upper, k = min(valid)
    lifted_grid_side = 2 * GRID_SIDE
    assert k == 8 and upper > lifted_grid_side
    log_upper = upper.ln() / two.ln()
    log_grid_side = Decimal(lifted_grid_side).ln() / two.ln()
    assert Decimal("51.589") < log_upper < Decimal("51.590")
    assert Decimal("43.491") < log_grid_side < Decimal("43.493")

    print(f"scalar_prime={SCALAR_PRIME}")
    print(f"lifted_subgroup_order={LIFTED_SUBGROUP}")
    print(f"eligible_k={[k for _, k in valid]}")
    print(f"best_k={k} theorem_side_upper_log2={log_upper:.9f}")
    print(f"proposed_lifted_grid_side={lifted_grid_side} log2={log_grid_side:.9f}")
    print(f"bound_over_proposed_side_log2={log_upper-log_grid_side:.9f}")
