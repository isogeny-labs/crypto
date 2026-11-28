#!/usr/bin/env python3
"""Validator: instance-level checks for the 28 curves.

- instance records equal pre-check 1's for the 24 grid curves (same f, order, r, h, n_points);
- r = order is prime (deterministic Miller-Rabin for n < 3.3e24);
- order lies in the Hasse-Weil interval [(sqrt p - 1)^6, (sqrt p + 1)^6];
- n_points recomputed by brute force for p <= 8209 (numpy, row by row);
- |n_points - p - 1| <= 6 sqrt p (Hasse-Weil for genus 3).
Usage: python3 -I instances.py STUDY_DIR PREV_RUN_JSONL > instances.json
"""
import json
import math
import sys

import numpy as np

STUDY, PREV = sys.argv[1], sys.argv[2]
M4 = [(a, b, 4 - a - b) for a in range(4, -1, -1) for b in range(4 - a, -1, -1)]


def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def npoints(f, p):
    z = np.arange(p, dtype=np.int64)
    zp = [np.ones(p, dtype=np.int64)]
    for _ in range(4):
        zp.append(zp[-1] * z % p)
    cnt = 0
    for y in range(p):
        acc = np.zeros(p, dtype=np.int64)
        for c, (a, b, cc) in zip(f, M4):
            if c:
                acc = (acc + (c * pow(y, b, p)) % p * zp[cc]) % p
        cnt += int((acc == 0).sum())
    # x = 0: [0, 1, z] and [0, 0, 1]
    for zz in range(p):
        if sum(c * pow(zz, cc, p) for c, (a, b, cc) in zip(f, M4) if a == 0) % p == 0:
            cnt += 1
    if f[M4.index((0, 0, 4))] == 0:
        cnt += 1
    return cnt


inst = {}
for fn in ["raw/run.jsonl", "raw/run65537_odd.jsonl", "raw/run65537_even.jsonl"]:
    for l in open(f"{STUDY}/{fn}"):
        r = json.loads(l)
        if r["kind"] == "instance" and r["family"] == "quartic":
            inst[(r["p"], r["curve_seed"])] = r
prev = {}
for l in open(PREV):
    r = json.loads(l)
    if r["kind"] == "instance" and r["family"] == "quartic":
        prev[(r["p"], r["curve_seed"])] = r
out = {"n": len(inst), "same_as_precheck1": 0, "in_precheck1": 0, "rows": []}
for k in sorted(inst):
    r = inst[k]
    p = r["p"]
    if k in prev:
        out["in_precheck1"] += 1
        out["same_as_precheck1"] += prev[k] == r
    lo = (math.sqrt(p) - 1) ** 6
    hi = (math.sqrt(p) + 1) ** 6
    row = {"p": p, "seed": r["curve_seed"], "r_prime": is_prime(r["r"]), "h": r["h"],
           "order_eq_hr": r["order"] == r["h"] * r["r"], "hasse_weil": lo <= r["order"] <= hi,
           "n1_hw": abs(r["n_points"] - p - 1) <= 6 * math.sqrt(p)}
    if p <= 8209:
        row["n_points_recomputed_eq"] = npoints(r["f"], p) == r["n_points"]
    out["rows"].append(row)
out["all_r_prime"] = all(x["r_prime"] for x in out["rows"])
out["all_hw"] = all(x["hasse_weil"] and x["n1_hw"] and x["order_eq_hr"] for x in out["rows"])
out["n_points_checked"] = sum(1 for x in out["rows"] if "n_points_recomputed_eq" in x)
out["n_points_ok"] = sum(1 for x in out["rows"] if x.get("n_points_recomputed_eq"))
json.dump(out, sys.stdout, indent=1)
print()
