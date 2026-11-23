"""Asymptotic exponent floor implied by Theorem C' for binary towers.  [pcc:unverified]
E >= c |B| H(k-m) max(1, n/|B|^m)/2, |B| <= 2^{k+1}, log2 H(k-m) = mk h2(1/m) (1 - o(1)) as k -> inf.
With |B| = n^beta:  log_n E >= g(beta) = beta (1 + m h2(1/m)) + max(0, 1 - m beta)  (k -> inf, beta fixed > 0).
Exponent floor e_m = inf_{beta > 0} g(beta) = 1 if 1 + m h2(1/m) >= m, else 1/m + h2(1/m).
Also a finite-size check: exact log2 H(k-m) at k = round(beta log2 n) for n = 2^256 at beta = 1/m."""
import math, json
from math import log2
def h2(x): return -x*log2(x) - (1-x)*log2(1-x)
def logH(m, k, t):
    N = m*k; c = 1; s = 1
    for j in range(1, t+1):
        c = c*(N-j+1)//j; s += c
    return math.log2(s)
for m in range(3, 21):
    h = h2(1/m)
    e = 1.0 if 1 + m*h >= m else 1/m + h
    # finite size: n = 2^256, |B| = 2^{k+1} with |B|^m = n  ->  k = 256/m - 1
    k = round(256/m - 1)
    fin = None
    if k > m:
        lb = (k + 1) + logH(m, k, k - m) - 1          # log2(|B| H / 2) at |B|^m = n
        fin = round(lb / 256, 3)
    print(json.dumps({"m": m, "e_m_asymptotic": round(e, 4), "beats_half": e >= 0.5,
                      "finite_n2^256_beta1/m": fin, "k_at_256": k}))

# ---- finite n: min over k >= m and |B| <= 2^{k+1} of log2( |B| * H(k-m) / 2 * max(1, n/|B|^m) ), kappa = c = 1
def finite_floor(m, L):           # L = log2 n
    best = None
    for k in range(m, 4 * L):
        lH = logH(m, k, k - m)
        lB = min(k + 1, L / m)    # worst |B| for this k (see note)
        val = lB + lH - 1 + max(0.0, L - m * lB)
        if best is None or val < best[0]: best = (val, k)
    return best
for L in (128, 256):
    for m in range(3, 13):
        v, k = finite_floor(m, L)
        print(json.dumps({"log2n": L, "m": m, "floor_log2_cost": round(v, 1), "floor_over_log2n": round(v / L, 3),
                          "argmin_k": k, "beats_sqrt_n": v >= L / 2}))
