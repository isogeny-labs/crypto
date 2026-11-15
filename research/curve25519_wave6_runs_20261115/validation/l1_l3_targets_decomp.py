#!/usr/bin/env python3
"""Validator check of the per-target content of L1 and L3 cells (and the recorded chain cells they are compared with),
with curve code written for this validation (x-only Montgomery arithmetic; the engine works in short Weierstrass form):
 1. curve: #E by a character sum over a primitive-root character table, r = #E / 8 prime; shift = A/3 (Montgomery u ->
    Weierstrass x = u + A/3, B = 1); G = smallest u >= 2 with u^3 + A u^2 + u a square and [r]G = O;
 2. targets: the engine's LCG (st = st*6364136223846793005 + 1442695040888963407 mod 2^64, e = 1 + (st >> 11) % (r - 1))
    from the cell's seed, x(eG) by a Montgomery ladder, compared with every per_target.target_x;
 3. correctness flag: brute force over V^m, V = mu_D (Montgomery frame: x = A/3 + y, y^D = 1, gidx 0), of the
    summation polynomial S_{m+1}(u_1..u_m, u_T) (S_3 in Montgomery form; S_4 = Res_X(S_3(u1,u2,X), S_3(u3,u_T,X)));
    the system is consistent iff a root exists, which must equal (not inconsistent) for every target.
Usage: l1_l3_targets_decomp.py <out.json> <cell.json> ...   (cells: L1 p=16777141,A=7842 or k = 20 family member)"""
import json, sys, time
import numpy as np

M64 = (1 << 64) - 1

def is_prime_td(n):
    if n < 2: return False
    if n % 2 == 0: return n == 2
    f = 3
    while f * f <= n:
        if n % f == 0: return False
        f += 2
    return True

def factor_td(n):
    out, f = [], 2
    while f * f <= n:
        while n % f == 0: out.append(f); n //= f
        f += 1
    if n > 1: out.append(n)
    return out

