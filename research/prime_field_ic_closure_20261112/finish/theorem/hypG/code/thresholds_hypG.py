"""Thresholds for the proven tower floor vs Corollary C2.  [pcc:unverified]
tau_m: h2(tau) = 1/2 - 1/m.  kmin_amended: least k with H(k-m) >= 2|B|^{m/2-1}; kmin_original: >= 4|B|^{m/2-1}.  Proven all-target floor: delta_weak, delta_det >= k-m+1, so every target is
good' at t = k - m, with per-attempt cost >= H(k-m), H(t) = sum_{j<=t} C(mk, j).
C1 (original constant): H(t) >= 4 kappa |B|^{m/2-1}; amended (Thm C'): H(t) >= kappa |B|^{m/2-1}; |B| <= 2^{k+1}."""
import math, json
from math import comb, log2
def h2(x): return 0 if x in (0, 1) else -x*log2(x) - (1-x)*log2(1-x)
def tau(m):
    lo, hi = 0.0, 0.5
    for _ in range(100):
        mid = (lo+hi)/2
        if h2(mid) < 0.5 - 1/m: lo = mid
        else: hi = mid
    return lo
def H(m, k, t):
    N = m*k; c = 1; s = 1 if t >= 0 else 0
    for j in range(1, t+1):
        c = c*(N-j+1)//j; s += c
    return s
if __name__ == '__main__':
    rows = []
    for m in range(3, 16):
        tm = tau(m)
        asym = h2(1/m) > 0.5 - 1/m
        kmin_orig = kmin_am = None
        for k in range(m, 1200):
            L = math.log2(H(m, k, k - m)); R = (k+1)*(m/2-1)
            if kmin_am is None and L >= R + 1: kmin_am = k   # H(k-m) >= 2|B|^{m/2-1} (Thm C', kappa=1)
            if kmin_orig is None and L >= R + 2: kmin_orig = k
            if kmin_am and kmin_orig: break
        rows.append({"m": m, "tau_m": round(tm, 4), "one_over_m": round(1/m, 4), "h2(1/m)": round(h2(1/m), 4),
                     "1/2-1/m": round(0.5-1/m, 4), "asymptotic_floor_suffices": asym,
                     "kmin_amended_kappa1": kmin_am, "kmin_original_C1_kappa1": kmin_orig})
        print(json.dumps(rows[-1]))
