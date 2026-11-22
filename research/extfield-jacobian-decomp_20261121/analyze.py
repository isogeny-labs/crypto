#!/usr/bin/env python3
"""Reads results/*.json, writes RESULTS.json, RESULTS.md and graph.svg (pure Python, no numpy)."""
import json, math, os, random, glob, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "results")
QS = [131, 251, 509, 1021, 2039]
SEEDS = [1, 2, 3]
ARMS = ["control_a1", "a0.75_c1", "a0.75_c2", "a0.6_c1", "a0.6_c2"]
WS = {"w0": "S_w0", "w0.1": "S_w0p1", "w1": "S_w1", "wcal": "S_wcal"}
REL = {"w0": "S_rel_w0", "w0.1": "S_rel_w0p1", "w1": "S_rel_w1"}
LA = {"w0": "S_la_w0", "w0.1": "S_la_w0p1", "w1": "S_la_w1"}
P_DECOMP = 0.0402

def load(p):
    with open(p) as f:
        return json.load(f)

lp, lp1, rho = {}, {}, {}
for q in QS:
    for s in SEEDS:
        p = os.path.join(RES, f"lp_q{q}_s{s}_t0.json")
        if os.path.exists(p) and os.path.getsize(p) > 0:
            lp[(q, s)] = load(p)
        p = os.path.join(RES, f"rho_q{q}_s{s}.json")
        if os.path.exists(p) and os.path.getsize(p) > 0:
            rho[(q, s)] = load(p)
    p = os.path.join(RES, f"lp_q{q}_s1_t1.json")
    if os.path.exists(p) and os.path.getsize(p) > 0:
        lp1[q] = load(p)

def arm_of(run, name):
    for a in run["ic_lp"]["arms"]:
        if a["arm"] == name:
            return a
    raise KeyError(name)

def mean(x): return sum(x) / len(x)

def ols(xs, ys):
    lx = [math.log(v) for v in xs]; ly = [math.log(v) for v in ys]
    mx, my = mean(lx), mean(ly)
    b = sum((a - mx) * (c - my) for a, c in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)
    return b, math.exp(my - b * mx)

def pct(v, p):
    v = sorted(v); k = (len(v) - 1) * p
    lo, hi = int(math.floor(k)), int(math.ceil(k))
    return v[lo] + (v[hi] - v[lo]) * (k - lo)

rng = random.Random(20261121)
NB = 2000

def boot_fit(valsets):
    """valsets: {q: [per-seed S]}; returns (slope, intercept, ci95 of slope)."""
    qs = sorted(valsets)
    b0, a0 = ols(qs, [mean(valsets[q]) for q in qs])
    bs = []
    for _ in range(NB):
        ys = [mean([rng.choice(valsets[q]) for _ in valsets[q]]) for q in qs]
        bs.append(ols(qs, ys)[0])
    return b0, a0, [pct(bs, 0.025), pct(bs, 0.975)]

def rho_S(q, s, w):
    r = rho[(q, s)]["rho"]
    return r[{"w0": "S_w0", "w0.1": "S_w0p1", "w1": "S_w1", "wcal": "S_wcal"}[w]]

out = {"label": "[pcc:unverified]", "qs": QS, "seeds": SEEDS, "arms": ARMS, "bootstrap_resamples": NB,
       "git_head_of_runs": sorted({r["git_head"] for r in lp.values()}), "cells": {}, "rho": {}, "ratios": {}, "fits": {},
       "la_share_q2039": {}, "aa": {}, "censored": [], "unsolved": []}

