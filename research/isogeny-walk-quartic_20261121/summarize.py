#!/usr/bin/env python3
"""Reads results/run_p*.jsonl and results/pushfwd2*.jsonl; writes RESULTS.json and RESULTS.md.
Pure Python (std only). All numbers [pcc:unverified]. usage: python3 summarize.py"""
import glob, json, math, os, random

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "results")
WS = [0.0, 0.1, 1.0]
random.seed(20261121)
PREV_BSGS_521 = 1.83e7  # first-round IC ops / target at p = 521 (mul+inv on the quartic, BSGS arm)


def mean(v):
    v = list(v)
    return sum(v) / len(v) if v else float("nan")


def load():
    curves, rho, rej, summ = [], [], [], []
    for f in sorted(glob.glob(os.path.join(RES, "run_p*.jsonl"))):
        for line in open(f):
            line = line.strip()
            if not line:
                continue
            r = json.loads(line)
            {"curve": curves, "rho_neg": rho, "reject": rej, "summary": summ}[r["kind"]].append(r)
    return curves, rho, rej, summ


def gae_h(c):
    return c["gae_h"]["M"] + 3 * c["gae_h"]["I"]


def tally(t):
    return t["M"] + 3 * t["I"], t["NAT"]


def phase_costs(c, which="main"):
    """Per-phase (field ops M + 3 I, native ops), shared part and per-target parts."""
    d = c["diem"][which]
    out = {}
    out["transfer_points_fit"] = (c["transfer"]["M"] + 3 * c["transfer"]["I"], 0)
    out["curve_setup_C"] = tally(d["c_setup"])
    out["pushforward_classes_cofactor"] = tally(d["phi_build"])
    out["relation_phase_C"] = (tally(d["fb_setup"])[0] + tally(d["relations"])[0], tally(d["fb_setup"])[1] + tally(d["relations"])[1] + d["elim_entries"])
    la = d["elim_ops"] + d["wied_ops"] + d["bm_ops"]
    out["linear_algebra_C"] = (0, la)
    fa = c["diem"]["failed_attempts_main" if which == "main" else "failed_attempts_aa"]
    fm = sum(tally(x["c_setup"])[0] + tally(x["phi_build"])[0] + tally(x["fb_setup"])[0] + tally(x["relations"])[0] for x in fa)
    fn = sum(tally(x["c_setup"])[1] + tally(x["phi_build"])[1] + tally(x["fb_setup"])[1] + tally(x["relations"])[1] + x["elim_entries"] + x["elim_ops"] + x["wied_ops"] + x["bm_ops"] for x in fa)
    out["failed_attempts"] = (fm, fn)
    z3 = c["order_z3"]
    out["order_z3"] = (z3["M"] + 3 * z3["I"], z3["NAT"]) if z3 else (float("nan"), float("nan"))
    z2 = c["order_z2b"]
    out["order_z2b"] = (z2["M"] + 3 * z2["I"], z2["NAT"])
    return out


def per_target(c, which="main"):
    d = c["diem"][which]
    rows = []
    for j, t in enumerate(d["targets"]):
        tg = c["targets"][j]
        if t["k"] is None:
            continue
        pb = tg["pullback"]
        rows.append({"descent": (t["M"] + 3 * t["I"], t["NAT"]), "pullback": (pb["M"] + 3 * pb["I"], 0), "verified_H": tg["verified_on_H"] if which == "main" else None})
    return rows


def S_curve(c, w, order="z3", which="main"):
    """Single-target S in GAE_H (mean over the found targets of one curve)."""
    g = gae_h(c)
    ph = phase_costs(c, which)
    shared = ["transfer_points_fit", "curve_setup_C", "pushforward_classes_cofactor", "relation_phase_C", "linear_algebra_C", "failed_attempts"]
    if order == "z3":
        shared.append("order_z3")
    elif order == "z2b":
        shared.append("order_z2b")
    f = sum(ph[k][0] for k in shared)
    n = sum(ph[k][1] for k in shared)
    pt = per_target(c, which)
    if not pt:
        return None
    tf = mean(r["descent"][0] + r["pullback"][0] for r in pt)
    tn = mean(r["descent"][1] + r["pullback"][1] for r in pt)
    return (f + tf) / g + w * (n + tn)


