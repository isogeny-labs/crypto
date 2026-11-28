#!/usr/bin/env python3
"""Bounded, resumable PKM isogeny-map relation screen; Python stdlib only."""
import argparse
import hashlib
import json
import math
import os
import random
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
P = 1000003
A, B = 2, 3
SEED = 20261123
SCREEN, CONFIRM = 48, 96
DELTAS = (0, 1, 2, 4, 8, 16, 32, 64)


def rhs(x):
    return (x * x * x + A * x + B) % P


def lift(x):
    z = rhs(x)
    y = pow(z, (P + 1) // 4, P)
    return y if y * y % P == z else None


def add(u, v):
    """Affine addition; return result and one inversion when needed."""
    if u is None:
        return v, 0
    if v is None:
        return u, 0
    x1, y1 = u
    x2, y2 = v
    if x1 == x2 and (y1 + y2) % P == 0:
        return None, 0
    if x1 == x2:
        d, n = 2 * y1 % P, (3 * x1 * x1 + A) % P
    else:
        d, n = (x2 - x1) % P, (y2 - y1) % P
    if not d:
        return None, 0
    slope = n * pow(d, -1, P) % P
    x3 = (slope * slope - x1 - x2) % P
    return (x3, (slope * (x1 - x3) - y1) % P), 1


def poly(coeffs, x):
    v = 0
    for c in reversed(coeffs):
        v = (v * x + c) % P
    return v


def map_value(steps, x):
    for s in steps:
        d = poly(s["den"], x)
        if d == 0:
            return None
        x = poly(s["num"], x) * pow(d, -1, P) % P
    return x


def source(name):
    raw = (HERE / (name + "_source.json")).read_bytes()
    outer = json.loads(raw)
    t = outer.get("tower", outer)
    assert t["p"] == P and t["kind"]["form"] == "isogeny"
    degree = math.prod(s["degree"] for s in t["steps"])
    assert degree == t["S"] == len(t["V"]) == len(set(t["V"]))
    assert all(map_value(t["steps"], x) == t["c"] for x in t["V"])
    return t, hashlib.sha256(raw).hexdigest()


def targets():
    rng = random.Random(SEED)
    out = []
    seen = set()
    while len(out) < SCREEN + CONFIRM:
        x = rng.randrange(P)
        if x in seen:
            continue
        y = lift(x)
        if y is not None and y != 0:
            seen.add(x)
            out.append((x, y if rng.randrange(2) else (-y) % P))
    return out


def interval_base(size):
    xs = []
    x = 0
    while len(xs) < size:
        if lift(x) is not None:
            xs.append(x)
        x += 1
    return xs


def candidate_base(t, delta):
    fiber = sorted((x + delta) % P for x in t["V"])
    lifting = [x for x in fiber if lift(x) is not None]
    return lifting, fiber


def query(base, target):
    member = set(base)
    count = 0
    inversions = 0
    for x in base:
        y = lift(x)
        assert y is not None
        for yy in (y, (-y) % P) if y else (0,):
            q, inv = add(target, (x, (-yy) % P))
            inversions += inv
            if q is not None and q[0] in member:
                assert rhs(q[0]) == q[1] * q[1] % P
                witness, _ = add((x, yy), q)
                assert witness == target
                count += 1
    return count, inversions


def measure(name, base, ts, extra):
    start_w, start_c = time.perf_counter(), time.process_time()
    counts = []
    inversions = 0
    for target in ts:
        n, inv = query(base, target)
        counts.append(n)
        inversions += inv
    wall, cpu = time.perf_counter() - start_w, time.process_time() - start_c
    return {"id": name, "base_size": len(base), "screen_hits": sum(n > 0 for n in counts[:SCREEN]),
            "confirm_hits": sum(n > 0 for n in counts[SCREEN:]),
            "screen_relations": sum(counts[:SCREEN]), "confirm_relations": sum(counts[SCREEN:]),
            "success_bits": [int(n > 0) for n in counts], "ordered_counts": counts,
            "query_wall_s": wall, "query_cpu_s": cpu, "query_inversions": inversions, **extra}


def paired_tail(a, b):
    wins = sum(x > y for x, y in zip(a, b))
    losses = sum(x < y for x, y in zip(a, b))
    n = wins + losses
    return (sum(math.comb(n, k) for k in range(wins, n + 1)) / 2**n if n else 1.0,
            wins, losses)


def append(path, row):
    with path.open("a") as f:
        f.write(json.dumps(row, sort_keys=True) + "\n")
        f.flush()
        os.fsync(f.fileno())


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, default=HERE / "results.jsonl")
    args = ap.parse_args()
    assert P % 4 == 3 and (4 * A**3 + 27 * B**2) % P != 0
    existing = {}
    if args.out.exists():
        for line in args.out.read_text().splitlines():
            row = json.loads(line)
            existing[row["id"]] = row
    ts = targets()
    candidates = [(name, delta) for name in ("cm", "isogeny") for delta in DELTAS]
    for name, delta in candidates:
        key = f"{name}:shift={delta}"
        if key in existing:
            continue
        t, digest = source(name)
        start_w, start_c = time.perf_counter(), time.process_time()
        base, fiber = candidate_base(t, delta)
        lifting = len(base)
        valid = bool(base)
        # Translation adds one degree-one map: x -> x-delta.
        checked = valid and all(map_value(t["steps"], (x-delta) % P) == t["c"] for x in fiber)
        build_wall = time.perf_counter()-start_w
        build_cpu = time.process_time()-start_c
        extra = {"kind":"pkm-isogeny", "source_sha256":digest, "delta":delta,
                 "low_degrees":[s["degree"] for s in t["steps"]], "high_degree":t["S"],
                 "terminal_c":t["c"], "fiber_size":len(fiber), "lifting_count":lifting,
                 "fiber_verified":bool(checked), "base_wall_s":build_wall, "base_cpu_s":build_cpu,
                 "source_search_build_charged":False}
        if not checked:
            row = {"id":key, "status":"invalid_or_insufficient_lifts", **extra}
        else:
            control_key = f"interval:size={lifting}"
            if control_key not in existing:
                ctrl_start = time.process_time()
                control_base = interval_base(lifting)
                control = measure(control_key, control_base, ts,
                                  {"kind":"size_matched_control", "base_cpu_s":time.process_time()-ctrl_start})
                append(args.out, control)
                existing[control_key] = control
            baseline = existing[control_key]
            row = measure(key, base, ts, extra)
            pval, wins, losses = paired_tail(row["success_bits"][SCREEN:], baseline["success_bits"][SCREEN:])
            row.update({"paired_p":pval,"paired_wins":wins,"paired_losses":losses,
                        "control_id":control_key, "control_screen_hits":baseline["screen_hits"],
                        "control_confirm_hits":baseline["confirm_hits"],
                        "screen_advances":row["screen_hits"] > baseline["screen_hits"]})
            ntest = len(candidates)
            row["confirmed_gain"] = (row["screen_advances"] and row["confirm_hits"] > baseline["confirm_hits"]
                  and pval < .01/ntest and row["query_cpu_s"] / max(row["confirm_hits"]+row["screen_hits"],1)
                  < baseline["query_cpu_s"] / max(baseline["confirm_hits"]+baseline["screen_hits"],1))
        append(args.out, row)
        existing[key] = row
        print(json.dumps({k:row.get(k) for k in ("id","screen_hits","confirm_hits","paired_p","query_cpu_s","confirmed_gain","status")}), flush=True)
        if row.get("confirmed_gain"):
            break


if __name__ == "__main__":
    main()