# --- cells
for a in ARMS:
    out["cells"][a] = {}
    for q in QS:
        runs = [arm_of(lp[(q, s)], a) for s in SEEDS if (q, s) in lp]
        if not runs:
            continue
        cell = {"n_seeds": len(runs), "censored_seeds": sum(r["censored"] for r in runs),
                "unsolved_seeds": sum(not r["solved"] for r in runs),
                "nf_classes": runs[0]["nf_classes"], "f_fraction": runs[0]["f_fraction"], "clamped": runs[0]["clamped"]}
        for key in ["trials_to_stop", "accepted_rows", "rows_at_solve", "complete_at_solve", "ncols_at_solve", "attempts",
                    "wiedemann_runs", "wrong_k_attempts", "filter_ops", "wiedemann_mulmods", "stream_native", "stream_group_adds",
                    "arm_extra_group_adds"]:
            cell[key] = {"per_seed": [r[key] for r in runs], "mean": mean([r[key] for r in runs])}
        cell["lp_counts_sum"] = [sum(r["accepted_by_lp_count"][i] for r in runs) for i in range(3)]
        for w, k in WS.items():
            cell["S_" + w] = {"per_seed": [r[k] for r in runs], "mean": mean([r[k] for r in runs])}
        for w in REL:
            cell["Srel_" + w] = {"per_seed": [r[REL[w]] for r in runs], "mean": mean([r[REL[w]] for r in runs])}
            cell["Sla_" + w] = {"per_seed": [r[LA[w]] for r in runs], "mean": mean([r[LA[w]] for r in runs])}
        out["cells"][a][str(q)] = cell
        for r, s in zip(runs, [s for s in SEEDS if (q, s) in lp]):
            if r["censored"]:
                out["censored"].append({"arm": a, "q": q, "seed": s, "trials": r["trials_to_stop"]})
            elif not r["solved"]:
                out["unsolved"].append({"arm": a, "q": q, "seed": s})
streams = {}
for q in QS:
    rs = [lp[(q, s)]["ic_lp"] for s in SEEDS if (q, s) in lp]
    if rs:
        streams[str(q)] = {"stream_trials": [r["stream_trials"] for r in rs],
                           "verified_decomps": [r["stream_verified_decomps"] for r in rs],
                           "solver_native_per_trial_mean": mean([r["solver_native_per_trial"] for r in rs]),
                           "solver_time_per_trial_s_mean": mean([r["solver_time_per_trial_s"] for r in rs]),
                           "fb_classes": rs[0]["fb_classes"], "capped": [r["capped"] for r in rs],
                           "wall_s": [r["wall_s"] for r in rs],
                           "w_cal": [lp[(q, s)]["w_cal"] for s in SEEDS if (q, s) in lp]}
out["streams"] = streams
tot_tr = sum(sum(v["stream_trials"]) for k, v in streams.items() if int(k) >= 251)
tot_dec = sum(sum(v["verified_decomps"]) for k, v in streams.items() if int(k) >= 251)
out["decomp_rate_per_trial_q_ge_251"] = tot_dec / tot_tr if tot_tr else None

# --- rho
for q in QS:
    rs = [(s, rho[(q, s)]["rho"]) for s in SEEDS if (q, s) in rho]
    if not rs:
        continue
    out["rho"][str(q)] = {"solved": [r["solved"] for _, r in rs], "n_seeds": len(rs)}
    for w, k in WS.items():
        out["rho"][str(q)]["S_" + w] = {"per_seed": [r[k] for _, r in rs], "mean": mean([r[k] for _, r in rs])}

# --- ratios (paired by instance)
def ratio_cell(a, q, w):
    ss = [s for s in SEEDS if (q, s) in lp and (q, s) in rho]
    num = [arm_of(lp[(q, s)], a)[WS[w]] for s in ss]
    den = [rho_S(q, s, w) for s in ss]
    r0 = mean(num) / mean(den)
    bs = []
    for _ in range(NB):
        idx = [rng.randrange(len(ss)) for _ in ss]
        bs.append(mean([num[i] for i in idx]) / mean([den[i] for i in idx]))
    return r0, [pct(bs, 0.025), pct(bs, 0.975)]

for a in ARMS:
    out["ratios"][a] = {}
    for w in WS:
        vals = []
        for q in QS:
            if all((q, s) in lp and (q, s) in rho for s in SEEDS):
                vals.append(ratio_cell(a, q, w))
            else:
                vals.append((None, None))
        out["ratios"][a][w] = {"values": [v[0] for v in vals], "ci95": [v[1] for v in vals]}

