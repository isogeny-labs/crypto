#!/usr/bin/env python3
"""Driver: paired instances (curve, x) across all configs. [pcc:unverified]"""
import random, json, sys, math, time
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from hyb import *

bits_list = [int(b) for b in sys.argv[1].split(',')]
ncurves, ninst = int(sys.argv[2]), int(sys.argv[3])
out = open(sys.argv[4], 'w')
master = random.Random(20261112)
EXTRA = (lambda n: [('bsgsU', 1, round(0.305 * math.sqrt(n)), True)]) if len(sys.argv) > 5 else None

def configs(n):
    cfg = [('rho', 0, 0, False)]
    c2 = 0.4
    v2 = (c2 * n / 4) ** (1/3)  # argmin V^2 + c n/(2V)
    for f in (0.5, 0.75, 1.0, 1.4, 2.0):
        cfg.append(('m2', max(4, round(f * v2)), 2, False))
    # m=3: entries ~ (2/3)V^3, h ~ 0.9V -> total ~ (2/3)V^3 + 0.9 n V/((4/3)V^3)
    v3 = (0.675 * n) ** 0.2  # argmin (2/3)V^3 + 0.675 n/V^2
    for f in (0.6, 0.8, 1.0, 1.3):
        cfg.append(('m3', max(3, round(f * v3)), 3, False))
    # BSGS end of the family: V=1 (P itself, known log), table {kP}
    cfg.append(('bsgsP', 1, round(math.sqrt(n / 2)), True))
    # V=2: P + one unknown FB point, table {aP + bF}
    for f in (0.7, 1.0):
        cfg.append(('v2P', 2, round(f * (2 * n) ** 0.25 * 1.3), True))
    return cfg if EXTRA is None else EXTRA(n)

for bits in bits_list:
    for ci in range(ncurves):
        crng = random.Random(master.randrange(1 << 62))
        E, P = gen_curve(bits, crng)
        for ii in range(ninst):
            x = crng.randrange(1, E.n); Q = E.mul(x, P)
            iseed = crng.randrange(1 << 62)
            for cfgi, (name, V, m, incP) in enumerate(configs(E.n)):
                for neg in (False, True):
                    rng = random.Random(iseed * 7 + (1 if neg else 0))
                    t = time.time()
                    r = run_instance(E, P, Q, x, rng, V, m, neg=neg, include_P=incP,
                                     use_table=(name != 'rho'))
                    row = dict(bits=bits, p=E.p, a=E.a, b=E.b, n=E.n, curve=ci, inst=ii,
                               arm=name, cfg=(100 + cfgi if EXTRA else cfgi), V=V, m=m, incP=incP, neg=neg,
                               secs=round(time.time() - t, 3), **r)
                    out.write(json.dumps(row) + '\n'); out.flush()
        print('done', bits, ci, flush=True)
