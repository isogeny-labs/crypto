#!/usr/bin/env python3
"""Reads results/raw/*.jsonl, writes RESULTS.json and RESULTS.md. Pure Python (std only).
usage: python3 summarize.py [results-dir containing raw*/ ] [out-dir]"""
import glob, json, math, os, random, sys

RAW = sys.argv[1] if len(sys.argv) > 1 else "results"
OUT = sys.argv[2] if len(sys.argv) > 2 else "."
WS = [0.0, 0.1, 1.0]
random.seed(20261115)


def load():
    ic, rho = [], []
    for f in sorted(glob.glob(os.path.join(RAW, "raw*", "*.jsonl"))):
        for line in open(f):
            line = line.strip()
            if not line:
                continue
            r = json.loads(line)
            (ic if r["kind"] == "ic" else rho).append(r)
    return ic, rho


def cI(q, rule):
    return 3.0 if rule == "3" else 2.0 * math.log2(q)


def gae(r, rule):
    return r["gae_M"] + cI(r["q"], rule) * r["gae_I"]


def S_ic(r, w, rule="3", dense=False):
    la = r["elim_ops"] + (r["dense_la"] if dense else r["wied_ops"] + r["bm_ops"])
    nat = r["NAT"] + r["elim_entries"]
    mpart = (r["M"] + cI(r["q"], rule) * r["I"]) / gae(r, rule)
    return mpart + w * (nat + la)


def la_ops(r, dense=False):
    return r["elim_ops"] + (r["dense_la"] if dense else r["wied_ops"] + r["bm_ops"])


def S_rho(r, w, rule="3"):
    return (r["M"] + cI(r["q"], rule) * r["I"]) / gae(r, rule) + w * r["NAT"]


def mean(v):
    return sum(v) / len(v) if v else float("nan")


def boot_ratio(num_by_inst, den_by_inst, n=2000):
    """ratio of means, bootstrap over instances (keys shared)."""
    keys = [k for k in num_by_inst if k in den_by_inst]
    if not keys:
        return None
    est = mean([num_by_inst[k] for k in keys]) / mean([den_by_inst[k] for k in keys])
    bs = []
    for _ in range(n):
        d = [random.choice(keys) for _ in keys]
        bs.append(mean([num_by_inst[k] for k in d]) / mean([den_by_inst[k] for k in d]))
    bs.sort()
    return [est, bs[int(0.025 * n)], bs[int(0.975 * n) - 1]]


def lsq(xs, ys):
    mx, my = mean(xs), mean(ys)
    sxx = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx if sxx else float("nan")


def boot_slope(qs, by_q, n=2000):
    """by_q: q -> {inst: value}. Slope of ln mean vs ln q, bootstrap over instances."""
    qs = [q for q in qs if by_q.get(q)]
    if len(qs) < 2:
        return None
    xs = [math.log(q) for q in qs]
    est = lsq(xs, [math.log(mean(list(by_q[q].values()))) for q in qs])
    bs = []
    for _ in range(n):
        ys = []
        for q in qs:
            vals = list(by_q[q].values())
            d = [random.choice(vals) for _ in vals]
            ys.append(math.log(mean(d)))
        bs.append(lsq(xs, ys))
    bs.sort()
    return [est, bs[int(0.025 * n)], bs[int(0.975 * n) - 1], len(qs)]


def load_calib():
    out = {}
    for f in sorted(glob.glob(os.path.join(RAW, "calib", "*.jsonl"))):
        for line in open(f):
            line = line.strip()
            if line:
                r = json.loads(line)
                out[r["q"]] = r
    return out


