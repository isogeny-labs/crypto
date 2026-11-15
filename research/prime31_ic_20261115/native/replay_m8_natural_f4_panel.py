#!/usr/bin/env python3
"""Producer-local independent prime31 EC replay of the frozen natural F4 panel.

Checks public masks, the Dickson factor base, and generic 4+4 validation
labels. It does NOT implement the prime-field F4 relation oracle, recover a
target log, validate cost, or provide an independently signed PCC verdict.
"""
import itertools
import json
from pathlib import Path

P = 2_147_483_647
N = 2_147_446_831
A = 847_514_846
B = 2_087_444_724
G = (625_037_715, 2_100_627_251)
Q = (1_547_953_785, 1_667_575_595)
MASK = (1 << 64) - 1


def sqrt_fp(v):
    v %= P
    x = pow(v, (P + 1) // 4, P)
    return x if x * x % P == v else None


def neg(p):
    return None if p is None else (p[0], (-p[1]) % P)


def add(p, q):
    if p is None:
        return q
    if q is None:
        return p
    x, y = p
    u, v = q
    if x == u:
        if (y + v) % P == 0:
            return None
        slope = (3 * x * x + A) * pow(2 * y, P - 2, P) % P
    else:
        slope = (v - y) * pow((u - x) % P, P - 2, P) % P
    xx = (slope * slope - x - u) % P
    return xx, (slope * (x - xx) - y) % P


def scalar(k, point):
    result = None
    while k:
        if k & 1:
            result = add(result, point)
        point = add(point, point)
        k >>= 1
    return result


def base():
    for seed in range(2, 10_000):
        terminal = seed
        for _ in range(6):
            terminal = (terminal * terminal - 2) % P
        if terminal in (2, P - 2):
            continue
        roots = [terminal]
        for _ in range(6):
            following = []
            for y in roots:
                root = sqrt_fp(y + 2)
                if root is not None:
                    following.extend([root, (-root) % P])
            roots = sorted(set(following))
        if len(roots) != 64:
            continue
        points = []
        for x in roots:
            y = sqrt_fp((x * x * x + A * x + B) % P)
            if y:
                points.append((x, min(y, P - y)))
        if len(points) >= 2:
            return seed, terminal, points
    raise AssertionError("No split Dickson fibre")


def fours(points, first, past):
    for indices in itertools.combinations(range(first, past), 4):
        for signs in range(16):
            point = None
            for i, index in enumerate(indices):
                q = points[index]
                point = add(point, neg(q) if signs & (1 << i) else q)
            code = sum(index << (6 * j) for j, index in enumerate(indices)) | (signs << 24)
            yield point, code


def splitmix_step(state):
    state = (state + 0x9e3779b97f4a7c15) & MASK
    x = state
    x = ((x ^ (x >> 30)) * 0xbf58476d1ce4e5b9) & MASK
    x = ((x ^ (x >> 27)) * 0x94d049bb133111eb) & MASK
    return state, x ^ (x >> 31)


def main():
    path = Path(__file__).with_name("M8_NATURAL_F4_PANEL_20261115.json")
    panel = json.loads(path.read_text())
    assert (panel["p"], panel["n"], panel["curve_a"], panel["curve_b"]) == (P, N, A, B)
    assert tuple(panel["g"]) == G and tuple(panel["q"]) == Q
    assert scalar(N, G) is None and scalar(N, Q) is None
    assert all((y * y - x * x * x - A * x - B) % P == 0 for x, y in (G, Q))
    seed, terminal, points = base()
    assert (seed, terminal, len(points)) == (3, 933677148, 31)
    left = list(fours(points, 0, 15))
    right = list(fours(points, 15, 31))
    assert len(left) == 21_840 and len(right) == 29_120
    left_table = {p: code for p, code in left}
    assert len(left_table) == len(left), "unexpected duplicate signed left sum"
    state = 0x4d384e4154555241
    positives = templates = 0
    for i, row in enumerate(panel["rows"]):
        assert row["index"] == i
        state, random_a = splitmix_step(state)
        state, random_b = splitmix_step(state)
        a, b = random_a % N, random_b % N
        assert b != 0 and (row["a"], row["b"]) == (a, b)
        target = add(scalar(a, G), scalar(b, Q))
        assert target is not None and list(target) == row["target"]
        first = None
        matches = 0
        for point, right_code in right:
            left_code = left_table.get(add(target, neg(point)))
            if left_code is not None:
                matches += 1
                if first is None:
                    first = [left_code, right_code]
        assert matches == row["split_witness_count"], (i, matches, row["split_witness_count"])
        assert first == row["first_witness_codes"], (i, first, row["first_witness_codes"])
        positives += matches > 0
        templates += matches
    assert len(panel["rows"]) == 16 and positives == 5 and templates == 6
    print(json.dumps({
        "standing": "[pcc:unverified]",
        "scope": "independent arithmetic replay of generic labels, not F4 or ECDLP",
        "public_masks": 16, "split_positives": positives,
        "split_misses": 11, "raw_signed_templates": templates,
        "left_templates": len(left), "right_templates": len(right)
    }, sort_keys=True))


if __name__ == "__main__":
    main()
