#!/usr/bin/env python3
"""LEAD-EXO-lattes-xline-dynamics: k-dependence of the F_p-factorisation pattern of
phi_2^(n)(x) - x(kP). Stdlib only, one core, fixed seeds. See PROTOCOL.md."""
import json, random, sys, time

SMOKE = "--smoke" in sys.argv
T0 = time.time()
DEADLINE = 8 if SMOKE else 520

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def factor(n):
    f = {}; d = 2
    while d * d <= n:
        while n % d == 0: f[d] = f.get(d, 0) + 1; n //= d
        d += 1
    if n > 1: f[n] = f.get(n, 0) + 1
    return f

# ---- polynomials over F_p (low->high)
def trim(a):
    while a and a[-1] == 0: a.pop()
    return a
def padd(a, b, p):
    n = max(len(a), len(b)); r = [0] * n
    for i, v in enumerate(a): r[i] = v
    for i, v in enumerate(b): r[i] = (r[i] + v) % p
    return trim(r)
def pscale(a, c, p): return trim([(v * c) % p for v in a])
def pmul(a, b, p):
    if not a or not b: return []
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b): r[i + j] += x * y
    return trim([v % p for v in r])
def pmod(a, m, p):
    a = a[:]; dm = len(m) - 1; inv = pow(m[-1], -1, p)
    while len(a) - 1 >= dm and a:
        c = (a[-1] * inv) % p; s = len(a) - 1 - dm
        if c:
            for i, v in enumerate(m): a[s + i] = (a[s + i] - c * v) % p
        a.pop(); trim(a)
    return a
def pgcd(a, b, p):
    a = trim(a[:]); b = trim(b[:])
    while b: a, b = b, pmod(a, b, p)
    return a
def pderiv(a, p): return trim([(i * a[i]) % p for i in range(1, len(a))])
def ppow_x(e, m, p):
    r = [1]; base = pmod([0, 1], m, p)
    while e:
        if e & 1: r = pmod(pmul(r, base, p), m, p)
        base = pmod(pmul(base, base, p), m, p); e >>= 1
    return r
def pdiv_exact(a, b, p):
    a = a[:]; q = [0] * max(1, len(a) - len(b) + 1); inv = pow(b[-1], -1, p)
    while len(a) >= len(b) and a:
        c = (a[-1] * inv) % p; s = len(a) - len(b); q[s] = c
        for i, v in enumerate(b): a[s + i] = (a[s + i] - c * v) % p
        a.pop(); trim(a)
    return trim(q)

def ddf_pattern(f, p):
    """Return ('sf'|'nsf', sorted tuple of (deg, count)) for the roots' F_p-degree structure.
    Squarefree part is used; 'nsf' flags a repeated factor."""
    f = trim(f[:]); flag = "sf"
    g = pgcd(f, pderiv(f, p), p)
    if len(g) > 1:
        flag = "nsf"; f = pdiv_exact(f, g, p)
    out = {}; h = [0, 1]; i = 0; cur = f
    while len(cur) - 1 >= 2 * (i + 1):
        i += 1
        h = pmod(ppow_x(p, cur, p) if i == 1 else _frob(h, cur, p), cur, p)
        d = pgcd(cur, padd(h, [0, p - 1], p), p)
        if len(d) > 1:
            out[i] = (len(d) - 1) // i
            cur = pdiv_exact(cur, d, p); h = pmod(h, cur, p) if cur else h
    if len(cur) > 1: out[len(cur) - 1] = out.get(len(cur) - 1, 0) + 1
    return (flag, tuple(sorted(out.items())))
def _frob(h, m, p):
    # h(x)^p mod m by square-and-multiply on the polynomial h
    r = [1]; base = h[:]; e = p
    while e:
        if e & 1: r = pmod(pmul(r, base, p), m, p)
        base = pmod(pmul(base, base, p), m, p); e >>= 1
    return r

