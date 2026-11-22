#!/usr/bin/env python3
"""Aggregate results/dlp_*.json and results/yield_*.jsonl into RESULTS.json and RESULTS.md.
Pure Python. Every number is [pcc:unverified]. usage: python3 analyze.py"""
import glob, json, math, os, random, statistics as st

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "results")
LADDER = [251, 509, 1021, 2039]
WS = {"w0": 0.0, "w0.1": 0.1, "w1": 1.0}
rng = random.Random(20261121)


def med(xs):
    return st.median(xs) if xs else None


def fit(xs, ys):
    lx = [math.log(x) for x in xs]
    ly = [math.log(y) for y in ys]
    n = len(lx)
    mx, my = sum(lx) / n, sum(ly) / n
    sxx = sum((a - mx) ** 2 for a in lx)
    return sum((a - mx) * (c - my) for a, c in zip(lx, ly)) / sxx


def boot_fit(samples_by_p, stat=med, nb=2000):
    """samples_by_p: {p: [values]} -> (point, lo, hi) exponent of stat vs p; needs >= 4 p with data."""
    ps = sorted(p for p, v in samples_by_p.items() if v)
    if len(ps) < 4:
        return None
    pt = fit(ps, [stat(samples_by_p[p]) for p in ps])
    bs = []
    for _ in range(nb):
        ys = []
        for p in ps:
            v = samples_by_p[p]
            ys.append(stat([rng.choice(v) for _ in v]))
        if min(ys) <= 0:
            continue
        bs.append(fit(ps, ys))
    bs.sort()
    return [pt, bs[int(0.025 * len(bs))], bs[int(0.975 * len(bs)) - 1]]


def load():
    out = []
    for f in sorted(glob.glob(os.path.join(RES, "dlp_*.json"))):
        d = json.load(open(f))
        if "error" in d:
            continue
        out.append(d)
    return out


def rung_costs(rungs, ntarg=3):
    """All-rungs and final-rung cost components of one pass."""
    def comp(rs):
        sh_ops = sum(r["ops_shared"] for r in rs)
        sh_nat = sum(r["native_p_rel"] + r["native_p_fb"] + r["nat_shared_indiv"] + r["native_ell_filter"] + r["native_ell_la"] for r in rs)
        rel_nat = sum(r["native_p_rel"] + r["native_p_fb"] for r in rs)
        la_nat = sum(r["native_ell_filter"] + r["native_ell_la"] for r in rs)
        t_ops = [sum(r["tgt_ops"][t] for r in rs) for t in range(ntarg)]
        t_nat = [sum(r["tgt_nat"][t] for r in rs) for t in range(ntarg)]
        return dict(sh_ops=sh_ops, sh_nat=sh_nat, rel_nat=rel_nat, la_nat=la_nat, t_ops=t_ops, t_nat=t_nat)
    return comp(rungs), comp(rungs[-1:])


def S(c, w, t=None):
    """cost per target (primary: full shared + own target) or mean over targets."""
    ts = range(len(c["t_ops"])) if t is None else [t]
    vals = [c["sh_ops"] + c["t_ops"][i] + w * (c["sh_nat"] + c["t_nat"][i]) for i in ts]
    return sum(vals) / len(vals)


