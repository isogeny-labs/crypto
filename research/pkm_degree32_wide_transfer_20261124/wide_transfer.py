#!/usr/bin/env python3
"""Exact PKM m=2 degree/valuation transfer calculations."""
import argparse
import hashlib
import json
import math
from pathlib import Path

CURVES = [
    ("NIST P-192", int("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFFFFFFFFFF", 16)),
    ("NIST P-224", int("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000001", 16)),
    ("NIST P-256", int("FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF", 16)),
    ("NIST P-384", int("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFF0000000000000000FFFFFFFF", 16)),
    ("NIST P-521", int("1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", 16)),
    ("s59 p-1 depth 16", 288230376154267649),
    ("s59 p+1 depth 18", 288230376151973887),
    ("s59 registry p-1 depth 7", 288230445959117953),
]
GRID_BITS = [10, 13, 14, 16, 20, 24, 32, 40, 48, 59, 192]


def v2(n):
    if n <= 0:
        raise ValueError("v2 requires a positive integer")
    return (n & -n).bit_length() - 1


def ceil_sqrt(n):
    r = math.isqrt(n)
    return r if r * r == n else r + 1


def ceil_log2(n):
    return 0 if n <= 1 else (n - 1).bit_length()


def profile(name, p):
    d_req = ceil_sqrt(2 * p)
    t_req = ceil_log2(d_req)
    vm, vp = v2(p - 1), v2(p + 1)
    best_pm_t = min(max(vm, vp), t_req)
    best_pm_log2_lambda = 2.0 * best_pm_t - 1.0 - math.log2(p)
    log2_lambda32 = 9.0 - math.log2(p)  # 32^2/(2p) = 2^9/p
    return {
        "name": name,
        "p": str(p),
        "field_bits": p.bit_length(),
        "v2_p_minus_1": vm,
        "v2_p_plus_1": vp,
        "m": 2,
        "required_degree_ceil_sqrt_2p": str(d_req),
        "required_power2_height": t_req,
        "required_power2_degree": str(1 << t_req),
        "kummer_p_minus_1": {"available": vm >= t_req, "missing_height_bits": max(0, t_req - vm), "max_height": vm},
        "dickson_p_plus_1": {"available": vp >= t_req, "missing_height_bits": max(0, t_req - vp), "max_height": vp},
        "best_p_plusminus_tower": {
            "height": best_pm_t,
            "side": "p-1" if vm >= vp else "p+1",
            "log2_expected_decompositions_per_target": best_pm_log2_lambda,
            "log2_reciprocal_expected_trials": -best_pm_log2_lambda,
        },
        "aux_isogeny": {
            "mathematically_requires_p_plusminus_condition": False,
            "required_low_degree_steps_per_coordinate": t_req,
            "two_coordinate_low_degree_steps": 2 * t_req,
            "current_u64_t20_implementation_supports": p < 2**64 and t_req <= 20,
            "current_implementation_height_cap": 20,
        },
        "degree32": {
            "height": 5,
            "log2_expected_decompositions_per_target": log2_lambda32,
            "log2_reciprocal_expected_trials": -log2_lambda32,
            "height_shortfall_to_required": max(0, t_req - 5),
        },
    }


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, required=True)
    args = ap.parse_args()
    rows = [profile(name, p) for name, p in CURVES]
    expected = {
        "NIST P-192": (97, 1, 64), "NIST P-224": (113, 96, 1),
        "NIST P-256": (129, 1, 96), "NIST P-384": (193, 1, 32),
        "NIST P-521": (261, 1, 521), "s59 p-1 depth 16": (30, 16, 1),
        "s59 p+1 depth 18": (30, 1, 18), "s59 registry p-1 depth 7": (30, 7, 1),
    }
    assert {r["name"]: (r["required_power2_height"], r["v2_p_minus_1"], r["v2_p_plus_1"])
            for r in rows} == expected
    grid = []
    for bits in GRID_BITS:
        # Use p≈2^bits solely for a bit-scale density boundary.
        log2_lambda = 9 - bits
        grid.append({"field_bits": bits, "degree": 32,
                     "derived_log2_lambda_at_p_approx_2powbits": log2_lambda,
                     "derived_log2_reciprocal_trials": -log2_lambda})
    source = Path(__file__).read_bytes()
    result = {
        "schema": "pkm.degree32-wide-transfer/v1",
        "protocol": "PROTOCOL.md",
        "script_sha256": hashlib.sha256(source).hexdigest(),
        "formula": {"m": 2, "lambda": "d^2/(2p)", "target": "d^2 >= 2p"},
        "implementation": {"prime_isogeny_field": "u64", "prime_isogeny_t_min": 1,
                           "prime_isogeny_t_max": 20, "enumerates_fiber_size": "2^t"},
        "curves": rows,
        "degree32_bit_grid": grid,
    }
    args.out.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"out": str(args.out), "curves": len(rows), "script_sha256": result["script_sha256"]}, sort_keys=True))


if __name__ == "__main__":
    main()
