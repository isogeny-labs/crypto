#!/usr/bin/env python3
"""Bounded, exact public-fixture relation panel; see PROTOCOL.md.

This is an independent finite-field instrument, not a PKM F4 implementation.
"""

import argparse
import json
import random
import time
from collections import defaultdict
from pathlib import Path

P = 127
R = 131
TARGET_K = 37
SEED = 20261125
QUERIES = 1024
Point = tuple[int, int] | None


class Curve:
    def __init__(self, a: int, b: int):
        self.a, self.b = a % P, b % P
        assert (4 * self.a**3 + 27 * self.b**2) % P
        self.ops = 0

    def rhs(self, x: int) -> int:
        return (x**3 + self.a * x + self.b) % P

    def points(self) -> list[Point]:
        squares: dict[int, list[int]] = defaultdict(list)
        for y in range(P):
            squares[y * y % P].append(y)
        return [None] + [(x, y) for x in range(P) for y in squares[self.rhs(x)]]

    @staticmethod
    def neg(q: Point) -> Point:
        return None if q is None else (q[0], -q[1] % P)

    def add(self, x: Point, y: Point) -> Point:
        self.ops += 1
        if x is None:
            return y
        if y is None:
            return x
        if x[0] == y[0] and (x[1] + y[1]) % P == 0:
            return None
        if x == y:
            s = (3 * x[0] ** 2 + self.a) * pow(2 * x[1], -1, P) % P
        else:
            s = (y[1] - x[1]) * pow(y[0] - x[0], -1, P) % P
        xr = (s * s - x[0] - y[0]) % P
        return xr, (s * (x[0] - xr) - x[1]) % P

    def mul(self, n: int, q: Point) -> Point:
        z = None
        while n:
            if n & 1:
                z = self.add(z, q)
            q = self.add(q, q)
            n >>= 1
        return z

    def order(self, q: Point, cap: int = 300) -> int:
        z = None
        for n in range(1, cap + 1):
            z = self.add(z, q)
            if z is None:
                return n
        raise AssertionError("order cap")


