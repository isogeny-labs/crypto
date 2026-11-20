#!/usr/bin/env python3
"""Finite-order certificate excluding every possible rational torsion prime."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent


def prime_divisors(n):
    found = []
    d = 2
    while d * d <= n:
        if n % d == 0:
            found.append(d)
            while n % d == 0:
                n //= d
        d += 1
    if n > 1:
        found.append(n)
    return found


def main():
    counts = json.loads((ROOT / "torsion-result.json").read_text())
    rows = []
    for item in counts["rows"]:
        first = item["good_prime_counts"][0]
        p0, n0 = first["p"], first["group_order"]
        candidates = sorted(set(prime_divisors(n0) + [p0]))
        exclusions = []
        for ell in candidates:
            witness = next((x for x in item["good_prime_counts"]
                            if x["p"] != ell and x["group_order"] % ell != 0), None)
            assert witness is not None
            exclusions.append({"torsion_prime": ell, "reduction_prime": witness["p"],
                               "reduced_group_order": witness["group_order"]})
        rows.append({"id": item["id"], "initial_good_prime": p0,
                     "initial_group_order": n0, "candidate_torsion_primes": candidates,
                     "exclusions": exclusions,
                     "conclusion": "rational_torsion_order_one_by_prime_to_p_reduction_injectivity"})
    print(json.dumps({"schema": "xedni-selected-torsion-certificate-v1",
                      "rows": rows}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