# --- fits
for a in ARMS:
    out["fits"][a] = {}
    for w in WS:
        vs = {q: [arm_of(lp[(q, s)], a)[WS[w]] for s in SEEDS if (q, s) in lp] for q in QS if any((q, s) in lp for s in SEEDS)}
        if len(vs) >= 4:
            b, c, ci = boot_fit(vs)
            out["fits"][a][w] = {"exponent": b, "coef": c, "ci95": ci, "n_points": len(vs)}
out["fits"]["rho"] = {}
for w in WS:
    vs = {q: [rho_S(q, s, w) for s in SEEDS if (q, s) in rho] for q in QS if any((q, s) in rho for s in SEEDS)}
    if len(vs) >= 4:
        b, c, ci = boot_fit(vs)
        out["fits"]["rho"][w] = {"exponent": b, "coef": c, "ci95": ci, "n_points": len(vs)}
# relation-phase-only exponent (stream part) at w = 0.1
out["fits_relation_phase_w0.1"] = {}
for a in ARMS:
    vs = {q: [arm_of(lp[(q, s)], a)["S_rel_w0p1"] for s in SEEDS if (q, s) in lp] for q in QS if any((q, s) in lp for s in SEEDS)}
    if len(vs) >= 4:
        b, c, ci = boot_fit(vs)
        out["fits_relation_phase_w0.1"][a] = {"exponent": b, "ci95": ci}
# trials-to-stop exponent (the relation count itself)
out["fits_trials"] = {}
for a in ARMS:
    vs = {q: [arm_of(lp[(q, s)], a)["trials_to_stop"] for s in SEEDS if (q, s) in lp] for q in QS if any((q, s) in lp for s in SEEDS)}
    if len(vs) >= 4:
        b, c, ci = boot_fit(vs)
        out["fits_trials"][a] = {"exponent": b, "ci95": ci}

# --- LA share at the top q
top = max(q for q in QS if all((q, s) in lp for s in SEEDS)) if lp else None
out["top_q"] = top
for a in ARMS:
    if top is None:
        break
    out["la_share_q2039"][a] = {}
    for w in ["w0", "w0.1", "w1"]:
        sh = [arm_of(lp[(top, s)], a)[LA[w]] / arm_of(lp[(top, s)], a)[WS[w]] for s in SEEDS]
        out["la_share_q2039"][a][w] = {"per_seed": sh, "mean": mean(sh), "max": max(sh)}

# --- A/A
for q, run1 in lp1.items():
    out["aa"][str(q)] = {}
    for a in ARMS:
        a0 = arm_of(lp[(q, 1)], a); a1 = arm_of(run1, a)
        out["aa"][str(q)][a] = {w: a0[WS[w]] / a1[WS[w]] for w in ["w0", "w0.1"]} | {"solved_both": a0["solved"] and a1["solved"]}

# --- predictions P5/P6/P7/P10 helpers
p5 = {}
for q in QS:
    if str(q) in out["cells"]["control_a1"]:
        p5[str(q)] = {a: out["cells"][a][str(q)]["S_w0.1"]["mean"] / out["cells"]["control_a1"][str(q)]["S_w0.1"]["mean"] for a in ARMS}
out["S_over_control_w0.1"] = p5

def pge2(f): return 6 * f * f * (1 - f) ** 2 + 4 * f ** 3 * (1 - f) + f ** 4
p7 = []
p10 = []
for a in ARMS[1:]:
    for q in QS:
        c = out["cells"][a].get(str(q))
        if not c or c["clamped"] or c["f_fraction"] > 0.5:
            continue
        rate = c["accepted_rows"]["mean"] / c["trials_to_stop"]["mean"]
        model = P_DECOMP * pge2(c["f_fraction"])
        p7.append({"arm": a, "q": q, "f": c["f_fraction"], "rate": rate, "model": model, "ratio": rate / model})
        ncl = streams[str(q)]["fb_classes"]
        mod_rows = 1.1 * (ncl - c["nf_classes"]) + c["nf_classes"] + 10
        p10.append({"arm": a, "q": q, "rows_at_solve": c["rows_at_solve"]["mean"], "model": mod_rows, "ratio": c["rows_at_solve"]["mean"] / mod_rows})
