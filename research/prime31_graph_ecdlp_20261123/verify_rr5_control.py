"""Independent integer group-law replay for fixed RR5 controls.

The exhaustive witness oracle is used only after extraction on these small
correctness panels. It is not an input to the native producer.
"""

import itertools
import json
from pathlib import Path


ROOT = Path(__file__).parent
EVIDENCE = ROOT / "evidence" / "rr5-control-01"
P = 2_147_483_647
A = P - 3
B = 21
N = 2_147_487_637
G = (1_391_443_431, 177_979_044)


def add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x, y = left
    z, w = right
    if x == z and (y + w) % P == 0:
        return None
    if x == z:
        slope = ((3 * x * x + A) * pow(2 * y, -1, P)) % P
    else:
        slope = ((w - y) * pow(z - x, -1, P)) % P
    xx = (slope * slope - x - z) % P
    return xx, (slope * (x - xx) - y) % P


def mul(scalar, point):
    out = None
    while scalar:
        if scalar & 1:
            out = add(out, point)
        point = add(point, point)
        scalar >>= 1
    return out


def signed_sum(points, terms):
    out = None
    for i, sign in terms:
        x, y = points[i]
        out = add(out, (x, y if sign == 1 else (-y) % P))
    return out


def verify(name):
    data = json.loads((EVIDENCE / f"{name}.json").read_text())
    assert data["curve"] == {"p": P, "a": A, "b": B, "n": N, "g": list(G)}
    points = [tuple(point) for point in data["base"] + data["large_prime_pool"]]
    base_size = data["base_size"]
    assert len(points) == base_size + 1 and len({x for x, _ in points}) == len(points)
    assert all((y * y - x * x * x - A * x - B) % P == 0 for x, y in points)
    query = tuple(data["public_query"])
    assert (query[1] ** 2 - query[0] ** 3 - A * query[0] - B) % P == 0
    if name == "smoke":
        assert query == mul(987_654_321, G)
    else:
        assert signed_sum(points, data["control_planted_terms"]) == query
    witnesses = set()
    for three in itertools.combinations(range(base_size), 3):
        for signs in itertools.product((-1, 1), repeat=4):
            terms = tuple(sorted(zip((*three, base_size), signs)))
            if signed_sum(points, terms) == query:
                witnesses.add(terms)
    actual = set()
    for relation in data["partials"]:
        terms = tuple(tuple(term) for term in relation["terms"])
        assert signed_sum(points, terms) == query
        actual.add(terms)
    assert data["one_lp_replayed_relations"] == len(actual)
    assert actual == witnesses, (name, actual, witnesses)
    assert (data["status"] == "Complete") if witnesses else (data["status"] == "Empty")
    return {"control": name, "independent_witnesses": len(witnesses), "replay_ok": True}


def verify_cycle():
    left = json.loads((EVIDENCE / "wide.json").read_text())
    right = json.loads((EVIDENCE / "wide2.json").read_text())
    assert left["base"] == right["base"]
    assert left["large_prime_pool"] == right["large_prime_pool"]
    assert left["status"] == right["status"] == "Complete"
    assert len(left["partials"]) == len(right["partials"]) == 1
    base_size = left["base_size"]
    coefficients = [0] * (base_size + 1)
    for i, sign in left["partials"][0]["terms"]:
        coefficients[i] += sign
    for i, sign in right["partials"][0]["terms"]:
        coefficients[i] -= sign
    assert coefficients[base_size] == 0
    terms = [(i, coefficient) for i, coefficient in enumerate(coefficients[:base_size]) if coefficient]
    assert all(abs(coefficient) == 1 for _, coefficient in terms)
    points = [tuple(point) for point in left["base"] + left["large_prime_pool"]]
    left_query = tuple(left["public_query"])
    right_query = tuple(right["public_query"])
    difference = add(left_query, (right_query[0], (-right_query[1]) % P))
    assert signed_sum(points, terms) == difference
    return {"control": "wide_cycle", "cancelled_lp_index": base_size,
            "factor_base_terms": terms, "point_replay_ok": True}


def verify_public_panel():
    data = json.loads((EVIDENCE / "public-panel.json").read_text())
    assert data["curve"] == {"p": P, "a": A, "b": B, "n": N, "g": list(G)}
    points = [tuple(point) for point in data["base"] + data["large_prime_pool"]]
    assert len(points) == 8 and len({x for x, _ in points}) == 8
    q = tuple(data["public_target"])
    assert q == mul(987_654_321, G)
    assert len(data["queries"]) == 16
    total_witnesses = 0
    for label, row in enumerate(data["queries"]):
        assert row["label"] == label
        query = tuple(row["query"])
        assert query == add(mul(row["a"], G), mul(row["b"], q))
        witnesses = set()
        for three in itertools.combinations(range(7), 3):
            for signs in itertools.product((-1, 1), repeat=4):
                terms = tuple(sorted(zip((*three, 7), signs)))
                if signed_sum(points, terms) == query:
                    witnesses.add(terms)
        actual = set()
        for relation in row["one_lp_relations"]:
            terms = tuple(tuple(term) for term in relation["terms"])
            assert signed_sum(points, terms) == query
            actual.add(terms)
        assert actual == witnesses, (label, actual, witnesses)
        if row["status"] == "Empty":
            assert not witnesses
        total_witnesses += len(witnesses)
    return {"control": "public_aG_plus_bQ_panel", "queries": 16,
            "independent_one_lp_witnesses": total_witnesses,
            "all_query_and_witness_replays_ok": True}


if __name__ == "__main__":
    assert mul(N, G) is None
    print(json.dumps([verify("positive"), verify("smoke"),
                      verify("wide"), verify("wide2"), verify_cycle(),
                      verify_public_panel()], indent=2))
