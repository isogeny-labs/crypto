#!/usr/bin/env python3
"""Reads results/*.json, writes RESULTS.json, RESULTS.md and graph.svg.
Pure Python. All numbers are [pcc:unverified] local development measurements."""
import json, glob, math, random, os, statistics

HERE = os.path.dirname(os.path.abspath(__file__))
R = os.path.join(HERE, "results")
WK = [("w0", "S_w0", 0.0), ("w0p1", "S_w0p1", 0.1), ("w1", "S_w1", 1.0), ("wcal", "S_wcal", None)]
random.seed(20261115)


def load():
    runs = {}
    for f in sorted(glob.glob(os.path.join(R, "run_q*_s*_t*.json"))):
        d = json.load(open(f))
        runs[(d["instance"]["q"], d["instance"]["seed"], int(os.path.basename(f).split("_t")[1][:-5]))] = d
    return runs


def fit(xs, ys):
    lx = [math.log(x) for x in xs]
    ly = [math.log(y) for y in ys]
    mx, my = sum(lx) / len(lx), sum(ly) / len(ly)
    b = sum((a - mx) * (c - my) for a, c in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)
    return b, my - b * mx  # slope, intercept (ln S = a + b ln q)


def wilson(k, n, z=1.96):
    p = k / n
    den = 1 + z * z / n
    c = (p + z * z / (2 * n)) / den
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / den
    return p, c - h, c + h


def pct(v, lo=2.5, hi=97.5):
    v = sorted(v)
    return v[int(len(v) * lo / 100)], v[min(len(v) - 1, int(len(v) * hi / 100))]


runs = load()
qs = sorted({k[0] for k in runs})
main = {q: [runs[(q, s, 0)] for s in (1, 2, 3) if (q, s, 0) in runs] for q in qs}


def cell_mean(q, arm, key, idx=None):
    return statistics.mean(r[arm][key] for r in main[q] if idx is None or r in idx)


def curves(resample):
    """cell means per arm/weight for a resample (list of seed lists)"""
    out = {}
    for arm in ("ic", "rho"):
        for wn, key, _ in WK:
            out[(arm, wn)] = [statistics.mean(r[arm][key] for r in resample[q]) for q in qs]
    return out


def analysis(resample):
    c = curves(resample)
    res = {}
    for arm in ("ic", "rho"):
        for wn, _, _ in WK:
            res[(arm, wn)] = fit(qs, c[(arm, wn)])
    cross = {}
    for wn, _, _ in WK:
        bi, ai = res[("ic", wn)]
        br, ar = res[("rho", wn)]
        cross[wn] = math.exp((ar - ai) / (bi - br)) if bi < br else None
    ratio = {wn: [c[("ic", wn)][i] / c[("rho", wn)][i] for i in range(len(qs))] for wn, _, _ in WK}
    extra = {}
    # rho walk-only exponent (steps) and q>=251 exponent of S_w0
    extra["rho_steps_slope"] = fit(qs, [statistics.mean(r["rho"]["walk_steps"] for r in resample[q]) for q in qs])[0]
    extra["rho_S0_slope_q_ge_251"] = fit(qs[2:], c[("rho", "w0")][2:])[0]
    extra["solver_per_trial_slope"] = fit(qs, [statistics.mean(r["ic"]["solver_native_per_trial"] for r in resample[q]) for q in qs])[0]
    return res, cross, ratio, extra


point = analysis(main)
B = 2000
boots = []
for _ in range(B):
    rs = {q: [random.choice(main[q]) for _ in main[q]] for q in qs}
    boots.append(analysis(rs))


def ci_of(getter):
    vals = [getter(b) for b in boots]
    vals = [v for v in vals if v is not None]
    return pct(vals) if len(vals) > 100 else None


out = {"label": "[pcc:unverified]", "git_head": runs[(qs[0], 1, 0)]["git_head"], "qs": qs,
       "bootstrap": f"{B} resamples of the 3 instance seeds within each q cell (seed 20261115)"}
