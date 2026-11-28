#!/usr/bin/env python3
"""Sensitivity of arm/R to counted native work the band leaves out (X2):
  base     IC group + w*native_mul;      rho group + w*native_mul        (the studies' rule)
  ic_adds  IC group + w*native_all;      rho as base
  rho_win  IC as base;                   rho group + w*(native_mul + k*steps)
  both     IC group + w*native_all;      rho group + w*(native_mul + k*steps)
k = rho's cycle-window word compares per step, measured by the validator dumper
(run_rho drops them): 17.295 at p = 4099 (out/dump_rho_4099.json) and 11.5-17.2 at
p = 131-2053; p > 4099 uses the p = 4099 value (an estimate; the measured trend rises
slowly with p, so it understates rho's count there).
Usage: pricing_sensitivity.py REPO_ROOT DUMP_DIR OUT_JSON"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vlib as V
ROOT, DD, OUT = sys.argv[1:4]
# measured window ops per step by p
k = {}
for f in ["dump_lucas_CLMR.json", "dump_rho_4099.json"]:
    rows = [r for r in json.load(open(os.path.join(DD, f)))["rows"] if r["kind"] == "rho"]
    for p in sorted({r["p"] for r in rows}):
        x = [r for r in rows if r["p"] == p]
        k[p] = sum(sum(r["val_rr"]["work_native"].values()) for r in x) / sum(r["val_rr"]["steps"] for r in x)
kk = lambda p: k.get(p, k[4099])
studies = {
    "lucas": ("research/ic_positive_control_lucas_20261116/run.json", None, ["C_double_lp", "C_lucas_roots"]),
    "la": ("research/ic_positive_control_la_20261116/run.json", None, ["C_coord_square"]),
    "unit_t": ("research/ic_positive_control_unit_t_20261116/run.json", None, ["C_unit_t", "C_unit_t_fast"]),
    "capped": ("research/ic_positive_control_capped_20261116/run_w.json", "research/ic_positive_control_unit_t_20261116/run.json", ["C_unit_t_capped_fast"]),
}
out = {"k_window_ops_per_step": k}
for st, (f, fr, arms) in studies.items():
    _, rows = V.load(os.path.join(ROOT, f))
    if fr:
        _, r2 = V.load(os.path.join(ROOT, fr))
        rows += [r for r in r2 if r["arm"] == "R_rho_negation"]
    by = V.index(rows)
    res = {}
    for p in sorted({r["p"] for r in rows}):
        if p < 4099:
            continue
        R = V.cell(by, "R_rho_negation", p)
        for a in arms:
            A = V.cell(by, a, p)
            for w in (0.1, 1.0):
                ic_b = lambda r: r["group_ops"] + w * r["native_mul"]
                ic_a = lambda r: r["group_ops"] + w * r["native_all"]
                rh_b = lambda r: r["group_ops"] + w * r["native_mul"]
                rh_w = lambda r: r["group_ops"] + w * (r["native_mul"] + kk(p) * r["steps"])
                res[f"{a} p={p} w={w}"] = {
                    "base": V.ratio_of_means(A, R, ic_b, rh_b), "ic_adds": V.ratio_of_means(A, R, ic_a, rh_b),
                    "rho_win": V.ratio_of_means(A, R, ic_b, rh_w), "both": V.ratio_of_means(A, R, ic_a, rh_w)}
    out[st] = res
json.dump(out, open(OUT, "w"), indent=1)
print("window ops per step:", {p: round(v, 2) for p, v in k.items()})
for st in studies:
    for key, v in out[st].items():
        print(st, key, {kk_: round(x, 2) for kk_, x in v.items()})
