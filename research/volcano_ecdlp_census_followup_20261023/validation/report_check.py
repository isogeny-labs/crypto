#!/usr/bin/env python3
"""Check REPORT.md table cells against analysis/results.json (exact, at the
report's rounding) and against the validator's recompute (recompute.json:
point estimates exact at the report's rounding; bootstrap endpoints within
Monte Carlo tolerance). Validator-written; reads no producer code.

Usage: python3 -I report_check.py STUDY_DIR RECOMPUTE_JSON OUT_JSON
"""
import json
import re
import sys

STUDY, RECOMP, OUT = sys.argv[1:4]
rep = open(f"{STUDY}/REPORT.md").read()
res = json.load(open(f"{STUDY}/analysis/results.json"))
mine = json.load(open(RECOMP))
cells = {"checked": 0, "equal_results": 0, "differ_results": [], "differ_recompute": [], "missing": []}


def rnd(x, d):
    return f"{x:.{d}f}"


def check(name, shown, claimed, recomputed=None, d=3, mc_tol=None):
    cells["checked"] += 1
    if shown is None:
        cells["missing"].append(name)
        return
    if rnd(claimed, d) == shown:
        cells["equal_results"] += 1
    else:
        cells["differ_results"].append({"cell": name, "report": shown, "results_json": claimed})
    if recomputed is not None:
        if mc_tol is None:
            if rnd(recomputed, d) != shown:
                cells["differ_recompute"].append({"cell": name, "report": shown, "recomputed": recomputed})
        elif abs(recomputed - float(shown)) > mc_tol + 0.5 * 10 ** (-d):
            cells["differ_recompute"].append({"cell": name, "report": shown, "recomputed": recomputed, "tol": mc_tol})


def table_rows(header_start):
    i = rep.index(header_start)
    out = []
    for line in rep[i:].splitlines()[2:]:
        if not line.startswith("|"):
            break
        out.append([c.strip() for c in line.strip().strip("|").split("|")])
    return out


def ci(s):
    m = re.match(r"\[([-0-9.]+), ([-0-9.]+)\]", s)
    return (m.group(1), m.group(2)) if m else (None, None)


# --- s4 level table
lev = mine["level_effect"]
for row in table_rows("| V | arm | L | curves |"):
    V, arm, L, nv, R, c95, c90, hw, margin, hp, verdict = row
    e = res["level_effect"][f"{V}|{arm}"]["R"][L]["S"]
    m = lev[f"{V}|{arm}|{L}"]
    tol = 4 * 0.0378 * e["boot_sd"]
    key = f"s4 {V} {arm} L{L}"
    check(key + " curves", nv, m["n_vertices"], m["n_vertices"], d=0)
    check(key + " R", R, e["ratio"], m["R"])
    for nm, s, ce, cm in (("ci95", c95, e["ci95"], m["ci95"]), ("ci90", c90, e["ci90"], m["ci90"])):
        lo, hi = ci(s)
        check(key + f" {nm} lo", lo, ce[0], cm[0], mc_tol=tol)
        check(key + f" {nm} hi", hi, ce[1], cm[1], mc_tol=tol)
    check(key + " half-width", hw, e["half_width95"], m["half_width95"], mc_tol=tol)
    cells["checked"] += 1
    if verdict != e.get("verdict") or verdict != m["verdict"]:
        cells["differ_results"].append({"cell": key + " verdict", "report": verdict, "results_json": e.get("verdict"),
                                        "recomputed": m["verdict"]})
    else:
        cells["equal_results"] += 1
    if hp != "-":
        check(key + " holm", hp, e["p_holm"], m["p_holm"], mc_tol=0.05)

# --- s5 curve effect table
for row in table_rows("| arm | F | permutation p |"):
    arm, F, p, ph, nf = row
    e = res["curve_effect"][f"V6|{arm}"]
    m = mine["curve_effect"][arm]
    if e.get("F") is None:
        cells["checked"] += 1
        cells["equal_results"] += int(F.startswith("n/a") and m.get("F") is None)
        continue
    check(f"s5 {arm} F", F, e["F"], m["F"], d=2)
    check(f"s5 {arm} p", p, e["p_perm"], m["p_perm"], mc_tol=0.01)
    check(f"s5 {arm} holm", ph, e["p_perm_holm"], m["p_perm_holm"], mc_tol=0.02)
    check(f"s5 {arm} flagged", nf, e["n_flagged"], m["n_flagged"], d=0)

# --- s7 H-T table
ht = res["H-T"]
mh = mine["H-T"]
for row in table_rows("| stratum | vertices |"):
    st, nv, t0, t01, t1, z0, z1 = row
    if "overall" in st:
        e, m, d = ht["overall"], mh["overall"], 4
        key = "s7 overall"
    else:
        L = re.search(r"level (\d)", st).group(1)
        e, m, d = ht["by_level"][L], mh["by_level"][L], 3
        key = f"s7 level {L}"
    for nm, s, t, p in (("T 0", t0, "T", "0.0"), ("T 0.1", t01, "T", "0.1"), ("T 1", t1, "T", "1.0"),
                        ("T0 0", z0, "T0", "0.0"), ("T0 1", z1, "T0", "1.0")):
        val = s.split(" [")[0]
        lo, hi = ci(s[len(val) + 1:])
        ev = e[t][p]
        mv = m[f"{t}|{p}"]
        sd = ev.get("boot_sd", (ev["ci95"][1] - ev["ci95"][0]) / 3.92)
        check(f"{key} {nm}", val, ev["value"], mv["value"], d=d)
        check(f"{key} {nm} lo", lo, ev["ci95"][0], mv["ci95"][0], d=d, mc_tol=4 * 0.0378 * sd)
        check(f"{key} {nm} hi", hi, ev["ci95"][1], mv["ci95"][1], d=d, mc_tol=4 * 0.0378 * sd)