out["cells"] = {}
for q in qs:
    cell = {"ell": [r["instance"]["ell"] for r in main[q]], "cofactor": [r["instance"]["cofactor"] for r in main[q]],
            "w_cal_per_run": [r["w_cal"] for r in main[q]]}
    for arm in ("ic", "rho"):
        for wn, key, _ in WK:
            vals = [r[arm][key] for r in main[q]]
            cell[f"{arm}_{wn}"] = {"per_seed": vals, "mean": statistics.mean(vals)}
    ic = [r["ic"] for r in main[q]]
    trials = sum(i["trials"] for i in ic)
    succ = sum(i["success_trials"] for i in ic)
    p, lo, hi = wilson(succ, trials)
    cell["decomposition_probability"] = {"successes": succ, "trials": trials, "p": p, "wilson95": [lo, hi],
                                         "p_times_24": p * 24, "within_30pct_of_1_over_24": 0.0292 <= p <= 0.0542}
    cell["degenerate_R_rate"] = sum(i["degenerate_R"] for i in ic) / trials
    cell["relations"] = [i["relations"] for i in ic]
    cell["classes"] = [i["fb_classes"] for i in ic]
    cell["solver_native_per_trial"] = statistics.mean(i["solver_native_per_trial"] for i in ic)
    cell["solver_time_per_trial_s"] = statistics.mean(i["solver_time_per_trial_s"] for i in ic)
    cell["la_dense_mulmods"] = [i["la_mulmods_dense"] for i in ic]
    cell["la_share_of_S_w0p1"] = [0.1 * i["la_mulmods_dense"] / i["S_w0p1"] for i in ic]
    cell["la_model_wiedemann_mulmods"] = [i["la_model_wiedemann_mulmods"] for i in ic]
    cell["ic_group_adds"] = [i["group_adds"] for i in ic]
    cell["rho_steps"] = [r["rho"]["walk_steps"] for r in main[q]]
    cell["rho_group_adds"] = [r["rho"]["group_adds"] for r in main[q]]
    cell["all_solved"] = all(r["ic"]["solved"] and r["rho"]["solved"] for r in main[q])
    cell["rejected_candidates"] = sum(i["rejected_candidates"] for i in ic)
    cell["ratio_ic_over_rho"] = {wn: statistics.mean(r["ic"][key] for r in main[q]) / statistics.mean(r["rho"][key] for r in main[q]) for wn, key, _ in WK}
    out["cells"][str(q)] = cell
pooled_k = sum(out["cells"][str(q)]["decomposition_probability"]["successes"] for q in qs if q >= 131)
pooled_n = sum(out["cells"][str(q)]["decomposition_probability"]["trials"] for q in qs if q >= 131)
p, lo, hi = wilson(pooled_k, pooled_n)
out["decomposition_probability_pooled_q_ge_131"] = {"successes": pooled_k, "trials": pooled_n, "p": p, "wilson95": [lo, hi], "p_times_24": p * 24}

res, cross, ratio, extra = point
out["fits"] = {}
for arm in ("ic", "rho"):
    for wn, _, _ in WK:
        out["fits"][f"{arm}_{wn}"] = {"exponent": res[(arm, wn)][0], "ln_const": res[(arm, wn)][1],
                                      "exponent_ci95": ci_of(lambda b, a=arm, w=wn: b[0][(a, w)][0])}
out["crossover_q_extrapolated"] = {wn: {"q_star": cross[wn], "ci95": ci_of(lambda b, w=wn: b[1][w])} for wn, _, _ in WK}
out["ratio_ic_over_rho_by_q"] = {wn: {"values": ratio[wn],
                                      "ci95_per_q": [ci_of(lambda b, w=wn, i=i: b[2][w][i]) for i in range(len(qs))]} for wn, _, _ in WK}
out["extra_exponents"] = {k: {"value": v, "ci95": ci_of(lambda b, k=k: b[3][k])} for k, v in extra.items()}

# A/A
aa = {}
for q in (131, 251):
    a, b = runs[(q, 1, 0)], runs[(q, 1, 1)]
    aa[str(q)] = {"ic_ratio_t1_over_t0": {wn: b["ic"][key] / a["ic"][key] for wn, key, _ in WK},
                  "rho_ratio_t1_over_t0": {wn: b["rho"][key] / a["rho"][key] for wn, key, _ in WK},
                  "ic_trials": [a["ic"]["trials"], b["ic"]["trials"]], "rho_steps": [a["rho"]["walk_steps"], b["rho"]["walk_steps"]]}