def main():
    ic, rho = load()
    calib = load_calib()
    res = {"git": ic[0]["git"] if ic else "unknown", "n_ic_records": len(ic), "n_rho_records": len(rho),
           "label": "[pcc:unverified] local development check", "cells": [], "rho": [], "fits": {},
           "predictions": {}, "ab": []}
    qs = sorted({r["q"] for r in ic})

    def wcal(q):
        """calibrated weight (Mont multiply-add over group addition) of the largest calibrated q <= q."""
        ks = [k for k in calib if k <= q]
        return calib[max(ks)]["w_cal_mont"] if ks else None

    res["calibration"] = [{"q": k, **{x: v[x] for x in ("t_add_s", "t_mont_op_s", "t_mulmod_s", "w_cal_mont", "w_cal_mulmod")}} for k, v in sorted(calib.items())]
    res["calibration_note"] = "best-of-N wall times on a shared 4-CPU VM (not isolated); q = 131071 uses the value of the largest calibrated q"
    # ---- rho: calibration and per-step costs
    plain = [r for r in rho if r["kind"] == "rho" and r["verified"]]
    negs = [r for r in rho if r["kind"] == "rhoneg" and r["verified"]]
    import math as m
    kappa_all = [r["steps"] / m.sqrt(r["ell"]) for r in plain]
    kappa = mean(kappa_all)
    res["rho_kappa"] = {"pooled_mean_steps_over_sqrt_ell": kappa, "n_runs": len(kappa_all),
                        "ideal_sqrt_pi_over_2": math.sqrt(math.pi / 2)}
    kn = [r["steps"] / m.sqrt(r["ell"]) for r in negs]
    res["rho_kappa"]["neg_pooled"] = mean(kn)
    res["rho_kappa"]["neg_n"] = len(kn)
    res["rho_runs_large_q"] = [{"q": r["q"], "inst": r["inst"], "ell": r["ell"], "cof": r["N"] // r["ell"], "steps": r["steps"],
                               "kappa": r["steps"] / m.sqrt(r["ell"])} for r in plain if r["q"] >= 16381]
    rq = sorted({r["q"] for r in plain})
    for q in rq:
        for kind, pool in (("rho", plain), ("rhoneg", negs)):
            sel = [r for r in pool if r["q"] == q]
            if not sel:
                continue
            row = {"q": q, "kind": kind, "n": len(sel), "mean_steps": mean([r["steps"] for r in sel]),
                   "kappa": mean([r["steps"] / m.sqrt(r["ell"]) for r in sel]),
                   "mean_adds": mean([r["adds"] for r in sel])}
            for w in WS:
                row["S_w%g" % w] = mean([S_rho(r, w) for r in sel])
            res["rho"].append(row)
    # per-step costs from the largest measured q (plain), used by the modelled rho
    qmax = max(rq)
    big = [r for r in plain if r["q"] == qmax]
    step_cost = {}
    for rule in ("3", "2log"):
        step_cost[rule] = mean([((r["M"] + cI(r["q"], rule) * r["I"]) / gae(r, rule)) / r["steps"] for r in big])
    nat_step = mean([r["NAT"] / r["steps"] for r in big])
    res["rho_model"] = {"from_q": qmax, "gae_per_step_cI3": step_cost["3"], "gae_per_step_cI2log2q": step_cost["2log"],
                        "nat_per_step": nat_step, "kappa": kappa}

    kappa_alt = mean([r["steps"] / m.sqrt(r["ell"]) for r in big])
    res["rho_model"]["kappa_alt_largest_measured_q"] = kappa_alt
    res["rho_model"]["kappa_alt_n"] = len(big)

    def rho_model(ell, q, w, rule="3", k=None):
        k = kappa if k is None else k
        steps = k * math.sqrt(ell)
        sc = step_cost["3"] if rule == "3" else step_cost["2log"]
        return steps * (sc + w * nat_step)

    # ---- IC cells
    by_cell = {}
    for r in ic:
        by_cell.setdefault((r["q"], r["c"], r["tag"]), []).append(r)
    S_by = {}  # (kind, q, c, w, rule) -> {inst: value}
    for (q, c, tag), recs in sorted(by_cell.items()):
        ok = [r for r in recs if r["status"] == "ok" and r["verified"]]
        cell = {"q": q, "c": c / 100, "tag": tag, "n": len(recs), "n_ok": len(ok),
                "failures": sorted({r["status"] for r in recs if r["status"] != "ok" or not r["verified"]})}
        if ok:
            def mm(f):
                return mean([f(r) for r in ok])
            cell.update({
                "b": ok[0]["b"], "pairs": mm(lambda r: r["pairs"]), "edges": mm(lambda r: r["edges"]),
                "rows_fb": mm(lambda r: r["rows_fb"]), "w_raw_max": max(r["w_raw_max"] for r in ok),
                "w_fb_mean": mm(lambda r: r["w_fb_mean"]), "rounds": mm(lambda r: r["rounds"]),
                "known_frac": mm(lambda r: r["known_frac"]),
                "adds_target": mm(lambda r: r["tgt_adds"]),
                "M": mm(lambda r: r["M"]), "I": mm(lambda r: r["I"]), "NAT": mm(lambda r: r["NAT"] + r["elim_entries"]),
                "LA_elim": mm(lambda r: r["elim_ops"]), "LA_wied": mm(lambda r: r["wied_ops"]),
                "LA_bm": mm(lambda r: r["bm_ops"]), "LA_dense": mm(lambda r: r["dense_la"]) if ok[0]["dense_ok"] else None,
                "wall_s": mm(lambda r: r["wall_rel"] + r["wall_elim"] + r["wall_wied"] + r["wall_tgt"]),
                "wall_rel": mm(lambda r: r["wall_rel"]), "wall_elim": mm(lambda r: r["wall_elim"]),
                "wall_wied": mm(lambda r: r["wall_wied"]), "wall_dense": mm(lambda r: r["wall_dense"]),
                "agree": [r["agree"] for r in ok],
            })
            rel_gae = mm(lambda r: ((r["setup_M"] + r["rel_M"]) + 3 * (r["setup_I"] + r["rel_I"])) / gae(r, "3"))
            tgt_gae = mm(lambda r: (r["tgt_M"] + 3 * r["tgt_I"]) / gae(r, "3"))
            cell["gae_relation_phase"] = rel_gae
            cell["gae_target_phase"] = tgt_gae
            for rule in ("3", "2log"):
                for w in WS:
                    vals = {r["inst"]: S_ic(r, w, rule) for r in ok}
                    # A/A and main share inst keys; keep tag-specific dicts
                    S_by[("ic", q, c, tag, w, rule)] = vals
                    cell["S_w%g_cI%s" % (w, rule)] = mean(list(vals.values()))
                    if rule == "3":
                        cell["LAshare_w%g" % w] = mean([w * la_ops(r) / S_ic(r, w, rule) for r in ok]) if w > 0 else 0.0
                if rule == "3" and wcal(q):
                    for lab, mult in (("wcal", 1.0), ("wcal10", 10.0)):
                        ww = wcal(q) * mult
                        cell["S_" + lab] = mean([S_ic(r, ww) for r in ok])
                        cell["LAshare_" + lab] = mean([ww * la_ops(r) / S_ic(r, ww) for r in ok])
                if ok[0]["dense_ok"]:
                    for w in WS:
                        vals = {r["inst"]: S_ic(r, w, rule, dense=True) for r in ok if r["dense_ok"]}
                        S_by[("dense", q, c, tag, w, rule)] = vals
                        if rule == "3":
                            cell["Sdense_w%g" % w] = mean(list(vals.values()))
        res["cells"].append(cell)
    # ---- A/A
    aa = []
    for q in qs:
        a = {r["inst"]: r for r in by_cell.get((q, 200, "main"), []) if r["status"] == "ok" and r["verified"]}
        b = {r["inst"]: r for r in by_cell.get((q, 200, "aa"), []) if r["status"] == "ok" and r["verified"]}
        for k in sorted(set(a) & set(b)):
            aa.append({"q": q, "inst": k, **{"ratio_w%g" % w: S_ic(a[k], w) / S_ic(b[k], w) for w in WS}})
    res["aa"] = aa
    # ---- distance to matched rho (modelled and measured), per cell
    dist = []
    ell_of = {}
    for r in ic:
        ell_of[(r["q"], r["inst"])] = r["ell"]
    all_cs = sorted({c for (_, c, _) in by_cell})
    for tag in ("main", "bfs", "light"):
      for c in all_cs:
        for q in qs:
            ok = {r["inst"]: r for r in by_cell.get((q, c, tag), []) if r["status"] == "ok" and r["verified"]}
            if not ok:
                continue
            row = {"q": q, "c": c / 100, "tag": tag, "n": len(ok)}
            for w in WS:
                num = {k: S_ic(r, w) for k, r in ok.items()}
                den_mod = {k: rho_model(ell_of[(q, k)], q, w) for k in ok}
                den_ideal = {k: math.sqrt(math.pi / 2) * math.sqrt(ell_of[(q, k)]) * (step_cost["3"] + w * nat_step) for k in ok}
                row["S_over_rho_model_w%g" % w] = boot_ratio(num, den_mod)
                den_alt = {k: rho_model(ell_of[(q, k)], q, w, k=kappa_alt) for k in ok}
                row["S_over_rho_kalt_w%g" % w] = boot_ratio(num, den_alt)
                row["S_over_rho_ideal_w%g" % w] = boot_ratio(num, den_ideal)
                for kind, pool in (("rho", plain), ("rhoneg", negs)):
                    d = {}
                    for k in ok:
                        sel = [S_rho(x, w) for x in pool if x["q"] == q and x["inst"] == k]
                        if sel:
                            d[k] = mean(sel)
                    if d:
                        row["S_over_%s_measured_w%g" % (kind, w)] = boot_ratio(num, d)
            for lab, mult in (("wcal", 1.0), ("wcal10", 10.0)):
                if wcal(q):
                    ww = wcal(q) * mult
                    num = {k: S_ic(r, ww) for k, r in ok.items()}
                    den = {k: rho_model(ell_of[(q, k)], q, ww) for k in ok}
                    row["S_over_rho_model_" + lab] = boot_ratio(num, den)
                    for kind, pool in (("rho", plain), ("rhoneg", negs)):
                        d = {}
                        for k in ok:
                            sel = [S_rho(x, ww) for x in pool if x["q"] == q and x["inst"] == k]
                            if sel:
                                d[k] = mean(sel)
                        if d:
                            row["S_over_%s_measured_%s" % (kind, lab)] = boot_ratio(num, d)
            # break-even price: the w at which S_IC(w) = S_rho_model(w) (linear in w)
            s0i = mean([S_ic(r, 0.0) for r in ok.values()])
            ni = mean([r["NAT"] + r["elim_entries"] + la_ops(r) for r in ok.values()])
            s0r = mean([rho_model(ell_of[(q, k)], q, 0.0) for k in ok])
            nr = mean([kappa * math.sqrt(ell_of[(q, k)]) * nat_step for k in ok])
            row["breakeven_w_vs_rho_model"] = (s0r - s0i) / (ni - nr) if ni > nr else None
            dist.append(row)
    res["distance"] = dist
    # ---- fits (q >= 2^12), IC main arm at each c with all 3 instances
    for c, tag in [(c, t) for t in ("main", "bfs", "light") for c in all_cs]:
        for w in WS:
            by_q = {}
            for q in qs:
                if q < 4093:
                    continue
                v = S_by.get(("ic", q, c, tag, w, "3"))
                if v:
                    by_q[q] = v
            f = boot_slope(sorted(by_q), by_q)
            res["fits"]["ic_%s_c%g_w%g" % (tag, c / 100, w)] = f
    for w in WS:
        by_q = {}
        for q in rq:
            if q < 4093:
                continue
            by_q[q] = {x["inst"]: S_rho(x, w) for x in plain if x["q"] == q}
        res["fits"]["rho_measured_w%g" % w] = boot_slope(sorted(by_q), by_q)
    for w in WS:
        by_q = {}
        for q in qs:
            if q < 4093:
                continue
            by_q[q] = {k: rho_model(ell_of[(q, k)], q, w) for (qq, k) in ell_of if qq == q}
        res["fits"]["rho_model_w%g" % w] = boot_slope(sorted(by_q), by_q)
    for lab, mult in (("wcal", 1.0), ("wcal10", 10.0)):
        by_q = {}
        for q in qs:
            if q < 4093 or not wcal(q):
                continue
            ok = [r for r in by_cell.get((q, 200, "main"), []) if r["status"] == "ok" and r["verified"]]
            if ok:
                by_q[q] = {r["inst"]: S_ic(r, wcal(q) * mult) for r in ok}
        res["fits"]["ic_main_c2_" + lab] = boot_slope(sorted(by_q), by_q)
    # component exponents (c = 2, primary arm and the exploratory modes): row weight, pair trials, relation-phase M-part
    for tag in ("main", "bfs", "light"):
        comp = {"wfb": lambda r: r["w_fb_mean"], "pairs": lambda r: r["pairs"],
                "relGAE": lambda r: ((r["setup_M"] + r["rel_M"]) + 3 * (r["setup_I"] + r["rel_I"])) / gae(r, "3"),
                "tgtadds": lambda r: r["tgt_adds"]}
        for name, fn in comp.items():
            by_q = {}
            for q in qs:
                if q < 4093:
                    continue
                ok = [r for r in by_cell.get((q, 200, tag), []) if r["status"] == "ok" and r["verified"]]
                if ok:
                    by_q[q] = {r["inst"]: fn(r) for r in ok}
            f = boot_slope(sorted(by_q), by_q)
            if f:
                res["fits"]["%s_%s_c2" % (name, tag)] = f
    # dense A/B exponent (c = 2): LA op count only
    for kind in ("wied", "dense"):
        by_q = {}
        for q in qs:
            if q < 4093:
                continue
            ok = [r for r in by_cell.get((q, 200, "main"), []) if r["status"] == "ok" and r["verified"]]
            if kind == "dense":
                ok = [r for r in ok if r["dense_ok"]]
            if ok:
                by_q[q] = {r["inst"]: la_ops(r, dense=(kind == "dense")) for r in ok}
        res["fits"]["LAops_%s_c2" % kind] = boot_slope(sorted(by_q), by_q)
    # ---- predictions
    p1 = {}
    for w in (0.1, 1.0):
        for q in qs:
            if q >= 16381:
                cell = next((c for c in res["cells"] if c["q"] == q and c["c"] == 2.0 and c["tag"] == "main"), None)
                if cell and cell["n_ok"]:
                    p1["q%d_w%g" % (q, w)] = cell["LAshare_w%g" % w]
    res["predictions"]["P1_LA_share"] = {"values": p1, "threshold": 0.2,
                                         "holds": all(v < 0.2 for v in p1.values()) if p1 else None}
    p2 = {}
    for w in WS:
        f = res["fits"].get("ic_main_c2_w%g" % w)
        if f:
            p2["w%g" % w] = f
    res["predictions"]["P2_exponent"] = {"values": p2, "interval": [0.95, 1.15],
                                         "holds": all(0.95 <= v[0] <= 1.15 for v in p2.values()) if p2 else None}
    p3 = {}
    for row in dist:
        if row["c"] == 2.0 and row["q"] >= 16381 and row["tag"] == "main":
            for w in WS:
                v = row["S_over_rho_model_w%g" % w]
                p3["q%d_w%g" % (row["q"], w)] = v
    res["predictions"]["P3_S_over_rho"] = {"values": p3,
                                           "holds": all(v is not None and v[0] < 1 for v in p3.values()) if p3 else None,
                                           "holds_by_w": {("w%g" % w): all(v[0] < 1 for k, v in p3.items() if k.endswith("_w%g" % w)) for w in WS}}
    p3c = {}
    for row in dist:
        if row["c"] == 2.0 and row["q"] >= 16381 and row["tag"] == "main" and "S_over_rho_model_wcal" in row:
            p3c["q%d" % row["q"]] = {"w0": row["S_over_rho_model_w0"], "wcal": row["S_over_rho_model_wcal"], "wcal10": row["S_over_rho_model_wcal10"]}
    res["predictions"]["P3_calibrated_variant"] = {"values": p3c, "holds": all(all(v[k] is not None and v[k][0] < 1 for k in v) for v in p3c.values()) if p3c else None,
                                                   "ranked_at_calibrated_price": {q: all(v[k] is not None and v[k][2] < 1 for k in v) for q, v in p3c.items()}}
    # crossover dense vs sparse
    for q in qs:
        cell = next((c for c in res["cells"] if c["q"] == q and c["c"] == 2.0 and c["tag"] == "main"), None)
        if cell and cell["n_ok"] and cell.get("LA_dense") is not None:
            res["ab"].append({"q": q, "b": cell["b"], "LA_sparse_total": cell["LA_elim"] + cell["LA_wied"] + cell["LA_bm"],
                              "LA_dense_total": cell["LA_elim"] + cell["LA_dense"], "w_fb_mean": cell["w_fb_mean"]})
    res["rho_model_by_q"] = []
    for q in qs:
        ells = [ell_of[(q, k)] for (qq, k) in ell_of if qq == q]
        row = {"q": q, "mean_ell": mean(ells), "n_inst": len(ells)}
        for w in WS:
            row["S_w%g" % w] = mean([rho_model(e, q, w) for e in ells])
            row["S_ideal_w%g" % w] = mean([math.sqrt(math.pi / 2) * math.sqrt(e) * (step_cost["3"] + w * nat_step) for e in ells])
        res["rho_model_by_q"].append(row)
    json.dump(res, open(os.path.join(OUT, "RESULTS.json"), "w"), indent=1)
    write_md(res, os.path.join(OUT, "RESULTS.md"))