out["P7_rows"] = p7
out["P10_rows"] = p10
p6 = {}
for q in QS:
    if str(q) not in streams:
        continue
    sh = {}
    for a in ARMS:
        c = out["cells"][a][str(q)]
        sh[a] = c["Srel_w0.1"]["mean"] and (0.1 * c["stream_native"]["mean"]) / c["S_w0.1"]["mean"]
    p6[str(q)] = {"solver_native_per_trial": streams[str(q)]["solver_native_per_trial_mean"], "native_share_of_S_w0.1": sh}
out["P6"] = p6

# --- crossover projections (honest): rho ~ rho_w * q^2 from the q>=509 cells; IC arm S ~ S(top) * (q/top)^b
cross = {}
for w in ["w0.1", "w1"]:
    rho_pts = [(q, mean([rho_S(q, s, w) for s in SEEDS if (q, s) in rho])) for q in QS if q >= 509 and any((q, s) in rho for s in SEEDS)]
    rho_w = st.median([v / q ** 2 for q, v in rho_pts]) if rho_pts else None
    cross[w] = {"rho_coef_over_q2": rho_w, "arms": {}}
    for a in ARMS:
        if top is None or rho_w is None:
            break
        s_top = out["cells"][a][str(top)]["S_" + w]["mean"]
        fb = out["fits"][a].get(w)
        d = {}
        for label, b in [("fitted_exponent", fb["exponent"] if fb else None), ("theory_relation_exponent", {"control_a1": 1.0, "a0.75_c1": 1.5, "a0.75_c2": 1.5, "a0.6_c1": 1.8, "a0.6_c2": 1.8}[a])]:
            if b is None or b >= 2:
                d[label] = {"b": b, "q_star": None}
                continue
            # S_top (q/top)^b = rho_w q^2  ->  q^(2-b) = S_top / (top^b rho_w)
            qstar = (s_top / (top ** b * rho_w)) ** (1.0 / (2 - b))
            d[label] = {"b": b, "q_star": qstar}
        cross[w]["arms"][a] = d
out["crossover_projection"] = cross

# --- two-term theory projection: relation phase ~ q^(3-2alpha), LA ~ q^(2 alpha), anchored at the top-q means; rho ~ rho_w q^2
AL = {"control_a1": 1.0, "a0.75_c1": 0.75, "a0.75_c2": 0.75, "a0.6_c1": 0.6, "a0.6_c2": 0.6}
cross2 = {}
for w in ["w0.1", "w1"]:
    rho_w = cross[w]["rho_coef_over_q2"]
    cross2[w] = {}
    for a in ARMS:
        rel_top = out["cells"][a][str(top)]["Srel_" + w]["mean"]; la_top = out["cells"][a][str(top)]["Sla_" + w]["mean"]
        b_rel = 3 - 2 * AL[a]; b_la = 2 * AL[a]
        f = lambda q: rel_top * (q / top) ** b_rel + la_top * (q / top) ** b_la - rho_w * q ** 2
        # find the smallest q above the top where IC < rho (scan on a log grid)
        qstar = None
        for i in range(0, 4000):
            q = top * 10 ** (i * 0.005)
            if f(q) < 0:
                qstar = q
                break
        cross2[w][a] = {"b_rel": b_rel, "b_la": b_la, "q_star": qstar, "rel_top": rel_top, "la_top": la_top}
out["crossover_two_term_theory"] = cross2
out["wiedemann_failures"] = {"wrong_k_attempts_total": sum(r["ic_lp"]["arms"][i]["wrong_k_attempts"] for r in lp.values() for i in range(5)),
    "wiedemann_runs_total": sum(r["ic_lp"]["arms"][i]["wiedemann_runs"] for r in lp.values() for i in range(5)),
    "wiedemann_tries_total": sum(r["ic_lp"]["arms"][i]["wiedemann_tries"] for r in lp.values() for i in range(5))}
