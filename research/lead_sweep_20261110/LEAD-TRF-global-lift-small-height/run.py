#!/usr/bin/env python3
"""Toy xedni-style lift test: how often do two small-height rational points on a
common curve y^2=x^3+Ax+B (A,B in Q fixed by the points) satisfy a*P1=+-b*P2,
as a function of the integer coordinate bound H. Stdlib only, deterministic."""
import sys, json, random, math
from fractions import Fraction as F

def add(P, Q, A):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if y1 != y2 or y1 == 0: return None
        l = (3*x1*x1 + A) / (2*y1)
    else:
        l = (y2 - y1) / (x2 - x1)
    x3 = l*l - x1 - x2
    return (x3, l*(x1 - x3) - y1)

def mults(P, A, K):
    out = [None]; cur = None
    for _ in range(K):
        cur = add(cur, P, A); out.append(cur)
    return out

def sample(rng, H):
    while True:
        x1, x2 = rng.randint(-H, H), rng.randint(-H, H)
        y1, y2 = rng.randint(1, H), rng.randint(1, H)
        if x1 != x2 and (x1, y1) != (x2, y2): break
    A = F((y1*y1 - x1**3) - (y2*y2 - x2**3), x1 - x2)
    B = y1*y1 - x1**3 - A*x1
    if 4*A**3 + 27*B*B == 0: return None
    return (F(x1), F(y1)), (F(x2), F(y2)), A

def classify(P1, P2, A, K=6):
    """'torsion' if either point has order<=K, 'dep' if a*P1=+-b*P2 nontrivially, else 'indep'."""
    m1, m2 = mults(P1, A, K), mults(P2, A, K)
    if any(m is None for m in m1[1:]) or any(m is None for m in m2[1:]): return "torsion"
    for a in range(1, K+1):
        for b in range(1, K+1):
            u, v = m1[a], m2[b]
            if u[0] == v[0]: return "dep"
    return "indep"

def rates(seed, H, n, K=6):
    rng = random.Random(seed); c = {"torsion": 0, "dep": 0, "indep": 0}; used = 0
    while used < n:
        s = sample(rng, H)
        if s is None: continue
        used += 1; c[classify(*s, K=K)] += 1
    return c

def planted(seed, n, H, K=6):
    """positive control: P2 = k*P1 planted on a curve through P1; detector must say dep/torsion."""
    rng = random.Random(seed); hit = 0
    for _ in range(n):
        while True:
            x, y = rng.randint(-H, H), rng.randint(1, H)
            A = F(rng.randint(-H, H)); B = y*y - x**3 - A*x
            if 4*A**3 + 27*B*B != 0: break
        P1 = (F(x), F(y)); k = rng.randint(2, 4)
        P2 = mults(P1, A, k)[k]
        if P2 is None: hit += 1; continue
        if classify(P1, P2, A, K) != "indep": hit += 1
    return hit / n

def slope(Hs, rs):
    pts = [(math.log(h), math.log(r)) for h, r in zip(Hs, rs) if r > 0]
    if len(pts) < 2: return None
    mx = sum(p[0] for p in pts)/len(pts); my = sum(p[1] for p in pts)/len(pts)
    return sum((p[0]-mx)*(p[1]-my) for p in pts) / sum((p[0]-mx)**2 for p in pts)

def main():
    smoke = "--smoke" in sys.argv
    Hs = [6, 12] if smoke else [8, 16, 32, 64, 128]
    n = 150 if smoke else 4000
    seeds = (101, 202)  # two independent seeds = A/A arms
    arm = {}
    for s in seeds:
        arm[s] = {H: rates(s*1000+H, H, n) for H in Hs}
    # null control: random pairs of points on DIFFERENT curves are not a lift; use
    # P2 with y2 sign/coordinates reshuffled -> classify on curve of P1 only is invalid, so the
    # null here is the planted-free baseline: dependence among y-flipped duplicates is excluded
    # by construction; we instead report the A/A discrepancy as the noise floor.
    dep = {H: sum(arm[s][H]["dep"] + arm[s][H]["torsion"] for s in seeds) for H in Hs}
    tot = n * len(seeds)
    r = [dep[H]/tot for H in Hs]
    sl = slope(Hs, r)
    aa = max(abs(arm[seeds[0]][H]["dep"] + arm[seeds[0]][H]["torsion"] - arm[seeds[1]][H]["dep"] - arm[seeds[1]][H]["torsion"]) for H in Hs)
    pos = planted(7, 40 if smoke else 400, 10)
    verdict = "inconclusive"
    if pos >= 0.95 and dep[Hs[0]] >= 20:
        # prediction: dependence rate ~ H^-alpha with alpha >= 1/2 (slope <= -0.5)
        if sl is not None and sl <= -0.5: verdict = "consistent_with_prediction"
        elif sl is not None and sl > -0.5 and dep[Hs[-1]] >= 10: verdict = "falsified"
    out = {"lead": "LEAD-TRF-global-lift-small-height", "smoke": smoke, "H_values": Hs,
           "n_per_H_per_seed": n, "seeds": list(seeds), "K": 6,
           "dep_or_torsion_counts": dep, "rates": r, "loglog_slope": sl,
           "aa_max_count_diff": aa, "positive_control_detect_rate": pos,
           "threshold_slope": -0.5, "verdict": verdict}
    print(json.dumps(out)); return 0

if __name__ == "__main__":
    sys.exit(main())