# ---- Lattes map phi_2 as homogeneous forms, iterated
def lattes_forms(a, b, n, p):
    X = [0, 1]; Z = [1]
    for _ in range(n):
        def P(*pairs):
            r = []
            for coef, ex, ez in pairs:
                t = [coef % p]
                for _ in range(ex): t = pmul(t, X, p)
                for _ in range(ez): t = pmul(t, Z, p)
                r = padd(r, t, p)
            return r
        NX = P((1, 4, 0), (-2 * a, 2, 2), (-8 * b, 1, 3), (a * a, 0, 4))
        DZ = P((4, 3, 1), (4 * a, 1, 3), (4 * b, 0, 4))
        X, Z = NX, DZ
    return X, Z
def target_poly(X, Z, c, p):
    return padd(X, pscale(Z, (-c) % p, p), p)

# ---- curve arithmetic
def count_points(a, b, p):
    sq = {(y * y) % p for y in range(p)}
    n = 1
    for x in range(p):
        v = (x * x * x + a * x + b) % p
        n += 1 if v == 0 else (2 if v in sq else 0)
    return n
def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0: return None
        l = (3 * P[0] * P[0] + a) * pow(2 * P[1], -1, p) % p
    else:
        l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (l * l - P[0] - Q[0]) % p
    return (x, (l * (P[0] - x) - P[1]) % p)
