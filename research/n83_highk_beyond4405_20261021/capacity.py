#!/usr/bin/env python3
"""Exact N83 symmetric S3 state and five-byte root-table capacity model."""

import json


N = 83
SLOTS_FIXED31 = 1 << 31
SLOT_BYTES = 5


def maximum_states(k: int) -> int:
    """The producer's symmetric-state upper bound for N83 and K orbit columns."""
    if k < 1:
        raise ValueError("K must be positive")
    return k * (k - 1) // 2 * N + k * (N // 2 + 1)


def current_slots(k: int) -> int:
    """Reproduce root11 load75 request and indexed table power-of-two sizing."""
    states = maximum_states(k)
    request = (4 * states + 2) // 3
    doubled = max(16, 2 * request)
    return 1 << (doubled - 1).bit_length()


def upper_load_with_fixed31(k: int) -> tuple[int, int]:
    """Worst-case two-root occupancy as an exact numerator and denominator."""
    return 2 * maximum_states(k), SLOTS_FIXED31


def largest_k_at_load(numerator: int, denominator: int) -> int:
    if not 0 < numerator < denominator:
        raise ValueError("load cap must lie strictly between zero and one")
    low, high = 1, 1
    while 2 * maximum_states(high) * denominator <= SLOTS_FIXED31 * numerator:
        high *= 2
    while low + 1 < high:
        middle = (low + high) // 2
        if 2 * maximum_states(middle) * denominator <= SLOTS_FIXED31 * numerator:
            low = middle
        else:
            high = middle
    return low


def report() -> dict:
    assert largest_k_at_load(3, 4) == 4405
    assert largest_k_at_load(4, 5) == 4549
    assert current_slots(4405) == SLOTS_FIXED31
    assert current_slots(4406) == 2 * SLOTS_FIXED31
    assert maximum_states(4549) < 1 << 30
    rows = []
    for k in (4300, 4405, 4406, 4500, 4549, 4550):
        numerator, denominator = upper_load_with_fixed31(k)
        slots = current_slots(k)
        rows.append({"k": k, "usable_factor_base_points": 166 * k,
                     "maximum_states": maximum_states(k),
                     "current_table_slots": slots,
                     "current_table_bytes": slots * SLOT_BYTES,
                     "fixed31_upper_load_numerator": numerator,
                     "fixed31_upper_load_denominator": denominator})
    return {"schema": "n83.highk-capacity-model/v1", "field_degree": N,
            "fixed31_slots": SLOTS_FIXED31, "slot_bytes": SLOT_BYTES,
            "largest_k_at_75_percent": 4405,
            "largest_k_at_80_percent": 4549, "rows": rows}


if __name__ == "__main__":
    print(json.dumps(report(), sort_keys=True, indent=2))
