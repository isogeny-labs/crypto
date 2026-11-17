"""Degree thresholds of Corollary C and constants of Theorem B (exact arithmetic).  [pcc:unverified]"""
import math, json
from math import comb, log2
def h2(x): return 0.0 if x <= 0 or x >= 1 else -x*log2(x) - (1-x)*log2(1-x)
def inv_h2(y):
    lo, hi = 0.0, 0.5
    for _ in range(100):
        mid = (lo+hi)/2
        if h2(mid) < y: lo = mid
        else: hi = mid
    return hi
out = {"binary_tower_tau_m": {}, "full_XL_alpha_m": {}, "F4_alpha_m_upper_route": {}}
for m in list(range(3, 11)) + [20, 50, 1000]:
    out["binary_tower_tau_m"][m] = round(inv_h2(0.5 - 1.0/m), 5)
    # unreduced XL in N = m k variables: log2 C(N + aN, N) / k ~ m (1+a) h2(a/(1+a)) >= m/2 - 1
    lo, hi = 0.0, 1.0
    for _ in range(100):
        a = (lo+hi)/2
        val = m*(1+a)*h2(a/(1+a))
        if val < m/2 - 1: lo = a
        else: hi = a
    out["full_XL_alpha_m"][m] = round(hi, 5)
# finite-k check of the tower threshold: least t with sum_{j<=t} C(mk, j) >= 2^{k(m/2-1)}
fin = {}
for m in (3, 4, 6, 8):
    row = {}
    for k in (8, 16, 32, 64, 128):
        target = 2 ** (k*(m/2 - 1))
        s = 0; t = -1
        while s < target:
            t += 1; s += comb(m*k, t)
        row[k] = {"t": t, "t_over_mk": round(t/(m*k), 4)}
    fin[m] = row
out["finite_k_tower_threshold"] = fin
# Theorem B constant: per-relation lower bound / n for univariate base, |B| = 2d (worst case)
dS = {2: 4, 3: 12, 4: 32}
tb = {}
for m in (2, 3, 4):
    row = {}
    for d in (2**8, 2**12, 2**16, 2**20):
        K = comb(m + d - dS[m], m) - 2**(m-1) * d**(m-1)
        Ndec = comb(2*d + m - 1, m)
        row[d] = K / Ndec
    tb[m] = {"per_relation_over_n": row, "limit_2^-m": 2.0**(-m)}
out["theoremB_constant"] = tb
print(json.dumps(out, indent=1))
