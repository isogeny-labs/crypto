#!/usr/bin/env python3
"""Memory-capped optimum with double large primes (L' <= L free, B free) vs no-LP, analysis only."""
import sys, math
sys.path.insert(0, "/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/eval2/g1")
import model
from model import *

def theta_fast(b):
    lo, hi = 1.0 + 1e-9, 60.0
    for _ in range(60):
        mid = (lo + hi) / 2
        if cycle_rank_per_node(mid) >= b: hi = mid
        else: lo = mid
    return hi / 2, hi

def lp2_point(lgc, lgB, lgLp, w, u=4.0):
    b = 2.0 ** (lgB - lgLp)
    if b >= 0.4: return None
    th, d = theta_fast(b)
    P2 = 10 * b**3 * (1 - b)**2
    edges = th * 2.0 ** lgLp
    attempts = 120 * edges / P2 / (2.0 ** (lgLp - LGL)) ** 5
    R = lg(attempts) + lgc + lg(u)
    LA = lg(kappa(w)) + 2 * lgB
    mem = max(edges * 40, la_mem_bytes(2.0 ** lgB, w))
    return lg_add(R, LA), R, LA, lg(mem)

def best_under_cap(lgc, lgcap, w):
    best = (1e9,)
    for lgLp in [x * 0.5 for x in range(60, 127)]:           # 30 .. 63
        if lgLp > LGL: continue
        for lgB in [x * 0.5 for x in range(40, 127)]:
            if lgB >= lgLp - 1.3: continue
            pt = lp2_point(lgc, lgB, lgLp, w)
            if pt is None or pt[3] > lgcap: continue
            if pt[0] < best[0]: best = (pt[0], lgB, lgLp, pt[3])
    return best

for lgc in (40, 50):
    print("log2 c =", lgc, " rho_words 2^%.1f" % rho_words())
    for lgcap in (50, 54, 57, 60, 64, 68):
        nl = subset_nolp(lgc, cap_lgB=lgcap - lg(280))
        row = "  cap 2^%d B: no-LP 2^%.1f (B=2^%.1f)" % (lgcap, nl[0], nl[1])
        for w in (15, 150):
            b = best_under_cap(lgc, lgcap, w)
            row += " | LP2 w=%d: 2^%.1f (B=2^%.1f, L'=2^%.1f)" % (w, b[0], b[1], b[2]) if b[0] < 1e8 else " | LP2 w=%d: n/a" % w
        print(row)
