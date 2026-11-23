#!/usr/bin/env python3
"""Validator's own GF(2^n) Koblitz-curve check of every recovered logarithm.

For each curve the reduction polynomial is found independently (the low-weight
irreducible trinomial/pentanomial under which the record's generator G lies on
y^2 + xy = x^3 + a x^2 + 1 and [r]G = O); then for every verified record of
every session given, [recovered]G == target, the target is on the curve and
has order r.  Shares no code with ecbench.  Python stdlib only.

Usage: scalarcheck.py OUT.json SESSION_DIR [SESSION_DIR ...]
"""
import json, sys, pathlib, itertools


def pmulmod(a, b, f, n):
    r = 0
    while b:
        if b & 1:
            r ^= a
        b >>= 1
        a <<= 1
        if a >> n & 1:
            a ^= f
    return r


def pinv(a, f):
    # extended Euclid in GF(2)[x]
    u, v, g1, g2 = a, f, 1, 0
    while u != 1:
        j = u.bit_length() - v.bit_length()
        if j < 0:
            u, v, g1, g2, j = v, u, g2, g1, -j
        u ^= v << j
        g1 ^= g2 << j
    return g1


class Curve:
    def __init__(self, n, f, a, b=1):
        self.n, self.f, self.a, self.b = n, f, a, b

    def m(self, x, y):
        return pmulmod(x, y, self.f, self.n)

    def on(self, P):
        if P is None:
            return True
        x, y = P
        lhs = self.m(y, y) ^ self.m(x, y)
        x2 = self.m(x, x)
        rhs = self.m(x2, x) ^ (x2 if self.a else 0) ^ self.b
        return lhs == rhs

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 ^ y2 == x1 or x1 == 0 and y1 != y2:  # Q = -P (neg (x,y) = (x, x+y))
                return None
            if y1 == y2:
                return self.dbl(P)
        lam = self.m(y1 ^ y2, pinv(x1 ^ x2, self.f))
        x3 = self.m(lam, lam) ^ lam ^ x1 ^ x2 ^ (1 if self.a else 0)
        y3 = self.m(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)

    def dbl(self, P):
        if P is None:
            return None
        x, y = P
        if x == 0:
            return None
        lam = x ^ self.m(y, pinv(x, self.f))
        x3 = self.m(lam, lam) ^ lam ^ (1 if self.a else 0)
        y3 = self.m(x, x) ^ self.m(lam ^ 1, x3)
        return (x3, y3)

    def mul(self, k, P):
        R = None
        for bit in bin(k)[2:] if k > 0 else "":
            R = self.dbl(R)
            if bit == "1":
                R = self.add(R, P)
        return R


def irreducible_prime_degree(f, n):
    # n prime: f irreducible iff x^(2^n) == x mod f and f has no root (no factor of degree 1)
    if f & 1 == 0 or bin(f).count("1") % 2 == 0:
        return False
    x = 2
    t = x
    for _ in range(n):
        t = pmulmod(t, t, f, n)
    return t == x


def find_field(n, a, G, r):
    found = []
    for k in range(1, n):
        f = (1 << n) | (1 << k) | 1
        if irreducible_prime_degree(f, n):
            E = Curve(n, f, a)
            if E.on(G) and E.mul(r, G) is None:
                found.append(f)
    if not found:
        for k1, k2, k3 in itertools.combinations(range(1, n), 3):
            f = (1 << n) | (1 << k1) | (1 << k2) | (1 << k3) | 1
            if irreducible_prime_degree(f, n):
                E = Curve(n, f, a)
                if E.on(G) and E.mul(r, G) is None:
                    found.append(f); break
    return found


def main():
    outp = sys.argv[1]
    curves, res = {}, {}
    for d in sys.argv[2:]:
        d = pathlib.Path(d)
        stats = {"records": 0, "verified": 0, "checked": 0, "ok": 0, "bad": []}
        seen = {}
        with open(d / "records.jsonl") as fh:
            for line in fh:
                r = json.loads(line)
                stats["records"] += 1
                if r["outcome"]["status"] != "verified":
                    continue
                stats["verified"] += 1
                c = r["workload"]["curve"]; spec = r["workload"]["curve_spec"]
                key = c["slug"]
                if key not in curves:
                    G = (int(c["generator"][0], 16), int(c["generator"][1], 16))
                    fs = find_field(spec["n"], spec["a"], G, c["r"])
                    if len(fs) != 1:
                        raise SystemExit(f"{key}: field polynomial not unique: {fs}")
                    E = Curve(spec["n"], fs[0], spec["a"])
                    curves[key] = {"E": E, "G": G, "r": c["r"], "info": {
                        "n": spec["n"], "a": spec["a"], "poly": hex(fs[0]), "r": c["r"],
                        "group_order": c["group_order"], "cofactor": c["cofactor"],
                        "G_on_curve": E.on(G), "rG_is_O": E.mul(c["r"], G) is None,
                        "NG_is_O": E.mul(c["group_order"], G) is None}}
                cv = curves[key]
                T = (int(r["workload"]["target"][0], 16), int(r["workload"]["target"][1], 16))
                k = int(r["outcome"]["recovered"])
                sk = (key, T, k)
                if sk not in seen:
                    E = cv["E"]
                    ok = E.on(T) and E.mul(k, cv["G"]) == T and E.mul(cv["r"], T) is None
                    seen[sk] = ok
                stats["checked"] += 1
                if seen[sk]:
                    stats["ok"] += 1
                else:
                    stats["bad"].append(r["record_id"])
        stats["distinct_target_log_pairs"] = len(seen)
        res[d.name] = stats
        print(d.name, {k: v for k, v in stats.items() if k != "bad"}, "bad:", len(stats["bad"]), flush=True)
    out = {"schema": "validator.scalarcheck/v1", "curves": {k: v["info"] for k, v in curves.items()}, "sessions": res}
    pathlib.Path(outp).write_text(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
