#!/usr/bin/env python3
"""Independent recompute of research/full_jacobian_ic_20261113 from full_jacobian.json.

Validator V, 2026-11-14. Shares no code with the producer (no import of
render.py, no Rust). Recomputes from the raw per-arm counters and class times:
band totals, calibrated totals, ratios to the negation/plain/best rho, order
classes, the 32+32 log-log fits with their 95% t intervals (own t quantile),
the frozen decision rule, the F1-F8 scorecard numbers, every RESULTS.md
number quoted, the 20 record metrics. Writes a JSON report.

Usage: recompute_fj.py JSON RECORD RESULTS_MD OUT_JSON
"""
import json, math, re, sys

src, rec_path, results_md, out_path = sys.argv[1:5]
d = json.load(open(src))
rec = json.load(open(rec_path))
res_md = open(results_md).read()

BAND = [0.0, 0.1, 1.0]
CAL = [0.0, 1.0, 10.0]
REL = 1e-9
mism = []
checked = 0


def close(a, b, tol=REL):
    if a is None or b is None:
        return a is None and b is None
    if a == b:
        return True
    return abs(a - b) <= tol * max(abs(a), abs(b))


def chk(name, claimed, mine, tol=REL):
    global checked
    checked += 1
    if not close(claimed, mine, tol):
        mism.append({"name": name, "claimed": claimed, "recomputed": mine})


def classify(a, r):
    if all(x > y for x, y in zip(a, r)):
        return "above"
    if all(x < y for x, y in zip(a, r)):
        return "below"
    return "unresolved"


# ---------- Student t quantile, own implementation (regularized incomplete beta) ----------
def betacf(a, b, x):
    MAXIT, EPS, FPMIN = 400, 3e-16, 1e-300
    qab, qap, qam = a + b, a + 1, a - 1
    c, dd = 1.0, 1 - qab * x / qap
    dd = 1 / (dd if abs(dd) > FPMIN else FPMIN)
    h = dd
    for m in range(1, MAXIT + 1):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        dd = 1 + aa * dd; dd = FPMIN if abs(dd) < FPMIN else dd
        c = 1 + aa / c; c = FPMIN if abs(c) < FPMIN else c
        dd = 1 / dd; h *= dd * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        dd = 1 + aa * dd; dd = FPMIN if abs(dd) < FPMIN else dd
        c = 1 + aa / c; c = FPMIN if abs(c) < FPMIN else c
        dd = 1 / dd; de = dd * c; h *= de
        if abs(de - 1) < EPS:
            break
    return h


def betai(a, b, x):
    if x <= 0: return 0.0
    if x >= 1: return 1.0
    bt = math.exp(math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log(1 - x))
    if x < (a + 1) / (a + b + 2):
        return bt * betacf(a, b, x) / a
    return 1 - bt * betacf(b, a, 1 - x) / b


def t_cdf(t, df):
    x = df / (df + t * t)
    p = 0.5 * betai(df / 2, 0.5, x)
    return 1 - p if t > 0 else p


def t_q975(df):
    lo, hi = 0.0, 100.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if t_cdf(mid, df) < 0.975: lo = mid
        else: hi = mid
    return (lo + hi) / 2


def ols(xs, ys):
    n = len(xs)
    lx = [math.log(v) for v in xs]; ly = [math.log(v) for v in ys]
    mx = sum(lx) / n; my = sum(ly) / n
    sxx = sum((v - mx) ** 2 for v in lx)
    sxy = sum((a - mx) * (b - my) for a, b in zip(lx, ly))
    s = sxy / sxx; a = my - s * mx
    sse = sum((y - a - s * x) ** 2 for x, y in zip(lx, ly))
    se = math.sqrt(sse / (n - 2) / sxx)
    t = t_q975(n - 2)
    return {"slope": s, "intercept": a, "se": se, "t975": t, "ci95": [s - t * se, s + t * se], "n": n}