def phase_share(c, w, which="main"):
    g = gae_h(c)
    ph = phase_costs(c, which)
    pt = per_target(c, which)
    if not pt:
        return None
    ph = dict(ph)
    ph["descent"] = (mean(r["descent"][0] for r in pt), mean(r["descent"][1] for r in pt))
    ph["pullback_H"] = (mean(r["pullback"][0] for r in pt), 0)
    tot = sum(v[0] / g + w * v[1] for k, v in ph.items() if k != "order_z2b")
    return {k: (v[0] / g + w * v[1]) / tot for k, v in ph.items() if k != "order_z2b"}, tot


def rho_cost(r, g, w):
    return (r["M"] + 3 * r["I"]) / g + w * r["NAT"]


def rho_walk_cost(r, g, w):
    """rho without the one-off setup (32 walk multipliers and the first start point)."""
    return (r["M"] - r["setup_M"] + 3 * (r["I"] - r["setup_I"])) / g + w * r["NAT"]


def boot_ratio(num, den, n=4000):
    """ratio of means, bootstrap over curves; num, den: dict curve -> value."""
    keys = [k for k in num if k in den and not math.isnan(num[k]) and not math.isnan(den[k])]
    if not keys:
        return None
    est = mean(num[k] for k in keys) / mean(den[k] for k in keys)
    bs = []
    for _ in range(n):
        d = [random.choice(keys) for _ in keys]
        bs.append(mean(num[k] for k in d) / mean(den[k] for k in d))
    bs.sort()
    return [est, bs[int(0.025 * n)], bs[int(0.975 * n) - 1]]


def lsq(xs, ys):
    mx, my = mean(xs), mean(ys)
    sxx = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx


def boot_slope(ps, by_p, n=3000):
    """by_p: p -> {curve: value}. Slope of ln mean vs ln p with bootstrap over curves."""
    ps = [p for p in ps if by_p.get(p)]
    if len(ps) < 4:
        return None
    xs = [math.log(p) for p in ps]
    est = lsq(xs, [math.log(mean(by_p[p].values())) for p in ps])
    bs = []
    for _ in range(n):
        ys = []
        for p in ps:
            v = list(by_p[p].values())
            ys.append(math.log(mean(random.choice(v) for _ in v)))
        bs.append(lsq(xs, ys))
    bs.sort()
    return [est, bs[int(0.025 * n)], bs[int(0.975 * n) - 1], len(ps)]