out["aa"] = aa


# ---- rho diagnostics, wall times, component model (post hoc, extrapolation) ----
allruns = [runs[(q, sd, 0)] for q in qs for sd in (1, 2, 3)]
steps_over_sqrt = {str(q): [r["rho"]["walk_steps"] / math.sqrt(r["instance"]["ell"]) for r in main[q]] for q in qs}
sl_ell = fit([r["instance"]["ell"] for r in allruns], [r["rho"]["walk_steps"] for r in allruns])[0]
out["rho_diagnostics"] = {"walk_steps_over_sqrt_ell_per_q": steps_over_sqrt,
                          "walk_steps_vs_ell_loglog_slope_all_18_runs": sl_ell,
                          "pooled_steps_over_sqrt_ell_mean": statistics.mean(v for l in steps_over_sqrt.values() for v in l),
                          "theory_negation_walk": "about 0.89 (sqrt(pi/4) ... 0.89) for ideal rho on ell/2 classes; DP delay and fruitless cycles add"}
out["wall_s_mean"] = {str(q): {"ic": statistics.mean(r["ic"]["wall_s"] for r in main[q]), "rho": statistics.mean(r["rho"]["wall_s"] for r in main[q])} for q in qs}
# OLS adds = A + B*steps for rho
xs_ = [r["rho"]["walk_steps"] for r in allruns]; ys_ = [r["rho"]["group_adds"] for r in allruns]
mx_, my_ = statistics.mean(xs_), statistics.mean(ys_)
B_rho = sum((x - mx_) * (y - my_) for x, y in zip(xs_, ys_)) / sum((x - mx_) ** 2 for x in xs_)
A_rho = my_ - B_rho * mx_
rho_c = statistics.mean(r["rho"]["walk_steps"] / math.sqrt(r["instance"]["ell"]) for r in allruns)
ell_c = statistics.mean(r["instance"]["ell"] / r["instance"]["q"] ** 4 for r in allruns)
nat_per_step = statistics.mean(r["rho"]["native"]["total"] / r["rho"]["walk_steps"] for r in allruns)
p_hat = out["decomposition_probability_pooled_q_ge_131"]["p"]
cls_c = statistics.mean(r["ic"]["fb_classes"] / r["instance"]["q"] for r in allruns)
sigma = statistics.mean(r["ic"]["solver_native_per_trial"] for r in allruns if r["instance"]["q"] >= 251)
kappa = statistics.mean(r["ic"]["native_fq_ops_total"] / (r["ic"]["solver_native_per_trial"] * r["ic"]["trials"]) for r in allruns)
A_ic = statistics.mean(r["ic"]["group_adds"] - r["ic"]["trials"] - 4 * r["ic"]["relations"] for r in allruns)
lam = statistics.mean(r["ic"]["la_mulmods_dense"] / r["ic"]["classes_used"] ** 3 for r in allruns if r["instance"]["q"] >= 1021)
wie = statistics.mean(r["ic"]["la_model_wiedemann_mulmods"] / r["ic"]["classes_used"] ** 2 for r in allruns if r["instance"]["q"] >= 1021)
wc = statistics.median(r["w_cal"] for r in allruns)


def model(q, w, la="wiedemann"):
    cls = cls_c * q
    rel = cls + 10
    T = rel / p_hat
    la_ops = (lam * cls ** 3) if la == "dense" else (wie * cls ** 2)
    ic = (T + 4 * rel + A_ic) + w * (kappa * sigma * T + la_ops)
    steps = rho_c * math.sqrt(ell_c * q ** 4)
    rh = (A_rho + B_rho * steps) + w * nat_per_step * steps
    return ic, rh


def qstar(w, la):
    prev = None
    q = 20.0
    while q < 1e14:
        ic, rh = model(q, w, la)
        cur = ic < rh
        if prev is not None and cur != prev:
            return q
        prev = cur
        q *= 1.02
    return None


