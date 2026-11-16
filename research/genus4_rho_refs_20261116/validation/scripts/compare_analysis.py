#!/usr/bin/env python3
"""Compare the validator's exact replication of the bootstrap (exact_intervals.json)
with every interval in the producers' analysis.json files, to 3 decimals.
Usage: compare_analysis.py ROOT exact_intervals.json"""
import json, sys
ROOT, EX = sys.argv[1], sys.argv[2]
ex = json.load(open(EX))
a1 = json.load(open(f"{ROOT}/research/genus4_positive_control_20261116/analysis.json"))
a2 = json.load(open(f"{ROOT}/research/genus4_rho_refs_20261116/analysis.json"))
n = bad = 0
for e in a1["per_p"]:
    p = e["p"]
    m = {"v_over_c_la": f"s1 {p} LA v/c", "w_over_c_la": f"s1 {p} LA w/c", "w_over_c_rel": f"s1 {p} rel w/c"}
    for arm in "cvw":
        for nm, w in (("0", 0.0), ("0.1", 0.1), ("1", 1.0)):
            m[f"{arm}_over_R_w{nm}"] = f"s1 {p} {arm}/R w={w}"
        m[f"{arm}_over_R_fpm"] = f"s1 {p} {arm}/R FPM"
    for arm in "vw":
        for nm, w in (("0.1", 0.1), ("1", 1.0)):
            m[f"{arm}_over_c_w{nm}"] = f"s1 {p} {arm}/c w={w}"
    for k, v in m.items():
        prod = [round(x, 3) for x in e[k]]; n += 1
        if prod != ex[v]:
            bad += 1; print("S1 DIFF", k, p, prod, ex[v])
wmap = {"0": 0.0, "0.1": 0.1, "1": 1.0}
for rd, cells in a2["readings"].items():
    for nm, v in cells.items():
        key = "s2 Qcal" if rd == "Qcal" else f"s2 {rd} w={wmap[nm]}"
        prod = [round(v["ratio"], 3), round(v["lo"], 3), round(v["hi"], 3)]; n += 1
        if prod != ex[key]:
            bad += 1; print("S2 DIFF", rd, nm, prod, ex[key])
for rd, cells in a2["loo"].items():
    for c, v in cells.items():
        prod = [round(v["ratio"], 3), round(v["lo"], 3), round(v["hi"], 3)]; n += 1
        if prod != ex[f"s2 {rd} LOO-{c} w=1"]:
            bad += 1; print("S2 LOO DIFF", rd, c, prod, ex[f"s2 {rd} LOO-{c} w=1"])
print("analysis.json cells compared", n, "differences", bad)
