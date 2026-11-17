#!/usr/bin/env python3
"""Independent scalar check for the ecbench sessions of this study.

Written by the validator; imports nothing from the repository and shares no
code with ecbench.  For every record of every session given on the command
line it checks, on the Koblitz curve y^2 + xy = x^3 + a x^2 + 1 over
GF(2^n) (a, n from workload.curve_spec):

  * the defining polynomial: the smallest irreducible degree-n polynomial
    (as an integer) on which the session's generator G lies, with [r]G = O;
    every target of the session must lie on the same curve, else FAIL;
  * [recovered] G == target (outcome.recovered, workload.target);
  * [r] target == O (the target is in the order-r subgroup);
  * the record's own outcome.matches_target and outcome.status agree.

Usage: python3 -I verify_scalars.py OUT.json SESSION_DIR...
"""
import json
import sys


def deg(a):
    return a.bit_length() - 1


def pmulmod(a, b, f):
    n = deg(f)
    r = 0
    while b:
        if b & 1:
            r ^= a
        b >>= 1
        a <<= 1
        if (a >> n) & 1:
            a ^= f
    return r


def pmod(a, f):
    df = deg(f)
    while a and deg(a) >= df:
        a ^= f << (deg(a) - df)
    return a


def pgcd(a, b):
    while b:
        a, b = b, pmod(a, b)
    return a


def pinv(a, f):
    # extended Euclid over GF(2)[x]
    r0, r1, s0, s1 = f, a, 0, 1
    while r1:
        q = 0
        while r0 and deg(r0) >= deg(r1):
            sh = deg(r0) - deg(r1)
            q ^= 1 << sh
            r0 ^= r1 << sh
        r0, r1 = r1, r0
        # s0 - q*s1 (carry-less)
        qs = 0
        t, qq = s1, q
        while qq:
            if qq & 1:
                qs ^= t
            qq >>= 1
            t <<= 1
        s0, s1 = s1, s0 ^ qs
    assert r0 == 1, "not invertible"
    return pmod(s0, f)


def prime_factors(n):
    out, p = [], 2
    while p * p <= n:
        if n % p == 0:
            out.append(p)
            while n % p == 0:
                n //= p
        p += 1
    if n > 1:
        out.append(n)
    return out


def x_pow_2k(k, f):
    x = 2
    for _ in range(k):
        x = pmulmod(x, x, f)
    return x


def irreducible(f):
    n = deg(f)
    if x_pow_2k(n, f) != 2:
        return False
    for q in prime_factors(n):
        if pgcd(f, x_pow_2k(n // q, f) ^ 2) != 1:
            return False
    return True


class Curve:
    def __init__(self, a, f):
        self.a, self.f = a, f

    def on(self, P):
        if P is None:
            return True
        x, y = P
        m = lambda u, v: pmulmod(u, v, self.f)
        lhs = m(y, y) ^ m(x, y)
        rhs = m(m(x, x), x) ^ (m(x, x) if self.a else 0) ^ 1
        return lhs == rhs

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        f, m = self.f, (lambda u, v: pmulmod(u, v, self.f))
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if y2 == (x1 ^ y1):
                return None
            return self.dbl(P)
        lam = m(y1 ^ y2, pinv(x1 ^ x2, f))
        x3 = m(lam, lam) ^ lam ^ x1 ^ x2 ^ self.a
        y3 = m(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)

    def dbl(self, P):
        if P is None:
            return None
        f, m = self.f, (lambda u, v: pmulmod(u, v, self.f))
        x1, y1 = P
        if x1 == 0:
            return None
        lam = x1 ^ m(y1, pinv(x1, f))
        x3 = m(lam, lam) ^ lam ^ self.a
        y3 = m(x1, x1) ^ m(lam ^ 1, x3)
        return (x3, y3)

    def mul(self, k, P):
        R = None
        for bit in bin(k)[2:]:
            R = self.dbl(R)
            if bit == "1":
                R = self.add(R, P)
        return R


def pt(h):
    return (int(h[0], 16), int(h[1], 16))


def check_session(d):
    recs = [json.loads(l) for l in open(f"{d}/records.jsonl")]
    spec = recs[0]["workload"]["curve_spec"]
    a, n = int(spec["a"]), int(spec["n"])
    cv0 = recs[0]["workload"]["curve"]
    G, r = pt(cv0["generator"]), int(cv0["r"])
    targets = {tuple(x["workload"]["target"]) for x in recs}
    f = None
    tried = 0
    for cand in range((1 << n) + 1, 1 << (n + 1), 2):
        if not irreducible(cand):
            continue
        tried += 1
        c = Curve(a, cand)
        if c.on(G) and c.mul(r, G) is None:
            f = cand
            break
    out = {"session": d.rstrip("/").split("/")[-1], "a": a, "n": n, "r": r,
           "modulus_hex": hex(f) if f else None, "irreducibles_tried": tried,
           "records": len(recs), "distinct_targets": len(targets)}
    if f is None:
        out["ok"] = False
        out["problem"] = "no irreducible polynomial puts G on the curve with [r]G = O"
        return out
    c = Curve(a, f)
    out["targets_on_curve"] = all(c.on(pt(t)) for t in targets)
    out["targets_in_subgroup"] = all(c.mul(r, pt(t)) is None for t in targets)
    ok_scalar = bad_scalar = flag_mismatch = 0
    bad = []
    for x in recs:
        w, o = x["workload"], x["outcome"]
        assert w["curve"]["slug"] == cv0["slug"] and tuple(w["curve"]["generator"]) == tuple(cv0["generator"])
        rec = o.get("recovered")
        good = rec is not None and 0 <= int(rec) < r and c.mul(int(rec), G) == pt(w["target"])
        if good:
            ok_scalar += 1
        else:
            bad_scalar += 1
            bad.append(x["record_id"])
        if good != bool(o.get("matches_target")) or (o.get("status") == "verified") != good:
            flag_mismatch += 1
    out.update({"scalars_verified": ok_scalar, "scalars_failed": bad_scalar,
                "record_flag_mismatches": flag_mismatch, "failed_record_ids": bad[:20]})
    out["ok"] = (out["targets_on_curve"] and out["targets_in_subgroup"]
                 and bad_scalar == 0 and flag_mismatch == 0)
    return out


def main():
    out_path, dirs = sys.argv[1], sys.argv[2:]
    res = [check_session(d) for d in dirs]
    doc = {"schema": "validator-scalar-check/v1",
           "method": "independent Python GF(2^n) affine arithmetic; no repository code",
           "sessions": res,
           "records": sum(s["records"] for s in res),
           "scalars_verified": sum(s.get("scalars_verified", 0) for s in res),
           "ok": all(s["ok"] for s in res)}
    with open(out_path, "w") as fh:
        json.dump(doc, fh, indent=1)
        fh.write("\n")
    for s in res:
        print(s["session"], s["modulus_hex"], s.get("scalars_verified"), s.get("scalars_failed"), s["ok"])
    print("ALL OK" if doc["ok"] else "FAIL")
    return 0 if doc["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
