#!/usr/bin/env python3
"""Toy equidistribution test of ell(k)=log_g Nm(f_{k,P}(S)) in Z/(p-1). stdlib only."""
import json, random, sys, time

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def factor(n):
    f = []; i = 2
    while i * i <= n:
        if n % i == 0:
            f.append(i)
            while n % i == 0: n //= i
        i += 1
    if n > 1: f.append(n)
    return f

def prim_root(p):
    fs = factor(p - 1)
    for g in range(2, p):
        if all(pow(g, (p - 1) // q, p) != 1 for q in fs): return g

def make_curve(p, rng):
    """random y^2=x^3+ax+b over F_p (p=3 mod 4) with prime order N, gcd(N,p-1)=1."""
    sq = [0] * p
    for y in range(p): sq[y * y % p] = 1
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        n = 1 + sum(1 + (1 if sq[(x ** 3 + a * x + b) % p] else -1) if (x ** 3 + a * x + b) % p else 1 for x in range(p))
        # n counts infinity + points
        if is_prime(n) and (p - 1) % n != 0 and all((p - 1) % n for _ in [0]) and __import__('math').gcd(n, p - 1) == 1:
            return a, b, n

def sqrt_mod(v, p): return pow(v, (p + 1) // 4, p)

def point_P(p, a, b, rng):
    while True:
        x = rng.randrange(p); r = (x ** 3 + a * x + b) % p
        y = sqrt_mod(r, p)
        if y * y % p == r and y: return x, y

def ell_sequence(p, a, b, N, rng):
    """ell[k-1]=log_g Nm f_{k,P}(S), k=1..N-1, S=(xs, c*i) on the twist, i^2=-1."""
    g = prim_root(p)
    dl = {}; v = 1
    for e in range(p - 1):
        dl[v] = e; v = v * g % p
    P = point_P(p, a, b, rng)
    while True:
        xs = rng.randrange(p); r = (xs ** 3 + a * xs + b) % p
        if r and pow(r, (p - 1) // 2, p) == p - 1:
            c = sqrt_mod(-r % p, p)
            if c * c % p == (-r) % p: break
    # f as (u,w) = u + w i in F_p^2
    fu, fw = 1, 0
    T = P; out = []
    xp, yp = P
    for k in range(1, N - 1):
        xt, yt = T
        if xt == xp:
            if (yt + yp) % p == 0: break
            m = (3 * xt * xt + a) * pow(2 * yt, -1, p) % p
        else:
            m = (yp - yt) * pow(xp - xt, -1, p) % p
        x3 = (m * m - xt - xp) % p
        y3 = (m * (xt - x3) - yt) % p
        # l(S) = (c i) - yt - m(xs-xt); v(S) = xs - x3
        la = (-yt - m * (xs - xt)) % p; lb = c
        vv = (xs - x3) % p
        if vv == 0 or (la * la + lb * lb) % p == 0: return None
        iv = pow(vv, -1, p)
        nu = (fu * la - fw * lb) % p; nw = (fu * lb + fw * la) % p
        fu, fw = nu * iv % p, nw * iv % p
        T = (x3, y3)
        out.append(dl[(fu * fu + fw * fw) % p])
    return out

def chi(counts, n, q):
    e = n / q
    s = sum((c - e) ** 2 / e for c in counts)
    df = q - 1
    return (s - df) / (2 * df) ** 0.5

def cnt(vals, q):
    c = [0] * q
    for v in vals: c[v % q] += 1
    return c

def stat(L, qs):
    """max standardized chi2 over: marginal, increment, doubling-defect, for each q."""
    n = len(L); best = -1e9; det = {}
    for q in qs:
        z1 = chi(cnt(L, q), n, q)
        z2 = chi(cnt([L[i + 1] - L[i] for i in range(n - 1)], q), n - 1, q)
        h = n // 2
        z3 = chi(cnt([L[2 * i - 1] - 2 * L[i - 1] for i in range(1, h)], q), h - 1, q)
        for nm, z in (("marg", z1), ("incr", z2), ("dbl", z3)):
            det["%s_q%d" % (nm, q)] = round(z, 3)
            best = max(best, z)
    return best, det

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    primes_lo, primes_hi = (400, 700) if smoke else (6000, 12000)
    ncurves = 1 if smoke else 6
    reps = 20 if smoke else 150
    rng = random.Random(20261110)
    res = []; null_all = []; pos_all = []; aa = []
    ps = [p for p in range(primes_lo, primes_hi) if p % 4 == 3 and is_prime(p)]
    rng.shuffle(ps)
    for p in ps:
        if len(res) >= ncurves: break
        a, b, N = make_curve(p, rng)
        L = ell_sequence(p, a, b, N, random.Random(p))
        if L is None or len(L) < 100: continue
        M = p - 1
        qs = [q for q in (3, 4, 5, 7, 8, 9, 11, 13, 16) if M % q == 0] or [2]
        s, det = stat(L, qs)
        nr = random.Random(1000 + p)
        nulls = sorted(stat([nr.randrange(M) for _ in L], qs)[0] for _ in range(reps))
        aa_s = stat([nr.randrange(M) for _ in L], qs)[0]
        cst = random.Random(7 + p).randrange(1, M)
        pr = random.Random(3 + p)
        planted = [(cst * k + (pr.randrange(M) if pr.random() < 0.3 else 0)) % M for k in range(1, len(L) + 1)]
        ps_ = stat(planted, qs)[0]
        q99 = nulls[int(0.99 * (reps - 1))]
        res.append({"p": p, "a": a, "b": b, "N": N, "n": len(L), "qs": qs, "T_real": round(s, 3), "detail": det,
                    "null_q99": round(q99, 3), "null_max": round(nulls[-1], 3), "T_planted": round(ps_, 3),
                    "T_null_AA": round(aa_s, 3)})
        null_all.append(q99); pos_all.append(ps_ > nulls[-1] and ps_ > 5); aa.append(aa_s <= nulls[-1])
    ok_pos = bool(res) and all(pos_all)
    ok_aa = bool(res) and all(aa)
    if not res or not ok_pos or not ok_aa:
        verdict = "inconclusive"
    elif any(r["T_real"] > r["null_max"] and r["T_real"] > 5 for r in res):
        verdict = "falsified"
    elif all(r["T_real"] <= r["null_q99"] for r in res):
        verdict = "consistent_with_prediction"
    else:
        verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-TRF-miller-cocycle-nonhom-transfer", "smoke": smoke, "seed": 20261110,
                      "params": {"p_range": [primes_lo, primes_hi], "curves": len(res), "null_reps": reps},
                      "statistic": "max standardized chi2 over marginal/increment/doubling tests",
                      "positive_control_detected": ok_pos, "AA_within_null": ok_aa, "curves": res,
                      "verdict": verdict, "elapsed_s": round(time.time() - t0, 2)}))

if __name__ == "__main__":
    main()