out["gae_per_trial_solver"] = {w: {"0.1": 0.1, "1": 1.0}[w] * 0 for w in []}

json.dump(out, open(os.path.join(HERE, "RESULTS.json"), "w"), indent=1, sort_keys=True)

# --- RESULTS.md
g = lambda v: "n/a" if v is None else f"{v:.3g}"
def ci(c): return "n/a" if c is None else f"[{c[0]:.3g}, {c[1]:.3g}]"
L = []
L.append("# RESULTS (round 2, extfield-jacobian-decomp, 2026-11-21)\n")
L.append("All numbers are [pcc:unverified]: local development check, no replay certificate, no validator report. Counted unit: group additions (GAE); S = adds + w * native ops.\n")
L.append(f"Runs: {len(lp)} lp streams (tag 0), {len(lp1)} A/A streams (tag 1), {len(rho)} rho runs; git HEAD of runs: {', '.join(out['git_head_of_runs'])}. Censored arm-runs: {len(out['censored'])}; unsolved: {len(out['unsolved'])}.\n")
for w in ["w0", "w0.1", "w1"]:
    L.append(f"\n## Total S at {w} (mean of 3 seeds, GAE) [pcc:unverified]\n")
    L.append("| q | rho | " + " | ".join(ARMS) + " |")
    L.append("|---|---|" + "---|" * len(ARMS))
    for q in QS:
        if str(q) not in out["rho"]:
            continue
        row = [str(q), g(out["rho"][str(q)]["S_" + w]["mean"])]
        for a in ARMS:
            c = out["cells"][a].get(str(q))
            row.append(g(c["S_" + w]["mean"]) + ("*" if c and c["censored_seeds"] else "") if c else "n/a")
        L.append("| " + " | ".join(row) + " |")
    L.append(f"\nDistance to matched rho, S(arm)/S(rho), paired by instance, 95% seed-bootstrap interval, at {w}:\n")
    L.append("| q | " + " | ".join(ARMS) + " |")
    L.append("|---|" + "---|" * len(ARMS))
    for i, q in enumerate(QS):
        row = [str(q)]
        for a in ARMS:
            r = out["ratios"][a][w]
            row.append(f"{g(r['values'][i])} {ci(r['ci95'][i])}")
        L.append("| " + " | ".join(row) + " |")
L.append("\n## Fitted exponents b in S = c q^b over q = 131..2039 (5 points), 95% bootstrap over seeds [pcc:unverified]\n")
L.append("| arm | w0 | w0.1 | w1 | wcal |")
L.append("|---|---|---|---|---|")
for a in ARMS + ["rho"]:
    row = [a]
    for w in WS:
        f = out["fits"][a].get(w)
        row.append("n/a" if not f else f"{f['exponent']:.3f} {ci(f['ci95'])}")
    L.append("| " + " | ".join(row) + " |")
L.append("\nTrials-to-solve exponent and relation-phase S(0.1) exponent:\n")
L.append("| arm | trials exponent | relation-phase S(0.1) exponent |")
L.append("|---|---|---|")
for a in ARMS:
    t = out["fits_trials"].get(a); r = out["fits_relation_phase_w0.1"].get(a)
    L.append(f"| {a} | {t['exponent']:.3f} {ci(t['ci95'])} | {r['exponent']:.3f} {ci(r['ci95'])} |" if t and r else f"| {a} | n/a | n/a |")
L.append("\n## Cell detail (means over seeds) [pcc:unverified]\n")
L.append("| arm | q | nF | f | trials | accepted rows | rows at solve | merged rows | cols | attempts | filter ops | Wiedemann mulmods | LA share S(0.1) | censored |")
L.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for a in ARMS:
    for q in QS:
        c = out["cells"][a].get(str(q))
        if not c:
            continue
        la = c["Sla_w0.1"]["mean"] / c["S_w0.1"]["mean"]
        L.append(f"| {a} | {q} | {c['nf_classes']}{' (clamped)' if c['clamped'] else ''} | {c['f_fraction']:.3f} | {g(c['trials_to_stop']['mean'])} | {g(c['accepted_rows']['mean'])} | {g(c['rows_at_solve']['mean'])} | {g(c['complete_at_solve']['mean'])} | {g(c['ncols_at_solve']['mean'])} | {g(c['attempts']['mean'])} | {g(c['filter_ops']['mean'])} | {g(c['wiedemann_mulmods']['mean'])} | {la:.2g} | {c['censored_seeds']} |")
