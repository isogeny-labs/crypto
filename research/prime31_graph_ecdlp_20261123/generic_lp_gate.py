"""Optimistic occupancy screen for a one-LP ordinary-EC relation source."""
import json
from math import isqrt, pi, sqrt

N = 2_147_487_637

def minimum_draws(base: int, pool: int, needed_rows: int) -> int:
    # Distinct draws t,u and distinct indices i,j give a collision in the
    # pool with probability S/n². Same-draw residuals are distinct.
    # Choose(t,2) B² S/n² is the exact expected pair count.
    threshold = 2 * needed_rows * N * N
    scale = base * base * pool
    t = (1 + isqrt(1 + 4 * (threshold // scale))) // 2
    while t * (t - 1) * scale < threshold:
        t += 1
    while t > 1 and (t - 1) * (t - 2) * scale >= threshold:
        t -= 1
    return t

def panel():
    rho = sqrt(pi * N / 2)
    for base in (1, 16, 64, 256, 1024):
        for divisor in (1, 16, 256):
            pool = N // divisor
            draws = minimum_draws(base, pool, base + 1)
            # Each draw tests all B factors. Public R generation is extra.
            additions = draws * base
            yield {
                "factor_base": base, "lp_pool": pool,
                "pool_fraction_denominator": divisor,
                "independent_rows_optimistically_needed": base + 1,
                "random_R_draws_for_expected_collision_count": draws,
                "source_additions_optimistic": additions,
                "rho_expected_collision_steps": round(rho),
                "source_additions_over_rho_steps": round(additions / rho, 3),
            }

if __name__ == "__main__":
    print(json.dumps({
        "schema": "prime31-generic-one-lp-screen-v1", "p": 2_147_483_647,
        "curve_order": N, "curve_a": 2_147_483_644,
        "curve_b": 21, "model": "iid_uniform_pool_optimistic_all_collisions_independent",
        "rows": list(panel()),
    }, indent=2))
