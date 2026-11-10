#!/usr/bin/env python3
"""Digit-character leakage of x(kG) vs additive characters of k. stdlib only."""
import sys, json, math, cmath, random, time

def fft(a, inv=False):
    n = len(a); j = 0
    for i in range(1, n):
        bit = n >> 1
        while j & bit:
            j ^= bit; bit >>= 1
        j ^= bit
        if i < j: a[i], a[j] = a[j], a[i]
    L = 2
    while L <= n:
        ang = 2 * math.pi / L * (1 if inv else -1)
        wl = [cmath.exp(1j * ang * t) for t in range(L // 2)]
        for s in range(0, n, L):
            h = L // 2
            for t in range(h):
                u = a[s + t]; v = a[s + t + h] * wl[t]
                a[s + t] = u + v; a[s + t + h] = u - v
        L <<= 1
    if inv:
        a[:] = [x / n for x in a]

class Blu:
    def __init__(s, N):
        s.N = N; M = 1
        while M < 2 * N: M <<= 1
        s.M = M
        s.w = [cmath.exp(-1j * math.pi * ((k * k) % (2 * N)) / N) for k in range(N)]
        b = [0j] * M
        for k in range(-(N - 1), N):
            b[k % M] = cmath.exp(1j * math.pi * ((k * k) % (2 * N)) / N)
        fft(b); s.B = b
    def dft(s, x):
        N, M = s.N, s.M
        a = [x[k] * s.w[k] for k in range(N)] + [0j] * (M - N)
        fft(a); a = [p * q for p, q in zip(a, s.B)]; fft(a, True)
        return [s.w[m] * a[m] for m in range(N)]

def find_curve(p, rng):
    sq = [0] * p
    for y in range(p): sq[y * y % p] += 1
    while True:
        a, b = rng.randrange(p), rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        n = 1 + sum(sq[(x ** 3 + a * x + b) % p] for x in range(p))
        if n < 100 or any(n % q == 0 for q in range(2, int(n ** .5) + 1)): continue
        for x in range(p):
            r = (x ** 3 + a * x + b) % p
            for y in range(p):
                if y * y % p == r:
                    return a, b, n, (x, y)
            # (slow search only until first point; p small)

def xs(p, a, G, n):
    out = [None]; P = G
    for k in range(1, n):
        out.append(P[0])
        if k == n - 1: break
        if P[0] == G[0]:
            l = (3 * P[0] ** 2 + a) * pow(2 * P[1], -1, p) % p
        else:
            l = (G[1] - P[1]) * pow(G[0] - P[0], -1, p) % p
        x3 = (l * l - P[0] - G[0]) % p
        P = (x3, (l * (P[0] - x3) - P[1]) % p)
    return out

def digit_chars(p, nb2, nb3):
    ch = []
    for j in range(nb2): ch.append(("b2d%d" % j, lambda x, j=j: 1.0 if (x >> j) & 1 else -1.0))
    for j in range(nb3):
        ch.append(("b3d%d" % j, lambda x, j=j: cmath.exp(2j * math.pi * ((x // 3 ** j) % 3) / 3)))
    return ch

def stat(F, n, chars, xv):
    """xv: x label per k (1..n-1). T = max over digits,m of |S|/norm."""
    best = 0.0
    for name, f in chars:
        v = [0j] + [complex(f(xv[k])) for k in range(1, n)]
        mu = sum(v) / (n - 1)
        v = [0j] + [z - mu for z in v[1:]]
        nrm = math.sqrt(sum(abs(z) ** 2 for z in v)) or 1.0
        S = F.dft(v)
        t = max(abs(S[m]) for m in range(1, n // 2 + 1)) / nrm
        best = max(best, t)
    return best

def null_label(xv, n, rng):
    h = (n - 1) // 2; vals = [xv[k] for k in range(1, h + 1)]; rng.shuffle(vals)
    out = [None] * n
    for k in range(1, h + 1): out[k] = out[n - k] = vals[k - 1]
    return out

def planted(xv, n, p, rng):
    h = (n - 1) // 2; out = [None] * n
    for k in range(1, h + 1):
        bit = math.cos(2 * math.pi * 7 * k / n) > 0
        if rng.random() < 0.2: bit = not bit
        x = (xv[k] & ~1) | (1 if bit else 0)
        out[k] = out[n - k] = x
    return out

def pval(t, nulls): return (1 + sum(1 for u in nulls if u >= t)) / (len(nulls) + 1)

def main():
    smoke = "--smoke" in sys.argv
    primes = [211] if smoke else [2053, 2999, 4001]
    R = 6 if smoke else 40
    nb2, nb3 = (3, 1) if smoke else (11, 4)
    seed = 20261110; rng = random.Random(seed)
    t0 = time.time(); rows = []
    for p in primes:
        a, b, n, G = find_curve(p, rng)
        F = Blu(n); chars = digit_chars(p, nb2, nb3)
        xv = xs(p, a, G, n)
        T = stat(F, n, chars, xv)
        nulls = [stat(F, n, chars, null_label(xv, n, rng)) for _ in range(R)]
        aa = nulls[0]; aap = pval(aa, nulls[1:])
        Tpos = stat(F, n, chars, planted(xv, n, p, rng))
        rows.append(dict(p=p, a=a, b=b, n=n, T=T, p_real=pval(T, nulls), T_pos=Tpos,
                         p_pos=pval(Tpos, nulls), p_AA=aap, null_mean=sum(nulls) / R, null_max=max(nulls)))
    C = len(rows); alpha = 0.05 / C
    pos_ok = all(r["p_pos"] <= 0.05 for r in rows)
    aa_ok = all(r["p_AA"] > 0.01 for r in rows)
    if not pos_ok or not aa_ok: v = "inconclusive"
    elif min(r["p_real"] for r in rows) <= alpha: v = "falsified"
    else: v = "consistent_with_prediction"
    print(json.dumps(dict(lead="LEAD-EXO-coordinate-statistic-leakage", smoke=smoke, seed=seed,
        params=dict(primes=primes, null_draws=R, digits_b2=nb2, digits_b3=nb3, alpha_bonferroni=alpha),
        rows=rows, positive_control_ok=pos_ok, aa_ok=aa_ok, seconds=round(time.time() - t0, 1), verdict=v)))
main()
