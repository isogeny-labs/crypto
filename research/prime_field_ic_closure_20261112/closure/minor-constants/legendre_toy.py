#!/usr/bin/env python3
"""Toy, [pcc:unverified]: rejection power of a Legendre pre-filter on the
incumbent m=2 pair-table probe.  Build the low-x ("prime-abscissa") base of
size N, the negation-folded pair-sum keys x(Pi +/- Pj), and measure
  - the fraction of keys with chi(x)=+1 (a chi filter can only reject
    candidates without losing hits if this is 0 or 1),
  - over random walk-like targets: hits found with and without a chi=+1
    filter, and probes saved.
Also the counted F_p multiplications of Euler's criterion vs Fermat inversion.
"""
import random, sys

def legendre(a, p):
    a %= p
    return 0 if a == 0 else (1 if pow(a, (p - 1) // 2, p) == 1 else -1)

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def sqrt_mod(n, p):  # p = 3 mod 4
    r = pow(n, (p + 1) // 4, p)
    return r if r * r % p == n % p else None

def run(p, a, b, N, trials, seed):
    base, x = [], 0
    while len(base) < N:
        y = sqrt_mod((x ** 3 + a * x + b) % p, p)
        if y is not None and y != 0:
            base.append((x, y))
        x += 1
    keys = set()
    for i in range(N):
        for j in range(i, N):
            for Q in (base[j], (base[j][0], (-base[j][1]) % p)):
                S = add(base[i], Q, a, p)
                if S is not None:
                    keys.add(S[0])
    plus = sum(1 for k in keys if legendre(k, p) == 1)
    rng = random.Random(seed)
    # targets: random points (walk outputs are uniformly distributed points)
    hits = hits_f = probes_f = 0
    t = 0
    while t < trials:
        xr = rng.randrange(p)
        if sqrt_mod((xr ** 3 + a * xr + b) % p, p) is None:
            continue
        t += 1
        ch = legendre(xr, p)
        inkey = xr in keys
        hits += inkey
        if ch == 1:
            probes_f += 1
            hits_f += inkey
    return dict(p=p, N=N, keys=len(keys), key_frac_chi_plus=plus / len(keys),
                trials=trials, hits=hits, hits_filtered=hits_f, probes_filtered=probes_f,
                hits_kept=hits_f / hits if hits else float("nan"),
                probes_kept=probes_f / trials)

def mulcount(e):  # left-to-right square-and-multiply
    return (e.bit_length() - 1) + (bin(e).count("1") - 1)

if __name__ == "__main__":
    print("p,N,|keys|,frac keys chi=+1,trials,hits,hits with chi filter,probes with filter,hits kept,probes kept")
    for (p, N, trials) in [(1048571, 45, 400000), (16777199, 115, 400000), (16777199, 162, 400000)]:
        r = run(p, p - 3, 31, N, trials, 7)
        print(",".join(str(round(v, 4)) if isinstance(v, float) else str(v) for v in r.values()))
    print()
    print("p,bits,Euler (p-1)/2 mults,Fermat p-2 mults,ratio")
    for p in (1048571, 16777199, 268435399, 4294967291):
        e, f = mulcount((p - 1) // 2), mulcount(p - 2)
        print(f"{p},{p.bit_length()},{e},{f},{e / f:.3f}")