def main():
    curves, rho, rej, summ = load()
    ps = sorted({c["p"] for c in curves})
    res = {"label": "[pcc:unverified] local development check", "git": summ[0]["git"] if summ else "unknown", "ps": ps, "per_p": {}, "fits": {}, "predictions": {}, "curves": []}
    S_by = {}
    rho_by = {}
    rho_sc = {}
    rho_wk = {}
    for p in ps:
        cs = [c for c in curves if c["p"] == p]
        rs = [r for r in rho if r["p"] == p]
        rj = [r for r in rej if r["p"] == p]
        sm = [s for s in summ if s["p"] == p][0]
        row = {"p": p, "curves": len(cs), "candidates": sm["candidates"], "accepted": sm["accepted"], "rejection_reasons": {}}
        for r in rj:
            key = r["reason"].split("(")[0].split("=")[0].strip()
            key = "no prime ell || N with N/ell <= 64" if "no prime ell" in r["reason"] else key
            row["rejection_reasons"][key] = row["rejection_reasons"].get(key, 0) + 1
        # verification
        tg = [(c["curve"], t) for c in cs for t in c["targets"]]
        row["targets"] = len(tg)
        row["targets_nontrivial"] = sum(1 for _, t in tg if not t["q_trivial"])
        row["logs_found_on_C"] = sum(1 for _, t in tg if t["k"] is not None)
        row["verified_on_H"] = sum(1 for _, t in tg if t["verified_on_H"])
        row["found_not_verified_on_H"] = row["logs_found_on_C"] - row["verified_on_H"]
        row["c_verified_inconclusive"] = sum(1 for _, t in tg if t["k"] is not None and not t["verified_on_C"])
        row["h_order_ok"] = all(c["h_order_ok"] for c in cs)
        row["c_order_ok"] = all(c["c_order_ok"] for c in cs)
        row["ic_status_main"] = [c["diem"]["main"]["status"] for c in cs]
        row["ic_status_aa"] = [c["diem"]["aa"]["status"] for c in cs]
        row["ic_failed_attempts_main"] = [len(c["diem"]["failed_attempts_main"]) for c in cs]
        row["b"] = [c["diem"]["main"]["b"] for c in cs]
        row["N"] = [c["N"] for c in cs]
        row["ell"] = [c["ell"] for c in cs]
        row["cof"] = [c["cof"] for c in cs]
        row["gae_h_field_ops"] = [gae_h(c) for c in cs]
        row["gae_c_field_ops"] = [c["diem"]["main"]["gae_c"]["M"] + 3 * c["diem"]["main"]["gae_c"]["I"] for c in cs]
        row["z3_agrees_with_z2b"] = all(c["order_z3"] is not None and c["order_z3"]["N"] == c["N"] for c in cs)
        # per-curve S, rho
        for w in WS:
            for order in ("z3", "z2b", "none"):
                for which in ("main", "aa"):
                    S_by[(p, w, order, which)] = {c["curve"]: S_curve(c, w, order, which) for c in cs}
                    S_by[(p, w, order, which)] = {k: v for k, v in S_by[(p, w, order, which)].items() if v is not None}
            rho_by[(p, w)] = {c["curve"]: mean(rho_cost(r, gae_h(c), w) for r in rs if r["curve"] == c["curve"]) for c in cs}
            rho_wk[(p, w)] = {c["curve"]: mean(rho_walk_cost(r, gae_h(c), w) for r in rs if r["curve"] == c["curve"]) * math.sqrt(p ** 3 / c["ell"]) for c in cs}
            rho_sc[(p, w)] = {c["curve"]: rho_by[(p, w)][c["curve"]] * math.sqrt(p ** 3 / c["ell"]) for c in cs}
        row["rho"] = {"runs": len(rs), "verified": sum(1 for r in rs if r["verified"]), "mean_steps": mean(r["steps"] for r in rs),
                      "steps_over_sqrt_ell": mean(r["steps"] / math.sqrt(r["ell"]) for r in rs),
                      "mean_field_ops_M_I_adds": mean(r["M"] + r["I"] + r["adds_field"] for r in rs),
                      "mean_native": mean(r["NAT"] for r in rs),
                      "cycle_escapes_per_step": mean(r["cycle_escapes"] / max(r["steps"], 1) for r in rs),
                      "aa_set0_mean_steps": mean(r["steps"] for r in rs if r["aa_set"] == 0), "aa_set1_mean_steps": mean(r["steps"] for r in rs if r["aa_set"] == 1)}
        # Q3 reference: sqrt(pi ell/4) steps x field ops per add (incl adds, as the first round)
        ratios = []
        for c in cs:
            ref = math.sqrt(math.pi * c["ell"] / 4) * (c["gae_h"]["M"] + c["gae_h"]["I"] + c["gae_h"]["adds_field"])
            for r in rs:
                if r["curve"] == c["curve"]:
                    ratios.append((r["M"] + r["I"] + r["adds_field"]) / ref)
        row["rho"]["measured_over_analytic_ref"] = mean(ratios)
        row["rho"]["analytic_ref_mean_field_ops"] = mean(math.sqrt(math.pi * c["ell"] / 4) * (c["gae_h"]["M"] + c["gae_h"]["I"] + c["gae_h"]["adds_field"]) for c in cs)
        # S and ratios
        row["S"] = {}
        for w in WS:
            e = {}
            for order in ("z3", "z2b", "none"):
                m = S_by[(p, w, order, "main")]
                a = S_by[(p, w, order, "aa")]
                e[order] = {"S_mean": mean(m.values()), "S_aa_mean": mean(a.values()), "main_over_aa": mean(m.values()) / mean(a.values()) if a else None,
                            "S_over_rho": boot_ratio(m, rho_by[(p, w)])}
            e["rho_mean"] = mean(rho_by[(p, w)].values())
            row["S"]["w%g" % w] = e
        # phase shares (mean over curves) and absolute (GAE_H)
        row["phase_share"] = {}
        for w in WS:
            sh = [phase_share(c, w) for c in cs]
            sh = [s for s in sh if s]
            if sh:
                keys = sh[0][0].keys()
                row["phase_share"]["w%g" % w] = {k: mean(s[0][k] for s in sh) for k in keys}
        # Q1: Diem arm relation phase + descent at w = 0, M + I per target (first-round unit)
        q1 = []
        for c in cs:
            d = c["diem"]["main"]
            pt = [t for t in d["targets"] if t["k"] is not None]
            if not pt:
                continue
            m = d["fb_setup"]["M"] + d["fb_setup"]["I"] + d["relations"]["M"] + d["relations"]["I"] + mean(t["M"] + t["I"] for t in pt)
            q1.append(m)
        row["diem_field_ops_per_target_M_plus_I_w0"] = mean(q1)
        # S / rho_scaled: rho rescaled to ell = p^3 by sqrt(p^3 / ell) (removes the spread of ell between curves)
        row["S_over_rho_scaled"] = {"w%g" % w: {o: boot_ratio(S_by[(p, w, o, "main")], rho_sc[(p, w)]) for o in ("z3", "z2b", "none")} for w in WS}
        row["rho_setup_share_w0"] = mean(((r["setup_M"] + 3 * r["setup_I"]) / ((r["M"] + 3 * r["I"]))) for r in rs)
        row["rho_walk_only_scaled_mean"] = {"w%g" % w: mean(rho_wk[(p, w)].values()) for w in WS}
        row["S_over_rho_walk_only_scaled"] = {"w%g" % w: {o: boot_ratio(S_by[(p, w, o, "main")], rho_wk[(p, w)]) for o in ("z2b", "none")} for w in WS}
        row["rho_scaled_mean"] = {"w%g" % w: mean(rho_sc[(p, w)].values()) for w in WS}
        # phase absolute cost without the order phase (GAE_H), mean over curves
        row["phase_abs_none"] = {}
        for w in WS:
            sh = [phase_share(c, w) for c in cs]
            sh = [x for x in sh if x]
            row["phase_abs_none"]["w%g" % w] = {k: mean(x[1] * x[0][k] for x in sh) for k in sh[0][0] if k != "order_z3"}
        # Q1 in GAE_H-weighted field ops: relation phase + linear algebra + descent, priced at w
        q1w = {}
        for w in WS:
            vals = []
            for c in cs:
                d = c["diem"]["main"]
                pt = [t for t in d["targets"] if t["k"] is not None]
                f = d["fb_setup"]["M"] + 3 * d["fb_setup"]["I"] + d["relations"]["M"] + 3 * d["relations"]["I"] + mean(t["M"] + 3 * t["I"] for t in pt)
                n = d["fb_setup"]["NAT"] + d["relations"]["NAT"] + d["elim_entries"] + d["elim_ops"] + d["wied_ops"] + d["bm_ops"] + mean(t["NAT"] for t in pt)
                vals.append(f + w * n * gae_h(c))
            q1w["w%g" % w] = mean(vals)
        row["diem_rel_la_descent_field_op_equivalents"] = q1w
        res["per_p"][str(p)] = row
        res["curves"].append({"p": p, "curve": [c["curve"] for c in cs]})
    # exponent fits
    for order in ("z3", "z2b", "none"):
        for w in WS:
            res["fits"]["S_%s_w%g" % (order, w)] = boot_slope(ps, {p: S_by[(p, w, order, "main")] for p in ps})
    for w in WS:
        res["fits"]["rho_w%g" % w] = boot_slope(ps, {p: rho_by[(p, w)] for p in ps})
        res["fits"]["rho_walk_only_scaled_to_ell_p3_w%g" % w] = boot_slope(ps, {p: rho_wk[(p, w)] for p in ps})
        res["fits"]["rho_scaled_to_ell_p3_w%g" % w] = boot_slope(ps, {p: rho_sc[(p, w)] for p in ps})
    # predictions
    pr = res["per_p"].get("521")
    if pr:
        d = pr["diem_field_ops_per_target_M_plus_I_w0"]
        q1w = pr["diem_rel_la_descent_field_op_equivalents"]
        res["predictions"]["Q1"] = {"diem_field_ops_per_target_M_plus_I_w0": d, "first_round_bsgs": PREV_BSGS_521, "ratio_bsgs_over_diem_w0": PREV_BSGS_521 / d,
                                    "diem_rel_la_descent_in_field_op_equivalents": q1w, "ratio_bsgs_over_diem_by_w": {k: PREV_BSGS_521 / v for k, v in q1w.items()}}
        # with LA priced: Diem arm S (GAE_H) at w = 0.1, 1 versus BSGS in the same unit (BSGS has no native work counted): report diem S for reference
        res["predictions"]["Q2"] = {"order_z3_share": {w: pr["phase_share"][w]["order_z3"] for w in pr["phase_share"]}}
        res["predictions"]["Q3"] = {"measured_over_analytic_ref_by_p": {str(p): res["per_p"][str(p)]["rho"]["measured_over_analytic_ref"] for p in ps}}
        res["predictions"]["Q4"] = {w: {o: pr["S"][w][o]["S_over_rho"] for o in ("z3", "z2b", "none")} for w in pr["S"]}
    # P7 re-run
    pf = os.path.join(RES, "pushfwd2.jsonl")
    if os.path.exists(pf):
        rows = [json.loads(l) for l in open(pf)]
        q = [r for r in rows if "bases" in r]
        res["pushfwd2"] = {"instances_with_quartic": len(q), "instances_without": len(rows) - len(q), "pairs_total": sum(r["pairs_total"] for r in q),
                           "pairs_tested_under_some_base": sum(r["pairs_tested_under_some_base"] for r in q),
                           "pairs_untested_under_all_bases": sum(r["pairs_untested_under_all_bases"] for r in q),
                           "instances_single_valued_under_every_base": sum(1 for r in q if r["single_valued_under_every_base"]),
                           "base_runs": sum(len(r["bases"]) for r in q),
                           "panics_in_base_runs": sum(b["panics"] for r in q for b in r["bases"]),
                           "no_phi_in_base_runs": sum(b["no_phi"] for r in q for b in r["bases"]),
                           "negctl_base_runs_with_pairs": sum(1 for r in q for b in r["bases"] if b["negctl_pairs"] > 0),
                           "negctl_base_runs_separating": sum(1 for r in q for b in r["bases"] if b["negctl_pairs"] > 0 and b["negctl_distinct_sums"] > 1)}
    ob = os.path.join(RES, "pushfwd2_bound12_only.jsonl")
    if os.path.exists(ob):
        rows = [json.loads(l) for l in open(ob)]
        q = [r for r in rows if "pairs_tested" in r]
        res["pushfwd_bound12_single_base"] = {"pairs_tested": sum(r["pairs_tested"] for r in q), "panics": sum(r["cantor_panics"] for r in q), "no_phi": sum(r["phi_failures"] for r in q)}
    json.dump(res, open(os.path.join(HERE, "RESULTS.json"), "w"), indent=1, default=str)
    write_md(res)