mdl = {"constants": {"rho_steps_over_sqrt_ell": rho_c, "ell_over_q4": ell_c, "rho_adds_intercept": A_rho, "rho_adds_per_step": B_rho,
                     "rho_native_per_step": nat_per_step, "p_decomp": p_hat, "classes_over_q": cls_c, "solver_mulmods_per_trial": sigma,
                     "native_over_solver_kappa": kappa, "ic_group_adds_fixed": A_ic, "dense_LA_mulmods_over_classes_cubed": lam,
                     "wiedemann_model_over_classes_squared": wie, "w_cal_median": wc},
       "note": "component model fitted to the measured cells; the crossovers below are extrapolations over many orders of magnitude [pcc:unverified]"}
mdl["crossover"] = {}
for wn, w in (("w0", 0.0), ("w0p1", 0.1), ("w1", 1.0), ("wcal", wc)):
    mdl["crossover"][wn] = {"q_star_wiedemann_LA_model": qstar(w, "wiedemann"), "q_star_dense_LA": qstar(w, "dense"),
                            "ratio_at_q_2039_wiedemann": model(2039, w)[0] / model(2039, w)[1],
                            "ratio_at_q_1e6_wiedemann": model(1e6, w)[0] / model(1e6, w)[1],
                            "ratio_at_q_1e9_wiedemann": model(1e9, w)[0] / model(1e9, w)[1]}
out["component_model"] = mdl

# oracle
orc = {}
for f in sorted(glob.glob(os.path.join(R, "oracle*.json"))):
    d = json.load(open(f))
    orc[os.path.basename(f)[:-5]] = {k: d[k] for k in ("q", "trials_requested", "tested_nondegenerate", "agree", "degenerate_R", "R_with_root_of_u_in_Fq_skipped", "oracle_decompositions", "solver_decompositions_group_law_verified")}
out["oracle"] = orc

# predictions
pred = []
ok = lambda b: "confirmed" if b else "refuted"
p1 = [out["cells"][str(q)]["decomposition_probability"]["within_30pct_of_1_over_24"] for q in qs if q >= 131]
pred.append(("P1", ok(all(p1)), f"per-q p*24 for q>=131: " + ", ".join(f"{out['cells'][str(q)]['decomposition_probability']['p_times_24']:.2f}" for q in qs if q >= 131) + "; band 0.70-1.30"))
p2 = [out["fits"][f"ic_{wn}"]["exponent"] for wn, _, _ in WK]
pred.append(("P2", ok(all(e <= 1.7 for e in p2)), "IC exponents w0/w0.1/w1/wcal: " + ", ".join(f"{e:.2f}" for e in p2)))
p3 = [out["fits"][f"rho_{wn}"]["exponent"] for wn in ("w0", "w1")]
pred.append(("P3", ok(all(1.85 <= e <= 2.15 for e in p3)), "rho exponents w0/w1: " + ", ".join(f"{e:.2f}" for e in p3) + f"; walk-steps-only {extra['rho_steps_slope']:.2f}; S_w0 for q>=251 {extra['rho_S0_slope_q_ge_251']:.2f}"))
tot_t = sum(v["tested_nondegenerate"] for k, v in orc.items() if "supp" not in k)
tot_a = sum(v["agree"] for k, v in orc.items() if "supp" not in k)
pred.append(("P4", ok(tot_t == tot_a), f"registered oracle runs agree {tot_a}/{tot_t}; supplemental " + ", ".join(f"{v['agree']}/{v['tested_nondegenerate']}" for k, v in orc.items() if "supp" in k)))
beyond = [cross[wn] is None or cross[wn] > 1021 for wn in ("w0p1", "w1", "wcal")]
win0 = all(r < 1 for r in ratio["w0"])
loss = all(all(r > 1 for r in ratio[wn]) for wn in ("w0p1", "w1", "wcal"))
pred.append(("P5", ok(win0 and loss and all(beyond)), f"ratio IC/rho at w=0 by q: {[round(r, 3) for r in ratio['w0']]}; w=0.1: {[round(r) for r in ratio['w0p1']]}; w_cal: {[round(r, 1) for r in ratio['wcal']]}; q* (fit) w0={cross['w0']}, w0.1={cross['w0p1']}, w1={cross['w1']}, wcal={cross['wcal']}"))
pred.append(("P6", ok(0 <= extra["solver_per_trial_slope"] <= 0.3), f"solver per-trial exponent {extra['solver_per_trial_slope']:.3f}"))
sh = max(max(out["cells"][str(q)]["la_share_of_S_w0p1"]) for q in qs)
pred.append(("P7", ok(sh < 0.05), f"max dense-LA share of S_IC(0.1) = {sh:.4f}"))
dg = [out["cells"][str(q)]["degenerate_R_rate"] for q in qs if q >= 131]
pred.append(("P8", ok(all(d < 0.05 for d in dg)), "degenerate-R rate q>=131: " + ", ".join(f"{d:.4f}" for d in dg)))
icr = [aa[q]["ic_ratio_t1_over_t0"][w] for q in aa for w in ("w0", "w0p1", "w1")]
rhr = [aa[q]["rho_ratio_t1_over_t0"][w] for q in aa for w in ("w0", "w1")]
pred.append(("P9", ok(all(0.5 <= x <= 2 for x in icr) and all(0.3 <= x <= 3.3 for x in rhr)), f"IC A/A ratios {[round(x, 2) for x in icr]}; rho A/A ratios {[round(x, 2) for x in rhr]}"))
out["predictions"] = [{"id": a, "outcome": b, "evidence": c} for a, b, c in pred]
json.dump(out, open(os.path.join(HERE, "RESULTS.json"), "w"), indent=1, default=str)

