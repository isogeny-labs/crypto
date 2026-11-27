#!/usr/bin/env python3
"""Independent replay of panel relation rows and target projection."""

import argparse
import json
from pathlib import Path

MOD = 127
ORDER = 131


def neg(q):
    return None if q is None else (q[0], -q[1] % MOD)


def plus(q, z):
    if q is None:
        return z
    if z is None:
        return q
    if q == neg(z):
        return None
    if q == z:
        slope = (3 * q[0] * q[0] + 1) * pow(2 * q[1], -1, MOD) % MOD
    else:
        slope = (z[1] - q[1]) * pow(z[0] - q[0], -1, MOD) % MOD
    x = (slope * slope - q[0] - z[0]) % MOD
    return x, (slope * (q[0] - x) - q[1]) % MOD


def times(k, q):
    ans = None
    for _ in range(k % ORDER):
        ans = plus(ans, q)
    return ans


def rank(rows):
    mat = [list(map(lambda x: x % ORDER, row)) for row in rows]
    j = 0
    for col in range(3):
        found = next((i for i in range(j, len(mat)) if mat[i][col]), None)
        if found is None:
            continue
        mat[j], mat[found] = mat[found], mat[j]
        inv = pow(mat[j][col], -1, ORDER)
        mat[j] = [v * inv % ORDER for v in mat[j]]
        for i in range(len(mat)):
            if i != j:
                c = mat[i][col]
                mat[i] = [(u - c * v) % ORDER for u, v in zip(mat[i], mat[j])]
        j += 1
    return j


def main(path: Path) -> None:
    data = json.loads(path.read_text())
    assert (data["p"], data["curve"], data["r"]) == (MOD, [1, 26], ORDER)
    gen, target = tuple(data["generator"]), tuple(data["target"])
    assert target == times(data["known_answer_validation"], gen)
    checked = 0
    for name, arm in data["arms"].items():
        fiber = arm["fiber_x"]
        assert len(fiber) == arm["fiber_size"] == len(set(fiber))
        liftable = [x for x in fiber if any(y * y % MOD == (x**3 + x + 26) % MOD
                                                for y in range(MOD))]
        assert len(liftable) == arm["target_lift_x_count"]
        base = [tuple(q) for q in arm["base_points"]]
        if arm["status"] == "censored_before_relation_search":
            assert len(base) < 4 and len(liftable) < 2
            continue
        assert len(base) == 4 and len(arm["selected_x"]) == 2
        assert base[1] == neg(base[0]) and base[3] == neg(base[2])
        rows, hits = [], set()
        records = arm["relations"]
        for item in records:
            qi = item["query"]
            a, b = data["stream_queries"][qi - 1]
            assert (a, b) == (item["a"], item["b"])
            i, j = item["point_indices"]
            lhs = plus(times(a, gen), times(b, target))
            assert lhs == plus(base[i], base[j])
            row = [b, 0, 0]
            for idx in (i, j):
                row[1 + idx // 2] -= 1 if idx % 2 == 0 else -1
            row = [x % ORDER for x in row]
            assert row == item["row"] and item["rhs"] == -a % ORDER
            rows.append(row)
            hits.add(qi)
            checked += 1
        assert len(rows) == arm["verified_rows"]
        assert len(hits) == arm["hit_queries"]
        assert rank(rows) == arm["coefficient_rank"]
        assert len({(tuple(x["row"]), x["rhs"]) for x in records}) == arm["unique_rows"]
        first_full = next((qi for qi in range(1, arm["queries"] + 1)
                           if rank([x["row"] for x in records if x["query"] <= qi]) == 3), None)
        assert (first_full == arm["queries"]) if arm["status"] == "full_rank" else first_full is None
    assert len(data["plain_rho"]) == 16
    assert all(rho["valid"] and rho["k"] == data["known_answer_validation"]
               for rho in data["plain_rho"])
    print(json.dumps({"status": "pass", "independently_checked_rows": checked,
                      "arms": len(data["arms"]), "rho_recoveries": 16}))


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("result", type=Path)
    main(ap.parse_args().result)
