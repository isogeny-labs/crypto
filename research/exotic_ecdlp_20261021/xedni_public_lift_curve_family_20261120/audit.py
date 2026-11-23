#!/usr/bin/env python3
"""Pure-Python curve-order, scalar, and globally sorted lift audit."""

import hashlib
import json
from fractions import Fraction
from math import gcd, isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def is_prime(n):
    return n > 1 and all(n % d for d in range(2, isqrt(n) + 1))


def curve_order(p, t):
    count = 1
    for x in range(p):
        v = (x**3 + t*x + 1) % p
        count += 1 if v == 0 else 2 if pow(v, (p-1)//2, p) == 1 else 0
    return count


def add(P, Q, p, t):
    if P is None:
        return Q
    if Q is None:
        return P
    x, y = P
    u, v = Q
    if x == u and (y + v) % p == 0:
        return None
    if P == Q:
        m = (3*x*x + t) * pow(2*y % p, -1, p) % p
    else:
        m = (v-y) * pow((u-x) % p, -1, p) % p
    nu = (m*m - x - u) % p
    nv = (m*(x-nu) - y) % p
    return nu, nv


def multiply(k, P, p, t):
    R = None
    while k:
        if k & 1:
            R = add(R, P, p, t)
        P = add(P, P, p, t)
        k >>= 1
    return R


def sorted_candidates(target, p, cap):
    xt = target[0]
    triples = []
    for b in range(1, cap+1):
        if b % p == 0:
            continue
        residue = xt*b*b % p
        first = -cap + (residue+cap) % p
        for a in range(first, cap+1, p):
            triples.append((max(abs(a), b), b, a))
    triples.sort()
    return triples


def check_lift(target, p, t, cap):
    xt, yt = target
    pairs = tests = roots = 0
    for H, b, a in sorted_candidates(target, p, cap):
        pairs += 1
        if gcd(abs(a), b) != 1:
            continue
        tests += 1
        C = a**3 + t*a*b**4 + b**6
        if C < 0 or isqrt(C)**2 != C:
            continue
        c0 = isqrt(C)
        for c in ((c0,) if c0 == 0 else (c0, -c0)):
            roots += 1
            x, y = Fraction(a, b*b), Fraction(c, b**3)
            assert y*y == x**3 + t*x + 1
            if c*pow(b, -3, p) % p == yt:
                assert a*pow(b, -2, p) % p == xt
                return {"status": "found", "height": H, "lift": [a, b, c],
                        "b_steps": H*(H-1)//2+b, "residue_pairs": pairs,
                        "exact_tests": tests, "rational_roots": roots}
    return {"status": "height_cap_exhausted", "height": cap, "lift": None,
            "b_steps": cap*(cap+1)//2, "residue_pairs": pairs,
            "exact_tests": tests, "rational_roots": roots}


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result_raw = (HERE / "result.json").read_bytes()
    result = json.loads(result_raw)
    data = {}
    for name, expected in result["output_hashes"].items():
        raw = (HERE / name).read_bytes()
        assert sha(raw) == expected
        data[name] = [json.loads(x) for x in raw.splitlines()]
    selection = data["selection-rows.jsonl"]
    fixtures = data["target-fixtures.jsonl"]
    rows = data["arm-rows.jsonl"]
    chosen = {}
    for p in protocol["primes"]:
        selected = 0
        observed = [x for x in selection if x["p"] == p]
        for expected_t, cell in enumerate(observed, 1):
            assert cell["t"] == expected_t
            t = cell["t"]
            if (4*t**3+27) % p == 0:
                assert cell["status"] == "singular"
                continue
            q = curve_order(p, t)
            assert q == cell["q"]
            if not is_prime(q):
                assert cell["status"] == "composite_order"
                continue
            if multiply(q, (0, 1), p, t) is not None:
                assert cell["status"] == "nongenerator"
                continue
            assert cell["status"] == "selected"
            chosen[p, t] = q
            selected += 1
        assert selected == protocol["selected_per_prime"]
        assert observed[-1]["status"] == "selected"
    checks = []
    for f in fixtures:
        p, t, q = f["p"], f["t"], f["q"]
        assert chosen[p, t] == q
        target = tuple(f["target"])
        assert multiply(f["fixture_scalar"], (0, 1), p, t) == target
        if f["input_kind"] == "seeded":
            j = f["target_index"] - 2
            digest = sha(f"{protocol['seed']}:{p}:{t}:{j}".encode())
            assert digest == f["digest"]
            assert f["fixture_scalar"] == 1 + int(digest, 16) % (q-1)
        matching = [r for r in rows if (r["p"],r["t"],r["target_index"])
                    == (p,t,f["target_index"])]
        assert len(matching) == 2 and {r["arm"] for r in matching} == {"lift", "rho"}
        lift = next(r for r in matching if r["arm"] == "lift")
        rho = next(r for r in matching if r["arm"] == "rho")
        assert rho["status"] == "verified"
        assert multiply(rho["scalar"], (0, 1), p, t) == target
        expected = check_lift(target, p, t, protocol["height_cap"])
        for key, value in expected.items():
            assert lift[key] == value, (p,t,f["target_index"],key,lift[key],value)
        checks.append({"p": p, "t": t, "target_index": f["target_index"],
                       "lift_status": lift["status"], "scalar_checked": True,
                       "height_order_checked": True})
    return {"schema": "xedni-public-lift-curve-family-audit-v1",
            "status": "PASS", "result_sha256": sha(result_raw),
            "selection_rows_checked": len(selection), "target_checks": checks}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