# ---------------------------------------------------------------- RESULTS.md
L = ["# RESULTS (all numbers [pcc:unverified], local development check)", "",
     f"git HEAD {out['git_head']}; bootstrap: {out['bootstrap']}.", "",
     "## Per-cell means over 3 instance seeds (S in group-addition equivalents)", "",
     "| q | ell (3 seeds) | IC S(w=0) | IC S(0.1) | IC S(1) | IC S(w_cal) | rho S(0) | rho S(0.1) | rho S(1) | rho S(w_cal) |", "|---|---|---|---|---|---|---|---|---|---|"]
for q in qs:
    c = out["cells"][str(q)]
    L.append(f"| {q} | {', '.join('%.2g' % e for e in c['ell'])} | " + " | ".join(f"{c[f'{a}_{w}']['mean']:.3g}" for a in ("ic", "rho") for w in ("w0", "w0p1", "w1", "wcal")) + " |")
L += ["", "Distance to matched rho, S_IC / S_rho (ratio of cell means; <1 means IC cheaper):", "",
      "| q | w=0 | w=0.1 | w=1 | w_cal |", "|---|---|---|---|---|"]
for i, q in enumerate(qs):
    L.append(f"| {q} | " + " | ".join(f"{ratio[w][i]:.3g} [{ci_of(lambda b, w=w, i=i: b[2][w][i])[0]:.3g}, {ci_of(lambda b, w=w, i=i: b[2][w][i])[1]:.3g}]" for w in ("w0", "w0p1", "w1", "wcal")) + " |")
L += ["", "w_cal (best-of-3 scalar F_q mulmod time / group-addition time, per run, noisy shared host): " + ", ".join(f"q={q}: {statistics.mean(out['cells'][str(q)]['w_cal_per_run']):.4f}" for q in qs), "",
      "## Decomposition probability per trial (degenerate R counted as failures)", "", "| q | trials | R with verified relation | p | Wilson 95% | 24 p | degenerate-R rate | solver cost per trial (counted F_q mulmods) | solver time per trial (ms) |", "|---|---|---|---|---|---|---|---|---|"]
for q in qs:
    c = out["cells"][str(q)]
    d = c["decomposition_probability"]
    L.append(f"| {q} | {d['trials']} | {d['successes']} | {d['p']:.4f} | [{d['wilson95'][0]:.4f}, {d['wilson95'][1]:.4f}] | {d['p_times_24']:.2f} | {c['degenerate_R_rate']:.4f} | {c['solver_native_per_trial']:.3g} | {1000 * c['solver_time_per_trial_s']:.3f} |")
