"""Independent integer group-law replay for the five-point RR5 controls.

The exhaustive witness oracle is used only after extraction, on this fixed
five-point correctness panel. It is not an input to the native producer.
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
    assert len(points) == 5 and len({x for x, _ in points}) == 5
    assert all((y * y - x * x * x - A * x - B) % P == 0 for x, y in points)
    query = tuple(data["public_query"])
    assert (query[1] ** 2 - query[0] ** 3 - A * query[0] - B) % P == 0
    if name == "smoke":
        assert query == mul(987_654_321, G)
    else:
        assert signed_sum(points, data["control_planted_terms"]) == query
    witnesses = set()
    for three in itertools.combinations(range(4), 3):
        for signs in itertools.product((-1, 1), repeat=4):
            terms = tuple(sorted(zip((*three, 4), signs)))
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


if __name__ == "__main__":
    assert mul(N, G) is None
    print(json.dumps([verify("positive"), verify("smoke")], indent=2))
