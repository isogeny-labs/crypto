#!/usr/bin/env python3
"""Independent height ordering, rational-curve, and finite-scalar audit."""

import hashlib
import json
from fractions import Fraction
from math import gcd, isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def add(P, Q, p):
    if P is None:
        return Q
    if Q is None:
        return P
    x, y = P
    u, v = Q
    if x == u and (y + v + 94 * x + 188) % p == 0:
        return None
    if P == Q:
        m = ((3 * x * x + 1738 - 94 * y)
             * pow((2 * y + 94 * x + 188) % p, -1, p)) % p
    else:
        m = ((v - y) * pow((u - x) % p, -1, p)) % p
    nu = (m * m + 94 * m - x - u) % p
    nv = (-(m + 94) * nu - (y - m * x) - 188) % p
    return nu, nv


def multiply(k, P, p):
    R = None
    while k:
        if k & 1:
            R = add(R, P, p)
        P = add(P, P, p)
        k >>= 1
    return R


def ordered_candidates(target, p, cap):
    xt = target[0]
    candidates = []
    for b in range(1, cap + 1):
        if b % p == 0:
            continue
        residue = xt * b * b % p
        first = -cap + (residue + cap) % p
        for a in range(first, cap + 1, p):
            candidates.append((max(abs(a), b), b, a))
    candidates.sort()
    return candidates


def exact_lift(target, p, cap):
    xt, yt = target
    pairs = tests = roots = 0
    for H, b, a in ordered_candidates(target, p, cap):
        pairs += 1
        if gcd(abs(a), b) != 1:
            continue
        tests += 1
        B = 94 * a * b + 188 * b**3
        C = a**3 + 1738 * a * b**4 + 3805 * b**6
        D = B * B + 4 * C
        if D < 0 or isqrt(D)**2 != D:
            continue
        root = isqrt(D)
        for s in ((root,) if root == 0 else (root, -root)):
            if (s - B) % 2:
                continue
            c = (s - B) // 2
            roots += 1
            x = Fraction(a, b * b)
            y = Fraction(c, b**3)
            assert y*y + 94*x*y + 188*y == x**3 + 1738*x + 3805
            if c * pow(b, -3, p) % p == yt:
                assert a * pow(b, -2, p) % p == xt
                return {"status": "found", "height": H, "lift": [a, b, c],
                        "b_steps": H * (H - 1) // 2 + b,
                        "residue_pairs": pairs, "discriminant_tests": tests,
                        "rational_roots": roots}
    return {"status": "height_cap_exhausted", "height": cap, "lift": None,
            "b_steps": cap * (cap + 1) // 2,
            "residue_pairs": pairs, "discriminant_tests": tests,
            "rational_roots": roots}


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    rows_raw = (HERE / "arm-rows.jsonl").read_bytes()
    fixtures_raw = (HERE / "target-fixtures.jsonl").read_bytes()
    assert sha(rows_raw) == result["arm_rows_sha256"]
    assert sha(fixtures_raw) == result["target_fixtures_sha256"]
    rows = [json.loads(x) for x in rows_raw.splitlines()]
    fixtures = [json.loads(x) for x in fixtures_raw.splitlines()]
    checks = []
    for f in fixtures:
        p, q, target = f["p"], f["q"], tuple(f["target"])
        G = (-4 % p, 19 % p)
        assert multiply(q, G, p) is None
        assert multiply(f["fixture_scalar"], G, p) == target
        matching = [r for r in rows if r["p"] == p and r["target_index"] == f["target_index"]]
        assert len(matching) == 2 and {r["arm"] for r in matching} == {"rho", "lift"}
        rho = next(r for r in matching if r["arm"] == "rho")
        lift = next(r for r in matching if r["arm"] == "lift")
        assert rho["status"] == "verified"
        assert multiply(rho["scalar"], G, p) == target
        expected = exact_lift(target, p, protocol["height_cap"])
        for key, value in expected.items():
            assert lift[key] == value, (p, target, key, lift[key], value)
        checks.append({"p": p, "target_index": f["target_index"],
                       "rho_scalar_checked": True, "lift_order_checked": True,
                       "lift_status": lift["status"]})
    return {"schema": "xedni-public-target-lift-audit-v1", "status": "PASS",
            "result_sha256": sha((HERE / "result.json").read_bytes()),
            "checks": checks}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
