"""Closed-form break-even table for an algebraic decomposition oracle (no measurement).

Model (research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md s3.6-3.7, PKM 2016 s3.2):
  total cost  ~ m! p T / B^(m-1) + B^2,  per-query cost T = p^gamma.
  Optimum B = p^b with b = (1+gamma)/(m+1); total p^(2(1+gamma)/(m+1)) < p^(1/2)
  iff gamma < (m-3)/4, and then b = 1/4 exactly (B = p^(1/4)).
Tower unknowns N = m*t = m*log2(B) = (m/4)*log2(p) at break-even.
If the solving degree is D = beta*N, the Macaulay width is 2^(H(beta)*N), and the
per-query cost is width^w (w = elimination exponent, 1 = one touch per column).
  gamma = w * H(beta) * m/4  <  (m-3)/4   <=>   H(beta) < (m-3)/(m*w).
Prints the largest beta that still breaks even. Polynomial prefactors are ignored.
"""
import math

def H(x):
    return 0.0 if x <= 0 or x >= 1 else -x*math.log2(x) - (1-x)*math.log2(1-x)

def beta_star(target):
    lo, hi = 0.0, 0.5
    for _ in range(80):
        mid = (lo + hi) / 2
        if H(mid) < target:
            lo = mid
        else:
            hi = mid
    return lo

print("m  w    gamma_max=(m-3)/4  H(beta*)=(m-3)/(m w)  beta*=max D/N")
for m in (3, 4, 5, 6):
    for w in (1.0, 2.0, 2.25):
        g = (m - 3) / 4
        t = (m - 3) / (m * w)
        b = beta_star(t) if t > 0 else float("nan")
        print(f"{m}  {w:<4} {g:<18.3f} {t:<21.4f} {b:.4f}")