def prim_root(p):
    qs = sorted(set(factor_td(p - 1))); g = 2
    while any(pow(g, (p - 1) // q, p) == 1 for q in qs): g += 1
    return g

def point_count(p, A):
    g = prim_root(p); B = 4096
    small = np.empty(B, dtype=np.int64); v = 1
    for i in range(B): small[i] = v; v = v * g % p
    nb = (p - 1 + B - 1) // B; big = np.empty(nb, dtype=np.int64); gb = pow(g, B, p); v = 1
    for j in range(nb): big[j] = v; v = v * gb % p
    pw = (np.outer(big, small) % p).ravel()[: p - 1]
    chi = np.zeros(p, dtype=np.int8); chi[pw] = np.where(np.arange(p - 1) % 2 == 0, 1, -1).astype(np.int8)
    u = np.arange(p, dtype=np.int64); f = ((u * u % p) * u % p + A * (u * u % p) % p + u) % p
    s = int(chi[f].sum(dtype=np.int64))
    return p + 1 + s

def ladder(u, k, A, p):
    """(X:Z) of [k](u:1) on y^2 = x^3 + A x^2 + x (x-only Montgomery ladder)."""
    a24 = (A + 2) * pow(4, -1, p) % p
    X2, Z2, X3, Z3 = 1, 0, u % p, 1
    for i in reversed(range(k.bit_length())):
        b = (k >> i) & 1
        if b: X2, Z2, X3, Z3 = X3, Z3, X2, Z2
        t1 = (X2 + Z2) % p; t2 = (X2 - Z2) % p; t3 = (X3 + Z3) % p; t4 = (X3 - Z3) % p
        aa = t1 * t1 % p; bb = t2 * t2 % p; e = (aa - bb) % p
        da = t4 * t1 % p; cb = t3 * t2 % p
        X3 = (da + cb) ** 2 % p; Z3 = u * (da - cb) ** 2 % p
        X2 = aa * bb % p; Z2 = e * (bb + a24 * e) % p
        if b: X2, Z2, X3, Z3 = X3, Z3, X2, Z2
    return X2, Z2

def xmul(u, k, A, p):
    X, Z = ladder(u, k, A, p)
    return None if Z == 0 else X * pow(Z, -1, p) % p

def s3_quad(x1, x2, A, p):
    """S3(x1, x2, X) = a X^2 + b X + c (Montgomery form) -- numpy int64 arrays or ints, p < 2^25."""
    s = (x1 + x2) % p; q = x1 * x2 % p
    a = (x1 - x2) % p; a = a * a % p
    b = (-2 * (((q + 1) % p) * s % p + 2 * A % p * q % p)) % p
    c = (q - 1) % p; c = c * c % p
    return a, b, c

def s3(x1, x2, x3, A, p):
    a, b, c = s3_quad(x1, x2, A, p)
    return (a * x3 % p * x3 % p + b * x3 % p + c) % p

def res2(a, b, c, d, e, f, p):
    """Res_X(aX^2+bX+c, dX^2+eX+f) = (af - cd)^2 - (ae - bd)(bf - ce)."""
    t1 = (a * f % p - c * d % p) % p; t2 = (a * e % p - b * d % p) % p; t3 = (b * f % p - c * e % p) % p
    return (t1 * t1 % p - t2 * t3 % p) % p

class Curve:
    def __init__(self, p, A):
        self.p, self.A = p, A
        assert is_prime_td(p) and p % 8 == 5
        self.n = point_count(p, A); assert self.n % 8 == 0 and is_prime_td(self.n // 8)
        self.r = self.n // 8; self.shift = A * pow(3, -1, p) % p; self.w = prim_root(p)
        u = 2
        while True:
            rhs = (u ** 3 + A * u * u + u) % p
            if rhs and pow(rhs, (p - 1) // 2, p) == 1 and ladder(u, self.r, A, p)[1] == 0: break
            u += 1
        self.gu = u
        self.selftest()

    def selftest(self):
        p, A = self.p, self.A; ok = 0
        for u0 in range(3, 60):
            rhs = (u0 ** 3 + A * u0 * u0 + u0) % p
            if not rhs or pow(rhs, (p - 1) // 2, p) != 1: continue
            x = {k: xmul(u0, k, A, p) for k in (1, 2, 3, 5, 6)}
            if None in x.values(): continue
            assert s3(x[1], x[2], x[3], A, p) == 0 and s3(x[2], x[3], x[5], A, p) == 0      # P + 2P = 3P, 2P + 3P = 5P
            a, b, c = s3_quad(x[1], x[2], A, p); d, e, f = s3_quad(x[3], x[6], A, p)
            assert res2(a, b, c, d, e, f, p) == 0                                                 # P + 2P + 3P = 6P
            assert s3(x[1], x[2], (x[3] + 1) % p, A, p) != 0
            ok += 1
        assert ok >= 5
        self.selftests = ok

    def targets(self, seed, n):
        st, out = seed, []
        for _ in range(n):
            st = (st * 6364136223846793005 + 1442695040888963407) & M64
            e = 1 + (st >> 11) % (self.r - 1)
            u = xmul(self.gu, e, self.A, self.p)
            out.append(None if u is None else (u + self.shift) % self.p)
        return out

    def mu(self, D):
        assert (self.p - 1) % D == 0
        z = pow(self.w, (self.p - 1) // D, self.p); s, v = [], 1
        for _ in range(D): s.append(v); v = v * z % self.p
        assert len(set(s)) == D and v == 1
        return np.array(s, dtype=np.int64)

    def decomposes(self, D, m, target_x):
        """number of tuples in V^m (V = mu_D in the Montgomery frame) with S_{m+1}(..., u_T) = 0"""
        p, A = self.p, self.A; V = self.mu(D); uT = (target_x - self.shift) % p
        X1, X2 = np.meshgrid(V, V, indexing="ij"); X1 = X1.ravel(); X2 = X2.ravel()
        if m == 2:
            return int(np.count_nonzero(s3(X1, X2, np.int64(uT), A, p) == 0))
        if m == 3:
            a, b, c = s3_quad(X1, X2, A, p); hits = 0
            for u3 in V.tolist():
                d, e, f = s3_quad(np.int64(u3), np.int64(uT), A, p)
                hits += int(np.count_nonzero(res2(a, b, c, d, e, f, p) == 0))
            return hits
        raise ValueError(m)

def curve_of(j):
    mini = j.get("miniature") or (str(j.get("k")) if j.get("k") else None)
    if j.get("a_mont"): return (j["p"], j["a_mont"])
    if str(j.get("miniature", "")).startswith("p="):
        ps, As = j["miniature"][2:].split(",A="); return (int(ps), int(As))
    return {"20": (1048573, 926)}[str(j.get("k", mini))]

def main():
    out_path, files = sys.argv[1], sys.argv[2:]
    curves, rows, t0 = {}, [], time.time()
    for f in files:
        j = json.load(open(f))
        if j.get("capped") and not j.get("per_target"):
            rows.append({"cell": f, "censored": True}); continue
        key = curve_of(j)
        if key not in curves: curves[key] = Curve(*key)
        C = curves[key]; D, m = j["d_or_t"], j["m"]
        pts = j["per_target"]; derived = C.targets(j["seed"], len(pts))
        recorded = [t["target_x"] for t in pts]
        dec = [C.decomposes(D, m, x) for x in recorded]
        flags_ok = [(h == 0) == bool(t["inconsistent"]) for h, t in zip(dec, pts)]
        rows.append({"cell": f, "p": C.p, "A": C.A, "m": m, "D": D, "seed": j["seed"],
                     "seed_rule_ok": j["seed"] == 20261108 + 1000 * m + D,
                     "targets_derived_equal": derived == recorded, "targets": recorded,
                     "decomposing_tuples": dec, "inconsistent_flags": [bool(t["inconsistent"]) for t in pts],
                     "flags_agree_with_bruteforce": all(flags_ok)})
    res = {"curves": {f"p={k[0]},A={k[1]}": {"n": c.n, "r": c.r, "r_twist": (2 * c.p + 2 - c.n) / 4, "shift_A_over_3": c.shift,
                                             "prim_root": c.w, "G_u": c.gu, "selftests_passed": c.selftests}
                      for k, c in curves.items()},
           "cells": rows, "seconds": round(time.time() - t0, 1),
           "all_targets_derived_equal": all(r.get("targets_derived_equal", True) for r in rows),
           "all_flags_agree": all(r.get("flags_agree_with_bruteforce", True) for r in rows),
           "all_seed_rule_ok": all(r.get("seed_rule_ok", True) for r in rows),
           "consistent_systems": sum(sum(1 for h in r.get("decomposing_tuples", []) if h) for r in rows),
           "targets_checked": sum(len(r.get("targets", [])) for r in rows)}
    json.dump(res, open(out_path, "w"), indent=1)
    print(json.dumps({k: v for k, v in res.items() if k != "cells"}, indent=1))
    for r in rows:
        if not r.get("censored") and not (r["targets_derived_equal"] and r["flags_agree_with_bruteforce"] and r["seed_rule_ok"]):
            print("PROBLEM", r["cell"], r["targets_derived_equal"], r["flags_agree_with_bruteforce"], r["seed_rule_ok"])

if __name__ == "__main__":
    main()
