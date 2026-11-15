#!/usr/bin/env python3
"""Toy test of the Tate obstruction for LEAD-TRF-prime-field-isogeny-to-weak. stdlib only."""
import json, random, sys

def primes_in(lo, hi):
    return [n for n in range(lo, hi) if n > 3 and all(n % q for q in range(2, int(n**.5) + 1))]

def lpf(n):
    f, m, q = 1, n, 2
    while q * q <= m:
        while m % q == 0:
            f, m = q, m // q
        q += 1
    return max(f, m) if m > 1 else f

def count(p, a, b, sq):
    # sq[v]: Legendre-style table (+1/-1/0) of v mod p
    return p + 1 + sum(sq[(x * x * x + a * x + b) % p] for x in range(p))

def weak(p, N):
    l = lpf(N)
    anom = (N == p)
    mov = any(pow(p, k, l) == 1 for k in range(1, 7)) and l > 3
    return anom, mov

def two_neighbors(p, a, b):
    out = []
    for x0 in range(p):
        if (x0**3 + a * x0 + b) % p == 0:
            t = (3 * x0 * x0 + a) % p
            w = x0 * t % p
            A, B = (a - 5 * t) % p, (b - 7 * w) % p
            if (4 * A**3 + 27 * B * B) % p:
                out.append((A, B))
    return out

def trial(seed, plist, ncurves, steps):
    rnd = random.Random(seed)
    pairs = bad_order = bad_weak = anom_seen = mov_seen = 0
    ctrl_pairs = ctrl_diff = 0
    for p in plist:
        sq = [0] * p
        for v in range(1, p):
            sq[v * v % p] = 1
        for v in range(1, p):
            if sq[v] == 0:
                sq[v] = -1
        for _ in range(ncurves):
            while True:
                a, b = rnd.randrange(p), rnd.randrange(p)
                if (4 * a**3 + 27 * b * b) % p:
                    break
            N0 = count(p, a, b, sq)
            w0 = weak(p, N0)
            anom_seen += w0[0]; mov_seen += w0[1]
            cur = (a, b)
            for _ in range(steps):
                nb = two_neighbors(p, *cur)
                if not nb:
                    break
                cur = rnd.choice(nb)
                N1 = count(p, cur[0], cur[1], sq)
                pairs += 1
                bad_order += (N1 != N0)
                bad_weak += (weak(p, N1) != w0)
            # positive control: independent random curve, same p
            while True:
                a2, b2 = rnd.randrange(p), rnd.randrange(p)
                if (4 * a2**3 + 27 * b2 * b2) % p:
                    break
            ctrl_pairs += 1
            ctrl_diff += (count(p, a2, b2, sq) != N0)
    return dict(isogenous_pairs=pairs, order_mismatch=bad_order, weak_status_mismatch=bad_weak,
                curves_anomalous=anom_seen, curves_mov_le6=mov_seen,
                control_pairs=ctrl_pairs, control_order_differs=ctrl_diff)

def main():
    smoke = "--smoke" in sys.argv
    if smoke:
        plist, nc, st = primes_in(100, 130), 5, 2
    else:
        plist, nc, st = primes_in(1000, 1300)[::6][:12], 150, 3
    a = trial(20261110, plist, nc, st)
    b = trial(20261111, plist, nc, st)   # A/A arm: same instrument, different seed
    ok = (a["order_mismatch"] == 0 and b["order_mismatch"] == 0
          and a["weak_status_mismatch"] == 0 and b["weak_status_mismatch"] == 0)
    pos = a["control_order_differs"] >= 0.5 * a["control_pairs"]
    if ok and pos and (a["curves_anomalous"] + a["curves_mov_le6"] > 0 or smoke):
        v = "consistent_with_prediction"
    elif not ok:
        v = "falsified"
    else:
        v = "inconclusive"
    print(json.dumps(dict(lead="LEAD-TRF-prime-field-isogeny-to-weak", smoke=smoke, primes=plist,
        curves_per_prime=nc, walk_steps=st, seeds=[20261110, 20261111], arm_main=a, arm_AA=b,
        positive_control_detects=pos, verdict=v)))

main()
