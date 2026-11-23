#!/usr/bin/env python3
"""Independent Python BigInt replay for the original Prime31 RR9 row-update gate.

Deliberately does not import Rust code or use its field/group arithmetic.
Checks every emitted target, support leaf, graph chart and 8x8 residue.
"""
import json
import sys

P = 2147483647
A = 847514846
B = 2087444724
G = (625037715, 2100627251)
W = [1, 3, 5, 7, 11, 13, 17, 19]


def inv(x):
    assert x % P
    return pow(x, P - 2, P)


def on_curve(q):
    if q is None:
        return True
    x, y = q
    return (y * y - x * x * x - A * x - B) % P == 0


def add_points(q, r):
    if q is None:
        return r
    if r is None:
        return q
    x, y = q
    u, v = r
    if x == u:
        if (y + v) % P == 0:
            return None
        m = (3 * x * x + A) * inv(2 * y) % P
    else:
        m = (v - y) * inv(u - x) % P
    X = (m * m - x - u) % P
    Y = (m * (x - X) - y) % P
    return (X, Y)


def scalar(n, q):
    out = None
    while n:
        if n & 1:
            out = add_points(out, q)
        q = add_points(q, q)
        n >>= 1
    return out


def canonical_base(size):
    points = []
    for x in range(10000):
        f = (x * x * x + A * x + B) % P
        y = pow(f, (P + 1) // 4, P)
        if y and y * y % P == f:
            points.append((x, min(y, P - y)))
            if len(points) == size:
                return points
    raise AssertionError("base enumeration exhausted")


def candidate_slots(base):
    all_slots = []
    seen = set()
    for w in W:
        positive = [scalar(w, q) for q in base]
        assert all(on_curve(q) and q is not None for q in positive)
        for q in positive:
            assert q[0] not in seen
            seen.add(q[0])
        all_slots.append(positive + [(q[0], (-q[1]) % P) for q in positive])
    return all_slots


def row(q, target):
    x, y = q
    r, s = target
    xp = [1, x % P, x * x % P, x * x * x % P]
    return [((x - r) * t) % P for t in xp] + [
        ((y + s) * t) % P for t in xp
    ]


def determinant(rows):
    a = [[t % P for t in rr] for rr in rows]
    n = len(a)
    prod = 1
    for k in range(n):
        t = next((i for i in range(k, n) if a[i][k]), None)
        if t is None:
            return 0
        if t != k:
            a[t], a[k] = a[k], a[t]
            prod = -prod % P
        pivot = a[k][k]
        prod = prod * pivot % P
        ip = inv(pivot)
        for i in range(k + 1, n):
            c = a[i][k] * ip % P
            for j in range(k + 1, n):
                a[i][j] = (a[i][j] - c * a[k][j]) % P
            a[i][k] = 0
    return prod


def inverse4(mat):
    aug = [[x % P for x in mat[i]] + [int(j == i) for j in range(4)]
           for i in range(4)]
    for k in range(4):
        t = next((i for i in range(k, 4) if aug[i][k]), None)
        if t is None:
            return None
        aug[k], aug[t] = aug[t], aug[k]
        ip = inv(aug[k][k])
        aug[k] = [(x * ip) % P for x in aug[k]]
        for i in range(4):
            if i != k:
                f = aug[i][k]
                aug[i] = [(a - f * b) % P
                          for a, b in zip(aug[i], aug[k])]
    return [rr[4:] for rr in aug]


def matmul4(x, y):
    return [[sum(x[i][k] * y[k][j] for k in range(4)) % P
             for j in range(4)] for i in range(4)]


def direct_chart(rows):
    ia = inverse4([r[:4] for r in rows])
    if ia is None:
        return None
    return matmul4(ia, [r[4:] for r in rows])


def row_update(rows, j, new_row):
    ia = inverse4([rr[:4] for rr in rows])
    assert ia is not None
    m = direct_chart(rows)
    assert m is not None
    u = [ia[i][j] for i in range(4)]
    d = (1 + sum((new_row[k] - rows[j][k]) * u[k]
                 for k in range(4))) % P
    if not d:
        return None
    v = [(new_row[k + 4] -
          sum(new_row[i] * m[i][k] for i in range(4))) % P
         for k in range(4)]
    return [[(m[i][k] + u[i] * inv(d) * v[k]) % P
             for k in range(4)] for i in range(4)]


def run(path):
    with open(path, encoding="utf-8") as fh:
        record = json.load(fh)
    assert record["p"] == P
    assert record["group_order"] == 2147446831
    assert record["original_curve_swaps_checked"] == 128
    assert record["bench_repetitions"] == 500
    assert record["full_b8_pair_comparisons_unchanged"] == 16 ** 8
    assert record["candidate_exhaustive_unknown_index"] is True
    assert record["complete_solver_speedup_claim"] is False
    assert len(record["timings_ms"]) == 4
    assert all(0 < t < 100000 for t in record["timings_ms"])
    assert len(record["cases"]) == 128

    base = canonical_base(8)
    slots = candidate_slots(base)
    planted = None
    for items in slots:
        planted = add_points(planted, items[0])
    allowed = {G, planted}
    assert len(allowed) == 2
    checked = set()

    for case in record["cases"]:
        target = tuple(case["target"])
        assert target in allowed
        assert on_curve(target)
        j, k = case["row"], case["choice"]
        assert 0 <= j < 4 and 0 <= k < 16
        expected_left = [slot[0] for slot in slots[:4]]
        expected_right = [slot[0] for slot in slots[4:]]
        actual_left = [tuple(q) for q in case["old_left_points"]]
        actual_right = [tuple(q) for q in case["right_points"]]
        leaf = tuple(case["new_leaf"])
        assert actual_left == expected_left
        assert actual_right == expected_right
        assert leaf == slots[j][k]
        assert all(on_curve(q) for q in actual_left + actual_right + [leaf])
        left = [row(q, target) for q in actual_left]
        replacement = row(leaf, target)
        changed = [r[:] for r in left]
        changed[j] = replacement
        right = [row(q, target) for q in actual_right]

        fresh = direct_chart(changed)
        updated = row_update(left, j, replacement)
        assert fresh is not None and updated is not None
        assert fresh == updated == case["fresh_chart"] == case["updated_chart"]
        residue = determinant(changed + right)
        assert residue == case["det8"] == case["schur"]
        checked.add((target, j, k))

    assert len(checked) == 128
    print(json.dumps({
        "verified": True,
        "independent_original_curve_cases": len(checked),
        "distinct_public_residuals": len(allowed),
        "original_curve": "Prime31 cofactor-one",
        "pcc": "unverified",
        "whole_solver_speedup": False
    }, sort_keys=True))


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("Usage: verify_prime31_rr9_rowupdate.py OUTPUT.json")
    run(sys.argv[1])