d = out["decomposition_probability_pooled_q_ge_131"]
L += ["", f"Pooled q >= 131: {d['successes']}/{d['trials']} = {d['p']:.4f}, Wilson 95% [{d['wilson95'][0]:.4f}, {d['wilson95'][1]:.4f}], 24p = {d['p_times_24']:.2f}.", "",
      "## Fitted exponents (ln S = a + b ln q over the six cell means; 95% seed-bootstrap interval)", "",
      "| arm | w=0 | w=0.1 | w=1 | w_cal |", "|---|---|---|---|---|"]
for arm in ("ic", "rho"):
    L.append(f"| {arm} | " + " | ".join(f"{out['fits'][f'{arm}_{w}']['exponent']:.2f} [{out['fits'][f'{arm}_{w}']['exponent_ci95'][0]:.2f}, {out['fits'][f'{arm}_{w}']['exponent_ci95'][1]:.2f}]" for w in ("w0", "w0p1", "w1", "wcal")) + " |")
ex = out["extra_exponents"]
L += ["", "Secondary (post hoc, not preregistered): rho walk steps only: %.2f [%.2f, %.2f]; rho S(w=0) over q >= 251 only: %.2f [%.2f, %.2f]; IC solver cost per trial: %.3f [%.3f, %.3f]." % (
    ex["rho_steps_slope"]["value"], *ex["rho_steps_slope"]["ci95"], ex["rho_S0_slope_q_ge_251"]["value"], *ex["rho_S0_slope_q_ge_251"]["ci95"],
    ex["solver_per_trial_slope"]["value"], *ex["solver_per_trial_slope"]["ci95"]), "",
      "## Extrapolated crossover q* (fit of IC against fit of rho; extrapolation over orders of magnitude, indicative only)", "",
      "| w | q* | 95% seed-bootstrap |", "|---|---|---|"]
for w in ("w0", "w0p1", "w1", "wcal"):
    cc = out["crossover_q_extrapolated"][w]
    L.append(f"| {w} | {('%.3g' % cc['q_star']) if cc['q_star'] else 'none (IC exponent >= rho exponent)'} | {('[%.2g, %.2g]' % tuple(cc['ci95'])) if cc['ci95'] else 'n/a (some resamples have no crossover)'} |")
L += ["", "At w = 0 IC is already cheaper than rho over the whole ladder, so the w=0 row gives the q at which the fitted lines would cross from the other side (it is below the ladder or meaningless if the ratio at the ladder is < 1 and falling).", "",
      "## A/A (two internal seeds, same instance, seed 1)", ""]
for q in aa:
    L.append(f"* q={q}: IC S ratios (t1/t0) w0={aa[q]['ic_ratio_t1_over_t0']['w0']:.2f}, w0.1={aa[q]['ic_ratio_t1_over_t0']['w0p1']:.2f}, w1={aa[q]['ic_ratio_t1_over_t0']['w1']:.2f}; rho S ratios w0={aa[q]['rho_ratio_t1_over_t0']['w0']:.2f}, w1={aa[q]['rho_ratio_t1_over_t0']['w1']:.2f}; rho steps {aa[q]['rho_steps']}")
L += ["", "## Oracle agreement (Macaulay solver vs pair-hash oracle, non-degenerate R)", "", "| run | q | R tested | agree | oracle decompositions | solver decompositions (group-law verified) | degenerate R | R skipped (root of u in F_q) |", "|---|---|---|---|---|---|---|---|"]
for k, v in orc.items():
    L.append(f"| {k} | {v['q']} | {v['tested_nondegenerate']} | {v['agree']} | {v['oracle_decompositions']} | {v['solver_decompositions_group_law_verified']} | {v['degenerate_R']} | {v['R_with_root_of_u_in_Fq_skipped']} |")
L += ["", "## Dense LA vs model", "", "| q | relations | classes | dense LA mulmods | block-Wiedemann model 2*m*nnz | dense share of S_IC(0.1) |", "|---|---|---|---|---|---|"]
for q in qs:
    c = out["cells"][str(q)]
    L.append(f"| {q} | {c['relations']} | {c['classes']} | {c['la_dense_mulmods']} | {c['la_model_wiedemann_mulmods']} | {[round(x, 5) for x in c['la_share_of_S_w0p1']]} |")