# --- s8 H-M table
hm = res["H-M"]["curves"]
mm = mine["H-M"]["curves"]
for row in table_rows("| curve | level | `ic` F |"):
    cur, lvl, icF, icH, icR, rnF, rnH, rnR = row
    for arm, F, H, R in (("ic", icF, icH, icR), ("rho-neg", rnF, rnH, rnR)):
        e = hm[cur]["arms"][arm]
        m = mm[cur][arm]
        key = f"s8 {cur} {arm}"
        check(key + " F", F, e["F"], m["F"], d=2)
        sig = H.endswith("*")
        Hn = H.rstrip(" *")
        check(key + " holm", Hn, e["p_holm"], m["p_holm"], d=4 if arm == "ic" else 3, mc_tol=0.03)
        cells["checked"] += 1
        if sig != e["significant"]:
            cells["differ_results"].append({"cell": key + " significance mark", "report": H, "results_json": e["significant"]})
        else:
            cells["equal_results"] += 1
        if sig != m["significant"]:
            cells["differ_recompute"].append({"cell": key + " significance mark", "report": H, "recomputed": m["significant"],
                                              "recomputed_holm": m["p_holm"]})
        val = R.split(" [")[0]
        lo, hi = ci(R[len(val) + 1:])
        check(key + " max/min", val, e["max_over_min_model_mean"]["value"], m["max_over_min"], d=2)
        check(key + " max/min lo", lo, e["max_over_min_model_mean"]["ci95"][0], m["max_over_min_ci95"][0], d=2, mc_tol=0.02)
        check(key + " max/min hi", hi, e["max_over_min_model_mean"]["ci95"][1], m["max_over_min_ci95"][1], d=2, mc_tol=0.03)

# --- s9 IC / matched rho table
icr = res["ic_vs_rho"]
mi = mine["ic_vs_rho"]
for row in table_rows("| V | level | matched rho |"):
    V, L, mr, c0, c01, c1, order = row
    for nm, s, p in (("0", c0, "0.0"), ("0.1", c01, "0.1"), ("1", c1, "1.0")):
        val = s.split(" [")[0]
        lo, hi = ci(s[len(val) + 1:])
        key = f"s9 {V} L{L} {mr} {nm}"
        if V == "V5" and L == "0":
            e = res["ic_vs_rho_cubic_crater"]["prices"][p]
            m = mi["V5|crater_rho_cubic"][p]
            check(key, val, e["ratio_ic_over_rho_cubic"], m["ic_over_cubic"], d=1)
            check(key + " lo", lo, e["ci95"][0], m["ci95_ic_over_cubic"][0], d=1, mc_tol=0.3)
            check(key + " hi", hi, e["ci95"][1], m["ci95_ic_over_cubic"][1], d=1, mc_tol=0.3)
        else:
            e = icr[V]["levels"][L]["prices"][p]
            check(key, val, e["ratio_ic_over_rho"], mi[f"{V}|{L}"][p], d=1)
            check(key + " lo", lo, e["ci95"][0], None, d=1)
            check(key + " hi", hi, e["ci95"][1], None, d=1)
for row in table_rows("| pooled | at 0 GAE |"):
    nm, c0, c01, c1 = row
    V = nm.split()[0]
    rk = "pooled_levels_ge1" if V == "V5" else "pooled"
    mk = "V5|pooled_L1plus" if V == "V5" else "V6|pooled_all"
    for pn, s, p in (("0", c0, "0.0"), ("0.1", c01, "0.1"), ("1", c1, "1.0")):
        val = s.split(" [")[0]
        lo, hi = ci(s[len(val) + 1:])
        e = icr[V][rk][p]
        check(f"s9 pooled {V} {pn}", val, e["ratio_ic_over_rho"], mi[mk][p], d=1)
        check(f"s9 pooled {V} {pn} lo", lo, e["ci95"][0], mi[mk][f"ci95_{p}"][0], d=1, mc_tol=0.3)
        check(f"s9 pooled {V} {pn} hi", hi, e["ci95"][1], mi[mk][f"ci95_{p}"][1], d=1, mc_tol=0.3)

# --- s10 wall table
for row in table_rows("| V | A/A ratio |"):
    V, aa, c, within, l2, adm = row
    e = res["wall_admission"][V]
    check(f"s10 {V} A/A", aa, e["aa_ratio"], None, d=4)
    lo, hi = ci(c)
    check(f"s10 {V} A/A lo", lo, e["ci95"][0], None, d=4)
    check(f"s10 {V} A/A hi", hi, e["ci95"][1], None, d=4)
    for part in l2.split(", "):
        arm, pc = part.rsplit(" ", 1)
        check(f"s10 {V} L2 {arm}", pc.rstrip("%"), 100 * e["arms"][arm]["frac_rows_L2"],
              100 * mine["wall"][V][f"L2|{arm}|iso"], d=0)

json.dump(cells, open(OUT, "w"), indent=1)
print(json.dumps({k: (v if isinstance(v, int) else len(v)) for k, v in cells.items()}))
for k in ("differ_results", "differ_recompute", "missing"):
    for x in cells[k]:
        print(k, x)
