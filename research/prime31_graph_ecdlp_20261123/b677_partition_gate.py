"""Check the source-bound B677 support and its graph partition witness counts.

The counts are exact uniform-query expectations for signed, distinct-x
four-point witness tuples. They do not estimate extraction completion or rank.
"""

import json
from math import comb
from pathlib import Path


ROOT = Path(__file__).parent
SOURCE = ROOT / "evidence" / "rr5-b677-prerequisite-01" / "structured-positive.stdout"


def add(curve, left, right):
    p = curve["p"]
    if left is None:
        return right
    if right is None:
        return left
    x, y = left
    z, w = right
    if x == z and (y + w) % p == 0:
        return None
    slope = (((3 * x * x + curve["a"]) * pow(2 * y, -1, p))
             if x == z else ((w - y) * pow(z - x, -1, p))) % p
    xx = (slope * slope - x - z) % p
    return xx, (slope * (x - xx) - y) % p


def dickson1024(x, p):
    for _ in range(10):
        x = (x * x - 2) % p
    return x


def main():
    records = {}
    for line in SOURCE.read_text().splitlines():
        label, value = line.split(" ", 1)
        records[label] = json.loads(value)
    source = records["B677_INPUT"]
    result = records["B677_RESULT"]
    curve = source["curve"]
    p, n = curve["p"], curve["n"]
    assert p == 2**31 - 1 and n == 2_147_446_831
    assert (curve["a"], curve["b"], tuple(curve["g"])) == (
        847_514_846, 2_087_444_724, (625_037_715, 2_100_627_251))
    points = [tuple(point) for point in source["public_base"]]
    assert len(points) == 677 and len({x for x, _ in points}) == 677
    assert all((y * y - x * x * x - curve["a"] * x - curve["b"]) % p == 0
               for x, y in points)
    terminal = source["dickson_terminal"]
    base = {i for i, (x, _) in enumerate(points) if dickson1024(x, p) == terminal}
    lp = set(range(len(points))) - base
    assert len(base) == 509 and len(lp) == 168
    assert source["fixture_certificate"]["dickson_curve_points"] == len(base)
    assert source["fixture_certificate"]["extra_points"] == len(lp)

    witness = source["control_expected_row"]["terms"]
    assert sum(i in base for i, _ in witness) == 2
    assert sum(i in lp for i, _ in witness) == 2
    total = None
    for i, sign in witness:
        x, y = points[i]
        total = add(curve, total, (x, y if sign == 1 else (-y) % p))
    assert total == tuple(source["public_target"])

    families = {}
    for base_terms in range(5):
        lp_terms = 4 - base_terms
        numerator = 16 * comb(len(base), base_terms) * comb(len(lp), lp_terms)
        families[str(base_terms)] = {
            "base_terms": base_terms,
            "lp_terms": lp_terms,
            "expected_signed_witnesses_per_uniform_query": numerator / n,
            "expectation_numerator": numerator,
            "expectation_denominator": n,
        }
    assert sum(row["expectation_numerator"] for row in families.values()) == 16 * comb(677, 4)
    candidate = result["candidate"]
    assert candidate["status"] == "ResourceLimit"
    assert "matrix_entries admission" in candidate["reason"]
    assert not candidate["relations"]
    larger_attempts = []
    for cap in (8, 32):
        larger = json.loads((ROOT / "evidence" / "rr5-b677-prerequisite-01"
                             / f"structured-cap{cap}m.json").read_text())
        assert larger["curve"] == curve
        assert larger["fixture_certificate"] == source["fixture_certificate"]
        assert larger["query"] == source["public_target"]
        assert larger["control_planted_terms"] == witness
        assert larger["status"] == "ResourceLimit"
        assert "matrix_entries admission" in larger["reason"]
        assert not larger["relations"]
        assert larger["traversal"]["complete"] is False
        assert larger["traversal"]["solutions_visited"] == 0
        assert larger["traversal"]["resource_limit"]["limit"] == cap * 1_000_000
        larger_attempts.append({"matrix_entry_cap": cap * 1_000_000,
                                "status": larger["status"],
                                "reason": larger["reason"],
                                "accepted_rows": len(larger["relations"])})
    print(json.dumps({
        "scope": "second ordinary curve over the same p; exact witness counts, not extraction yield",
        "curve": curve,
        "dickson_factor_base_size": len(base),
        "y7_large_prime_pool_size": len(lp),
        "planted_two_lp_control_replay": True,
        "families": families,
        "positive_gate_status": candidate["status"],
        "positive_gate_reason": candidate["reason"],
        "positive_gate_accepted_rows": len(candidate["relations"]),
        "larger_cap_attempts": larger_attempts,
    }, indent=2))


if __name__ == "__main__":
    main()