L += ["", "## Preregistered predictions", "", "| id | outcome | evidence |", "|---|---|---|"]
for p_ in out["predictions"]:
    L.append(f"| {p_['id']} | {p_['outcome']} | {p_['evidence']} |")

mm = out["component_model"]
L += ["", "## Rho diagnostics and wall time (post hoc)", "",
      "walk steps / sqrt(ell) per q (3 seeds): " + "; ".join(f"q={q}: " + ", ".join(f"{v:.2f}" for v in out['rho_diagnostics']['walk_steps_over_sqrt_ell_per_q'][str(q)]) for q in qs),
      "", f"log-log slope of walk steps on ell over all 18 runs: {out['rho_diagnostics']['walk_steps_vs_ell_loglog_slope_all_18_runs']:.3f} (ideal 0.5). Pooled steps/sqrt(ell) = {out['rho_diagnostics']['pooled_steps_over_sqrt_ell_mean']:.2f}.",
      f"Rho group additions = {mm['constants']['rho_adds_intercept']:.0f} + {mm['constants']['rho_adds_per_step']:.3f} * steps (OLS, 18 runs): the intercept is the 64-entry walk pool and is why S_rho grows with exponent below 2 over this ladder.", "",
      "Mean wall seconds per run (IC / rho), same host: " + "; ".join(f"q={q}: {out['wall_s_mean'][str(q)]['ic']:.1f} / {out['wall_s_mean'][str(q)]['rho']:.2f}" for q in qs), "",
      "## Component-model projection (extrapolation, NOT a measurement)", "",
      "Constants fitted from the 18 main runs: " + ", ".join(f"{k}={v:.4g}" for k, v in mm["constants"].items()), "",
      "| w | q* (IC cheaper above), Wiedemann LA model | q*, dense LA | ratio IC/rho at q=2039 | at q=1e6 | at q=1e9 |", "|---|---|---|---|---|---|"]
for wn in ("w0", "w0p1", "w1", "wcal"):
    c_ = mm["crossover"][wn]
    f_ = lambda v: ("%.3g" % v) if v else "none below 1e14"
    L.append(f"| {wn} | {f_(c_['q_star_wiedemann_LA_model'])} | {f_(c_['q_star_dense_LA'])} | {c_['ratio_at_q_2039_wiedemann']:.3g} | {c_['ratio_at_q_1e6_wiedemann']:.3g} | {c_['ratio_at_q_1e9_wiedemann']:.3g} |")
L += ["", "Reading the model: at w = 0 only group additions are counted, IC needs about 12 q of them against rho's ~0.38 q^2, so IC is cheaper at every q (no crossover): this is the zero-priced-work win, an accounting artifact of w = 0. For w > 0 the relation phase costs a constant ~4.6e5 mulmods per trial (linear in q overall) but the linear-algebra phase is Theta(q^2) even when sparse (the block-Wiedemann model 2*m*nnz = ~2.5 q^2 mulmods), the same exponent as rho. The basic algorithm (factor base of size q, no large primes) therefore has exponent 2, not 1.5, asymptotically; the sub-linear measured exponents on this ladder (0.7-0.9) are a small-q artifact of the linear solver term dominating. w = 1: LA alone exceeds the whole rho cost in the model, so no crossover; w = 0.1 and w_cal: crossover only at q* above 5e4, i.e. group order above 1e18 (far beyond the measured ladder and beyond a toy-size study). Projections beyond q = 2039 are extrapolations."]

open(os.path.join(HERE, "RESULTS.md"), "w").write("\n".join(L) + "\n")