L.append("\n## Stream facts [pcc:unverified]\n")
L.append("| q | classes | trials per seed | verified decompositions per seed | solver native per trial | solver ms per trial | wall s per seed |")
L.append("|---|---|---|---|---|---|---|")
for q in QS:
    s = streams.get(str(q))
    if s:
        L.append(f"| {q} | {s['fb_classes']} | {s['stream_trials']} | {s['verified_decomps']} | {g(s['solver_native_per_trial_mean'])} | {s['solver_time_per_trial_s_mean']*1e3:.3g} | {[round(x,1) for x in s['wall_s']]} |")
L.append(f"\n[pcc:unverified] Verified decompositions per trial pooled for q >= 251: {g(out['decomp_rate_per_trial_q_ge_251'])} (round 1 pooled success-trial rate 0.0402; here counts decompositions, including several per R).\n")
L.append("\n## Distance to control, S(arm)/S(control) at w0.1 [pcc:unverified]\n")
L.append("| q | " + " | ".join(ARMS) + " |")
L.append("|---|" + "---|" * len(ARMS))
for q in QS:
    if str(q) in p5:
        L.append("| " + str(q) + " | " + " | ".join(g(p5[str(q)][a]) for a in ARMS) + " |")
L.append("\n## LA share at the top q (mean over seeds, max) [pcc:unverified]\n")
L.append("| arm | w0 | w0.1 | w1 |")
L.append("|---|---|---|---|")
for a, v in out["la_share_q2039"].items():
    L.append(f"| {a} | " + " | ".join(f"{g(v[w]['mean'])} (max {g(v[w]['max'])})" for w in ['w0', 'w0.1', 'w1']) + " |")
L.append("\n## A/A (tag 0 / tag 1 on the same instance, seed 1) [pcc:unverified]\n")
L.append("| q | arm | S(w0) ratio | S(w0.1) ratio |")
L.append("|---|---|---|---|")
for q, d in out["aa"].items():
    for a, v in d.items():
        L.append(f"| {q} | {a} | {g(v['w0'])} | {g(v['w0.1'])} |")
L.append("\n## Crossover projection (extrapolation, not a measurement) [pcc:unverified]\n")
L.append("rho ~ rho_coef * q^2 (median of S/q^2 over q >= 509); arm anchored at the top-q mean with exponent b.\n")
L.append("| w | arm | exponent source | b | q* (IC below rho above) |")
L.append("|---|---|---|---|---|")
for w, d in cross.items():
    for a, v in d["arms"].items():
        for lab, x in v.items():
            L.append(f"| {w} | {a} | {lab} | {g(x['b'])} | {g(x['q_star'])} |")
L.append("\nTwo-term theory projection (relation phase q^(3-2 alpha) plus LA q^(2 alpha), both anchored at the top-q means; rho ~ coef q^2):\n")
L.append("| w | arm | b_rel | b_la | q* |")
L.append("|---|---|---|---|---|")
for w, d in cross2.items():
    for a, v in d.items():
        L.append(f"| {w} | {a} | {v['b_rel']:.2f} | {v['b_la']:.2f} | {g(v['q_star']) if v['q_star'] else 'none up to 1e' + str(int(math.log10(top)+20))} |")
wf = out["wiedemann_failures"]
L.append(f"\nWiedemann runs {wf['wiedemann_runs_total']}, projection tries {wf['wiedemann_tries_total']}, attempts returning a wrong k {wf['wrong_k_attempts_total']}.\n")
open(os.path.join(HERE, "RESULTS.md"), "w").write("\n".join(L) + "\n")
print("wrote RESULTS.json and RESULTS.md")
