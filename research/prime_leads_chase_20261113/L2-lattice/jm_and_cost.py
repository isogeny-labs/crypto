#!/usr/bin/env python3
"""Control: reproduce JM box reach (program note §2.2) and translate reach -> IC exponent (§2.4)."""
from fractions import Fraction as Fr
import json
def box(m, l):
    d = 2**(m-1)
    num = sum((d*i+1)**m for i in range(l))
    den = Fr(m*(d*l+1)**m*d*l, 2)
    return num/den
rows = []
for m in (2,3,4):
    for l in (1,2,3,6,20,100):
        rows.append(dict(m=m, level=l, delta=float(box(m,l))))
    rows.append(dict(m=m, level='inf', delta=1/(m*(m+1)*2**(m-2))))
def ic_exp(m, dr):
    """total exponent with B = p^min(dr, 1/(m+1)), poly-time oracle, LA m*B^2"""
    b = min(dr, 1/(m+1))
    return max(1-(m-1)*b, 2*b)
cost = []
for label, m, dr in [("S3 rigorous HG, measured (Probe B, 40/56 bit)",2,0.10),
                     ("S3 heuristic >=2 vanishing, measured upper (Probe B/C, 56 bit lvl3: 0.12 ok, 0.14 3/8)",2,0.12),
                     ("S3 heuristic, measured best cell (40 bit, 0.14 at >=6/8)",2,0.14),
                     ("S3 JM asymptote (derived)",2,1/6),
                     ("S3 any-lattice bound, equal bounds (cited smallroot_optimal_reach)",2,0.3536/2),
                     ("S4 best feasible-dim determinant bound (cited L4 §3.2, dim 969)",3,1/35.5),
                     ("S4 JM asymptote",3,1/24),
                     ("S4 any-lattice bound (cited §7c)",3,1/21.8),
                     ("S4 milestone",3,1/4),
                     ("S5 JM asymptote",4,1/80),
                     ("S5 any-lattice bound (cited §7c)",4,1/70.8),
                     ("S5 target",4,1/5)]:
    e = ic_exp(m, dr)
    cost.append(dict(row=label, m=m, reach=round(dr,4), ic_exponent=round(e,4), distance_to_rho=round(e-0.5,4)))
print(json.dumps(dict(jm=rows, cost=cost), indent=1))