def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R
def rand_point(a, b, p, rng):
    while True:
        x = rng.randrange(p); v = (x ** 3 + a * x + b) % p
        if v and pow(v, (p - 1) // 2, p) == 1:
            return (x, pow(v, (p + 1) // 4, p))   # p = 3 mod 4
def point_order(P, N, a, p):
    o = N
    for q in factor(N):
        while o % q == 0 and mul(o // q, P, a, p) is None: o //= q
    return o
def rand_prime(lo, hi, rng):
    while True:
        q = rng.randrange(lo, hi) | 1
        if q % 4 == 3 and isprime(q): return q
def rand_curve(lo, hi, rng):
    while True:
        p = rand_prime(lo, hi, rng); a = rng.randrange(p); b = rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p: return p, a, b

def patterns_for_curve(p, a, b, n, P, order, ks, rng_unused=None):
    X, Z = lattes_forms(a, b, n, p)
    pats = []
    for k in ks:
        Q = mul(k, P, a, p)
        if Q is None or Q[1] == 0: continue
        pats.append((k, ddf_pattern(target_poly(X, Z, Q[0], p), p)))
    return pats

def tv(c1, c2):
    keys = set(c1) | set(c2); s1 = sum(c1.values()) or 1; s2 = sum(c2.values()) or 1
    return 0.5 * sum(abs(c1.get(k, 0) / s1 - c2.get(k, 0) / s2) for k in keys)

def main():
    rng = random.Random(20261110)
    lo, hi = (101, 300) if SMOKE else (2000, 4000)
    cfg = {"n1": (6, 2, 2, 3), "n2": (4, 3, 3, 3)} if False else None
    # arms: (n, curves, ks per curve) for null; smaller for n=3 (degree 64)
    if SMOKE:
        plan = [(1, 2, 4), (2, 2, 4)]; pos_plan = [(2, 2, 6)]; rm_plan = 2; rm_k = 6
    else:
        plan = [(1, 12, 60), (2, 12, 60), (3, 3, 5)]; pos_plan = [(2, 10, 60), (3, 2, 6)]
        rm_plan = 10; rm_k = 60
    null_rows = []; dep_null = 0; n_null = 0; nsf_null = 0; xgen_mismatch = 0
    for n, ncurves, nk in plan:
        done = 0
        while done < ncurves and time.time() - T0 < DEADLINE * 0.5:
            p, a, b = rand_curve(lo, hi, rng)
            N = count_points(a, b, p)
            if N % 2 == 0: continue
            P = rand_point(a, b, p, rng); o = point_order(P, N, a, p)
            if o < 20: continue
            ks = [rng.randrange(1, o) for _ in range(nk)]
            pats = patterns_for_curve(p, a, b, n, P, o, ks)
            P2 = mul(rng.randrange(2, o), P, a, p)
            if P2 is None: continue
            pats2 = patterns_for_curve(p, a, b, n, P2, point_order(P2, N, a, p), [1])
            S = {x[1] for x in pats}
            dep = len(S) > 1; dep_null += dep; n_null += 1
            nsf_null += any(x[1][0] == "nsf" for x in pats)
            xgen_mismatch += (pats2 and pats2[0][1] not in S) and not dep
            null_rows.append({"n": n, "p": p, "a": a, "b": b, "N": N, "ord": o,
                              "k_samples": len(pats), "distinct_patterns": len(S),
                              "pattern": S and sorted(S)[0][1][:6]})
            done += 1
    # positive control A: d=2 divides ord(P) with 2^n | ord -> pattern must depend on k
    pos_dep = 0; pos_n = 0; pos_counts_a = {}; pos_counts_b = {}; split_tv = []
    for n, ncurves, nk in pos_plan:
        done = 0; tries = 0
        while done < ncurves and tries < 4000 and time.time() - T0 < DEADLINE * 0.75:
            tries += 1
            p, a, b = rand_curve(lo, hi, rng); N = count_points(a, b, p)
            if N % (2 ** n) or N < 40: continue
            P = rand_point(a, b, p, rng); o = point_order(P, N, a, p)
            if o % (2 ** n): continue
            ks = [rng.randrange(1, o) for _ in range(nk)]
            pats = patterns_for_curve(p, a, b, n, P, o, ks)
            S = {x[1] for x in pats}; pos_dep += len(S) > 1; pos_n += 1; done += 1
            h = len(pats) // 2; c1 = {}; c2 = {}
            for i, (_, pt) in enumerate(pats):
                (c1 if i < h else c2)[pt] = ((c1 if i < h else c2).get(pt, 0)) + 1
            split_tv.append(tv(c1, c2))
    # positive control B: random degree-4 rational map, random targets
    rm_dep = 0; rm_n = 0
    for _ in range(rm_plan):
        p = rand_prime(lo, hi, rng)
        Nn = [rng.randrange(p) for _ in range(5)]; Nn[4] = 1
        Dd = [rng.randrange(p) for _ in range(4)]; Dd[3] = rng.randrange(1, p)
        S = set()
        for _ in range(rm_k):
            c = rng.randrange(p)
            S.add(ddf_pattern(target_poly(trim(Nn[:]), trim(Dd[:]), c, p), p))
        rm_dep += len(S) > 1; rm_n += 1
    pos_ok = pos_n > 0 and pos_dep / pos_n >= 0.8
    rm_ok = rm_n > 0 and rm_dep / rm_n >= 0.9
    aa_ok = (not split_tv) or (sum(split_tv) / len(split_tv) <= 0.35)
    timed_out = time.time() - T0 >= DEADLINE * 0.5
    if not (pos_ok and rm_ok) or n_null == 0 or timed_out:
        verdict = "inconclusive"
    elif dep_null == 0 and xgen_mismatch == 0 and aa_ok:
        verdict = "consistent_with_prediction"
    elif dep_null > 0 or xgen_mismatch > 0:
        verdict = "falsified"
    else:
        verdict = "inconclusive"
    print(json.dumps({
        "lead": "LEAD-EXO-lattes-xline-dynamics", "smoke": SMOKE, "seed": 20261110,
        "params": {"p_range": [lo, hi], "null_plan(n,curves,k)": plan, "pos_plan": pos_plan,
                   "random_map_curves": rm_plan, "random_map_targets": rm_k},
        "statistic": {"null_curves": n_null, "null_curves_with_k_dependence": dep_null,
                      "null_cross_generator_mismatch": int(xgen_mismatch),
                      "null_nonsquarefree_cases": int(nsf_null)},
        "controls": {"positive_dividing_d": {"curves": pos_n, "dependent": pos_dep, "ok": pos_ok},
                     "positive_random_rational_map": {"maps": rm_n, "dependent": rm_dep, "ok": rm_ok},
                     "aa_split_half_tv_mean": (sum(split_tv) / len(split_tv)) if split_tv else None,
                     "aa_ok": aa_ok},
        "null_rows": null_rows[:40], "timed_out": timed_out,
        "elapsed_s": round(time.time() - T0, 2), "verdict": verdict}))
main()
