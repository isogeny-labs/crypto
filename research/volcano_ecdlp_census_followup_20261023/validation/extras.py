#!/usr/bin/env python3
"""Validator extras (follow-up study): wall A/A ratio over the isogenous
families and over isogenous + ctl (ANALYSIS_PLAN s8 lists ctl), transport-then-
solve point ratios for every V6 (family, arm, destination level) cell at
p = 0 with the native control (the source vertex itself excluded from the
destinations at its own level), and the H-T M_v per source vertex
(velu_point_maps of the tcr-v<id> transfer_cost line + 2; check = 2 x
path_len_to_crater). Validator-written; reads no producer code.
Usage: python3 -I extras.py STUDY_DIR ROWS_DIR OUT_JSON"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vfu_data  # noqa: E402

STUDY, ROWS, OUT = sys.argv[1:4]
df, verts, tcr, checks = vfu_data.load(STUDY, ROWS)
ISO = {"V5": ["D", "Ua", "Ub", "N"], "V6": ["D", "Ua", "Ub", "S", "N"]}
out = {"wall": {}, "tts": {}, "M_v": {}}
for V in ("V5", "V6"):
    for nm, fams in (("iso", ISO[V]), ("iso+ctl", ISO[V] + ["ctl"])):
        sub = df[(df.V == V) & df.fam.isin(fams) & df.arm.isin(["rho-neg", "rho-neg-aa"])]
        piv = sub.pivot_table(index=["fam", "wid", "tidx", "round"], columns="arm", values="solve_wall_ns",
                              aggfunc="first").dropna()
        out["wall"][f"{V}|{nm}"] = {"aa_ratio": float(piv["rho-neg-aa"].mean() / piv["rho-neg"].mean()), "n_pairs": int(len(piv))}
src = json.load(open(f"{STUDY}/census/V6/sources.json"))
famsrc = {"D": src["crater_root"]["id"], "Ua": src["floor_a"]["id"], "Ub": src["floor_b"]["id"], "S": src["crater_b"]["id"]}
for fam, s0 in famsrc.items():
    for arm in ("rho-neg", "rho-plain", "bsgs", "bsgs-neg", "ic"):
        sub = df[(df.V == "V6") & (df.fam == fam) & (df.arm == arm)]
        nat = df[(df.V == "V6") & (df.fam == "N") & (df.arm == arm)]
        sm = sub[sub.vertex == s0]["total_gae"].mean()
        nm = nat[nat.vertex == s0]["total_gae"].mean()
        for L in range(5):
            d = sub[(sub.level == L) & (sub.vertex != s0)]
            n = nat[(nat.level == L) & (nat.vertex != s0)]
            out["tts"][f"V6|{fam}|{arm}|L{L}"] = {"source_vertex": s0, "ratio_p0": float(d["total_gae"].mean() / sm),
                                                 "control_native_ratio": float(n["total_gae"].mean() / nm)}
tc = {}
for l in open(f"{STUDY}/runs/transfer_cost.jsonl"):
    d = json.loads(l)
    if str(d.get("src", "")).startswith("tcr-v"):
        tc[int(d["tcr_source_id"])] = d
for v, d in tcr.items():
    out["M_v"][str(v)] = {"M_v": tc[v]["velu_point_maps"] + 2, "two_x_path_len": 2 * d["path_len_to_crater"]}
# per-vertex H-T point values (s5b): T_v(p), T0_v(p) on (tidx, round) pairs
out["T_v"] = {}
tcr_rows = df[df.fam == "Tcr"]
nrows = df[(df.V == "V5") & (df.fam == "N") & (df.arm == "rho-neg")]
for v in tcr:
    t = tcr_rows[tcr_rows.vertex == v]
    a = t[t.arm == "rho-cubic"].set_index(["tidx", "round"])
    b = t[t.arm == "rho-neg"].set_index(["tidx", "round"])
    c = nrows[nrows.vertex == v].set_index(["tidx", "round"])
    k = sorted(set(a.index) & set(b.index) & set(c.index))
    Mv = out["M_v"][str(v)]["M_v"]
    out["T_v"][str(v)] = {}
    for p in (0.0, 0.1, 1.0):
        den = (c.loc[k, "total_gae"] + p * c.loc[k, "unc"]).mean()
        out["T_v"][str(v)][f"T|{p}"] = float(((a.loc[k, "total_gae"] + p * a.loc[k, "unc"]).mean() + p * Mv) / den)
        out["T_v"][str(v)][f"T0|{p}"] = float(((b.loc[k, "total_gae"] + p * b.loc[k, "unc"]).mean() + p * Mv) / den)
# V5 BSGS family-mix check (F7): constant R(L) explained by D+N at the crater vs D+Ua+Ub+N elsewhere
out["v5_bsgs_mix"] = {}
for arm in ("bsgs", "bsgs-neg"):
    sb = df[(df.V == "V5") & (df.arm == arm) & df.fam.isin(ISO["V5"])]
    m = sb.groupby("fam")["s"].mean()
    x = float(m["N"] / m["D"])
    lv = sb.groupby("level")["s"].mean()
    out["v5_bsgs_mix"][arm] = {"N_over_D": x, "predicted_R": (3 + x) / (2 * (1 + x)),
                               "observed_R": {int(L): float(lv[L] / lv[0]) for L in lv.index if L > 0},
                               "Ua_Ub_equal_D": bool(m["Ua"] == m["D"] and m["Ub"] == m["D"])}
# s5a band: pooled mean priced S per arm (isogenous families), V6 all levels and V5 levels 1-6
out["band"] = {}
for V, Lmin in (("V6", 0), ("V5", 1)):
    sub = df[(df.V == V) & df.fam.isin(ISO[V]) & (df.level >= Lmin)]
    for arm in ("ic", "rho-neg"):
        a = sub[sub.arm == arm]
        for p in (0.0, 0.1, 1.0):
            out["band"][f"{V}|{arm}|{p}"] = float((a["s"] * (a["total_gae"] + p * a["unc"]) / a["total_gae"]).mean())
        out["band"][f"{V}|{arm}|n"] = int(len(a))
json.dump(out, open(OUT, "w"), indent=1)
print(json.dumps(out["wall"]))
