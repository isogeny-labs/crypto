"""Independent replay and FLINT rank check of the generic B677 graph control.

The validation scalar is used here only, after the native producer has written
its public-query relations and recovered result. It is not a solver input.
"""

import json
from collections import Counter
from pathlib import Path

from flint import nmod_mat


ROOT = Path(__file__).parent
RECEIPT = ROOT / "evidence" / "mitm-b677-control-01" / "forty-cap-full-cost.json"
B677_INPUT = ROOT / "evidence" / "rr5-b677-prerequisite-01" / "structured-positive.stdout"
N = 2_147_446_831
P = 2_147_483_647
A = 847_514_846
B = 2_087_444_724
G = (625_037_715, 2_100_627_251)
Q = (976_979_610, 1_228_979_761)


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


def scalar(k, point):
    out = None
    while k:
        if k & 1:
            out = add(out, point)
        point = add(point, point)
        k >>= 1
    return out


def dickson1024(x):
    for _ in range(10):
        x = (x * x - 2) % P
    return x


def splitmix64(state):
    mask = (1 << 64) - 1
    state = (state + 0x9e3779b97f4a7c15) & mask
    z = state
    z = ((z ^ (z >> 30)) * 0xbf58476d1ce4e5b9) & mask
    z = ((z ^ (z >> 27)) * 0x94d049bb133111eb) & mask
    return state, z ^ (z >> 31)


def main():
    receipt = json.loads(RECEIPT.read_text())
    fixture_line = next(line for line in B677_INPUT.read_text().splitlines()
                        if line.startswith("B677_INPUT "))
    fixture = json.loads(fixture_line.split(" ", 1)[1])
    points = [tuple(point) for point in fixture["public_base"]]
    assert len(points) == 677 and len(set(x for x, _ in points)) == 677
    assert receipt["curve"] == fixture["curve"]
    assert receipt["fixture_certificate"] == fixture["fixture_certificate"]
    assert tuple(receipt["public_target"]) == Q
    assert scalar(N, G) is None
    assert scalar(987_654_321, G) == Q
    assert all((y * y - x * x * x - A * x - B) % P == 0 for x, y in points)
    base = [i for i, (x, _) in enumerate(points)
            if dickson1024(x) == fixture["dickson_terminal"]]
    lp = [i for i in range(len(points)) if i not in set(base)]
    assert (len(base), len(lp)) == (509, 168)
    columns = {i: col for col, i in enumerate(base + lp)}
    assert receipt["pair_entries"] == 4 * 677 * 676 // 2
    assert receipt["probe_additions"] == receipt["pair_entries"] * len(receipt["queries"])

    state = 0x7033315f6d340101
    queries = []
    for label, query in enumerate(receipt["queries"]):
        state, raw_a = splitmix64(state)
        state, raw_b = splitmix64(state)
        assert (query["label"], query["a"], query["b"]) == (
            label, raw_a % N, raw_b % N)
        actual = tuple(query["query"])
        assert actual == add(scalar(query["a"], G), scalar(query["b"], Q))
        queries.append(actual)

    matrix = []
    classes = Counter()
    seen = set()
    for relation in receipt["relation_rows"]:
        label = relation["label"]
        terms = [tuple(term) for term in relation["terms"]]
        assert len(terms) == 4 and terms == sorted(terms)
        assert len({i for i, _ in terms}) == 4
        assert all(sign in (-1, 1) for _, sign in terms)
        key = (label, tuple(terms))
        assert key not in seen
        seen.add(key)
        summed = None
        base_terms = 0
        row = [0] * 679
        for i, sign in terms:
            x, y = points[i]
            summed = add(summed, (x, y if sign == 1 else (-y) % P))
            base_terms += i in set(base)
            row[columns[i]] = sign % N
        assert summed == queries[label]
        classes[base_terms] += 1
        if base_terms >= 2:
            row[677] = (-receipt["queries"][label]["b"]) % N
            row[678] = receipt["queries"][label]["a"]
            matrix.append(row)
    assert [classes[k] for k in range(5)] == receipt["classes_by_base_terms"]
    assert len(receipt["queries"]) == 11
    assert receipt["recovered_scalar_replay"] is True
    assert receipt["recovered_scalar"] == 987_654_321

    reduced, augmented_rank = nmod_mat(matrix, N).rref()
    target_only = []
    contradictions = 0
    for i in range(len(matrix)):
        row = [int(reduced[i, j]) for j in range(679)]
        if all(value == 0 for value in row[:678]) and row[678] != 0:
            contradictions += 1
        if all(value == 0 for value in row[:677]) and row[677] == 1:
            target_only.append(row[678])
    assert contradictions == 0
    assert target_only == [receipt["recovered_scalar"]], target_only
    assert scalar(target_only[0], G) == Q
    print(json.dumps({
        "curve": receipt["curve"],
        "queries_independently_replayed": len(queries),
        "relations_independently_replayed": len(seen),
        "classes_by_base_terms": [classes[k] for k in range(5)],
        "graph_eligible_rows": len(matrix),
        "flint_augmented_rank": augmented_rank,
        "target_only_rows": target_only,
        "contradictions": contradictions,
        "scalar_replay_ok": True,
    }, indent=2))


if __name__ == "__main__":
    main()
