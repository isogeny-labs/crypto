#!/usr/bin/env python3
"""E3 follow-up: the g=10 p=11 row with the admitted-but-omitted curve 27,
using the certified g16f re-run of curve 27 (P4_followup/certs) beside the
committed P1/supp records of curves 7 and 12.  Uses tables.py's definitions."""
import glob
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tables as T  # noqa: E402

os.chdir(T.ROOT)
runs, _, _ = T.load(glob.glob("P1/supp/g10_p11_*.jsonl") + ["P4_followup/certs/g10_p11_c27_cert.jsonl"])
rows = T.pairs_of(runs)[0][(10, 11)]
gm = lambda xs: math.exp(sum(map(math.log, xs)) / len(xs))  # noqa: E731
v = {a: [] for a in ("c", "D", "E", "cD", "ED")}
for key, arms in rows:
    c, D, E, R = (arms[T.ARMS[a]] for a in "cDER")
    rE = T.rho_expected(R)
    v["c"].append(T.fpm(c) / rE)
    v["D"].append(T.fpm(D) / rE)
    v["E"].append(T.fpm(E) / rE)
    v["cD"].append(T.fpm(c) / T.fpm(D))
    v["ED"].append(T.fpm(E) / T.fpm(D))
    print(key, f"c/rho {v['c'][-1]:.4g}{' (exhausted)' if c['exhausted'] else ''}  D/rho {v['D'][-1]:.4g}  "
          f"E/rho {v['E'][-1]:.4g}{' (exhausted)' if E['exhausted'] else ''}")
for label, sl in (("4 pairs (curves 7, 12; committed table)", slice(0, 4)), ("6 pairs (curves 7, 12, 27)", slice(0, 6))):
    print(f"{label}: c/rho >={gm(v['c'][sl]):.4g}  D/rho {gm(v['D'][sl]):.4g}  E/rho {gm(v['E'][sl]):.4g}  "
          f"gain c/D >={gm(v['cD'][sl]):.4g}  E/D {gm(v['ED'][sl]):.4g}")
