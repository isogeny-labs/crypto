#!/usr/bin/env python3
"""Distance from the clique-sum / tensor-contraction thresholds to cryptographic sizes.

For a target, the bar is (log2 of the best conventional attack, per call) / U, where U is the
number of edge bits of the network the counting route must contract (both ends counted, the
convention of Majdoddin and the Anthropic paper). A route helps only at instance sizes with
log2(size) >= log2 m*(bar). The available size is the target's own network, log2 U.

Targets (order-of-magnitude assumptions, stated in the note):
  ECDLP-256 via Bochum index calculus, m = 4: ~3.4e7 edges per decomposition query
      (<1100(d+1) edges per d-bit multiply, ~2 multiplies per modular multiply, ~15 modular
       multiplies per point addition with both branches, plus one per membership test);
      per-query budget to beat rho: (1/2 - 1/m) * 256 = 64 bits.  bar = 64 / 6.8e7.
  AES-128 key recovery from one pair as 3-CNF: ~1e6 clauses, U = 2(3m - n) ~ 6e6;
      competitor exhaustive key search 2^128.                    bar = 128 / 6e6.
  Ring-LWE n = 1024, binary secret, 20-bit uniform noise (the LWE paper's network):
      24 n r + n edges with r = 23, U ~ 1.13e6; competitor a 128-bit parameter set.
                                                                  bar = 128 / 1.13e6.
Reference SAT bars (Majdoddin): #3-SAT density 1: 0.1321; density 4.26: 0.0302; general 3-SAT ~3.2e-4.
"""
import math, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else ".")
from threshold_model import run_levels, lemmaC_threshold
from qb_model import run_levels_qb, top_qb

R2 = 1 / math.sqrt(2)
ZG = [2.0 ** (-j / 2.0) for j in range(2, 80)]
ZTOP = [2.0 ** (-j / 10.0) for j in range(10, 700)]
SG = [2 ** j for j in range(0, 80)]
STOP = [2 ** j for j in range(0, 90)]


def hoeffding(bar_, oz, oK, rho=R2, e0=0.2554, x0s=range(6, 13)):
    best = None
    for x0 in x0s:
        fr = run_levels(x0, e0, rho, bar_, oz, oK, zgrid=ZG, max_levels=70)
        r = lemmaC_threshold(fr, bar_, zgrid_top=ZTOP)
        if r and (best is None or r[0] < best[0]):
            best = (r[0], r[1])
    return best


def quantile(bar_, rho=R2, e0=0.2554, x0s=range(6, 13)):
    best = None
    for x0 in x0s:
        fr = run_levels_qb(x0, e0, rho, bar_, 1.0, max_levels=70, sgrid=SG)
        r = top_qb(fr, bar_, sgrid=STOP)
        if r and (best is None or r[0] < best[0]):
            best = (r[0], r[1])
    return best


# OpenAI Appendix D, Majdoddin's reading: log2 s* = (C / bar)^2, calibrated on his density-one
# values with bar 0.0881: 4.4e10 (constants as written) and 3.7e5 (his unproved improvements).
C_D, C_D_OPT = 0.0881 * math.sqrt(4.4e10), 0.0881 * math.sqrt(3.7e5)

targets = [
    ("#3-SAT density 1 (reference)", 0.1321, None),
    ("#3-SAT density 4.26 (reference)", 0.0302, None),
    ("general 3-SAT (reference)", 3.2e-4, None),
    ("Ring-LWE-1024 binary secret", 128 / 1.13e6, math.log2(1.13e6)),
    ("AES-128 key recovery (CNF)", 128 / 6e6, math.log2(6e6)),
    ("ECDLP-256 (Bochum IC, m=4)", 64 / 6.8e7, math.log2(6.8e7)),
]

hdr = f"{'target':34s} {'bar':>9s} {'have':>5s} | {'Majd.':>10s} {'+reread':>10s} {'+LemmaQ':>10s} {'Q+rho1/2':>10s} | {'AppD':>9s} {'AppD-opt':>9s} | {'grid floor':>10s} {'poly b=1':>8s}"
print(hdr)
print("-" * len(hdr))
rows = []
for name, b, have in targets:
    m0 = hoeffding(b, 3, 3)
    m1 = hoeffding(b, 2, 1)
    m2 = quantile(b)
    m3 = quantile(b, rho=0.5)
    d0, d1 = (C_D / b) ** 2, (C_D_OPT / b) ** 2
    floor = 2.0 / b
    poly = 2.0 * math.log2(1.0 / b) + 2.0
    rows.append((name, b, have, m0, m1, m2, m3, d0, d1, floor, poly))
    hv = f"{have:5.1f}" if have else "  -  "
    print(f"{name:34s} {b:9.2e} {hv} | {m0[0]:10.3g} {m1[0]:10.3g} {m2[0]:10.3g} {m3[0]:10.3g} | {d0:9.2g} {d1:9.2g} | {floor:10.3g} {poly:8.1f}", flush=True)

print("\nlevels used (Majd., +reread, +LemmaQ, Q+rho1/2):")
for r in rows:
    print(f"  {r[0]:34s} {r[3][1]:3d} {r[4][1]:3d} {r[5][1]:3d} {r[6][1]:3d}")