# ---------- per row ----------
rows = d["rows"]
cells = []
row_out = []
for row in rows:
    p = row["p"]; ct = row["class_times_ns"]
    tE = row["t_add_ns"]["E1"]; tJ = row["t_add_ns"]["J_word"]
    rn = row["rho_E"]["negation"]; rp = row["rho_E"]["plain"]
    RHO_CLS = [("canon", "canonicalisations"), ("hash", "partition_hashes"), ("lookup", "dp_lookups"),
               ("insert", "dp_inserts"), ("word", "word_ops")]

    def rho_band(m):
        nat = sum(m[f] for _, f in RHO_CLS)
        return [m["total_ops"] + w * nat for w in BAND], nat

    def rho_cal(m):
        s = sum(ct[c] / tE * m[f] for c, f in RHO_CLS)
        return [m["total_ops"] + f * s for f in CAL]

    nb, nnat = rho_band(rn); pb, _ = rho_band(rp)
    nc = rho_cal(rn); pc = rho_cal(rp)
    chk(f"p{p}.rho_E.negation.native", rn["native"], nnat)
    for i in range(3):
        chk(f"p{p}.rho_E.band_negation[{i}]", row["rho_E"]["band_negation"][i], nb[i])
        chk(f"p{p}.rho_E.cal_negation[{i}]", row["rho_E"]["cal_negation"][i], nc[i])
    bb = [min(a, b) for a, b in zip(nb, pb)]; bc = [min(a, b) for a, b in zip(nc, pc)]
    # sanity: rho runs certified
    chk(f"p{p}.rho_E.negation.certified==runs", rn["runs"], rn["certified"])
    chk(f"p{p}.rho_E.plain.certified==runs", rp["runs"], rp["certified"])
    chk(f"p{p}.rho_E.negation.runs", rn["runs"], row["instances"] * row["rho_seeds_per_instance"])
    # structural checks
    chk(f"p{p}.jac_order=e1*e2", row["jac_order"], row["e1_order"] * row["e2_order"])
    chk(f"p{p}.h=jac/r", row["h"], row["jac_order"] // row["r"])
    chk(f"p{p}.r|jac", 0, row["jac_order"] % row["r"])
    chk(f"p{p}.r_divides_e2_order", row["r_divides_e2_order"], row["e2_order"] % row["r"] == 0)
    chk(f"p{p}.r2|jac", row["r_squared_divides_jac_order"], row["jac_order"] % (row["r"] ** 2) == 0)
    ro = {"p": p, "stage": row["stage"], "r": row["r"], "h": row["h"], "rho_neg_band": nb, "rho_neg_cal": nc,
          "rho_plain_band": pb, "arms": {}}
    for an, a in row["arms"].items():
        if an == "rho_J_negation":
            m = a["mean"]
            nat = {"canon": m["canonicalisations"], "hash": m["partition_hashes"], "lookup": m["dp_lookups"],
                   "insert": m["dp_inserts"], "word": m["word_ops"]}
            go = m["total_ops"]
            chk(f"p{p}.{an}.group_ops", a["group_ops"], go)
        else:
            nat = {"mul_p": a["rel_mul"], "mul_r": a["la_mul"]}
            go = a["group_ops"]
            # la_mul is the sum of the three LA parts
            chk(f"p{p}.{an}.la_mul=graph+peel+wied", a["la_mul"], a["la_graph_mul"] + a["la_peel_mul"] + a["la_wiedemann_mul"], 1e-12)
            chk(f"p{p}.{an}.walk+setup+verify<=group_ops", 1, int(a["walk_ops"] + a["setup_ops"] + a["verify_ops"] <= go * (1 + 1e-12)))
            chk(f"p{p}.{an}.w1_with_additions", a["total_w1_with_additions"],
                go + a["rel_mul"] + a["rel_add"] + a["la_mul"] + a["la_add"])
            chk(f"p{p}.{an}.certified==runs", a["runs"], a["certified"])
            chk(f"p{p}.{an}.exhausted", 0, a["exhausted"])
            chk(f"p{p}.{an}.bad_relations", 0.0, a["bad_relations"])
            exp_runs = row["instances"] * (row["ctl_seeds"] if an.endswith("_S") else row["ic_seeds"])
            chk(f"p{p}.{an}.runs", exp_runs, a["runs"])
            chk(f"p{p}.{an}.rels_by_lp_sum", a["relations"], a["rels_with_0_lp"] + a["rels_with_1_lp"] + a["rels_with_2_lp"], 1e-12)
        for k, v in nat.items():
            chk(f"p{p}.{an}.native.{k}", a["native"].get(k, 0.0), v)
        ntot = sum(nat.values())
        tb = [go + w * ntot for w in BAND]
        tadd = a["t_add_ns"]
        chk(f"p{p}.{an}.t_add_ns", tadd, tJ)
        tc = [go + f * sum(ct[k] / tadd * v for k, v in nat.items()) for f in CAL]
        for i in range(3):
            chk(f"p{p}.{an}.total_band[{i}]", a["total_band"][i], tb[i])
            chk(f"p{p}.{an}.total_cal[{i}]", a["total_cal"][i], tc[i])
            chk(f"p{p}.{an}.ratio_neg_band[{i}]", a["ratio_neg_band"][i], tb[i] / nb[i])
            chk(f"p{p}.{an}.ratio_neg_cal[{i}]", a["ratio_neg_cal"][i], tc[i] / nc[i])
            chk(f"p{p}.{an}.ratio_plain_band[{i}]", a["ratio_plain_band"][i], tb[i] / pb[i])
            chk(f"p{p}.{an}.ratio_best_band[{i}]", a["ratio_best_band"][i], tb[i] / bb[i])
        cls = {"class_neg_band": classify(tb, nb), "class_neg_cal": classify(tc, nc),
               "class_plain_band": classify(tb, pb), "class_best_band": classify(tb, bb),
               "class_best_cal": classify(tc, bc)}
        for k, v in cls.items():
            checked += 1
            if a[k] != v:
                mism.append({"name": f"p{p}.{an}.{k}", "claimed": a[k], "recomputed": v})
        # ranked at calibrated price: order the same at 0, w_cal, 10 w_cal
        checked += 1
        if a["ranked_at_calibrated_price"] != (cls["class_neg_cal"] in ("above", "below")):
            mism.append({"name": f"p{p}.{an}.ranked_at_calibrated_price", "claimed": a["ranked_at_calibrated_price"]})
        ro["arms"][an] = {"group_ops": go, "native_total": ntot, "total_band": tb, "total_cal": tc,
                          "ratio_neg_band": [tb[i] / nb[i] for i in range(3)],
                          "ratio_neg_cal": [tc[i] / nc[i] for i in range(3)], **cls}
        if an != "rho_J_negation":
            cells.append((p, row["stage"], an, cls["class_neg_band"], cls["class_neg_cal"]))
    row_out.append(ro)

# ---------- cell counts ----------
summary = {
    "ic_cells": len(cells),
    "above_band": sum(1 for c in cells if c[3] == "above"),
    "above_cal": sum(1 for c in cells if c[4] == "above"),
    "below_band_any": [c for c in cells if c[3] == "below"],
    "not_above": [c for c in cells if c[3] != "above" or c[4] != "above"],
}

# ---------- fits ----------
def fitset(rs):
    ps = [r["p"] for r in rs]
    F = {}
    for an in ["A", "B", "A_S", "B_S"]:
        F[f"{an}_group_ops"] = ols(ps, [r["arms"][an]["group_ops"] for r in rs])
        for i, w in enumerate(["w0", "w0.1", "w1"]):
            F[f"{an}_ratio_to_rho_E_{w}"] = ols(ps, [r["arms"][an]["total_band"][i] / r["rho_E"]["band_negation"][i] for r in rs])
            F[f"{an}_total_{w}"] = ols(ps, [r["arms"][an]["total_band"][i] for r in rs])
    F["rho_E_negation_group_ops"] = ols(ps, [r["rho_E"]["negation"]["total_ops"] for r in rs])
    F["rho_E_negation_total_w0.1"] = ols(ps, [r["rho_E"]["band_negation"][1] for r in rs])
    F["rho_J_negation_group_ops"] = ols(ps, [r["arms"]["rho_J_negation"]["mean"]["total_ops"] for r in rs])
    return F

st1 = [r for r in rows if r["stage"] == 1]
F1 = fitset(st1); FA = fitset(rows)
for lab, F, G in (("fits_stage1", F1, d["fits_stage1"]), ("fits_all", FA, d["fits_all"])):
    for k, v in F.items():
        g = G[k]
        chk(f"{lab}.{k}.slope", g["slope"], v["slope"], 1e-9)
        chk(f"{lab}.{k}.intercept", g["intercept"], v["intercept"], 1e-9)
        chk(f"{lab}.{k}.se", g["se"], v["se"], 1e-9)
        chk(f"{lab}.{k}.n", g["n"], v["n"])
        # the producer uses a 3-decimal t table; check (a) the claimed CI equals slope -/+ t_tab*se with
        # t_tab = exact t rounded to 3 decimals, and (b) the exact-t CI differs only by |t_tab - t| * se
        t_tab = round(v["t975"], 3)
        for i, sgn in ((0, -1), (1, 1)):
            chk(f"{lab}.{k}.ci95[{i}]@t_tab", g["ci95"][i], v["slope"] + sgn * t_tab * v["se"], 1e-9)
            checked_abs = abs(g["ci95"][i] - v["ci95"][i]) <= abs(t_tab - v["t975"]) * v["se"] * (1 + 1e-6) + 1e-15
            if not checked_abs:
                mism.append({"name": f"{lab}.{k}.ci95[{i}] exact-t", "claimed": g["ci95"][i], "recomputed": v["ci95"][i]})
    chk(f"{lab}.label_n", len(G) - 1, len(F))

# ---------- frozen decision rule ----------
rule = {}
all_closed = True
any_below3 = False
for an in ["A", "B"]:
    ab = sum(1 for c in cells if c[1] == 1 and c[2] == an and c[3] == "above")
    ac = sum(1 for c in cells if c[1] == 1 and c[2] == an and c[4] == "above")
    lo = F1[f"{an}_ratio_to_rho_E_w0.1"]["ci95"][0]
    closed = ab == len(st1) and ac == len(st1) and lo > 0.2
    all_closed &= closed
    any_below3 |= any(c[3] == "below" for c in cells if c[1] == 1 and c[2] == an)
    rule[an] = {"above_band": ab, "above_cal": ac, "cells": len(st1), "slope_w0.1": F1[f"{an}_ratio_to_rho_E_w0.1"]["slope"],
                "ci95_lower_exact_t": lo, "meets_closed": closed}
    chk(f"verdict.{an}.above_band", d["verdict"]["per_arm"][an]["above_band"], ab)
    chk(f"verdict.{an}.above_cal", d["verdict"]["per_arm"][an]["above_cal"], ac)
    chk(f"verdict.{an}.slope", d["verdict"]["per_arm"][an]["ratio_slope_w0.1"], rule[an]["slope_w0.1"])
    chk(f"verdict.{an}.ci_lower", d["verdict"]["per_arm"][an]["ratio_slope_ci95_lower"], lo, 2e-3)
my_verdict = "advantage seen" if any_below3 else ("closed by data" if all_closed else "unresolved")
rule["verdict"] = my_verdict
checked += 1
if not d["verdict"]["verdict"].startswith("unresolved") or my_verdict != "unresolved":
    mism.append({"name": "verdict", "claimed": d["verdict"]["verdict"], "recomputed": my_verdict})

# ---------- scorecard and RESULTS.md numbers ----------
R = {r["p"]: r for r in rows}
sc = {}
sc["F1"] = {"A_above_band_cal": [rule["A"]["above_band"], rule["A"]["above_cal"]],
            "B_above_band_cal": [rule["B"]["above_band"], rule["B"]["above_cal"]]}
sc["F2"] = {"A_slope": F1["A_ratio_to_rho_E_w0.1"]["slope"], "B_slope": F1["B_ratio_to_rho_E_w0.1"]["slope"],
            "A_ci": F1["A_ratio_to_rho_E_w0.1"]["ci95"], "B_ci": F1["B_ratio_to_rho_E_w0.1"]["ci95"]}
sc["F2"]["held"] = all(0.3 <= sc["F2"][k] <= 0.7 for k in ("A_slope", "B_slope")) and \
    all(sc["F2"][k][0] > 0.2 for k in ("A_ci", "B_ci"))
sc["F3"] = {"A": F1["A_group_ops"]["slope"], "B": F1["B_group_ops"]["slope"], "rho": F1["rho_E_negation_group_ops"]["slope"]}
sc["F3"]["held"] = 0.8 <= sc["F3"]["A"] <= 1.25 and 0.8 <= sc["F3"]["B"] <= 1.25 and 0.4 <= sc["F3"]["rho"] <= 0.6
a12, b12, as12 = R[12841]["arms"]["A"], R[12841]["arms"]["B"], R[12841]["arms"]["A_S"]
sc["F4"] = {"B_w1": b12["total_band"][2], "A_w1": a12["total_band"][2], "held": b12["total_band"][2] < a12["total_band"][2]}
sc["F5"] = {"A_over_A_S_group_ops": a12["group_ops"] / as12["group_ops"]}
sc["F5"]["held"] = 0.4 <= sc["F5"]["A_over_A_S_group_ops"] <= 2.5
sr = [(r["p"], an, r["arms"][an]["smooth_rate_deg2"]) for r in rows for an in ("A", "B")]
sc["F6"] = {"per_prime_mean_range": [min(x[2] for x in sr), max(x[2] for x in sr)],
            "note": "json holds per-prime means over runs, not per-run rates; 'in every run' is not checkable from the json"}
sc["F6"]["means_in_range"] = all(0.4 <= x[2] <= 0.6 for x in sr)
f7 = []
for r in st1:
    for an, a in r["arms"].items():
        f7.append((r["p"], an, a["certified"], a["runs"]))
    f7.append((r["p"], "rho_E_neg", r["rho_E"]["negation"]["certified"], r["rho_E"]["negation"]["runs"]))
    f7.append((r["p"], "rho_E_plain", r["rho_E"]["plain"]["certified"], r["rho_E"]["plain"]["runs"]))
sc["F7"] = {"all_certified": all(c == n for _, _, c, n in f7), "cells": len(f7),
            "skipped": [r.get("skipped") for r in rows if "skipped" in r]}
sc["F8"] = {"stage2_primes_present": [r["p"] for r in rows if r["stage"] == 2], "stage2_not_run": d["stage2_not_run"],
            "protocol_stage2_list": [25601, 51203, 102407]}

# RESULTS.md table at w=0.1 (2-3 significant figures as printed)
def sig(x):
    if x >= 1e4:
        m = f"{x:.1e}".replace("e+0", "e")
        return m
    return f"{x:.0f}" if x >= 100 else f"{x:.0f}"

tab = []
for r in rows:
    vals = [r["arms"][an]["total_band"][1] / r["rho_E"]["band_negation"][1] for an in ("A", "B", "A_S", "B_S")]
    tab.append({"p": r["p"], "r": r["r"], "w0.1": vals})
# parse the RESULTS table
md_tab = {}
for line in res_md.splitlines():
    m = re.match(r"\|\s*(\d+)(?: \(stage 2\))?\s*\|\s*(\d+)\s*\|\s*([^|]+)\|\s*([^|]+)\|\s*([^|]+)\|\s*([^|]+)\|", line)
    if m:
        md_tab[int(m.group(1))] = (int(m.group(2)), [float(m.group(i).strip()) for i in range(3, 7)])
tab_check = []
for t in tab:
    if t["p"] in md_tab:
        rr, printed = md_tab[t["p"]]
        okr = rr == t["r"]
        oks = []
        for pv, mv in zip(printed, t["w0.1"]):
            # printed with 2 significant digits (e.g. 7.0e3, 406, 1.8e3): relative error of rounding <= 5% / leading digit
            oks.append(abs(pv - mv) / mv < 0.051)
        tab_check.append({"p": t["p"], "r_ok": okr, "printed": printed, "recomputed": t["w0.1"], "ok": okr and all(oks)})
w0 = sorted(((r["arms"][an]["ratio_neg_band"][0], r["p"], an) for r in rows for an in ("A", "B")))
w0_st1_B = [r["arms"]["B"]["ratio_neg_band"][0] for r in st1]
w01_whole = [r["arms"][an]["ratio_neg_band"][1] for r in rows for an in ("A", "B")]
texts = {
    "smallest_w0_ratio_wholeJ": {"value": w0[0][0], "p": w0[0][1], "arm": w0[0][2],
                                 "results_md_says": "18.7, for A at p = 211"},
    "smallest_w0_ratio_A": min(x for x in w0 if x[2] == "A"),
    "B_w0_stage1_range": [min(w0_st1_B), max(w0_st1_B)],
    "wholeJ_w0.1_range": [min(w01_whole), max(w01_whole)],
    "rho_fit_on_lnp_stage1": F1["rho_E_negation_group_ops"],
    "rho_fit_on_lnr_stage1": ols([r["r"] for r in st1], [r["rho_E"]["negation"]["total_ops"] for r in st1]),
    "unfrozen_fit_groupops_over_sqrt_r_A": ols([r["p"] for r in st1], [r["arms"]["A"]["group_ops"] / math.sqrt(r["r"]) for r in st1])["slope"],
    "unfrozen_fit_groupops_over_sqrt_r_B": ols([r["p"] for r in st1], [r["arms"]["B"]["group_ops"] / math.sqrt(r["r"]) for r in st1])["slope"],
    "h_range": [min(r["h"] for r in rows), max(r["h"] for r in rows)],
    "r_by_p": {r["p"]: r["r"] for r in rows},
}

# ---------- record metrics ----------
recm = []
for m in rec["metrics"]:
    mm = re.match(r"total_w0\.1_(a|b)_p(\d+)", m["name"])
    if mm:
        an = mm.group(1).upper(); p = int(mm.group(2))
        v = R[p]["arms"][an]["total_band"][1]; ref = R[p]["rho_E"]["band_negation"][1]
        okv = abs(v - m["value"]) / v < 5e-6 and abs(ref - m["reference"]) / ref < 5e-6
        exp_n = R[p]["arms"][an]["runs"]
        recm.append({"name": m["name"], "value": m["value"], "recomputed": v, "reference": m["reference"],
                     "ref_recomputed": ref, "n": m["n"], "runs": exp_n, "ok": okv, "n_ok": m["n"] == exp_n})
    elif m["name"].startswith("ratio_slope_w0.1_"):
        an = m["name"][-1].upper()
        f = F1[f"{an}_ratio_to_rho_E_w0.1"]
        okv = abs(round(f["slope"], 3) - m["value"]) < 1e-12 and all(abs(round(f["slope"] + sg * round(f["t975"], 3) * f["se"], 3) - m["spread"][i]) < 1e-12 for i, sg in ((0, -1), (1, 1)))
        recm.append({"name": m["name"], "value": m["value"], "spread": m["spread"], "recomputed": [f["slope"], f["ci95"]], "n": m["n"], "runs": f["n"], "ok": okv, "n_ok": m["n"] == f["n"]})
    else:
        recm.append({"name": m["name"], "unparsed": True})

out = {"source": src, "checked_values": checked, "mismatches": mism, "summary": summary, "rule": rule,
       "scorecard": sc, "results_table_check": tab_check, "results_text_numbers": texts,
       "record_metrics": recm, "rows": row_out,
       "t975": {str(k): t_q975(k) for k in (5, 7)}}
json.dump(out, open(out_path, "w"), indent=1, default=str)
print(f"checked {checked} values, {len(mism)} mismatches")
print("cells", summary["ic_cells"], "above_band", summary["above_band"], "above_cal", summary["above_cal"])
print("rule", json.dumps(rule))
print("t975(5)=%.5f t975(7)=%.5f" % (t_q975(5), t_q975(7)))
print("table rows ok:", sum(t["ok"] for t in tab_check), "/", len(tab_check))
print("record metrics ok:", sum(1 for m in recm if m.get("ok")), "/", len(recm), " n_ok:", sum(1 for m in recm if m.get("n_ok")))
for m in mism[:30]:
    print("MISMATCH", m)