def main():
    data = load()
    by_p = {}
    for d in data:
        by_p.setdefault(d["p"], []).append(d)
    R = {"git_note": "see PLAN.md", "cells": [], "rho": {}, "arms": {}, "fits": {}, "yield": {}, "checks": {}}
    # rho per p
    for p, ds in by_p.items():
        runs = [(r, d) for d in ds for r in d["rho"]]
        if not runs:
            continue
        adds = [r["setup_adds"] + r["walk_adds"] for r, d in runs]
        nat = [r["native"] for r, d in runs]
        ratio = [r["steps"] / math.sqrt(d["ell"]) for r, d in runs]
        R["rho"][p] = dict(runs=len(runs), verified=sum(1 for r, d in runs if r["verified"]), adds_mean=sum(adds) / len(adds),
                           adds_all=adds, native_mean=sum(nat) / len(nat), steps_over_sqrt_ell=ratio,
                           r_mean=sum(ratio) / len(ratio), ell=[d["ell"] for d in ds])
    allratio = [x for p in R["rho"] for x in R["rho"][p]["steps_over_sqrt_ell"]]
    rhat = sum(allratio) / len(allratio) if allratio else None
    R["rho_model_r"] = rhat

    def rho_S(p, w, model=False):
        r = R["rho"][p]
        if model:
            ads = [rhat * math.sqrt(ell) * 1.0 + 900 for ell in r["ell"]]
            return sum(ads) / len(ads) * (1 + w)  # one native op per step
        return r["adds_mean"] + w * r["native_mean"]

    alphas = sorted({a["alpha"] for d in data for a in d["arms"]})
    for al in alphas:
        key = "%.3f" % al
        R["arms"][key] = {}
        for p in LADDER:
            ds = [d for d in by_p.get(p, [])]
            rec = []
            for d in ds:
                arm = [a for a in d["arms"] if abs(a["alpha"] - al) < 1e-9][0]
                for pi, ps in enumerate(arm["passes"]):
                    rungs = ps["rungs"]
                    allc, finc = rung_costs(rungs)
                    solved = ps["solved_at"] is not None
                    last = rungs[-1]
                    row = dict(curve=d["curve"], pass_=pi, solved=solved, solved_at=ps["solved_at"], nrungs=len(rungs),
                               m_final=last["m"], phi_final=last["m"] / d["npts"], rows=last["rows"], nused=last["nused"],
                               u_size=last["u_size"], la_ops=last["la_ops"], nnz=last["nnz"], partial=last["partial"],
                               lp_vertices=last["lp_vertices"], filter_complete=last["filter_complete"],
                               reason=last["reason"], pairs=last["pairs"], n_f=last["n_f"], split=last["split"],
                               rel_nat_all=allc["rel_nat"], rel_nat_final=finc["rel_nat"], la_nat_all=allc["la_nat"],
                               la_nat_final=finc["la_nat"], sh_ops_all=allc["sh_ops"], w_cal=d["calib"]["w_cal_p"], npts=d["npts"])
                    for wn, w in WS.items():
                        row["S_all_" + wn] = S(allc, w) if solved else None
                        row["S_fin_" + wn] = S(finc, w) if solved else None
                        row["S_amort_" + wn] = None
                        if solved:
                            row["S_amort_" + wn] = allc["sh_ops"] / 3 + sum(allc["t_ops"]) / 3 + w * (allc["sh_nat"] / 3 + sum(allc["t_nat"]) / 3)
                    wc = d["calib"]["w_cal_p"]
                    row["S_all_wcal"] = S(allc, wc) if solved else None
                    if p in R["rho"] and solved:
                        for wn, w in WS.items():
                            row["ratio_" + wn] = row["S_all_" + wn] / rho_S(p, w)
                            row["ratio_model_" + wn] = row["S_all_" + wn] / rho_S(p, w, True)
                        row["ratio_wcal"] = row["S_all_wcal"] / (R["rho"][p]["adds_mean"] + wc * R["rho"][p]["native_mean"])
                    rec.append(row)
            R["arms"][key][p] = rec
    # fits
    for key, per in R["arms"].items():
        F = {}
        for metric in ["rel_nat_all", "rel_nat_final", "la_nat_all", "S_all_w0", "S_all_w0.1", "S_all_w1", "m_final"]:
            samples = {p: [r[metric] for r in per.get(p, []) if r["solved"] and r[metric]] for p in LADDER}
            F[metric] = boot_fit(samples)
        R["fits"][key] = F
    # rho fit
    samples = {p: R["rho"][p]["adds_all"] for p in LADDER if p in R["rho"]}
    R["fits"]["rho_adds"] = boot_fit(samples, stat=lambda v: sum(v) / len(v))
    # ratio trend at w = 0.1 per arm
    trend = {}
    for key, per in R["arms"].items():
        trend[key] = {p: med([r["ratio_w0.1"] for r in per.get(p, []) if r.get("ratio_w0.1")]) for p in LADDER}
    R["ratio_w0.1_median"] = trend
    # yield + checks
    ylines = []
    for f in sorted(glob.glob(os.path.join(RES, "yield_*.jsonl"))):
        ylines += [json.loads(l) for l in open(f) if l.strip()]
    yp = {}
    for y in ylines:
        yp.setdefault(y["p"], []).append(y)
    for p, ys in sorted(yp.items()):
        fib = [sum(y["fib_hist_pencil1"][3] + y["fib_hist_pencil2"][3] for y in ys) / (2 * sum(y["fibres"] for y in ys))]
        cells = {}
        for phi_t in [0.3, 0.5, 0.7, 1.0]:
            cs = [c for y in ys for c in y["cells"] if c["phi_target"] == phi_t]
            pairs = sum(c["pairs"] for c in cs)
            comp = sum(c["n_f"][3] for c in cs)
            phi = sum(c["phi"] * c["pairs"] for c in cs) / pairs
            pred = phi ** 3 / 6
            scan_ok = all(c["scan"] is None or c["scan"]["hits_a"] == c["n_f"][3] for c in cs)
            has_scan = any(c["scan"] is not None for c in cs)
            nf = [sum(c["n_f"][k] for c in cs) for k in range(4)]
            tot = sum(c["split"] for c in cs)
            cells[str(phi_t)] = dict(pairs=pairs, hits=comp, phi=phi, yield_=comp / pairs, pred=pred, ratio=(comp / pairs) / pred if pred else None,
                                     n_f=nf, split_frac=tot / pairs, scan_equal=scan_ok if has_scan else None,
                                     anomalies=sum(c["anomalies"] for c in cs), degenerate=sum(c["degenerate"] for c in cs),
                                     native_per_pair=sum(c["native"] for c in cs) / pairs)
        R["yield"][p] = dict(curves=len(ys), fibre_split3=fib[0], cells=cells)
    # fibre law from dlp curves too
    fl = {}
    for p, ds in by_p.items():
        h = sum(d["fib_hist_pencil1"][3] + d["fib_hist_pencil2"][3] for d in ds)
        n = 2 * sum(d["fibres"] for d in ds)
        fl[p] = h / n
    R["fibre_law_dlp_curves"] = fl
    R["cells"] = [dict(p=d["p"], curve=d["curve"], N=d["N"], ell=d["ell"], npts=d["npts"], calib=d["calib"], total_secs=d["total_secs"],
                       curve_seed=d["curve_seed"], git=d["git"], rho_runs=len(d["rho"])) for d in data]
    json.dump(R, open(os.path.join(HERE, "RESULTS.json"), "w"), indent=1)
    write_md(R)