def nonsquare() -> int:
    return next(d for d in range(2, P) if pow(d, (P - 1) // 2, P) == P - 1)


def subgroup_coset(curve: Curve, group_order: int, h_order: int) -> list[int]:
    pts = [q for q in curve.points() if q is not None]
    assert len(pts) + 1 == group_order
    h = next(curve.mul(group_order // h_order, q) for q in pts
             if curve.order(curve.mul(group_order // h_order, q)) == h_order)
    subgroup = [curve.mul(j, h) for j in range(h_order)]
    r = next(q for q in pts if curve.mul(2, q) not in subgroup)
    fiber = [curve.add(r, z) for z in subgroup]
    xs = [q[0] for q in fiber]
    assert len(set(xs)) == h_order
    return xs


def f2mul(x: tuple[int, int], y: tuple[int, int]) -> tuple[int, int]:
    return ((x[0] * y[0] - x[1] * y[1]) % P,
            (x[0] * y[1] + x[1] * y[0]) % P)


def f2pow(x: tuple[int, int], n: int) -> tuple[int, int]:
    z = (1, 0)
    while n:
        if n & 1:
            z = f2mul(z, x)
        x = f2mul(x, x)
        n >>= 1
    return z


def torus_fiber() -> list[int]:
    assert P % 4 == 3 and (P + 1) % 8 == 0
    g = next((a, b) for a in range(P) for b in range(P)
             if (a * a + b * b) % P == 1 and f2pow((a, b), 64) != (1, 0))
    assert f2pow(g, 128) == (1, 0)
    h = f2pow(g, 16)
    xs = [2 * f2mul(g, f2pow(h, j))[0] % P for j in range(8)]
    assert len(set(xs)) == 8
    c = 2 * f2pow(g, 8)[0] % P
    assert c not in (2, P - 2)
    def dickson(x: int) -> int:
        u, v = 2, x
        for _ in range(2, 9):
            u, v = v, (x * v - u) % P
        return v
    assert sorted(xs) == [x for x in range(P) if dickson(x) == c]
    return xs


def lifts(curve: Curve, xs: list[int]) -> list[int]:
    return sorted({x % P for x in xs if any(y * y % P == curve.rhs(x % P) for y in range(P))})


def signed_base(curve: Curve, xs: list[int]) -> tuple[list[Point], list[int]]:
    selected = lifts(curve, xs)[:2]
    if len(selected) < 2:
        return [], selected
    pts = []
    for x in selected:
        y = min(y for y in range(P) if y * y % P == curve.rhs(x))
        assert y != 0
        pts.extend([(x, y), (x, -y % P)])
    assert len(pts) == 4
    return pts, selected


def rank(rows: list[list[int]]) -> int:
    if not rows:
        return 0
    a = [[v % R for v in row] for row in rows]
    pivot = 0
    for col in range(3):
        src = next((j for j in range(pivot, len(a)) if a[j][col]), None)
        if src is None:
            continue
        a[pivot], a[src] = a[src], a[pivot]
        inv = pow(a[pivot][col], -1, R)
        a[pivot] = [v * inv % R for v in a[pivot]]
        for j in range(len(a)):
            if j != pivot:
                f = a[j][col]
                a[j] = [(v - f * w) % R for v, w in zip(a[j], a[pivot])]
        pivot += 1
    return pivot


def rho(curve: Curve, gen: Point, target: Point, seed: int) -> dict:
    before = curve.ops
    rng = random.Random(seed)
    a, b = rng.randrange(R), rng.randrange(R)
    start = curve.add(curve.mul(a, gen), curve.mul(b, target))
    def step(state):
        q, aa, bb = state
        slot = 0 if q is None else q[0] % 3
        if slot == 0:
            return curve.add(q, gen), (aa + 1) % R, bb
        if slot == 1:
            return curve.add(q, target), aa, (bb + 1) % R
        return curve.add(q, q), 2 * aa % R, 2 * bb % R
    slow = step((start, a, b))
    fast = step(step((start, a, b)))
    for i in range(1, 10 * R + 1):
        if slow[0] == fast[0]:
            den = (slow[2] - fast[2]) % R
            if den:
                k = (fast[1] - slow[1]) * pow(den, -1, R) % R
                ops = curve.ops - before
                valid = curve.mul(k, gen) == target
                return {"steps": i, "ops": ops, "k": k, "valid": valid}
            return {"steps": i, "ops": curve.ops - before, "valid": False, "degenerate": True}
        slow = step(slow)
        fast = step(step(fast))
    return {"steps": 10 * R, "ops": curve.ops - before, "valid": False, "censored": True}


def main(out: Path) -> None:
    started = time.perf_counter(); cpu = time.process_time()
    e = Curve(1, 26)
    assert len(e.points()) == R
    gen = e.points()[1]
    assert e.mul(R, gen) is None
    target = e.mul(TARGET_K, gen)
    d = nonsquare()
    twist_wall0, twist_cpu0 = time.perf_counter(), time.process_time()
    twist = Curve(d * d, d * d * d * 26)
    assert len(twist.points()) == 125
    twist_x = subgroup_coset(twist, 125, 5)
    normalized = [x * pow(d, -1, P) % P for x in twist_x]
    assert not lifts(e, normalized)
    twist_setup = {"wall_s": time.perf_counter() - twist_wall0,
                   "cpu_s": time.process_time() - twist_cpu0,
                   "aux_group_ops": twist.ops}
    shift_wall0, shift_cpu0 = time.perf_counter(), time.process_time()
    shift = next(c for c in range(1, P) if len(lifts(e, [(x + c) % P for x in normalized])) >= 2)
    shift_setup = {"wall_s": time.perf_counter() - shift_wall0,
                   "cpu_s": time.process_time() - shift_cpu0,
                   "aux_group_ops": 0}
    aux_wall0, aux_cpu0 = time.perf_counter(), time.process_time()
    aux = Curve(1, 0)
    assert len(aux.points()) == 128
    aux_x = subgroup_coset(aux, 128, 8)
    aux_setup = {"wall_s": time.perf_counter() - aux_wall0,
                 "cpu_s": time.process_time() - aux_cpu0,
                 "aux_group_ops": aux.ops}
    torus_wall0, torus_cpu0 = time.perf_counter(), time.process_time()
    torus_x = torus_fiber()
    torus_setup = {"wall_s": time.perf_counter() - torus_wall0,
                   "cpu_s": time.process_time() - torus_cpu0,
                   "aux_group_ops": 0}
    fibers = {
        "small_x": list(range(P)),
        "pplus1_torus": torus_x,
        "unrelated_aux": aux_x,
        "twist_raw": normalized,
        "twist_shifted": [(x + shift) % P for x in normalized],
    }
    setup = {"small_x": {"wall_s": 0, "cpu_s": 0, "aux_group_ops": 0},
             "pplus1_torus": torus_setup, "unrelated_aux": aux_setup,
             "twist_raw": twist_setup,
             "twist_shifted": {k: twist_setup[k] + shift_setup[k] for k in twist_setup}}
    stream_rng = random.Random(SEED)
    stream = [(stream_rng.randrange(R), stream_rng.randrange(R)) for _ in range(QUERIES)]
    results = {}
    for name, fiber in fibers.items():
        wall0, cpu0, op0 = time.perf_counter(), time.process_time(), e.ops
        fb, selected = signed_base(e, fiber)
        record = {"fiber_x": sorted(set(fiber)), "fiber_size": len(set(fiber)),
                  "target_lift_x_count": len(lifts(e, fiber)),
                  "selected_x": selected, "columns": len(selected), "signed_points": len(fb),
                  "base_points": fb, "setup": setup[name]}
        if len(fb) < 4:
            record.update({"status": "censored_before_relation_search", "reason": "underfilled target base"})
        else:
            pairs: dict[Point, list[tuple[int, int]]] = defaultdict(list)
            for i, x in enumerate(fb):
                for j, y in enumerate(fb):
                    pairs[e.add(x, y)].append((i, j))
            rows, rhs, hits, all_hits, stopping = [], [], 0, 0, None
            relation_records = []
            for qi, (a, b) in enumerate(stream, 1):
                q = e.add(e.mul(a, gen), e.mul(b, target))
                for i, j in pairs.get(q, []):
                    assert e.add(fb[i], fb[j]) == q
                    row = [b, 0, 0]
                    row[1 + i // 2] = (row[1 + i // 2] - (1 if i % 2 == 0 else -1)) % R
                    row[1 + j // 2] = (row[1 + j // 2] - (1 if j % 2 == 0 else -1)) % R
                    rows.append(row); rhs.append(-a % R); all_hits += 1
                    relation_records.append({"query": qi, "a": a, "b": b,
                                             "point_indices": [i, j],
                                             "row": row.copy(), "rhs": -a % R})
                if pairs.get(q):
                    hits += 1
                if rank(rows) == 3:
                    stopping = qi
                    break
            record.update({"status": "full_rank" if stopping else "censored_at_query_cap",
                           "queries": stopping or QUERIES, "hit_queries": hits,
                           "verified_rows": all_hits, "coefficient_rank": rank(rows),
                           "unique_rows": len(set(zip(map(tuple, rows), rhs))),
                           "relations": relation_records,
                           "query_group_ops": e.ops - op0,
                           "query_wall_s": time.perf_counter() - wall0,
                           "query_cpu_s": time.process_time() - cpu0})
            # Independently validate the scalar equation for every row.
            validation_before = e.ops
            scalar_of = {e.mul(k, gen): k for k in range(R)}
            logs = [scalar_of[fb[0]], scalar_of[fb[2]]]
            assert all((row[0] * TARGET_K + row[1] * logs[0] +
                        row[2] * logs[1] - value) % R == 0
                       for row, value in zip(rows, rhs))
            record["validation_group_ops"] = e.ops - validation_before
        record.update({"target_group_ops": record.get("query_group_ops", e.ops - op0),
                       "wall_s": record.get("query_wall_s", time.perf_counter() - wall0),
                       "cpu_s": record.get("query_cpu_s", time.process_time() - cpu0)})
        record["cold_wall_s"] = setup[name]["wall_s"] + record["wall_s"]
        record["cold_cpu_s"] = setup[name]["cpu_s"] + record["cpu_s"]
        results[name] = record
    rho_results = [rho(e, gen, target, SEED + i) for i in range(16)]
    result = {"protocol": "PROTOCOL.md", "p": P, "curve": [1, 26], "r": R,
              "generator": gen, "target": target, "known_answer_validation": TARGET_K,
              "twist_nonsquare": d, "twist_shift": shift,
              "stream_seed": SEED, "stream_queries": stream,
              "arms": results, "plain_rho": rho_results,
              "total_wall_s": time.perf_counter() - started,
              "total_cpu_s": time.process_time() - cpu,
              "scope": "bounded exact Python instrument; no native PKM solver or L2 timing"}
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"arms": {k: (v["status"], v.get("coefficient_rank")) for k, v in results.items()},
                      "rho_valid": sum(v["valid"] for v in rho_results), "out": str(out)}))


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", type=Path, required=True)
    main(ap.parse_args().out)
