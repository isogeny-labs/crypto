#!/usr/bin/env python3
"""Haar-model prediction for the auxiliary-curve search of pkm_tower_pilot
(isogeny_with): a curve E' is accepted iff E'(F_p) has a point of exact order
2^t that [#E'/2^t]P reaches, i.e. its 2-Sylow is cyclic of order >= 2^t.
Model: Frobenius F uniform in {F in GL2(Z/2^t): det F = p}; p = 1 mod 2^27,
so for t <= 27 this is SL2(Z/2^t). E'(F_p)[2^inf] = coker(F - I), so
accept <=> tr F = 2 mod 2^t and F != I mod 2.  Exact count; brute force
for t <= 4 (--selftest).  Expected curves tried = 1/P (geometric)."""
import itertools, json, sys
def v2(m, k):
    if m % (1 << k) == 0: return k
    v = 0
    while m % 2 == 0: m //= 2; v += 1
    return v
def N(m, k):  # #(b,c) mod 2^k with bc = m
    if k == 0: return 1
    m %= 1 << k
    v = v2(m, k)
    return (min(v, k - 1) + 1) * (1 << (k - 1)) + ((1 << k) if m == 0 else 0)
def N_even(m, t):  # both b, c even
    m %= 1 << t
    if t <= 2: return (1 << (t - 1)) ** 2 if m == 0 else 0
    if m % 4: return 0
    return N(m // 4, t - 2) * 4
def count(t):
    M = 1 << t; acc = 0
    for a in range(M):
        m = -((a - 1) ** 2)
        acc += N(m, t)
        if a % 2 == 1: acc -= N_even(m, t)  # d = 2 - a odd too: F = I mod 2
    return acc
def order_sl2(t): return 6 * 8 ** (t - 1)
def brute(t):
    M = 1 << t; acc = tot = 0
    for a, b, c, d in itertools.product(range(M), repeat=4):
        if (a * d - b * c) % M != 1: continue
        tot += 1
        if (a + d) % M == 2 % M and not (a % 2 and d % 2 and b % 2 == 0 and c % 2 == 0): acc += 1
    return acc, tot
if __name__ == "__main__":
    if "--selftest" in sys.argv:
        for t in range(1, 5):
            acc, tot = brute(t)
            assert tot == order_sl2(t) and acc == count(t), (t, acc, count(t), tot)
        print("selftest ok t=1..4"); sys.exit()
    rows = []
    for t in range(1, 15):
        P = count(t) / order_sl2(t)
        rows.append({"t": t, "p_accept": P, "expected_curves_tried": 1 / P})
    print(json.dumps(rows, indent=1))
