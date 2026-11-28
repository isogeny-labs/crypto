#!/usr/bin/env python3
"""(1) Arm W (capped study) against arm U (unit-T study), row by row, in every
deterministic field except arm/oracle/walk labels; the cap max(64, sqrt(r)/16)
per prime. (2) The producer's pre-freeze smoke (scratchpad smokeW.json) against
the committed rows.
Usage: check_w_vs_u.py REPO_ROOT SMOKEW_JSON"""
import json, math, os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vlib as V
ROOT, SMOKE = sys.argv[1], sys.argv[2]
_, rw = V.load(os.path.join(ROOT, "research/ic_positive_control_capped_20261116/run_w.json"))
_, ru = V.load(os.path.join(ROOT, "research/ic_positive_control_unit_t_20261116/run.json"))
W = V.index(rw)["C_unit_t_capped_fast"]
ub = V.index(ru)
U = ub["C_unit_t"]
st = collections.defaultdict(lambda: [0, 0, set()])
for k, w in W.items():
    same = not V.compare_rows(w, U[k], ignore=V.NONDET | {"arm", "oracle", "walk"})
    st[k[0]][0] += 1
    st[k[0]][1] += same
    st[k[0]][2].add(min(4096, max(64, int(math.sqrt(w["r"]) / 16))))
print("p | W runs | W row == U row (all deterministic fields) | restart cap per curve")
for p in sorted(st):
    print(p, st[p][0], st[p][1], sorted(st[p][2]))
sm = json.load(open(SMOKE))
key = lambda r: (r["arm"], r["p"], r["curve_idx"], r["seed"])
ref = {key(r): r for r in rw}
ref.update({key(r): r for r in ru})
n = same = 0
for r in sm["rows"]:
    c = ref.get(key(r))
    if c is None:
        print("smoke row without committed partner", key(r))
        continue
    n += 1
    same += not V.compare_rows(r, c)
print("smokeW cfg", sm["cfg"], "rows compared", n, "identical", same)