def fmt(v, p=3):
    if v is None:
        return "-"
    if isinstance(v, list):
        return "%.*g [%.*g, %.*g]" % (p, v[0], p, v[1], p, v[2])
    return "%.*g" % (p, v)


def write_md(res, path):
    L = []
    L.append("# RESULTS (%s)\n" % res["label"])
    L.append("Git HEAD at run time: `%s`. IC records: %d, rho records: %d.\n" % (res["git"], res["n_ic_records"], res["n_rho_records"]))
    k = res["rho_kappa"]
    L.append("Rho calibration: pooled mean steps/sqrt(ell) = %.3f over %d verified plain runs (textbook sqrt(pi/2) = %.3f); negation map %.3f over %d runs.\n" % (k["pooled_mean_steps_over_sqrt_ell"], k["n_runs"], k["ideal_sqrt_pi_over_2"], k["neg_pooled"], k["neg_n"]))
    m = res["rho_model"]
    L.append("Sensitivity: kappa at the largest measured q (q = %d, %d runs) is %.3f against the pooled %.3f; the `S_over_rho_kalt_w*` fields of RESULTS.json use it.\n" % (res["rho_model"]["from_q"], res["rho_model"]["kappa_alt_n"], res["rho_model"]["kappa_alt_largest_measured_q"], k["pooled_mean_steps_over_sqrt_ell"]))
    L.append("Modelled rho (q above the measured range): steps = kappa sqrt(ell); per step %.3f GAE (cI = 3) and %.2f native operations, from the plain runs at q = %d.\n" % (m["gae_per_step_cI3"], m["nat_per_step"], m["from_q"]))
    L.append("## Measured rho\n")
    L.append("| q | arm | runs | mean steps | steps/sqrt(ell) | mean additions | S w=0 | S w=0.1 | S w=1 |\n|--:|:--|--:|--:|--:|--:|--:|--:|--:|")
    for r in res["rho"]:
        L.append("| %d | %s | %d | %.0f | %.2f | %.0f | %.4g | %.4g | %.4g |" % (r["q"], r["kind"], r["n"], r["mean_steps"], r["kappa"], r["mean_adds"], r["S_w0"], r["S_w0.1"], r["S_w1"]))
    L.append("\n## IC cells (verified units only; means over instances)\n")
    L.append("| q | c | tag | ok/n | b | pairs | rows after elim | row weight (after) | LA elim | LA Wiedemann | LA BM | LA dense (A/B) | target adds | S w=0 | S w=0.1 | S w=1 | LA share w=0.1 | LA share w=1 | wall s |\n|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
    for c in res["cells"]:
        if not c["n_ok"]:
            L.append("| %d | %g | %s | 0/%d | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | (%s)" % (c["q"], c["c"], c["tag"], c["n"], ",".join(c["failures"])))
            continue
        L.append("| %d | %g | %s | %d/%d | %d | %.0f | %.0f | %.1f | %.3g | %.3g | %.3g | %s | %.0f | %.4g | %.4g | %.4g | %.2f | %.2f | %.1f |" % (
            c["q"], c["c"], c["tag"], c["n_ok"], c["n"], c["b"], c["pairs"], c["rows_fb"], c["w_fb_mean"], c["LA_elim"], c["LA_wied"], c["LA_bm"],
            ("%.3g" % c["LA_dense"]) if c.get("LA_dense") is not None else "-", c["adds_target"], c["S_w0_cI3"], c["S_w0.1_cI3"], c["S_w1_cI3"],
            c["LAshare_w0.1"], c["LAshare_w1"], c["wall_s"]))
    L.append("\nFailures: " + "; ".join("q=%d c=%g %s: %s" % (c["q"], c["c"], c["tag"], ",".join(c["failures"])) for c in res["cells"] if c["failures"]) + "\n")
    L.append("## Distance to matched rho (S_IC / S_rho, bootstrap over instances)\n")
    L.append("Modelled rho = kappa sqrt(ell) with pooled kappa; ideal = sqrt(pi/2) sqrt(ell) steps at the same per-step cost (a lower reference). Measured columns where rho runs exist.\n")
    L.append("| q | c | tag | w=0 model | w=0.1 model | w=1 model | w=0 ideal | w=0.1 ideal | w=1 ideal | w=1 measured rho | w=1 measured rhoneg | break-even w |\n|--:|--:|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|")
    for r in res["distance"]:
        L.append("| %d | %g | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (r["q"], r["c"], r["tag"], fmt(r["S_over_rho_model_w0"]), fmt(r["S_over_rho_model_w0.1"]), fmt(r["S_over_rho_model_w1"]),
                                                     fmt(r["S_over_rho_ideal_w0"]), fmt(r["S_over_rho_ideal_w0.1"]), fmt(r["S_over_rho_ideal_w1"]),
                                                     fmt(r.get("S_over_rho_measured_w1")), fmt(r.get("S_over_rhoneg_measured_w1")), fmt(r.get("breakeven_w_vs_rho_model"))))
    L.append("\n## Distance to matched rho at the calibrated price (AGENTS.md section 5a, w_cal = one mod-ell multiply-add / one group addition)\n")
    L.append("Native operations (mod-ell and hash) priced at w_cal and 10 w_cal; w_cal per q: " + ", ".join("%d: %.2e" % (c["q"], c["w_cal_mont"]) for c in res["calibration"]) + ". " + res["calibration_note"] + ".\n")
    L.append("| q | c | tag | w=0 | w_cal | 10 w_cal | w_cal vs measured rho | w_cal vs measured rhoneg |\n|--:|--:|:--|:--|:--|:--|:--|:--|")
    for r in res["distance"]:
        if "S_over_rho_model_wcal" in r:
            L.append("| %d | %g | %s | %s | %s | %s | %s | %s |" % (r["q"], r["c"], r["tag"], fmt(r["S_over_rho_model_w0"]), fmt(r["S_over_rho_model_wcal"]), fmt(r["S_over_rho_model_wcal10"]),
                                                      fmt(r.get("S_over_rho_measured_wcal")), fmt(r.get("S_over_rhoneg_measured_wcal"))))
    L.append("\n## Exponents on q >= 2^12 (log-log slope, 95% bootstrap over instances, number of q)\n")
    L.append("| quantity | slope | 95% | n_q |\n|:--|--:|:--|--:|")
    for k2, v in res["fits"].items():
        if v:
            L.append("| %s | %.3f | [%.3f, %.3f] | %d |" % (k2, v[0], v[1], v[2], v[3]))
    L.append("\n## Dense A/B (c = 2): LA operations on the same rows\n")
    L.append("| q | b | row weight | sparse (elim + Wiedemann + BM) | dense (elim + Gauss-Jordan) | dense/sparse |\n|--:|--:|--:|--:|--:|--:|")
    for r in res["ab"]:
        L.append("| %d | %d | %.1f | %.3g | %.3g | %.2f |" % (r["q"], r["b"], r["w_fb_mean"], r["LA_sparse_total"], r["LA_dense_total"], r["LA_dense_total"] / r["LA_sparse_total"]))
    L.append("\n## A/A (c = 2, two IC seeds, same instance): S(seed 1)/S(seed 2)\n")
    L.append("| q | inst | w=0 | w=0.1 | w=1 |\n|--:|--:|--:|--:|--:|")
    for r in res["aa"]:
        L.append("| %d | %d | %.3f | %.3f | %.3f |" % (r["q"], r["inst"], r["ratio_w0"], r["ratio_w0.1"], r["ratio_w1"]))
    L.append("\n## Predictions\n")
    for k2, v in res["predictions"].items():
        L.append("* %s: holds = %s; %s" % (k2, v.get("holds"), json.dumps(v.get("values"), default=str)[:1500]))
        if "holds_by_w" in v:
            L.append("  per price: %s" % v["holds_by_w"])
    open(path, "w").write("\n".join(L) + "\n")


main()
