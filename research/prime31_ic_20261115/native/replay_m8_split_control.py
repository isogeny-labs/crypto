#!/usr/bin/env python3
"""Separate exact point-law and modular-rank replay of the public m8 transcript.

This is a producer-operated validator, not an independently signed PCC verdict
or a Rust speed benchmark. No stored factor-base logs or target scalar enter
row construction. Requires Python 3.10+ and no external packages.
"""
import json
import sys
from pathlib import Path

P = 2147483647
N = 2147446831
A, B = 847514846, 2087444724
G = (625037715, 2100627251)
Q = (1547953785, 1667575595)


def root(v):
    v %= P
    r = pow(v, (P + 1) // 4, P)
    return r if r * r % P == v else None


def base_build(depth):
    for seed in range(2, 10000):
        c = seed
        for _ in range(depth):
            c = (c * c - 2) % P
        if c in (2, P - 2):
            continue
        xs = [c]
        for _ in range(depth):
            nx = set()
            for z in xs:
                r = root(z + 2)
                if r is not None:
                    nx.update((r, (-r) % P))
            xs = sorted(nx)
        if len(xs) != 1 << depth:
            continue
        pts = []
        for x in xs:
            y = root(x * x * x + A * x + B)
            if y is not None and y != 0:
                pts.append((x, min(y, P - y)))
        if len(pts) >= 2:
            return c, pts
    raise ValueError("no admissible deterministic Dickson fibre")


def neg(q):
    return None if q is None else (q[0], (-q[1]) % P)


def add(a, b):
    if a is None:
        return b
    if b is None:
        return a
    x, y = a
    u, v = b
    if x == u and (y + v) % P == 0:
        return None
    if x == u:
        slope = (3 * x * x + A) * pow(2 * y % P, -1, P) % P
    else:
        slope = (v - y) * pow((u - x) % P, -1, P) % P
    xx = (slope * slope - x - u) % P
    return xx, (slope * (x - xx) - y) % P


def scalar(point, k):
    out = None
    while k:
        if k & 1:
            out = add(out, point)
        point = add(point, point)
        k >>= 1
    return out


def decode(code):
    assert 0 <= code < (1 << 28)
    indices = [(code >> (6 * j)) & 63 for j in range(4)]
    signs = [-1 if (code >> (24 + j)) & 1 else 1 for j in range(4)]
    assert indices == sorted(set(indices))
    return list(zip(indices, signs))


def replay(data):
    assert data["curve"] == {
        "p": P, "a": A, "b": B, "n": N, "g": list(G), "cofactor": 1
    }
    assert data["public_target"] == list(Q)
    assert data["depth"] == 6 and data["B"] == 31
    assert data["queries"] == 124 and data["positive_queries"] == 26
    assert data["left_templates"] == 21840
    assert data["left_unique"] == 21840
    assert data["right_templates"] == 29120
    term, points = base_build(6)
    assert term == data["terminal"] == 933677148
    assert len(points) == 31
    assert scalar(G, N) is None and scalar(G, 1) == G
    basis = {}
    for a, b, lc, rc in data["witnesses"]:
        assert 0 <= a < N and 0 < b < N
        left, right = decode(lc), decode(rc)
        assert len(set(i for i, _ in left + right)) == 8
        assert all(i < 15 for i, _ in left)
        assert all(15 <= i < 31 for i, _ in right)
        point = None
        for i, sign in left + right:
            point = add(point, points[i] if sign == 1 else neg(points[i]))
        assert point == add(scalar(G, a), scalar(Q, b)), "false EC relation"
        row = [0] * 33
        for i, sign in left + right:
            row[i] = (row[i] + sign) % N
        row[31] = (-b) % N
        row[32] = a
        for col, pivot in sorted(basis.items()):
            k = row[col]
            if k:
                row = [(v - k * w) % N for v, w in zip(row, pivot)]
        nz = next((i for i, v in enumerate(row[:32]) if v), None)
        assert nz is not None, "dependent or contradictory transcript"
        factor = pow(row[nz], -1, N)
        basis[nz] = [(v * factor) % N for v in row]
    assert len(basis) == data["rank"] == data["raw_rows"] == 32
    sol = [0] * 32
    for i in sorted(basis, reverse=True):
        row = basis[i]
        sol[i] = (row[-1] - sum(row[j] * sol[j] for j in range(i + 1, 32))) % N
    assert all(scalar(G, sol[i]) == points[i] for i in range(31))
    assert scalar(G, sol[31]) == Q
    assert sol[31] == data["known_answer_verifier_only"] == data["recovered_scalar"]
    return {
        "standing": "[pcc:unverified]",
        "transcript_rows_checked": len(data["witnesses"]),
        "independent_rank": len(basis),
        "basis_logs_replayed": len(points),
        "public_target_replayed": True,
    }


def main():
    path = (Path(sys.argv[1]) if len(sys.argv) > 1 else
            Path(__file__).with_name("M8_SPLIT_CONTROL.results.json"))
    data = json.loads(path.read_text(encoding="utf-8"))
    print(json.dumps(replay(data), sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