# ---------------------------------------------------------------- SVG graph
def svg():
    W, H = 1100, 520
    colors = {"ic_w0": "#1b7837", "ic_w0p1": "#e08214", "ic_w1": "#b2182b", "ic_wcal": "#762a83", "rho_w0": "#2166ac", "rho_w1": "#67a9cf"}
    def panel(x0, y0, w, h, xs, series, ylab, ymin, ymax, title, hline=None):
        lx = lambda q: x0 + w * (math.log(q) - math.log(xs[0])) / (math.log(xs[-1]) - math.log(xs[0]))
        ly = lambda v: y0 + h - h * (math.log10(v) - math.log10(ymin)) / (math.log10(ymax) - math.log10(ymin))
        s = [f'<text x="{x0}" y="{y0 - 14}" font-size="14" font-weight="600" fill="#222">{title}</text>',
             f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" fill="none" stroke="#999"/>']
        e = int(math.log10(ymin))
        while 10 ** e <= ymax:
            if 10 ** e >= ymin:
                y = ly(10 ** e)
                s.append(f'<line x1="{x0}" x2="{x0 + w}" y1="{y:.1f}" y2="{y:.1f}" stroke="#ddd"/><text x="{x0 - 6}" y="{y + 4:.1f}" font-size="11" text-anchor="end" fill="#444">1e{e}</text>')
            e += 1
        for q in xs:
            s.append(f'<line x1="{lx(q):.1f}" x2="{lx(q):.1f}" y1="{y0 + h}" y2="{y0 + h + 4}" stroke="#666"/><text x="{lx(q):.1f}" y="{y0 + h + 17}" font-size="11" text-anchor="middle" fill="#444">{q}</text>')
        s.append(f'<text x="{x0 + w / 2}" y="{y0 + h + 36}" font-size="12" text-anchor="middle" fill="#333">q  (group order about q^4)</text>')
        s.append(f'<text x="{x0 - 52}" y="{y0 + h / 2}" font-size="12" text-anchor="middle" fill="#333" transform="rotate(-90 {x0 - 52} {y0 + h / 2})">{ylab}</text>')
        if hline:
            y = ly(hline)
            s.append(f'<line x1="{x0}" x2="{x0 + w}" y1="{y:.1f}" y2="{y:.1f}" stroke="#000" stroke-dasharray="5 4"/><text x="{x0 + 4}" y="{y - 4:.1f}" font-size="11" fill="#000">IC = rho</text>')
        for name, pts, col, dash in series:
            path = " ".join(f"{'M' if i == 0 else 'L'}{lx(q):.1f},{ly(v):.1f}" for i, (q, v) in enumerate(pts))
            s.append(f'<path d="{path}" fill="none" stroke="{col}" stroke-width="2"{(" stroke-dasharray=%s" % chr(34) + dash + chr(34)) if dash else ""}/>')
            for q, v in pts:
                s.append(f'<circle cx="{lx(q):.1f}" cy="{ly(v):.1f}" r="3" fill="{col}"/>')
            q, v = pts[-1]
            s.append(f'<text x="{lx(q) + 6:.1f}" y="{ly(v) + 4:.1f}" font-size="11" fill="{col}">{name}</text>')
        return s
    sers = []
    names = {"w0": "w=0", "w0p1": "w=0.1", "w1": "w=1", "wcal": "w=cal"}
    for key in ("ic_w0", "ic_w0p1", "ic_w1", "ic_wcal"):
        sers.append((f"IC {names[key[3:]]}", [(q, out["cells"][str(q)][key]["mean"]) for q in qs], colors[key], None))
    for key in ("rho_w0", "rho_w1"):
        sers.append((f"rho {names[key[4:]]}", [(q, out["cells"][str(q)][key]["mean"]) for q in qs], colors[key], "6 3"))
    ratios = []
    for key in ("w0", "w0p1", "w1", "wcal"):
        ratios.append((f"IC/rho {names[key]}", [(q, ratio[key][i]) for i, q in enumerate(qs)], colors["ic_" + key], None))
    parts = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="Helvetica, Arial, sans-serif"><rect width="{W}" height="{H}" fill="#fff"/>']
    parts += panel(70, 50, 400, 380, qs, sers, "S (group-addition equivalents), mean of 3 seeds", 1e3, 1e11, "Counted cost S vs q (solid IC, dashed rho)")
    parts += panel(640, 50, 330, 380, qs, ratios, "S_IC / S_rho", 1e-2, 1e4, "Distance to matched rho", hline=1.0)
    parts.append(f'<text x="{W / 2}" y="{H - 20}" font-size="12" text-anchor="middle" fill="#555">[pcc:unverified] local development check, genus-2 Jacobian over F_(q^2), no cover; w = native-op price in GAE</text></svg>')
    open(os.path.join(HERE, "graph.svg"), "w").write("\n".join(parts))


svg()
print(open(os.path.join(HERE, "RESULTS.md")).read())