def f3(x):
    return "n/a" if x is None else ("%.3g" % x)


def ci(v):
    return "n/a" if not v else "%.3g [%.3g, %.3g]" % tuple(v)


def write_md(res):
    L = ["# RESULTS (round 2) [pcc:unverified] local development check", "", "git HEAD at run time: `%s`. Unit: GAE_H (field mults + 3 inversions of one Cantor addition on H; field adds not counted). Native work priced at w per operation on every arm." % res["git"], ""]
    L += ["## Instances and end-to-end verification", "", "| p | candidates tried | accepted | targets | logs found on C | verified k G' = Q' on H | found, not verified | ell | N/ell | b (factor base) | IC attempts failed before ok |", "|--:|--:|--:|--:|--:|--:|--:|:--|:--|:--|:--|"]
    for p, r in res["per_p"].items():
        L.append("| %s | %d | %d | %d | %d | %d | %d | %s | %s | %s | %s |" % (p, r["candidates"], r["accepted"], r["targets"], r["logs_found_on_C"], r["verified_on_H"], r["found_not_verified_on_H"], r["ell"], r["cof"], r["b"], r["ic_failed_attempts_main"]))
    L += ["", "Rejections (not charged): " + "; ".join("p=%s: %s" % (p, r["rejection_reasons"]) for p, r in res["per_p"].items()), ""]
    L += ["`verified_on_C` is inconclusive for %s targets (the scalar multiple on the quartic returned `None`); the verification on H is authoritative. Group order checks (N G = 0 on H and ell G' = 0 on C): H %s, C %s. Z3 order equals Z2B order on every curve: %s." % (
        {p: r["c_verified_inconclusive"] for p, r in res["per_p"].items()}, {p: r["h_order_ok"] for p, r in res["per_p"].items()}, {p: r["c_order_ok"] for p, r in res["per_p"].items()}, {p: r["z3_agrees_with_z2b"] for p, r in res["per_p"].items()}), ""]
    L += ["## Measured negation-map rho on J_H", "", "| p | runs | verified | mean steps | steps / sqrt(ell) | mean field ops (M+I+adds) | analytic reference sqrt(pi ell/4) x ops/add | measured / reference | cycle escapes per step | A/A mean steps (set 0, set 1) |", "|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|"]
    for p, r in res["per_p"].items():
        h = r["rho"]
        L.append("| %s | %d | %d | %.0f | %.2f | %.3g | %.3g | %.2f | %.3f | %.0f, %.0f |" % (p, h["runs"], h["verified"], h["mean_steps"], h["steps_over_sqrt_ell"], h["mean_field_ops_M_I_adds"], h["analytic_ref_mean_field_ops"], h["measured_over_analytic_ref"], h["cycle_escapes_per_step"], h["aa_set0_mean_steps"], h["aa_set1_mean_steps"]))
    L += ["", "## S (single target, GAE_H) and S / rho", ""]
    for w in ("w0", "w0.1", "w1"):
        L += ["### price w = %s" % w[1:], "", "| p | rho (measured) | S, Z3 charged | S / rho (Z3) | S, Z2B charged | S / rho (Z2B) | S, no order phase | S / rho (none) | main / A/A (Z3) |", "|--:|--:|--:|:--|--:|:--|--:|:--|--:|"]
        for p, r in res["per_p"].items():
            e = r["S"][w]
            L.append("| %s | %.3g | %.3g | %s | %.3g | %s | %.3g | %s | %.2f |" % (p, e["rho_mean"], e["z3"]["S_mean"], ci(e["z3"]["S_over_rho"]), e["z2b"]["S_mean"], ci(e["z2b"]["S_over_rho"]), e["none"]["S_mean"], ci(e["none"]["S_over_rho"]), e["z3"]["main_over_aa"]))
        L.append("")
    L += ["Intervals: 95% bootstrap over the 3 curves of each p (ratio of means); with 3 curves they are wide and indicative only.", ""]
    L += ["## Phase shares of S (Z3 charged; mean over curves)", ""]
    for w, sh in ((w, r) for w, r in res["per_p"][list(res["per_p"])[-1]]["phase_share"].items()):
        pass
    for p, r in res["per_p"].items():
        for w, sh in r["phase_share"].items():
            L.append("- p = %s, w = %s: " % (p, w[1:]) + ", ".join("%s %.1f%%" % (k, 100 * v) for k, v in sh.items()))
    L += ["", "## Exponent fits (log-log slope of mean cost vs p, 4 points, bootstrap over curves)", "", "| quantity | slope [95%] | points |", "|:--|:--|--:|"]
    for k, v in res["fits"].items():
        L.append("| %s | %s | %s |" % (k, ci(v[:3]) if v else "not available", v[3] if v else "-"))
    L += ["", "## S / rho with rho rescaled to ell = p^3 (removes the ell spread between curves)", "", "| p | w | rho scaled | S/rho_scaled (Z3) | (Z2B) | (none) |", "|--:|--:|--:|:--|:--|:--|"]
    for p, r in res["per_p"].items():
        for w in ("w0", "w0.1", "w1"):
            e = r["S_over_rho_scaled"][w]
            L.append("| %s | %s | %.3g | %s | %s | %s |" % (p, w[1:], r["rho_scaled_mean"][w], ci(e["z3"]), ci(e["z2b"]), ci(e["none"])))
    L += ["", "## rho without its one-off setup (32 multipliers + first start), rescaled to ell = p^3", "", "| p | setup share of rho cost (w=0) | rho walk-only scaled, w=0 | S/rho_walk (Z2B), w=0 | S/rho_walk (none), w=0 | S/rho_walk (none), w=0.1 | S/rho_walk (none), w=1 |", "|--:|--:|--:|:--|:--|:--|:--|"]
    for p, r in res["per_p"].items():
        e = r["S_over_rho_walk_only_scaled"]
        L.append("| %s | %.2f | %.3g | %s | %s | %s | %s |" % (p, r["rho_setup_share_w0"], r["rho_walk_only_scaled_mean"]["w0"], ci(e["w0"]["z2b"]), ci(e["w0"]["none"]), ci(e["w0.1"]["none"]), ci(e["w1"]["none"])))
    L += ["", "## Phase costs without the order phase (GAE_H, mean over curves)", ""]
    for p, r in res["per_p"].items():
        for w, ph in r["phase_abs_none"].items():
            L.append("- p = %s, w = %s: " % (p, w[1:]) + ", ".join("%s %.3g" % (k, v) for k, v in ph.items()))
    L += ["", "## Predictions", ""]
    for k, v in res["predictions"].items():
        L.append("- %s: `%s`" % (k, json.dumps(v, default=str)))
    if "pushfwd2" in res:
        L += ["", "## Push-forward test re-run (plane_quartic_pic copy)", "", "`%s`" % json.dumps(res["pushfwd2"]), "", "Single base point, bound 12 only: `%s`" % json.dumps(res.get("pushfwd_bound12_single_base"))]
    open(os.path.join(HERE, "RESULTS.md"), "w").write("\n".join(L) + "\n")


if __name__ == "__main__":
    main()
