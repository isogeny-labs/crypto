#!/usr/bin/env python3
"""Margin change of the C1 Gaudry-Diem n = 4 cost model (crypto-autoresearcher
analysis/ecdlp2m-iteration-20261009/c1-gaudry-n4-c2pnb/c1_model.py, copied
unchanged as c1_model_upstream.py) when the factor base x in F_Q is replaced
by an all-lift line u*F_Q (u in F_{2^16}, Tr_N(a) = 0).

Two inputs change and nothing else:
- fb_factor = 2: every nonzero x of the line lifts (2 points), against about
  half of F_Q, so the orbit count N doubles at the same orbit size 2k;
- p_dec = 16/24: the per-trial decomposition probability is
  |F|^4 / (4! #E) with |F| ~ 2Q instead of ~Q (H0 gives 1/24).
c_trial is held fixed: the descended system is S_5(u y_1..u y_4, x_R), the
same degree and shape in y. That is an assumption, stated in the report.

Usage: python3 c2pnb_delta.py <out.json>
"""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c1_model_upstream as M

_r = M.reproduce_program_numbers()
derived = next(x for x in _r if isinstance(x, list) and x and isinstance(x[0], dict))
APPLIES = {"c2pnb208w1", "c2pnb368w1"}  # Tr_N(a) = 0, from c2pnb_lines.py
out = []
for ct in (2.0 ** 27, 2.0 ** 30):
    base = M.with_(M.DEFAULTS, c_trial=ct)
    dense = M.with_(base, fb_factor=2.0, p_dec=16.0 / 24)
    for lp in ("2LP", "none"):
        b2 = M.with_(base, lp=lp)
        d2 = M.with_(dense, lp=lp)
        for d in derived:
            for key, desc, _ in M.METRICS:
                m0 = M.margin(key, d, b2)
                m1 = M.margin(key, d, d2)
                out.append(dict(curve=d["name"], applies=d["name"] in APPLIES,
                                log2_c_trial=M.LOG2(ct), lp=lp, metric=key,
                                margin_std=round(m0, 3), margin_lift_dense=round(m1, 3),
                                delta_bits=round(m1 - m0, 3)))
json.dump(out, open(sys.argv[1], "w"), indent=1)
for r in out:
    if r["metric"] in ("time", "mesh-a", "TxM-a-p0") and r["log2_c_trial"] == 30:
        print(r)
