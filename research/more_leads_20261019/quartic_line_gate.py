#!/usr/bin/env python3
"""Exact affine line-incidence gate on the two published Tian quartics.

This is a relation *geometry* census.  It neither builds Jacobian classes nor
transfers an elliptic target.  The pair and point-slope enumerators are checked
against each other; every four-point line is separately checked by polynomial
specialization and exhaustive root enumeration.
"""

from collections import Counter, defaultdict
from hashlib import sha256
from itertools import combinations
import json
from math import comb


POWERS = [(4, 0, 0), (3, 1, 0), (2, 2, 0), (1, 3, 0), (0, 4, 0),
          (3, 0, 1), (2, 1, 1), (1, 2, 1), (0, 3, 1),
          (2, 0, 2), (1, 1, 2), (0, 2, 2), (1, 0, 3), (0, 1, 3), (0, 0, 4)]
FIXTURES = [
    ("Tian-2012.07173-F523", 523,
     [1, 144, 112, 393, 488, 397, 49, 299, 500, 311, 185, 288, 462, 468, 284]),
    ("Tian-2007.03172-F257", 257,
     [1, 89, 244, 126, 113, 131, 3, 255, 65, 172, 139, 21, 201, 228, 70]),
]


def affine_points(p, coeffs):
    terms = [(a, b, c) for (a, b, _), c in zip(POWERS, coeffs)]
    points = []
    for x in range(p):
        xp = [1, x, x*x % p, pow(x, 3, p), pow(x, 4, p)]
        for y in range(p):
            yp = [1, y, y*y % p, pow(y, 3, p), pow(y, 4, p)]
            if sum(c * xp[a] * yp[b] for a, b, c in terms) % p == 0:
                points.append((x, y))
    return points


def specialized_coeffs(p, coeffs, slope, intercept):
    """Coefficients of F(x, slope*x+intercept, 1), low degree first."""
    out = [0] * 5
    for (ax, ay, _), c in zip(POWERS, coeffs):
        for k in range(ay + 1):
            out[ax + k] += c * comb(ay, k) * pow(slope, k, p) * pow(intercept, ay-k, p)
    return [c % p for c in out]


def census(name, p, coeffs):
    points = affine_points(p, coeffs)
    assert len(set(points)) == len(points)
    # Enumerator A: one incidence for each (rational point, slope).
    incidence = defaultdict(list)
    for x, y in points:
        for m in range(p):
            incidence[(m, (y - m*x) % p)].append((x, y))
    # Enumerator B: pair all points with distinct abscissae.  A line with k
    # affine rational points must be found exactly binom(k,2) times.
    pairs = Counter()
    distinct_x_pairs = 0
    for (x1, y1), (x2, y2) in combinations(points, 2):
        if x1 == x2:
            continue
        distinct_x_pairs += 1
        m = (y2 - y1) * pow((x2-x1) % p, -1, p) % p
        pairs[(m, (y1 - m*x1) % p)] += 1
    histogram = Counter(len(v) for v in incidence.values())
    histogram[0] = p*p - len(incidence)
    assert max(histogram) <= 4, (name, histogram)
    assert sum(histogram.values()) == p*p
    assert len(points)*p == sum(k*n for k, n in histogram.items())
    assert distinct_x_pairs == sum(pairs.values())
    assert set(pairs) == {line for line, ps in incidence.items() if len(ps) >= 2}
    assert all(pairs[line] == comb(len(ps), 2)
               for line, ps in incidence.items() if len(ps) >= 2)
    complete = []
    for (m, b), ps in sorted(incidence.items()):
        if len(ps) != 4:
            continue
        poly = specialized_coeffs(p, coeffs, m, b)
        roots = tuple(x for x in range(p)
                      if (((((poly[4]*x + poly[3])*x + poly[2])*x
                            + poly[1])*x + poly[0]) % p) == 0)
        assert roots == tuple(sorted(x for x, _ in ps)), (name, m, b, poly, roots, ps)
        assert poly[4] != 0  # No missing fifth/projective intersection.
        complete.append([m, b, [[x, y] for x, y in sorted(ps)]])
    encoded = json.dumps(complete, separators=(',', ':')).encode()
    ranked_points = sorted(points, key=lambda z: sha256(
        f"quartic-small-base-v1:{name}:{z[0]}:{z[1]}".encode()).digest())
    small_base_gate = {}
    for fraction in ("1/8", "1/4", "1/2"):
        numerator, denominator = map(int, fraction.split('/'))
        selected = set(ranked_points[:len(points)*numerator//denominator])
        small_counts = Counter(sum(tuple(point) in selected for point in line[2])
                               for line in complete)
        small_base_gate[fraction] = {
            "small_points": len(selected),
            "four_point_lines_by_small_point_count": {
                str(k): small_counts.get(k, 0) for k in range(5)},
            "lines_with_at_most_two_large_points": sum(
                v for k, v in small_counts.items() if k >= 2),
        }
    return {
        "fixture": name, "p": p, "affine_rational_points": len(points),
        "possible_nonvertical_affine_lines": p*p,
        "lines_with_k_points": {str(k): histogram.get(k, 0) for k in range(5)},
        "distinct_x_point_pairs": distinct_x_pairs,
        "complete_four_point_lines": len(complete),
        "complete_lines_sha256": sha256(encoded).hexdigest(),
        "fixed_small_base_gate": small_base_gate,
        "first_three_complete_lines": complete[:3],
        "checks": {"pair_counts_equal_binomial_incidence": True,
                   "all_four_point_specializations_reenumerated": True,
                   "all_complete_lines_have_degree_four": True},
    }


if __name__ == "__main__":
    for fixture in FIXTURES:
        print(json.dumps(census(*fixture), sort_keys=True))