def f3(x):
    if x is None:
        return "-"
    return "%.3g" % x


def write_md(R):
    L = ["# RESULTS (all numbers [pcc:unverified]; local development check)", "",
         "Generated by `analyze.py` from `results/`. Costs in GAE (one combine). Per-target IC cost = ALL rungs' shared cost + that target's own cost.", ""]
    L += ["## Fibre law and A-line yield (R4, R5)", "",
          "| p | curves | fibres with 3 pts (pooled, both pencils) | phi | pairs | complete hits | yield/pair | phi^3/6 | ratio | split fraction of pairs | scan == root finder | anomalies | degenerate planes | native/pair |",
          "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|--:|--:|--:|"]
    for p, y in sorted(R["yield"].items(), key=lambda t: int(t[0])):
        for phi_t, c in y["cells"].items():
            L.append("| %s | %d | %s | %s | %d | %d | %s | %s | %s | %s | %s | %d | %d | %.0f |" % (
                p, y["curves"], f3(y["fibre_split3"]), f3(c["phi"]), c["pairs"], c["hits"], f3(c["yield_"]), f3(c["pred"]), f3(c["ratio"]),
                f3(c["split_frac"]), c["scan_equal"], c["anomalies"], c["degenerate"], c["native_per_pair"]))
    L += ["", "## DLP ladder: per arm, per curve (pass 0 and 1 = A/A)", ""]
    for key, per in R["arms"].items():
        L += ["### alpha = %s" % key, "",
              "| p | curve | pass | solved at rung | m (final) | phi | rows/cols used | partial rels | LP vertices | |U| | rel native (all rungs) | LA+filter native (all) | S all w=0 | S all w=0.1 | S all w=1 | S all w_cal | ratio w=0 | ratio w=0.1 | ratio w=1 | ratio w_cal | reason (last rung) |",
              "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|"]
        for p in LADDER:
            for r in per.get(p, []) or per.get(str(p), []):
                L.append("| %d | %d | %d | %s | %d | %.3f | %d/%d | %d | %d | %d | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (
                    p, r["curve"], r["pass_"], r["solved_at"], r["m_final"], r["phi_final"], r["rows"], r["nused"], r["partial"], r["lp_vertices"], r["u_size"],
                    f3(r["rel_nat_all"]), f3(r["la_nat_all"]), f3(r["S_all_w0"]), f3(r["S_all_w0.1"]), f3(r["S_all_w1"]), f3(r["S_all_wcal"]),
                    f3(r.get("ratio_w0")), f3(r.get("ratio_w0.1")), f3(r.get("ratio_w1")), f3(r.get("ratio_wcal")), r["reason"]))
        L.append("")
    L += ["## Matched rho (plain DP rho, same arithmetic; negation not built)", "",
          "| p | runs | verified | mean adds (incl. setup) | mean native | mean steps/sqrt(ell) |", "|--:|--:|--:|--:|--:|--:|"]
    for p, r in sorted(R["rho"].items(), key=lambda t: int(t[0])):
        L.append("| %s | %d | %d | %s | %s | %.3f |" % (p, r["runs"], r["verified"], f3(r["adds_mean"]), f3(r["native_mean"]), r["r_mean"]))
    L += ["", "Pooled steps/sqrt(ell) over all measured runs: %s" % f3(R["rho_model_r"]), "",
          "## Median S_IC / S_rho by p (all rungs, per target, pooled measured rho)", "",
          "| alpha | p | median ratio w=0.1 |", "|--:|--:|--:|"]
    for key, t in R["ratio_w0.1_median"].items():
        for p, v in t.items():
            L.append("| %s | %s | %s |" % (key, p, f3(v)))
    L += ["", "## Exponent fits (4 ladder points, bootstrap 95% over curves/runs; point [lo, hi])", "",
          "| series | metric | exponent |", "|:--|:--|:--|"]
    for key, F in R["fits"].items():
        if key == "rho_adds":
            L.append("| rho | adds | %s |" % ("%.2f [%.2f, %.2f]" % tuple(F) if F else "-"))
            continue
        for m, v in F.items():
            L.append("| alpha=%s | %s | %s |" % (key, m, "%.2f [%.2f, %.2f]" % tuple(v) if v else "no fit (< 4 solved p)"))
    open(os.path.join(HERE, "RESULTS.md"), "w").write("\n".join(L) + "\n")


if __name__ == "__main__":
    main()
