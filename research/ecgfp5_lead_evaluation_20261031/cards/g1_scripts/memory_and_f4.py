#!/usr/bin/env python3
"""Memory at the optimum, memory-capped optimum, and F4 RAM extrapolation (analysis only)."""
import math, sys
sys.path.insert(0, "/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/eval2/g1")
from model import *

print("== memory at the unconstrained optimum (bytes) ==")
for lgc in [40, 44, 50, 60]:
    n0, b0, r0, l0 = subset_nolp(lgc)
    mem = la_mem_bytes(2.0 ** b0, 5)
    relmem = 2.0 ** b0 * 40
    print("log2c=%d nolp: B=2^%.1f  LA mem 2^%.1f B (%.3g B); relation storage = matrix = 2^%.1f B; total time 2^%.1f" %
          (lgc, b0, lg(mem), mem, lg(relmem), n0))
    o1 = lp1(lgc)
    print("          lp1: B=2^%.1f  1-LP relations stored M1=2^%.1f x 40 B = 2^%.1f B ; LA(w=8) mem 2^%.1f B; time 2^%.1f" %
          (o1[1], o1[4], o1[4] + lg(40), lg(la_mem_bytes(2.0 ** o1[1], 8)), o1[0]))
print("rho DP memory: 2^40 stored points x 40 B = 2^%.1f B ; walkers share nothing" % (40 + lg(40)))

print("\n== memory-capped optimum (no LP; B <= cap/280) vs rho_words 2^%.1f ==" % rho_words())
for lgc in [40, 50, 60]:
    print(" log2 c =", lgc)
    for lgcap in [40, 45, 50, 55, 57, 60, 62, 65, 68]:
        Bmax = lgcap - lg(280)
        v = subset_nolp(lgc, cap_lgB=Bmax)
        print("   mem cap 2^%d B (%.3g B): B=2^%.1f time 2^%.1f  %s rho" % (lgcap, 2.0 ** lgcap, v[1], v[0],
              "<" if v[0] < rho_words() else ">"))
    # break-even memory vs rho
    lo, hi = 30.0, 75.0
    for _ in range(60):
        mid = (lo + hi) / 2
        v = subset_nolp(lgc, cap_lgB=mid - lg(280))[0]
        if v > rho_words():
            lo = mid
        else:
            hi = mid
    print("   -> break-even with rho at memory 2^%.1f B (%.3g B); 2^128 needs unconstrained 2^%.1f B" %
          (hi, 2.0 ** hi, lg(la_mem_bytes(2.0 ** subset_nolp(lgc)[1], 5))))

print("\n== F4 RAM extrapolation: dense echelon bound  rank_d x N_d x 4 B at degree d ==")
import numpy as np

def hilbert_ci(degs, maxd):
    """coefficients of prod (1-t^d)/(1-t)"""
    h = np.zeros(maxd + 1, dtype=object)
    h[0] = 1
    for d in degs:
        # multiply by (1 - t^d)/(1-t) = 1 + t + ... + t^{d-1}
        new = np.zeros(maxd + 1, dtype=object)
        for i in range(maxd + 1):
            if h[i]:
                for j in range(d):
                    if i + j <= maxd:
                        new[i + j] += h[i]
        h = new
    return h

def ncols(d, nv=5):
    return math.comb(d + nv - 1, nv - 1)

for label, degs in [("S5/raw-shape D=16^5=2^20 (5 eqs deg 16)", [16] * 5),
                    ("rational quotient 5 eqs deg 8 (+ w^2 relation) D=8^5*2=2^16", [8] * 5)]:
    top = sum(d - 1 for d in degs) + 1
    h = hilbert_ci(degs, top + 2)
    D = sum(h)
    print(label, " D=%d=2^%.2f  Macaulay bound d_reg=%d" % (D, lg(D), top))
    peak = max(range(len(h)), key=lambda i: h[i])
    print("   Hilbert function peak h(%d)=%d" % (peak, h[peak]))
    for d in [20, 28, 32, 33, 34, 38, 45, 55, 65, top]:
        if d >= len(h):
            continue
        N = ncols(d)
        r = N - int(h[d])
        mem = r * N * 4
        print("   d=%2d  N_d=%9d  rank=%9d  dense echelon %.3g B = %.1f GiB" % (d, N, r, mem, mem / 2 ** 30))
    print("   sum over d<=top of N_d*rank (dense flops proxy): 2^%.1f" %
          lg(sum((ncols(d) - int(h[d])) * ncols(d) * max(1, (ncols(d) - int(h[d]))) for d in range(top + 1))))

# measured lower bound on c from the committed C1 ladder
for threads in (1, 4):
    cyc = 836 * threads * 2.8e9
    print("EXP-GFPN-05ff43 m=5 S5 arm: 836 s x %d thread(s) x 2.8 GHz = 2^%.1f cycles before OOM at degree 33 (p'=4111)" %
          (threads, lg(cyc)))
