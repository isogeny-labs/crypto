#!/usr/bin/env python3
"""EXPLORATORY, POST HOC deep dive on H-T and H-M of the volcano census follow-up (study 20261023).

Nothing here changes a preregistered verdict (REPORT.md, analysis/results.json stay as they are); it reads only
committed inputs through analysis/analyze.py's loaders and writes results_deepdive.json, figures/ (SVG + PNG).
Seed 20261106 for every new bootstrap / permutation / simulation, 10,000 replicates unless a section says otherwise.

usage: deepdive.py [--cache PATH] [--quick]    (--cache: scratch pickle of the compact subset, speeds up development)
"""
import argparse, hashlib, json, math, os, pickle, sys, time
import numpy as np
import pandas as pd
from scipy import stats

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, f"{ROOT}/analysis")
import analyze as A  # committed analysis module (unmodified): loaders, holm, ht_arrays

SEED = 20261106
REPS = 10000
PRICES = [0.0, 0.1, 1.0]
THEORY = 1 / math.sqrt(3)
ARMS_M = ["ic", "rho-neg", "bsgs"]
R = {}  # results


def sha256(path):
    return A.sha256(path)


def log(*a):
    print(f"[{time.strftime('%H:%M:%S')}]", *a, flush=True)


def pct(x, lo=2.5, hi=97.5):
    return [float(np.percentile(x, lo)), float(np.percentile(x, hi))]


def rng_for(*key):
    return np.random.default_rng(np.random.SeedSequence([SEED, *key]))


def load(cache=None):
    if cache and os.path.exists(cache):
        return pickle.load(open(cache, "rb"))
    d, chk, VERT, names, done, prog = A.load()
    tc, tcr = A.transfer_cost(VERT)
    keep = (d.volcano == "V5") | (d.family == "M") | ((d.volcano == "V6") & d.family.isin(["D", "ctl"]) & d.arm.isin(["ic", "rho-neg", "bsgs"]))
    d = d[keep].drop(columns=["online_wall_ns", "solve_wall_ns", "spec", "role", "status", "isolation", "total_gae", "s"]).reset_index(drop=True)
    out = (d, tcr, VERT, json.load(open(f"{ROOT}/analysis/results.json")))
    if cache:
        pickle.dump(out, open(cache, "wb"))
    return out


# =====================================================================================================
# H-T
# =====================================================================================================
def ht_columns(d, tcr):
    """analysis' per-vertex pair arrays (11 columns) + extras: 11 canon(cubic) 12 canon(neg on Tcr) 13 canon(neg on N)
    14 resid(cubic)=ops-canon 15 resid(neg Tcr) 16 resid(N)  (canon = uncharged canonicalisations, derived as the price-1 minus price-0 column)."""
    Aarr, level, Mv, chk = A.ht_arrays(d, tcr)
    out = {}
    for v, a in Aarr.items():
        cc, cn, cN = a[:, 2] - a[:, 0], a[:, 5] - a[:, 3], a[:, 8] - a[:, 6]
        out[v] = np.column_stack([a, cc, cn, cN, a[:, 0] - cc, a[:, 3] - cn, a[:, 6] - cN])
    return out, level, Mv, chk


def ht_stats(cm, mv):
    """cm (..., k, 17) per-vertex means, mv (..., k); vertex-averaged ratios. Returns dict of arrays (..., 3) / (...)."""
    p = np.array(PRICES)
    mean = cm.mean(-2)
    cub, negt, nat = mean[..., 0:3], mean[..., 3:6], mean[..., 6:9]
    mb = mv.mean(-1)[..., None]
    o = {}
    o["T"] = (cub + p * mb) / nat
    o["T0"] = (negt + p * mb) / nat
    o["F"] = (cub + p * mb) / (negt + p * mb)       # crater factor incl. transport in both
    o["G"] = cub / negt                              # pure automorphism factor in priced ops_gae (no transport)
    o["share"] = p * mb / (cub + p * mb)
    o["S_ratio"] = mean[..., 9] / mean[..., 10]
    o["canon_ratio"] = mean[..., 11] / mean[..., 12]
    o["resid_cub"], o["resid_neg"] = mean[..., 14], mean[..., 15]
    o["canon_neg"], o["canon_cub"] = mean[..., 12], mean[..., 11]
    fo, cn = mean[..., 15], mean[..., 12]
    o["pred_ops_ratio_theory"] = (THEORY * cn + fo) / (cn + fo)    # ops ratio if iterations scaled by exactly 1/sqrt3 and the rest stayed
    o["ops_ratio_p0"] = mean[..., 0] / mean[..., 3]
    o["nat_over_negt"] = nat[..., 0] / negt[..., 0]
    o["mbar"] = mb[..., 0]
    return o


def ht_section(d, tcr, rt):
    Aa, level, Mv, chk = ht_columns(d, tcr)
    levels = sorted(set(level.values()))
    verts = {L: [v for v in sorted(Aa) if level[v] == L] for L in levels}
    vm = {v: Aa[v].mean(0) for v in Aa}
    def pooled(vs):
        cm = np.stack([vm[v] for v in vs])[None]
        mv = np.array([Mv[v] for v in vs])[None]
        return {k: x[0] for k, x in ht_stats(cm, mv).items()}
    obs_all = pooled(sorted(Aa))
    obs_lvl = {L: pooled(verts[L]) for L in levels}
    # reproduce the preregistered point values (sanity: same data, same formula)
    for i, p in enumerate(PRICES):
        assert abs(obs_all["T"][i] - rt["H-T"]["overall"]["T"][str(p)]["value"]) < 1e-9
        assert abs(obs_all["T0"][i] - rt["H-T"]["overall"]["T0"][str(p)]["value"]) < 1e-9
    # cluster bootstrap (vertices within level, then pairs jointly), SEED 20261106, 10,000 replicates
    rng = rng_for(1)
    nv = sum(len(verts[L]) for L in levels)
    keys = ["T", "T0", "F", "G", "share", "S_ratio", "canon_ratio", "pred_ops_ratio_theory", "ops_ratio_p0", "canon_neg", "canon_cub", "resid_cub", "resid_neg", "mbar", "nat_over_negt"]
    rep_all = {k: [] for k in keys}
    rep_lvl = {L: {k: [] for k in keys} for L in levels}
    Aarr = {L: np.stack([Aa[v] for v in verts[L]]) for L in levels}
    Marr = {L: np.array([Mv[v] for v in verts[L]]) for L in levels}
    done = 0
    while done < REPS:
        c = min(100, REPS - done)
        scm, smv = np.zeros((c, 17)), np.zeros(c)
        for L in levels:
            m, n = Aarr[L].shape[:2]
            vi = rng.integers(0, m, (c, m))
            pi = rng.integers(0, n, (c, m, n))
            vals = Aarr[L][vi[:, :, None], pi].mean(2)
            mvv = Marr[L][vi]
            st = ht_stats(vals, mvv)
            for k in keys:
                rep_lvl[L][k].append(st[k])
            scm += vals.sum(1); smv += mvv.sum(1)
        st = ht_stats((scm / nv)[:, None, :], (smv / nv)[:, None])
        for k in keys:
            rep_all[k].append(st[k])
        done += c
    rep_all = {k: np.concatenate(v) for k, v in rep_all.items()}
    rep_lvl = {L: {k: np.concatenate(v) for k, v in r.items()} for L, r in rep_lvl.items()}

    def pack(obs, rep):
        o = {}
        for k in keys:
            ov, rv = np.atleast_1d(obs[k]), rep[k]
            if rv.ndim == 1:
                rv = rv[:, None]
            o[k] = {"value": [float(x) for x in ov], "ci95": [pct(rv[:, i]) for i in range(rv.shape[1])]}
        return o
    res = {"n_vertices": nv, "seed": SEED, "reps": REPS, "label": "EXPLORATORY, POST HOC",
           "overall": pack(obs_all, rep_all), "by_level": {str(L): dict(pack(obs_lvl[L], rep_lvl[L]), n_vertices=len(verts[L])) for L in levels},
           "prices": PRICES, "theory": THEORY}
    # log-variance decomposition of T at p = 1 (which factor carries the variation of T across bootstrap replicates)
    lt, lf, l0 = np.log(rep_all["T"][:, 2]), np.log(rep_all["F"][:, 2]), np.log(rep_all["T0"][:, 2])
    res["log_variance_share_p1"] = {"var_logT": float(lt.var()), "var_logF": float(lf.var()), "var_logT0": float(l0.var()), "cov": float(np.cov(lf, l0)[0, 1]),
                                    "note": "T = F * T0 exactly; var(logT)=var(logF)+var(logT0)+2cov; F is the crater (automorphism) factor with transport in numerator and denominator, T0 the control ratio"}
    # identity check T = F * T0 at every price, overall and per level
    res["identity_max_abs_err"] = float(max(np.abs(obs_all["T"] - obs_all["F"] * obs_all["T0"]).max(), *[np.abs(obs_lvl[L]["T"] - obs_lvl[L]["F"] * obs_lvl[L]["T0"]).max() for L in levels]))
    # exact decomposition of log T into log G (pure automorphism), log of transport correction, and log T0
    res["log_decomp_overall_p1"] = {"log_G": float(np.log(obs_all["G"][2])), "log_transport_corr": float(np.log(obs_all["F"][2] / obs_all["G"][2])),
                                    "log_T0": float(np.log(obs_all["T0"][2])), "log_T": float(np.log(obs_all["T"][2]))}

    # ---------- Q2: gap to theory, fixed overhead ----------
    q2 = {"label": "EXPLORATORY, POST HOC", "theory": THEORY}
    q2["tcr_overall"] = {"S_ratio": res["overall"]["S_ratio"], "ops_ratio_p0": res["overall"]["ops_ratio_p0"], "canon_ratio": res["overall"]["canon_ratio"],
                         "pred_ops_ratio_if_iterations_exactly_theory": res["overall"]["pred_ops_ratio_theory"],
                         "mean_canon_neg": res["overall"]["canon_neg"], "mean_canon_cub": res["overall"]["canon_cub"],
                         "mean_resid_neg": res["overall"]["resid_neg"], "mean_resid_cub": res["overall"]["resid_cub"]}
    # per-row facts on the Tcr pairs (n rows): is ops - canon identical between arms and independent of walk length?
    allp = np.concatenate([Aa[v] for v in sorted(Aa)])
    q2["n_pairs_tcr"] = int(len(allp))
    q2["resid_cub_equals_resid_neg_fraction"] = float(np.mean(allp[:, 14] == allp[:, 15]))
    q2["corr_resid_with_canon_neg"] = float(np.corrcoef(allp[:, 15], allp[:, 12])[0, 1])
    q2["corr_resid_with_canon_cub"] = float(np.corrcoef(allp[:, 14], allp[:, 11])[0, 1])
    q2["resid_mean_sd_min_max"] = [float(allp[:, 15].mean()), float(allp[:, 15].std(ddof=1)), float(allp[:, 15].min()), float(allp[:, 15].max())]
    assert np.allclose(allp[:, 0] / allp[:, 9], 4082.361204, rtol=1e-9) and np.allclose(allp[:, 3] / allp[:, 10], 4082.361204, rtol=1e-9)  # S = ops_gae / sqrt(r)
    q2["sqrt_r"] = 4082.361204
    q2["theory_iterations_over_sqrt_r"] = {"k2": math.sqrt(math.pi / 4), "k6": math.sqrt(math.pi / 12)}
    q2["measured_canon_over_sqrt_r"] = {"rho-neg": float(allp[:, 12].mean() / 4082.361204), "rho-cubic": float(allp[:, 11].mean() / 4082.361204)}
    q2["measured_over_theory"] = {"rho-neg": q2["measured_canon_over_sqrt_r"]["rho-neg"] / q2["theory_iterations_over_sqrt_r"]["k2"],
                                  "rho-cubic": q2["measured_canon_over_sqrt_r"]["rho-cubic"] / q2["theory_iterations_over_sqrt_r"]["k6"]}
    # distribution shapes (rows pooled over the 63 vertices; Tcr pairs)
    qs = [5, 25, 50, 75, 95, 99]
    def shape(x):
        return {"n": int(len(x)), "mean": float(x.mean()), "median": float(np.median(x)), "cv": float(x.std(ddof=1) / x.mean()), "skew": float(stats.skew(x)),
                "quantiles": {str(q): float(np.percentile(x, q)) for q in qs}, "median_over_mean": float(np.median(x) / x.mean())}
    Sc, Sn = allp[:, 9], allp[:, 10]
    q2["shapes_tcr"] = {"S_cubic": shape(Sc), "S_neg": shape(Sn), "canon_cubic": shape(allp[:, 11]), "canon_neg": shape(allp[:, 12]),
                        "rayleigh_reference": {"cv": math.sqrt((4 - math.pi) / math.pi), "median_over_mean": math.sqrt(2 * math.log(2)) / math.sqrt(math.pi / 2)}}
    rb = rng_for(2)
    nb = len(allp)
    idx = rb.integers(0, nb, (REPS, nb))
    qr = {}
    for name, ci_, ni_ in [("S", 9, 10), ("canon", 11, 12)]:
        qr[name] = {}
        bs_c = np.stack([np.percentile(allp[idx[i], ci_], qs) for i in range(0, REPS, 10)])
        bs_n = np.stack([np.percentile(allp[idx[i], ni_], qs) for i in range(0, REPS, 10)])
        ratio = bs_c / bs_n
        qr[name] = {"quantiles": qs, "ratio_cubic_over_neg": [float(np.percentile(allp[:, ci_], q) / np.percentile(allp[:, ni_], q)) for q in qs],
                    "ci95": [pct(ratio[:, j]) for j in range(len(qs))], "n_boot": int(len(ratio)), "note": "row bootstrap over the pooled Tcr pairs ignoring vertex clustering; 1000 replicates for the quantile CIs"}
    q2["quantile_ratios"] = qr
    del idx
    res["q2_gap"] = q2
    return res, Aa, level, Mv, vm


def crater_section(d, res):
    """Positive-control rows: V5 crater, D + N + ctl pooled, paired rho-cubic / rho-neg on (family, curve_slug, workload, target, round)."""
    x = d[(d.volcano == "V5") & (d.vertex == 0) & d.family.isin(["D", "N", "ctl"]) & d.arm.isin(["rho-cubic", "rho-neg"])]
    k = ["family", "curve_slug", "workload_id", "target_index", "round"]
    c = x[x.arm == "rho-cubic"].set_index(k)[["S", "ops_gae", "unc_total"]]
    n = x[x.arm == "rho-neg"].set_index(k)[["S", "ops_gae", "unc_total"]]
    j = c.join(n, how="inner", lsuffix="_c", rsuffix="_n")
    m = j.values.astype(float)  # S_c ops_c unc_c S_n ops_n unc_n
    rng = rng_for(3)
    idx = rng.integers(0, len(m), (REPS, len(m)))
    bm = np.concatenate([m[idx[i0:i0 + 500]].mean(1) for i0 in range(0, REPS, 500)])
    def r(a, b):
        return {"value": float(m[:, a].mean() / m[:, b].mean()), "ci95": pct(bm[:, a] / bm[:, b])}
    out = {"n_pairs": int(len(m)), "n_cubic_rows": int(len(c)), "S_ratio": r(0, 3), "ops_ratio": r(1, 4), "canon_ratio": r(2, 5),
           "mean_resid_cubic": float((m[:, 1] - m[:, 2]).mean()), "mean_resid_neg": float((m[:, 4] - m[:, 5]).mean()),
           "mean_canon_cubic": float(m[:, 2].mean()), "mean_canon_neg": float(m[:, 5].mean())}
    cn, f = m[:, 5].mean(), (m[:, 4] - m[:, 5]).mean()
    out["pred_ops_ratio_if_iterations_exactly_theory"] = float((THEORY * cn + f) / (cn + f))
    bcn = bm[:, 5]; bf = bm[:, 4] - bm[:, 5]
    out["pred_ops_ratio_ci95"] = pct((THEORY * bcn + bf) / (bcn + bf))
    qs = [5, 25, 50, 75, 95, 99]
    out["canon_shape"] = {"cubic": {"cv": float(m[:, 2].std(ddof=1) / m[:, 2].mean()), "median_over_mean": float(np.median(m[:, 2]) / m[:, 2].mean()), "q": {str(q): float(np.percentile(m[:, 2], q)) for q in qs}},
                          "neg": {"cv": float(m[:, 5].std(ddof=1) / m[:, 5].mean()), "median_over_mean": float(np.median(m[:, 5]) / m[:, 5].mean()), "q": {str(q): float(np.percentile(m[:, 5], q)) for q in qs}}}
    out["canon_quantile_ratio"] = [float(np.percentile(m[:, 2], q) / np.percentile(m[:, 5], q)) for q in qs]
    out["canon_over_sqrt_r"] = {"cubic": float(m[:, 2].mean() / 4082.361204), "neg": float(m[:, 5].mean() / 4082.361204)}
    res["crater_positive_control_rows"] = out
    return out


def per_vertex_section(res, rt, Aa, level, Mv, vm):
    """Heterogeneity of T_v (results.json by_vertex CIs, reused), trend with level, break-even prices, extrapolation."""
    bv = rt["H-T"]["by_vertex"]
    vs = sorted(int(v) for v in bv)
    rows = []
    for v in vs:
        b = bv[str(v)]
        r = {"vertex": v, "level": b["level"], "M_v": b["velu_point_maps"]}
        for p in PRICES:
            t = b["T"][str(p)]
            r[f"T{p}"] = t["value"]; r[f"lo{p}"] = t["ci95"][0]; r[f"hi{p}"] = t["ci95"][1]
            t0 = b["T0"][str(p)]
            r[f"T0_{p}"] = t0["value"]; r[f"T0lo{p}"] = t0["ci95"][0]; r[f"T0hi{p}"] = t0["ci95"][1]
        o = vm[v]
        r.update({"nat0": o[6], "cub0": o[0], "neg_t0": o[3], "unc_cub": o[11], "unc_nat": o[13]})
        rows.append(r)
    pv = pd.DataFrame(rows)
    out = {"label": "EXPLORATORY, POST HOC", "n_vertices": len(pv)}
    for p in PRICES:
        out[f"n_ci_includes_1_price{p}"] = int(((pv[f"lo{p}"] <= 1) & (pv[f"hi{p}"] >= 1)).sum())
        out[f"max_hi_price{p}"] = float(pv[f"hi{p}"].max())
        out[f"n_T0_ci_includes_1_price{p}"] = int(((pv[f"T0lo{p}"] <= 1) & (pv[f"T0hi{p}"] >= 1)).sum())
    out["T_range_price1"] = [float(pv["T1.0"].min()), float(pv["T1.0"].max())]
    out["vertices_with_hi_ge_1_price1"] = [int(v) for v in pv[pv["hi1.0"] >= 1].vertex]
    for p in PRICES:
        lt = np.log(pv[f"T{p}"].values)
        se = (np.log(pv[f"hi{p}"].values) - np.log(pv[f"lo{p}"].values)) / (2 * 1.96)
        w = 1 / se ** 2
        mu = (w * lt).sum() / w.sum()
        Q = float((w * (lt - mu) ** 2).sum())
        out[f"cochran_Q_price{p}"] = {"Q": Q, "df": len(lt) - 1, "p": float(stats.chi2.sf(Q, len(lt) - 1)), "I2": float(max(0, (Q - (len(lt) - 1)) / Q)),
                                      "fixed_effect_mean_T": float(np.exp(mu)), "note": "SE from the per-vertex percentile CI width / (2*1.96): approximate"}
    lt = np.log(pv["T1.0"].values); se = (np.log(pv["hi1.0"].values) - np.log(pv["lo1.0"].values)) / (2 * 1.96); w = 1 / se ** 2
    lev = pv.level.values.astype(float)
    def wls(y, x):
        xb, yb = (w * x).sum() / w.sum(), (w * y).sum() / w.sum()
        return float((w * (x - xb) * (y - yb)).sum() / (w * (x - xb) ** 2).sum())
    b_obs = wls(lt, lev)
    rng = rng_for(4)
    nullb = np.array([wls(lt, rng.permutation(lev)) for _ in range(REPS)])
    sp = stats.spearmanr(lev, pv["T1.0"].values)
    nullr = np.array([stats.spearmanr(rng.permutation(lev), pv["T1.0"].values)[0] for _ in range(2000)])
    out["trend_level_price1"] = {"wls_slope_logT_per_level": b_obs, "ratio_per_level": float(np.exp(b_obs)),
                                 "perm_p_two_sided": float((1 + (np.abs(nullb) >= abs(b_obs)).sum()) / (REPS + 1)),
                                 "spearman_rho": float(sp[0]), "spearman_perm_p": float((1 + (np.abs(nullr) >= abs(sp[0])).sum()) / 2001), "spearman_perm_reps": 2000,
                                 "level_means_T": {str(L): float(pv[pv.level == L]["T1.0"].mean()) for L in sorted(pv.level.unique())},
                                 "note": "level = path length to the crater here (M_v = 2 * level), so level and path length are one variable"}
    nat0, cub0, unc_c, unc_n, M = pv.nat0.values, pv.cub0.values, pv.unc_cub.values, pv.unc_nat.values, pv.M_v.values
    pa = (nat0 - cub0) / M
    pb = ((nat0 + unc_n) - (cub0 + unc_c)) / M
    den = M + unc_c - unc_n
    out["breakeven"] = {"definition": "p* = (C(rho-neg,N_v) - C(rho-cubic,Tcr_v)) / M_v. (a) C = charged ops_gae only (uncharged work unpriced); (b) C with every uncharged op priced at 1 GAE; (c) the plan's T_v(p)=1 with the same p on uncharged counters and point maps: finite only if M_v + unc(cubic) - unc(neg) > 0",
                        "a_per_vertex": {"min": float(pa.min()), "median": float(np.median(pa)), "max": float(pa.max())},
                        "b_per_vertex": {"min": float(pb.min()), "median": float(np.median(pb)), "max": float(pb.max())},
                        "c_n_vertices_with_finite_pstar": int((den > 0).sum()), "c_denominator_range": [float(den.min()), float(den.max())],
                        "pooled_a": float((nat0.mean() - cub0.mean()) / M.mean()), "pooled_b": float((((nat0 + unc_n) - (cub0 + unc_c)).mean()) / M.mean()),
                        "by_level_a": {str(L): float((pv[pv.level == L].nat0 - pv[pv.level == L].cub0).mean() / pv[pv.level == L].M_v.mean()) for L in sorted(pv.level.unique())}}
    pv["pstar_a"], pv["pstar_b"] = pa, pb
    ov, bl = res["overall"], res["by_level"]
    out["transport_share_by_level"] = {str(L): {str(p): {"value": bl[str(L)]["share"]["value"][i], "ci95": bl[str(L)]["share"]["ci95"][i]} for i, p in enumerate(PRICES)} for L in sorted(pv.level.unique())}
    out["transport_share_overall"] = {str(p): {"value": ov["share"]["value"][i], "ci95": ov["share"]["ci95"][i]} for i, p in enumerate(PRICES)}
    out["M_range"] = [float(M.min()), float(M.max())]
    out["mean_depth_vertex_weighted"] = float(np.mean(pv.level))
    mean_cub_S = float(np.mean([vm[v][9] for v in sorted(vm)]))
    ex = {"label": "MODEL EXTRAPOLATION, not a measurement",
          "assumptions": ["solve cost = c * sqrt(r) GAE with c = measured vertex-averaged mean S of rho-cubic on the Tcr rows; the ~371-op additive part is taken to scale with sqrt(r) like the rest",
                          "transport = 2 * depth Velu point maps (G and Q), each priced at p GAE, p in {0.1, 1}, independent of bit size (an l = 2 Velu map is a fixed number of field operations, like a group operation)",
                          "depth scenarios: measured vertex-weighted mean depth; 6 (the measured maximum, held fixed); 6 * bits / 26 (depth growing linearly with the bit size)",
                          "not modelled: finding the path to the crater, existence of a j = 0 volcano of that depth at that size, cofactors, the order-6 class machinery at that size"],
          "c_S_cubic": mean_cub_S, "table": []}
    for bits in [26, 64, 128, 256]:
        solve = mean_cub_S * 2.0 ** (bits / 2)
        for scen, D in [("measured mean depth", out["mean_depth_vertex_weighted"]), ("depth 6", 6.0), ("depth 6*bits/26", 6.0 * bits / 26)]:
            for pr in (0.1, 1.0):
                tr = 2 * D * pr
                ex["table"].append({"bits_r": bits, "depth_scenario": scen, "depth": D, "price": pr, "solve_gae_model": solve, "transport_gae": tr, "transport_share": tr / (tr + solve)})
    out["extrapolation"] = ex
    res["per_vertex"] = out
    return pv, out


# =====================================================================================================
# H-M
# =====================================================================================================
def m_matrices(d):
    """{(vid): {arm: Y (8 models x 64 blocks)}} with blocks = sorted (target_index, round); also checks that every model has the same blocks."""
    out = {}
    x = d[d.family == "M"]
    for vid, g in sorted(x.groupby("vid"), key=lambda t: (t[0][:2], int(t[0].split(":")[1]))):
        out[vid] = {"level": int(g.vlevel.iloc[0]), "arms": {}}
        for arm in ARMS_M:
            h = g[g.arm == arm]
            piv = h.pivot_table(index="model", columns=["target_index", "round"], values="S", aggfunc="first")
            assert piv.shape == (8, 64) and not piv.isna().any().any(), (vid, arm, piv.shape)
            out[vid]["arms"][arm] = piv.values.astype(float)
        out[vid]["models"] = None
    return out


def f_from_ssm(ssm, sst, ssb, nm=8, nb=64, paired=False):
    if paired:
        sse = sst - ssb - ssm
        return (ssm / (nm - 1)) / (sse / ((nm - 1) * (nb - 1)))
    sse = sst - ssm
    return (ssm / (nm - 1)) / (sse / (nm * nb - nm))


def perm_curve(Y, B, rng, paired, chunk=2000):
    """Permutation null for one (curve, arm). Y: (8 models, 64 blocks). unpaired: shuffle all 512 rows across models (frozen design);
    paired: shuffle the 8 model labels inside each (target, round) block. Returns observed F, null F (B,), observed max/min of model means, null max/min (B,)."""
    nm, nb = Y.shape
    y = Y.ravel()
    gm = y.mean()
    sst = ((y - gm) ** 2).sum()
    ssb = nm * ((Y.mean(0) - gm) ** 2).sum()
    msum = Y.sum(1)
    ssm_obs = nb * ((msum / nb - gm) ** 2).sum()
    den_obs = (sst - ssb - ssm_obs) if paired else (sst - ssm_obs)
    degenerate = den_obs <= 1e-9 * max(sst, 1.0)
    Fobs = np.nan if degenerate else f_from_ssm(ssm_obs, sst, ssb, nm, nb, paired)
    mobs = Y.mean(1)
    mmobs = float(mobs.max() / mobs.min())
    Fn = np.empty(B); mm = np.empty(B)
    for i0 in range(0, B, chunk):
        c = min(chunk, B - i0)
        if paired:
            Yt = np.broadcast_to(Y.T, (c, nb, nm)).copy()          # (c, blocks, models)
            P = rng.permuted(Yt, axis=2)
            sums = P.sum(1)                                         # (c, models)
        else:
            P = rng.permuted(np.broadcast_to(y, (c, nm * nb)).copy(), axis=1)
            sums = P.reshape(c, nm, nb).sum(2)
        ssm = nb * ((sums / nb - gm) ** 2).sum(1)
        Fn[i0:i0 + c] = np.nan if degenerate else f_from_ssm(ssm, sst, ssb, nm, nb, paired)
        mn = sums / nb
        mm[i0:i0 + c] = mn.max(1) / mn.min(1)
    return Fobs, Fn, mmobs, mm


def holm_adj(p):
    return A.holm(list(p)) if len(p) else []


def hm_section(d, rt, quick=False):
    M = m_matrices(d)
    vids = list(M)
    out = {"label": "EXPLORATORY, POST HOC", "seed": SEED, "curves": vids}
    # ---- frozen reproduction through the committed function (same rows) ----
    t = time.time()
    fr = A.hypothesis_M(d, 10000)
    for vid in vids:
        for arm in ARMS_M:
            a, b = fr["curves"][vid]["arms"][arm], rt["H-M"]["curves"][vid]["arms"][arm]
            assert abs(a["F"] - b["F"]) < 1e-9 and abs(a["p_perm"] - b["p_perm"]) < 1e-12, (vid, arm)
    out["frozen_reproduced_exactly"] = True
    out["frozen_significant_ic"] = [vid for vid in vids if fr["curves"][vid]["arms"]["ic"]["significant"]]
    log("H-M frozen reproduction ok", round(time.time() - t, 1), "s")
    B = 2000 if quick else 20000
    out["B_main"] = B
    store = {}   # (vid, arm, paired) -> (Fobs, Fn, mmobs, mm)
    for ci, vid in enumerate(vids):
        for ai, arm in enumerate(ARMS_M):
            for pi, paired in enumerate((False, True)):
                store[(vid, arm, paired)] = perm_curve(M[vid]["arms"][arm], B, rng_for(10, ci, ai, pi), paired)
    log("H-M main permutations done")
    res = {}
    for arm in ARMS_M:
        for paired in (False, True):
            key = f"{arm}|{'paired' if paired else 'unpaired'}"
            rows = []
            for vid in vids:
                Fo, Fn, mmo, mm = store[(vid, arm, paired)]
                if not np.isfinite(Fo):
                    rows.append({"vid": vid, "F": None, "p": None}); continue
                rows.append({"vid": vid, "F": float(Fo), "p": float((1 + (Fn >= Fo).sum()) / (1 + B)), "p10k": float((1 + (Fn[:10000] >= Fo).sum()) / (1 + min(B, 10000))),
                             "p_param": float(stats.f.sf(Fo, 7, 504 if not paired else 441))})
            idx = [i for i, r in enumerate(rows) if r["p"] is not None]
            for name in ("p", "p10k", "p_param"):
                if idx:
                    adj = holm_adj([rows[i][name] for i in idx])
                    for i, a in zip(idx, adj):
                        rows[i]["holm_" + name] = float(a)
            nsig = {name: int(sum(1 for i in idx if rows[i]["holm_" + name] < 0.05)) for name in ("p", "p10k", "p_param")} if idx else {}
            res[key] = {"rows": rows, "n_sig_holm": nsig}
    out["per_curve"] = res
    # ---- pooled across curves (sum of F, Fisher) with independent label permutations per curve ----
    def pooled_stats(vl, arm, paired):
        ok = [v for v in vl if np.isfinite(store[(v, arm, paired)][0])]
        if not ok:
            return {"note": "zero within-model variance (bsgs): F undefined / identically 0"}
        Fo = np.array([store[(v, arm, paired)][0] for v in ok])
        Fn = np.stack([store[(v, arm, paired)][1] for v in ok])        # (k, B)
        sumF_obs, sumF_null = Fo.sum(), Fn.sum(0)
        def pvals_of(x, null):
            s_ = np.sort(null)
            return (1 + (len(null) - np.searchsorted(s_, x, side="left"))) / (1 + len(null))
        fish_obs = -2 * sum(math.log(pvals_of(Fo[i], Fn[i])) for i in range(len(ok)))
        fish_null = np.zeros(Fn.shape[1])
        for i in range(len(ok)):
            fish_null += -2 * np.log(pvals_of(Fn[i], Fn[i]))
        return {"n_curves": len(ok), "sum_F": float(sumF_obs), "sum_F_null_mean": float(sumF_null.mean()), "sum_F_null_p95": float(np.percentile(sumF_null, 95)),
                "p_sum_F": float((1 + (sumF_null >= sumF_obs).sum()) / (1 + B)), "fisher_stat": float(fish_obs), "p_fisher": float((1 + (fish_null >= fish_obs).sum()) / (1 + B)),
                "mean_F": float(Fo.mean()), "B": B}
    pooled = {}
    for arm in ARMS_M:
        for paired in (False, True):
            pooled[f"{arm}|{'paired' if paired else 'unpaired'}"] = pooled_stats(vids, arm, paired)
    rest = [v for v in vids if v not in out["frozen_significant_ic"]]
    for paired in (False, True):
        pooled[f"ic|{'paired' if paired else 'unpaired'}|excluding_the_5_frozen_significant_curves"] = dict(pooled_stats(rest, "ic", paired), curves=rest)
    out["pooled"] = pooled
    # ---- calibrated effect size: observed max/min of model means vs its permutation null ----
    cal = {}
    for arm in ARMS_M:
        for paired in (False, True):
            key = f"{arm}|{'paired' if paired else 'unpaired'}"
            rows = []
            for vid in vids:
                Fo, Fn, mmo, mm = store[(vid, arm, paired)]
                boot = rt["H-M"]["curves"][vid]["arms"][arm]["max_over_min_model_mean"]
                rows.append({"vid": vid, "observed": mmo, "null_median": float(np.median(mm)), "null_p95": float(np.percentile(mm, 95)), "null_p99": float(np.percentile(mm, 99)),
                             "p_maxmin": float((1 + (mm >= mmo - 1e-12).sum()) / (1 + B)), "excess_over_null_median": float(mmo / np.median(mm)),
                             "excess_over_null_p95": float(mmo / np.percentile(mm, 95)), "reported_bootstrap_ci95": boot["ci95"], "reported_point": boot["value"],
                             "reported_ci_excludes_point": bool(not (boot["ci95"][0] <= boot["value"] <= boot["ci95"][1]))})
            cal[key] = rows
    out["maxmin_calibration"] = cal
    # ---- V6:98 boundary: p vs shuffle count, repeated 10k runs, Holm stability ----
    out["boundary"] = boundary_section(M, vids, quick)
    # ---- between-model variance components (method of moments, per curve and arm) ----
    vc = {}
    for arm in ARMS_M:
        r = []
        for vid in vids:
            Y = M[vid]["arms"][arm]
            nm, nb = Y.shape
            gm = Y.mean(); mm_ = Y.mean(1)
            msb = nb * ((mm_ - gm) ** 2).sum() / (nm - 1); msw = ((Y - mm_[:, None]) ** 2).sum() / (nm * (nb - 1))
            tau2 = max(0.0, (msb - msw) / nb)
            # block (target, round) variance share with model removed
            bl = Y.mean(0)
            msbl = nm * ((bl - gm) ** 2).sum() / (nb - 1)
            sse = ((Y - mm_[:, None] - bl[None, :] + gm) ** 2).sum() / ((nm - 1) * (nb - 1))
            r.append({"vid": vid, "grand_mean": float(gm), "msb": float(msb), "msw": float(msw), "tau": float(math.sqrt(tau2)), "cv_between_models": float(math.sqrt(tau2) / gm),
                      "icc_model": float(tau2 / (tau2 + msw)), "cv_within": float(math.sqrt(msw) / gm), "block_icc": float(max(0.0, (msbl - sse) / nm) / (max(0.0, (msbl - sse) / nm) + sse)),
                      "mse_paired_over_msw": float(sse / msw)})
        vc[arm] = r
    out["variance_components"] = vc
    out["_M"] = M
    return out, store


def boundary_section(M, vids, quick):
    out = {"label": "EXPLORATORY, POST HOC", "curve": "V6:98", "arm": "ic"}
    Y = M["V6:98"]["arms"]["ic"]
    y = Y.ravel()
    F_obs, _, _, _ = perm_curve(Y, 10, rng_for(20), False)
    out["F"] = float(F_obs)
    out["p_parametric_F_7_504"] = float(stats.f.sf(F_obs, 7, 504))
    # one long stream, running p at checkpoints
    NL = 100000 if quick else 1000000
    _, Fn, _, _ = perm_curve(Y, NL, rng_for(21), False, chunk=5000)
    cps = [10 ** k for k in range(3, int(math.log10(NL)) + 1)]
    out["running_p_one_stream"] = {str(c): float((1 + (Fn[:c] >= F_obs).sum()) / (1 + c)) for c in cps}
    out["p_at_max"] = out["running_p_one_stream"][str(cps[-1])]
    out["max_shuffles"] = NL
    pm = out["p_at_max"]
    out["mc_se_at_max"] = float(math.sqrt(pm * (1 - pm) / NL))
    # the 12 ic curves at high precision (unpaired and paired)
    BH = 20000 if quick else 200000
    hp = {}
    for paired in (False, True):
        ps = []
        for ci, vid in enumerate(vids):
            Fo, Fn_, _, _ = perm_curve(M[vid]["arms"]["ic"], BH, rng_for(22, ci, int(paired)), paired, chunk=5000)
            ps.append(float((1 + (Fn_ >= Fo).sum()) / (1 + BH)))
        adj = holm_adj(ps)
        hp["paired" if paired else "unpaired"] = {"p": dict(zip(vids, ps)), "holm": dict(zip(vids, [float(a) for a in adj])), "n_sig": int(sum(a < 0.05 for a in adj)), "B": BH}
    out["high_precision_ic"] = hp
    # repeated independent 10,000-shuffle runs of the whole ic row of the table (12 curves): how often is it 5 of 12 vs 6 of 12?
    NR = 20 if quick else 100
    rep = {}
    for paired in (False, True):
        ns, hv, pv = [], [], []
        for r in range(NR):
            ps = []
            for ci, vid in enumerate(vids):
                Fo, Fn_, _, _ = perm_curve(M[vid]["arms"]["ic"], 10000, rng_for(23, int(paired), r, ci), paired, chunk=5000)
                ps.append((1 + (Fn_ >= Fo).sum()) / 10001)
            adj = holm_adj(ps)
            ns.append(int(sum(a < 0.05 for a in adj))); hv.append(float(adj[vids.index("V6:98")])); pv.append(float(ps[vids.index("V6:98")]))
        rep["paired" if paired else "unpaired"] = {"n_runs": NR, "n_sig_counts": {str(k): int(ns.count(k)) for k in sorted(set(ns))}, "p_V698_quantiles": [float(np.percentile(pv, q)) for q in (2.5, 50, 97.5)],
                                                  "holm_V698_quantiles": [float(np.percentile(hv, q)) for q in (2.5, 50, 97.5)], "holm_V698_values": hv, "frac_holm_V698_below_0.05": float(np.mean(np.array(hv) < 0.05))}
    out["repeat_runs_10k_each"] = rep
    out["_Fn_stream_for_plot"] = None
    return out


def parse_unc(s):
    return {k: float(v) for k, v in (kv.split("=") for kv in s.split(";"))} if isinstance(s, str) and s else {}


def nth_prime(n):
    ps, c = [], 2
    while len(ps) < n:
        if all(c % q for q in ps if q * q <= c):
            ps.append(c)
        c += 1
    return ps[-1]



# ---- exploratory pure-Python EC check (not an ecbench measurement): rebuild the 64-smallest-abscissa factor base of each M model ----
def _ec_add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)


def _ec_mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = _ec_add(R, P, a, p)
        P = _ec_add(P, P, a, p); k >>= 1
    return R


def _sqrt_mod(a, p):
    a %= p
    if p % 4 == 3:
        return pow(a, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1: tt = tt * tt % p; i += 1
        b = pow(c, 1 << (m - i - 1), p); m = i; c = b * b % p; t = t * c % p; r = r * b % p
    return r


def ic_yield_table():
    """Per (curve, model): rebuild the factor base as src/cryptanalysis/ic_boundary.rs prime_factor_base does (the `size` smallest abscissae x = 1, 2, ... with
    rhs a square, both signs), check abscissae_scanned against the recorded counter of every ic row, and count the relation sources that land in <G>:
    k0 = FB columns inside <G> ([r]P = O), npair = unordered column pairs with [r]P_i = +-[r]P_j (T != O), H = 2 k0 + 4 C(k0,2) + 2 npair = number of distinct
    elements +-P_i, +-P_i +- P_j that lie in <G>. Streams records.jsonl of the 12 M session archives through tar (nothing is written to disk)."""
    import collections, glob, subprocess
    mm, cur = {}, {}
    for V in ("V5", "V6"):
        for c in json.load(open(f"{ROOT}/census/{V}/M.json"))["curves"]:
            cur[(V, c["id"])] = c
            for m in c["models"]:
                mm[(V, c["id"], m["a"], m["b"])] = m["m"]
    fbi = {}
    for (V, i), c in cur.items():
        p, r = c["p"], c["r"]
        for m in c["models"]:
            a, b = m["a"], m["b"]
            pts, x, scanned = [], 1, 0
            while len(pts) < 64 and x < p:
                scanned += 1
                f = (x ** 3 + a * x + b) % p
                if f == 0: pts.append((x, 0))
                elif pow(f, (p - 1) // 2, p) == 1: pts.append((x, _sqrt_mod(f, p)))
                x += 1
            assert _ec_mul(r, (m["gx"], m["gy"]), a, p) is None
            cnt = collections.Counter("O" if (T := _ec_mul(r, P, a, p)) is None else T[0] for P in pts)
            k0 = cnt.get("O", 0)
            npair = sum(v * (v - 1) // 2 for k, v in cnt.items() if k != "O")
            fbi[(V, i, m["m"])] = {"scanned_rebuilt": scanned, "k0": k0, "npair": npair, "H": 2 * k0 + 4 * (k0 * (k0 - 1) // 2) + 2 * npair, "cofactor": c["order"] // r}
    rows = []
    for f in sorted(glob.glob(f"{ROOT}/sessions/*-M-L*-c*.tar.xz")):
        name = os.path.basename(f)[:-len(".tar.xz")]
        V, i = name.split("-")[0], int(name.split("-c")[-1])
        pr = subprocess.Popen(["tar", "-xJOf", f, f"{name}/records.jsonl"], stdout=subprocess.PIPE, text=True)
        for l in pr.stdout:
            r = json.loads(l)
            if r["arm"] != "ic" or r.get("warmup"):
                continue
            ph = {x["name"]: x for x in r["phases"]}
            cs = r["workload"]["curve_spec"]
            rows.append({"vid": f"{V}:{i}", "model": mm[(V, i, cs["a"], cs["b"])], "S": r["cost"]["s"], "trials": ph["relations"]["native"]["trials"], "rel": r["counters"]["relations_found"],
                         "scanned": ph["factor_base"]["native"]["abscissae_scanned"], "walk_steps": ph["relations"]["native"].get("walk_steps", 0)})
        pr.wait()
    d = pd.DataFrame(rows)
    g = d.groupby(["vid", "model"]).agg(S_rec=("S", "mean"), trials=("trials", "mean"), rel=("rel", "mean"), scanned=("scanned", "mean"), n=("S", "size")).reset_index()
    for k in ("scanned_rebuilt", "k0", "npair", "H", "cofactor"):
        g[k] = [fbi[(v.split(":")[0], int(v.split(":")[1]), m)][k] for v, m in zip(g.vid, g.model)]
    g["tpr"] = g.trials / g.rel
    assert (g.scanned == g.scanned_rebuilt).all() and len(g) == 96 and (g.n == 64).all()
    return g


def mechanism_section(d, M, VERT):
    """Q9: does per-model IC cost track anything measurable per model? Features from the committed rows (uncharged counters) and census/M.json."""
    x = d[(d.family == "M") & (d.arm == "ic")].copy()
    U = pd.DataFrame([parse_unc(s) for s in x.unc_detail.values], index=x.index)
    x = pd.concat([x, U], axis=1)
    feats = ["legendre_symbols_uncharged", "sqrt_solves_uncharged", "lookups_uncharged", "row_ops_uncharged", "target_guard_probes_uncharged"]
    g = x.groupby(["vid", "model"])
    t = g[["S", "ops_gae", "unc_total"] + feats].mean()
    t["leg_nunique"] = g["legendre_symbols_uncharged"].nunique()
    t = t.reset_index()
    coef = []
    for V in ("V5", "V6"):
        for c in json.load(open(f"{ROOT}/census/{V}/M.json"))["curves"]:
            p = c["p"]
            for m in c["models"]:
                coef.append({"vid": f"{V}:{c['id']}", "model": m["m"], "u_over_p": m["u"] / p, "a_over_p": m["a"] / p, "b_over_p": m["b"] / p, "gx_over_p": m["gx"] / p, "gy_over_p": m["gy"] / p})
    t = t.merge(pd.DataFrame(coef), on=["vid", "model"], validate="1:1")
    xr = d[(d.family == "M") & (d.arm == "rho-neg")].groupby(["vid", "model"]).S.mean().rename("S_rho_neg").reset_index()
    t = t.merge(xr, on=["vid", "model"], validate="1:1")
    t["bound_prime"] = [nth_prime(int(v)) for v in t.legendre_symbols_uncharged]
    yt = ic_yield_table()
    t = t.merge(yt[["vid", "model", "trials", "rel", "tpr", "k0", "npair", "H", "cofactor", "scanned_rebuilt"]], on=["vid", "model"], validate="1:1")
    assert np.allclose(t.legendre_symbols_uncharged, t.scanned_rebuilt)
    out = {"label": "EXPLORATORY, POST HOC", "n_models": int(len(t)), "n_curves": int(t.vid.nunique()),
           "legendre_constant_within_model": bool((t.leg_nunique == 1).all()), "sqrt_solves_values": sorted(t.sqrt_solves_uncharged.unique().tolist()),
           "legendre_range": [int(t.legendre_symbols_uncharged.min()), int(t.legendre_symbols_uncharged.max())],
           "legendre_within_curve_range_median": float(t.groupby("vid").legendre_symbols_uncharged.apply(lambda s: s.max() - s.min()).median()),
           "interpretation": "legendre_symbols_uncharged (constant within a model) counts the Legendre symbols evaluated while sampling the 64-point prime-abscissa factor base on that model (docs/ecbench/README.md: Legendre symbols and square roots of point sampling); sqrt_solves is 64 everywhere. It is a measure of how many candidate abscissas were needed to find 64 points, not of relation yield."}
    # within-curve demeaned correlations with permutation p (labels of the feature permuted within curve), seed SEED
    def cen(v):
        return v - t.groupby("vid")[v.name].transform("mean")
    ycols = {"ic_log_S": np.log(t.S), "rho_neg_log_S": np.log(t.S_rho_neg)}
    res = {}
    for yn, yv in ycols.items():
        yc = yv - yv.groupby(t.vid).transform("mean")
        res[yn] = {}
        for fn in feats + ["unc_total", "ops_gae", "u_over_p", "a_over_p", "b_over_p", "gx_over_p", "gy_over_p", "model"]:
            fv = t[fn].astype(float)
            fc = fv - fv.groupby(t.vid).transform("mean")
            if fc.std() == 0:
                res[yn][fn] = None; continue
            r_obs = float(np.corrcoef(yc, fc)[0, 1])
            rng = rng_for(30, hash((yn, fn)) % 10**6)
            groups = [np.where(t.vid.values == v)[0] for v in t.vid.unique()]
            null = np.empty(REPS)
            fcv, ycv = fc.values, yc.values
            for i in range(REPS):
                pf = fcv.copy()
                for gi in groups:
                    pf[gi] = fcv[rng.permutation(gi)]
                null[i] = np.corrcoef(ycv, pf)[0, 1]
            res[yn][fn] = {"r_within_curve": r_obs, "perm_p": float((1 + (np.abs(null) >= abs(r_obs)).sum()) / (REPS + 1))}
    out["correlations_within_curve"] = res
    # relation-yield mechanism: log(trials per relation) and log S against log H (relation sources inside <G>), within-curve, permutation p (labels permuted within curve)
    def cen_(x):
        x = pd.Series(np.asarray(x, float), index=t.index)
        return x - x.groupby(t.vid).transform("mean")
    lH, ltpr, lS, lSn = np.log(t.H), np.log(t.tpr), np.log(t.S), np.log(t.S_rho_neg)
    groups = [np.where(t.vid.values == v)[0] for v in t.vid.unique()]
    rngm = rng_for(31)
    def permp(y, x):
        yc, xc = cen_(y).values, cen_(x).values
        r0 = float(np.corrcoef(yc, xc)[0, 1])
        null = np.empty(REPS)
        for i in range(REPS):
            pf = xc.copy()
            for gi in groups:
                pf[gi] = xc[rngm.permutation(gi)]
            null[i] = np.corrcoef(yc, pf)[0, 1]
        return {"r_within_curve": r0, "perm_p": float((1 + (np.abs(null) >= abs(r0)).sum()) / (REPS + 1)), "n_models": int(len(t))}
    out["yield_mechanism"] = {
        "label": "EXPLORATORY, POST HOC; H and the factor base are rebuilt in Python from census coefficients and checked against the recorded abscissae_scanned of all 96 models; the ic records come from streaming the 12 M session archives",
        "factor_base_rule": "the 64 smallest abscissae x = 1, 2, ... on the curve, both signs (src/cryptanalysis/ic_boundary.rs prime_factor_base); 'prime-abscissa' means prime field, not prime x",
        "H_definition": "H = 2 k0 + 4 C(k0, 2) + 2 npair: distinct elements +-P_i (P_i in <G>), +-P_i +- P_j (both in <G>, or [r]P_i = +-[r]P_j) that lie in <G>, i.e. relation sources with at most two factor-base columns",
        "scanned_matches_recorded_all_models": True, "cofactors": sorted(set(int(c) for c in t.cofactor)),
        "corr_log_trials_per_relation_vs_log_H": permp(ltpr, lH), "corr_log_S_ic_vs_log_H": permp(lS, lH), "corr_log_S_ic_vs_log_trials_per_relation": permp(lS, ltpr),
        "corr_log_S_rho_neg_vs_log_H_negative_control": permp(lSn, lH),
        "corr_log_S_ic_vs_k0": permp(lS, t.k0 + 0.5), "corr_log_S_ic_vs_npair": permp(lS, t.npair + 0.5),
        "relations_per_solve_mean": float(t.rel.mean()), "trials_per_relation_range": [float(t.tpr.min()), float(t.tpr.max())],
        "H_range_by_curve": {v: [int(t[t.vid == v].H.min()), int(t[t.vid == v].H.max())] for v in t.vid.unique()}}
    yc, xc = cen_(lS).values, cen_(lH).values
    out["yield_mechanism"]["within_curve_regression_logS_on_logH"] = {"slope": float((yc * xc).sum() / (xc ** 2).sum()), "R2_within_curve": float(np.corrcoef(yc, xc)[0, 1] ** 2),
        "note": "R2 is on model means of 64 rows each, so row noise caps it below 1"}
    pcs = {v: {"cofactor": int(g.cofactor.iloc[0]), "n_models": int(len(g)), "spearman_H_vs_trials_per_relation": float(stats.spearmanr(g.H, g.tpr)[0]),
               "spearman_H_vs_S_ic": float(stats.spearmanr(g.H, g.S)[0])} for v, g in t.groupby("vid")}
    out["yield_mechanism"]["per_curve"] = pcs
    out["yield_mechanism"]["n_curves_negative_spearman_tpr"] = int(sum(1 for x in pcs.values() if x["spearman_H_vs_trials_per_relation"] < 0))
    out["yield_mechanism"]["n_curves_negative_spearman_S"] = int(sum(1 for x in pcs.values() if x["spearman_H_vs_S_ic"] < 0))
    byv = {}
    ct, cs_, ch = cen_(ltpr).values, cen_(lS).values, cen_(lH).values
    for V in ("V5", "V6"):
        mk = t.vid.str.startswith(V).values
        byv[V] = {"n_models": int(mk.sum()), "n_curves": int(t[mk].vid.nunique()), "r_log_tpr_vs_log_H": float(np.corrcoef(ct[mk], ch[mk])[0, 1]), "r_log_S_vs_log_H": float(np.corrcoef(cs_[mk], ch[mk])[0, 1])}
    out["yield_mechanism"]["by_volcano"] = byv
    out["yield_models_table"] = t[["vid", "model", "S", "S_rho_neg", "tpr", "rel", "k0", "npair", "H", "cofactor"]].round(4).to_dict("records")
    # share of between-model variation of log S(ic) explained by the legendre count (within-curve regression R^2) and slope
    yc = np.log(t.S) - np.log(t.S).groupby(t.vid).transform("mean")
    lc = t.legendre_symbols_uncharged - t.groupby("vid").legendre_symbols_uncharged.transform("mean")
    if lc.std() > 0:
        slope = float((yc * lc).sum() / (lc ** 2).sum())
        out["legendre_regression"] = {"slope_logS_per_legendre_symbol": slope, "R2_within_curve": float(np.corrcoef(yc, lc)[0, 1] ** 2)}
    # detail table for the two most significant curves
    out["detail"] = {v: t[t.vid == v][["model", "S", "S_rho_neg", "legendre_symbols_uncharged", "bound_prime", "lookups_uncharged", "row_ops_uncharged"]].round(3).to_dict("records") for v in ("V6:214", "V6:99")}
    # per-row: is S(ic) driven by lookups (an uncharged-work proxy for relation tries)?
    out["row_corr_S_lookups"] = float(np.corrcoef(x.S, x.lookups_uncharged)[0, 1])
    out["row_corr_S_rowops"] = float(np.corrcoef(x.S, x.row_ops_uncharged)[0, 1])
    return out, t


def v6_model_effect(d):
    """Q10: Q1's data as a model-effect test. Each V6 vertex has D + 2 re-scalings (ctl): 3 isomorphic models, same instances.
    Pooled one-way statistic over vertices, permutation across the 3 models inside a vertex; blocked version shuffles inside (target, round)."""
    x = d[(d.volcano == "V6") & d.family.isin(["D", "ctl"])]
    out = {"label": "EXPLORATORY, POST HOC", "arms": {}}
    for ai, arm in enumerate(ARMS_M):
        h = x[x.arm == arm]
        mats = {}
        for v, g in h.groupby("vertex"):
            slugs = sorted(g.curve_slug.unique())
            if len(slugs) != 3:
                continue
            pivs = [g[g.curve_slug == s].set_index(["target_index", "round"]).S.sort_index() for s in slugs]
            if not (pivs[0].index.equals(pivs[1].index) and pivs[0].index.equals(pivs[2].index)):
                continue
            Y = np.stack([p.values for p in pivs])
            mats.setdefault(Y.shape[1], []).append((int(v), Y))
        nV = sum(len(v) for v in mats.values())
        rngp = rng_for(40, ai)
        groups = {}
        for n, lst in mats.items():
            Ys = np.stack([Y for _, Y in lst])                         # (V, 3, n)
            groups[n] = Ys
        # observed
        SSM = SST = SSB = 0.0
        dfw_u = dfw_p = 0
        for n, Ys in groups.items():
            gm = Ys.mean((1, 2), keepdims=True)
            SST += ((Ys - gm) ** 2).sum()
            SSM += (n * (Ys.mean(2, keepdims=True) - gm) ** 2).sum()
            SSB += (3 * (Ys.mean(1, keepdims=True) - gm) ** 2).sum()
            dfw_u += Ys.shape[0] * 3 * (n - 1); dfw_p += Ys.shape[0] * 2 * (n - 1)
        r = {"n_vertices": nV, "rows_per_model_by_n": {str(n): int(len(v)) for n, v in mats.items()}}
        if SST - SSM <= 1e-9 * max(SST, 1):
            r["note"] = "identical across models in every block (bsgs): statistic 0 by construction"; out["arms"][arm] = r; continue
        r["F_pooled_unpaired"] = float((SSM / (2 * nV)) / ((SST - SSM) / dfw_u))
        r["F_pooled_paired"] = float((SSM / (2 * nV)) / ((SST - SSB - SSM) / dfw_p)) if SST - SSB - SSM > 0 else None
        r["F_pooled_note"] = "mean square between the 3 models inside a vertex / mean square within model (pooled over vertices)"
        B = 2000
        null_u = np.empty(B); null_p = np.empty(B)
        for b in range(B):
            su = sp = 0.0
            for n, Ys in groups.items():
                V_ = Ys.shape[0]
                flat = Ys.reshape(V_, 3 * n)
                pu = rngp.permuted(flat, axis=1).reshape(V_, 3, n)
                gm = Ys.mean((1, 2), keepdims=True)
                su += (n * (pu.mean(2, keepdims=True) - gm) ** 2).sum()
                bl = np.transpose(Ys, (0, 2, 1))                          # (V, n, 3)
                pp = rngp.permuted(bl, axis=2)
                sp += (n * (pp.mean(1, keepdims=True) - gm.transpose(0, 2, 1)) ** 2).sum()
            null_u[b], null_p[b] = su, sp
        r["perm_p_unpaired"] = float((1 + (null_u >= SSM - 1e-9).sum()) / (B + 1))
        r["perm_p_paired"] = float((1 + (null_p >= SSM - 1e-9).sum()) / (B + 1))
        r["perm_reps"] = B
        r["ssm_obs_over_null_mean"] = float(SSM / null_u.mean())
        # between-model CV (method of moments), vertex bootstrap
        tau2, gms = [], []
        for n, Ys in groups.items():
            gm = Ys.mean((1, 2))
            msb = n * ((Ys.mean(2) - gm[:, None]) ** 2).sum(1) / 2
            msw = ((Ys - Ys.mean(2, keepdims=True)) ** 2).sum((1, 2)) / (3 * (n - 1))
            tau2.append((msb - msw) / n); gms.append(gm)
        tau2, gms = np.concatenate(tau2), np.concatenate(gms)
        r["cv_between_models"] = float(math.sqrt(max(0.0, tau2.mean())) / gms.mean())
        rb = rng_for(41, ai)
        ii = rb.integers(0, len(tau2), (REPS, len(tau2)))
        bb = np.sqrt(np.maximum(0.0, tau2[ii].mean(1))) / gms[ii].mean(1)
        r["cv_between_models_ci95"] = pct(bb)
        # Q1's own numbers for the same quantity: var of the three model means inside a vertex vs sampling-only expectation
        exp_samp = float(np.mean(np.concatenate([((Ys - Ys.mean(2, keepdims=True)) ** 2).sum((1, 2)) / (3 * (n - 1)) / n for n, Ys in groups.items()])))
        obs_var3 = float(np.mean(np.concatenate([Ys.mean(2).var(1, ddof=1) for n, Ys in groups.items()])))
        r["within_vertex_var_of_model_means_obs"] = obs_var3
        r["within_vertex_var_expected_from_row_noise_only"] = exp_samp
        out["arms"][arm] = r
    return out


def power_section(M, quick=False):
    """Q11: rows per model needed for >= 6/12 ic curves significant (Holm, alpha 0.05) at 80% power; parametric F test inside the simulation."""
    reps = 200 if quick else 1000
    ns = [64, 80, 96, 112, 128, 192, 256, 384, 512]
    vids = list(M)
    P = {}
    for vid in vids:
        Y = M[vid]["arms"]["ic"]
        gm = Y.mean(); mm = Y.mean(1)
        nm, nb = Y.shape
        msb = nb * ((mm - gm) ** 2).sum() / (nm - 1); msw = ((Y - mm[:, None]) ** 2).sum() / (nm * (nb - 1))
        F = msb / msw
        shrink = math.sqrt(max(0.0, 1 - 1 / F)) if F > 1 else 0.0
        P[vid] = (Y, gm, mm, shrink)
    out = {"label": "EXPLORATORY, POST HOC; design recommendation, not a result", "reps": reps, "scenarios": {}, "n_values": ns,
           "method": "per curve: model true means = grand + shrink * (observed model mean - grand) with shrink = sqrt(max(0, 1 - 1/F_obs)) (moment shrinkage; 'observed' scenario: shrink = 1; 'null': shrink = 0); each model's observed 64 rows are rescaled to the true model mean and resampled with replacement n times; parametric F(7, 8n-8) p-values; Holm over the 12 curves; counted: ic significant on >= 6 of 12"}
    for scen in ("shrunk", "observed", "null"):
        rows = []
        for n in ns:
            rng = rng_for(50, ns.index(n), ["shrunk", "observed", "null"].index(scen))
            ps = np.empty((reps, len(vids)))
            for ci, vid in enumerate(vids):
                Y, gm, mm, shrink = P[vid]
                s = 0.0 if scen == "null" else (1.0 if scen == "observed" else shrink)
                tm = gm + s * (mm - gm)
                scaled = Y * (tm / mm)[:, None]                                     # (8, 64)
                idx = rng.integers(0, 64, (reps, 8, n))
                v = scaled[np.arange(8)[None, :, None], idx]                         # (reps, 8, n)
                mean_m = v.mean(2)
                g_ = mean_m.mean(1, keepdims=True)
                msb = n * ((mean_m - g_) ** 2).sum(1) / 7
                msw = ((v - mean_m[:, :, None]) ** 2).sum((1, 2)) / (8 * (n - 1))
                F = msb / msw
                ps[:, ci] = stats.f.sf(F, 7, 8 * n - 8)
            order = np.argsort(ps, axis=1)
            sp = np.take_along_axis(ps, order, 1)
            mult = np.arange(len(vids), 0, -1)[None, :]
            adj = np.maximum.accumulate(np.minimum(1.0, sp * mult), axis=1)
            nsig = (adj < 0.05).sum(1)
            rows.append({"n_rows_per_model": n, "P_at_least_6_of_12": float((nsig >= 6).mean()), "mean_n_sig": float(nsig.mean()), "P_at_least_5": float((nsig >= 5).mean())})
        out["scenarios"][scen] = rows
    for scen, rows in out["scenarios"].items():
        need = next((r["n_rows_per_model"] for r in rows if r["P_at_least_6_of_12"] >= 0.8), None)
        out.setdefault("n_for_80pct_power", {})[scen] = need
    return out


# =====================================================================================================
# figures
# =====================================================================================================
def figures(R, pv, M, tmech, outdir):
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    os.makedirs(outdir, exist_ok=True)
    plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False, "figure.dpi": 100, "svg.fonttype": "none"})
    C = {"cub": "#0072B2", "t0": "#999999", "T": "#D55E00", "ic": "#CC79A7", "neg": "#009E73", "bsgs": "#E69F00", "th": "#444444"}
    TAG = "EXPLORATORY, POST HOC: no verdict, no speedup claim"
    def save(fig, name):
        fig.text(0.01, -0.035, TAG + "  |  seed 20261106  |  study volcano_ecdlp_census_followup_20261023", fontsize=6.5, color="#555555")
        fig.savefig(f"{outdir}/{name}.svg", bbox_inches="tight")
        fig.savefig(f"{outdir}/{name}.png", dpi=150, bbox_inches="tight")
        plt.close(fig)
    ht = R["H-T_decomposition"]
    # ---- fig1 decomposition by level ----
    fig, ax = plt.subplots(1, 2, figsize=(9.5, 4), sharey=True)
    for k, p in enumerate((0, 2)):
        a = ax[k]
        cats = [str(L) for L in range(1, 7)] + ["all"]
        src = [ht["by_level"][str(L)] for L in range(1, 7)] + [ht["overall"]]
        nv = [ht["by_level"][str(L)]["n_vertices"] for L in range(1, 7)] + [63]
        for j, (key, col, lab, off) in enumerate([("F", C["cub"], "crater factor F = rho-cubic / rho-neg on Tcr (transport in both)", -0.2), ("T0", C["t0"], "control T0 = rho-neg on Tcr / rho-neg at source", 0.0), ("T", C["T"], "T = F x T0 (verdict quantity)", 0.2)]):
            v = np.array([s[key]["value"][p] for s in src]); lo = np.array([s[key]["ci95"][p][0] for s in src]); hi = np.array([s[key]["ci95"][p][1] for s in src])
            a.errorbar(np.arange(7) + off, v, yerr=[v - lo, hi - v], fmt="o", color=col, ms=4, capsize=2, label=lab)
        a.axhline(1, color="k", lw=0.6); a.axhline(THEORY, color=C["th"], ls="--", lw=0.8)
        a.text(6.45, THEORY - 0.03, "1/sqrt3", ha="right", fontsize=7, color=C["th"])
        a.set_xticks(range(7)); a.set_xticklabels([f"L{c}\nn={n}" if c != "all" else f"all\nn={n}" for c, n in zip(cats, nv)])
        a.set_title(f"transport priced at {PRICES[p]:g} GAE per point map", fontsize=9)
        a.set_ylim(0.45, 1.2)
    ax[0].set_ylabel("ratio of vertex-averaged mean priced ops_gae")
    ax[0].legend(fontsize=6.5, loc="upper left", frameon=False)
    fig.suptitle("H-T decomposed: T = F x T0; the automorphism factor carries the gain, the control is ~0.97 (CI includes 1)\nn = 63 source vertices (levels 1-6), cluster bootstrap 95% CI, 10,000 replicates", fontsize=9.5, y=1.04)
    save(fig, "fig1_T_decomposition_by_level")
    # ---- fig2 forest ----
    pvs = pv.sort_values(["level", "vertex"]).reset_index(drop=True)
    fig, ax = plt.subplots(figsize=(7, 11))
    cols = plt.cm.viridis(np.linspace(0, 0.9, 6))
    for i, r in pvs.iterrows():
        ax.errorbar(r["T1.0"], i, xerr=[[r["T1.0"] - r["lo1.0"]], [r["hi1.0"] - r["T1.0"]]], fmt="o", ms=3, color=cols[int(r.level) - 1], elinewidth=1, capsize=0)
        ax.plot(r["T0_1.0"], i, marker="D", ms=2.5, color="#999999", ls="none")
    ax.axvline(1, color="k", lw=0.7); ax.axvline(THEORY, color=C["th"], ls="--", lw=0.8); ax.axvline(ht["overall"]["T"]["value"][2], color=C["T"], lw=0.8)
    ax.set_yticks(range(len(pvs))); ax.set_yticklabels([f"L{int(r.level)} v{int(r.vertex)}" for _, r in pvs.iterrows()], fontsize=5.5); ax.invert_yaxis()
    ax.set_xlabel("T_v at 1 GAE per point map (dots, colour = level, 95% pair-bootstrap CI); grey diamonds = control T0_v")
    ax.set_title(f"Per-vertex T_v, n = 63 vertices (8 targets x rounds per vertex)\nCI includes 1 at {R['H-T_vertices']['n_ci_includes_1_price1.0']} of 63; pooled T = {ht['overall']['T']['value'][2]:.3f} (orange), 1/sqrt3 dashed", fontsize=9)
    save(fig, "fig2_T_v_forest")
    # ---- fig3 break-even and extrapolation ----
    hv = R["H-T_vertices"]
    fig, ax = plt.subplots(1, 3, figsize=(13, 4.2))
    a = ax[0]
    for L in range(1, 7):
        s = pvs[pvs.level == L]
        a.scatter(np.full(len(s), L) + np.random.default_rng(SEED).uniform(-0.15, 0.15, len(s)), s.pstar_a, s=10, color=cols[L - 1])
    a.set_yscale("log"); a.set_xlabel("source level = path length to the crater"); a.set_ylabel("break-even price p* (GAE per point map)")
    a.axhline(1, color="k", lw=0.8); a.text(1, 1.15, "priced at 1 GAE (the study's upper price)", fontsize=7)
    a.set_title(f"(a) measured: p* where T_v = 1, n = 63\npooled p* = {hv['breakeven']['pooled_a']:.0f} GAE (charged ops only)", fontsize=8.5)
    a = ax[1]
    xs = np.arange(1, 7)
    for pi, (p, col) in enumerate([(1, C["cub"]), (2, C["T"])]):
        v = [hv["transport_share_by_level"][str(L)][str(PRICES[p])]["value"] for L in xs]
        lo = [hv["transport_share_by_level"][str(L)][str(PRICES[p])]["ci95"][0] for L in xs]; hi = [hv["transport_share_by_level"][str(L)][str(PRICES[p])]["ci95"][1] for L in xs]
        a.errorbar(xs, np.array(v) * 100, yerr=[(np.array(v) - lo) * 100, (np.array(hi) - np.array(v)) * 100], fmt="o-", color=col, capsize=2, label=f"{PRICES[p]:g} GAE per point map")
    a.set_xlabel("source level (depth)"); a.set_ylabel("transport share of the T numerator (%)"); a.legend(frameon=False, fontsize=7)
    a.set_title("(b) measured: transport share by depth\n(levels 1-6; vertices 1/2/4/8/16/32)", fontsize=8.5)
    a = ax[2]
    ex = hv["extrapolation"]["table"]
    for scen, ls in [("measured mean depth", ":"), ("depth 6", "-"), ("depth 6*bits/26", "--")]:
        for pr, col in [(0.1, C["cub"]), (1.0, C["T"])]:
            rows = [r for r in ex if r["depth_scenario"] == scen and r["price"] == pr]
            a.plot([r["bits_r"] for r in rows], [r["transport_share"] * 100 for r in rows], ls, marker="o", ms=3, color=col, label=f"{scen}, {pr:g} GAE")
    a.set_yscale("log"); a.set_xscale("log", base=2); a.set_xticks([26, 64, 128, 256]); a.set_xticklabels(["26\n(measured)", "64", "128", "256"])
    a.set_xlabel("bit size of r (model)"); a.set_ylabel("projected transport share (%)"); a.legend(frameon=False, fontsize=5.8, ncol=1)
    a.set_title("(c) MODEL EXTRAPOLATION, not a measurement:\n2 x depth point maps vs c*sqrt(r) solve", fontsize=8.5, color="#B00020")
    fig.suptitle("Break-even price and transport share (H-T, V5)", fontsize=10, y=1.03)
    fig.tight_layout()
    save(fig, "fig3_breakeven_and_extrapolation")
    # ---- fig4 H-M paired vs unpaired ----
    hm = R["H-M"]
    fig, ax = plt.subplots(1, 3, figsize=(13, 4.2))
    vids = hm["curves"]
    for k, arm in enumerate(ARMS_M):
        a = ax[k]
        ru = hm["per_curve"][f"{arm}|unpaired"]["rows"]; rp = hm["per_curve"][f"{arm}|paired"]["rows"]
        y = np.arange(len(vids))
        pu = np.array([r["holm_p"] if r["p"] is not None else np.nan for r in ru]); pp = np.array([r["holm_p"] if r["p"] is not None else np.nan for r in rp])
        for i in range(len(vids)):
            if np.isfinite(pu[i]) and np.isfinite(pp[i]):
                a.plot([pu[i], pp[i]], [i, i], color="#bbbbbb", lw=1)
        a.plot(pu, y, "o", color=C["T"], label="unpaired (frozen design), Holm"); a.plot(pp, y, "s", color=C["cub"], label="paired: labels shuffled within (target, round)", ms=4)
        a.axvline(0.05, color="k", ls="--", lw=0.8); a.set_xscale("log"); a.set_yticks(y); a.set_yticklabels(vids, fontsize=7); a.invert_yaxis()
        ns_u, ns_p = hm["per_curve"][f"{arm}|unpaired"]["n_sig_holm"].get("p"), hm["per_curve"][f"{arm}|paired"]["n_sig_holm"].get("p")
        a.set_title(f"{arm}: Holm-significant {ns_u} of 12 unpaired, {ns_p if ns_p is not None else 'n/a'} paired" if arm != "bsgs" else "bsgs: F = 0 (unpaired), blocked F undefined: identical in every block", fontsize=8)
        a.set_xlabel("Holm-adjusted p (20,000 shuffles per curve)")
        if k == 0:
            a.legend(frameon=False, fontsize=6.5, loc="lower left")
    fig.suptitle("H-M per curve, unpaired vs paired permutation test (n = 8 models x 64 (target, round) blocks per curve)", fontsize=9.5, y=1.04)
    fig.tight_layout()
    save(fig, "fig4_HM_paired_vs_unpaired")
    # ---- fig5 calibrated max/min ----
    fig, ax = plt.subplots(1, 3, figsize=(13, 4.2), sharey=False)
    for k, arm in enumerate(ARMS_M):
        a = ax[k]
        rows = hm["maxmin_calibration"][f"{arm}|unpaired"]
        for i, r in enumerate(rows):
            a.plot([i, i], r["reported_bootstrap_ci95"], color="#bbbbbb", lw=3, solid_capstyle="butt", label="reported bootstrap 95% CI" if i == 0 else None)
            a.plot(i, r["null_p95"], "_", color=C["th"], ms=11, mew=2, label="95th pct of max/min under model-label permutation" if i == 0 else None)
            a.plot(i, r["observed"], "o", color=C["ic"] if arm == "ic" else C["neg"] if arm == "rho-neg" else C["bsgs"], ms=5, label="observed max/min of model means" if i == 0 else None)
        a.set_xticks(range(12)); a.set_xticklabels([r["vid"] for r in rows], rotation=70, fontsize=6.5)
        a.set_title(arm, fontsize=9); a.axhline(1, color="k", lw=0.5)
        if k == 0:
            a.set_ylabel("max/min of 8 model means"); a.legend(frameon=False, fontsize=6)
    fig.suptitle("Effect-size calibration: max/min of model means is biased above 1 by sampling noise alone (unpaired permutation null)", fontsize=9.5, y=1.04)
    fig.tight_layout()
    save(fig, "fig5_null_calibrated_maxmin")
    # ---- fig6 IC model means on significant curves ----
    sig = list(hm["frozen_significant_ic"]) + ["V6:98"]
    fig, axs = plt.subplots(2, 3, figsize=(11, 6))
    for a, vid in zip(axs.ravel(), sig):
        Y = M[vid]["arms"]["ic"]
        m = Y.mean(1); se = Y.std(1, ddof=1) / np.sqrt(Y.shape[1])
        leg = tmech[tmech.vid == vid].sort_values("model").H.values
        a.errorbar(range(8), m, yerr=1.96 * se, fmt="o", color=C["ic"], capsize=2)
        a.axhline(Y.mean(), color="k", lw=0.5, ls="--")
        a.set_xticks(range(8)); a.set_xticklabels([f"{i}\n({int(leg[i])})" for i in range(8)], fontsize=7)
        a.set_title(f"{vid} (level {M[vid]['level']}): F = {hm['per_curve']['ic|unpaired']['rows'][vids.index(vid)]['F']:.2f}", fontsize=8.5)
        a.set_xlabel("model index (in brackets: H = relation sources inside <G>)", fontsize=6.5); a.set_ylabel("mean S (ic), +-1.96 SE, n = 64 per model", fontsize=7)
    for a in list(axs.ravel())[len(sig):]:
        a.axis("off")
    fig.suptitle("IC model means on the curves significant in the frozen-seed run (5) plus V6:98 (boundary curve)", fontsize=9.5, y=1.04)
    fig.tight_layout()
    save(fig, "fig6_IC_model_means_significant_curves")
    # ---- fig7 boundary ----
    b = hm["boundary"]
    fig, ax = plt.subplots(1, 3, figsize=(13, 4))
    a = ax[0]
    cps = sorted(int(c) for c in b["running_p_one_stream"]); a.plot(cps, [b["running_p_one_stream"][str(c)] for c in cps], "o-", color=C["T"], label="running permutation p of V6:98 (one stream)")
    a.axhline(b["p_parametric_F_7_504"], color=C["cub"], ls="--", label=f"parametric F(7,504): {b['p_parametric_F_7_504']:.4f}")
    a.axhline(0.05 / 7, color="k", ls=":", label="0.05/7 (Holm multiplier 7 at rank 6)")
    a.set_xscale("log"); a.set_xlabel("number of shuffles"); a.set_ylabel("p"); a.legend(frameon=False, fontsize=6.5); a.set_title("(a) V6:98 p vs shuffle count", fontsize=8.5)
    for k, des in enumerate(("unpaired", "paired")):
        a = ax[1 + k]
        hvv = b["repeat_runs_10k_each"][des]["holm_V698_values"]
        a.hist(hvv, bins=18, color=C["T"] if des == "unpaired" else C["cub"], alpha=0.8)
        a.axvline(0.05, color="k", ls="--"); a.axvline(0.0511, color="#555", ls=":", lw=1)
        a.set_xlabel("Holm-adjusted p of V6:98"); a.set_ylabel("independent 10,000-shuffle runs")
        cnt = b["repeat_runs_10k_each"][des]["n_sig_counts"]
        a.set_title(f"({'bc'[k]}) {des}: {len(hvv)} runs; ic significant on 5: {cnt.get('5', 0)} runs, 6: {cnt.get('6', 0)} runs", fontsize=8)
    fig.suptitle("H-M boundary: the 5-vs-6 split rests on V6:98 (frozen-seed Holm 0.0511, dotted)", fontsize=9.5, y=1.04)
    fig.tight_layout()
    save(fig, "fig7_V698_boundary")
    # ---- fig8 gap to theory ----
    q = R["H-T_gap"]; cp = R["crater_rows"]
    fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
    a = ax[0]
    qs = q["quantile_ratios"]["S"]["quantiles"]
    for name, col, lab in [("S", C["cub"], "S (charged ops / sqrt r)"), ("canon", C["T"], "canonicalisation count (uncharged)")]:
        qq = q["quantile_ratios"][name]
        v = np.array(qq["ratio_cubic_over_neg"]); ci = np.array(qq["ci95"])
        off = -0.8 if name == "S" else 0.8
        a.errorbar(np.array(qs) + off, v, yerr=[v - ci[:, 0], ci[:, 1] - v], fmt="o-", color=col, capsize=2, label=lab)
    a.axhline(THEORY, color=C["th"], ls="--"); a.text(5, THEORY - 0.012, "1/sqrt3", fontsize=7)
    a.set_xlabel("quantile of the per-solve cost distribution (%)"); a.set_ylabel("rho-cubic / rho-neg at that quantile"); a.legend(frameon=False, fontsize=7)
    a.set_title(f"(a) quantile ratios, {q['n_pairs_tcr']} Tcr pairs (row bootstrap)", fontsize=8.5)
    a = ax[1]
    ov = q["tcr_overall"]
    items = [("Tcr: charged ops (S) ratio", ov["S_ratio"]["value"][0], ov["S_ratio"]["ci95"][0]), ("Tcr: canonicalisation ratio", ov["canon_ratio"]["value"][0], ov["canon_ratio"]["ci95"][0]),
             ("Tcr: ops ratio predicted if iterations\nscale exactly 1/sqrt3 and the rest stays", ov["pred_ops_ratio_if_iterations_exactly_theory"]["value"][0], ov["pred_ops_ratio_if_iterations_exactly_theory"]["ci95"][0]),
             ("crater (D,N,ctl): charged ops ratio", cp["S_ratio"]["value"], cp["S_ratio"]["ci95"]), ("crater: canonicalisation ratio", cp["canon_ratio"]["value"], cp["canon_ratio"]["ci95"]),
             ("crater: predicted (as above)", cp["pred_ops_ratio_if_iterations_exactly_theory"], cp["pred_ops_ratio_ci95"])]
    for i, (lab, v, ci) in enumerate(items):
        a.errorbar(v, i, xerr=[[v - ci[0]], [ci[1] - v]], fmt="o", color=C["cub"] if "ops" in lab or "charged" in lab else C["T"], capsize=2)
    a.axvline(THEORY, color=C["th"], ls="--"); a.set_yticks(range(len(items))); a.set_yticklabels([i[0] for i in items], fontsize=6.5); a.invert_yaxis()
    a.set_xlabel("rho-cubic / rho-neg (95% CI)"); a.set_title("(b) the ~6% gap: a charged additive part (~371 ops) that does not shrink", fontsize=8.5)
    fig.suptitle("Gap between the measured cubic/neg ratio and 1/sqrt3", fontsize=9.5, y=1.04)
    fig.tight_layout()
    save(fig, "fig8_gap_to_theory")
    # ---- fig9 power ----
    pw = R["H-M_power"]
    fig, ax = plt.subplots(figsize=(6.5, 4))
    for scen, col in [("shrunk", C["T"]), ("observed", C["cub"]), ("null", C["th"])]:
        rows = pw["scenarios"][scen]
        ax.plot([r["n_rows_per_model"] for r in rows], [r["P_at_least_6_of_12"] for r in rows], "o-", color=col, label={"shrunk": "truth = shrunk observed means", "observed": "truth = observed means (optimistic)", "null": "no model effect (size check)"}[scen])
    ax.axhline(0.8, color="k", ls="--", lw=0.7); ax.axvline(64, color="#999", ls=":"); ax.text(66, 0.05, "this study: 64", fontsize=7)
    ax.set_xlabel("rows per model (n)"); ax.set_ylabel("P(ic significant on >= 6 of 12 curves)"); ax.legend(frameon=False, fontsize=7)
    ax.set_title(f"Power simulation, {pw['reps']} simulated studies per point; design guidance only", fontsize=8.5)
    save(fig, "fig9_power")
    # ---- fig11 yield mechanism ----
    fig, ax = plt.subplots(1, 2, figsize=(10, 4.2))
    cmap = plt.cm.tab20(np.linspace(0, 1, 12))
    ym = R["mechanism"]["yield_mechanism"]
    for k, (ycol, lab, key) in enumerate([("tpr", "trials per relation (log), centred within curve", "corr_log_trials_per_relation_vs_log_H"), ("S", "mean S (ic) (log), centred within curve", "corr_log_S_ic_vs_log_H")]):
        a = ax[k]
        for ci, v in enumerate(sorted(tmech.vid.unique(), key=lambda z: (z[:2], int(z.split(":")[1])))):
            s_ = tmech[tmech.vid == v]
            a.scatter(np.log(s_.H) - np.log(s_.H).mean(), np.log(s_[ycol]) - np.log(s_[ycol]).mean(), s=14, color=cmap[ci], label=v)
        a.axhline(0, color="#ccc", lw=0.6); a.axvline(0, color="#ccc", lw=0.6)
        a.set_xlabel("log H (relation sources inside <G>), centred within curve"); a.set_ylabel(lab, fontsize=8)
        a.set_title(f"r = {ym[key]['r_within_curve']:.2f}, permutation p = {ym[key]['perm_p']:.4f}, n = 96 models (12 curves)", fontsize=8.5)
    ax[1].legend(fontsize=5.5, ncol=2, frameon=False)
    fig.suptitle("IC relation yield tracks the factor base on each model (mechanism candidate)", fontsize=9.5, y=1.04)
    fig.tight_layout()
    save(fig, "fig11_ic_yield_mechanism")
    # ---- fig10 V6 model effect ----
    v6 = R["V6_model_effect"]
    fig, ax = plt.subplots(figsize=(7, 4))
    vc = hm["variance_components"]
    labels, vals, los, his = [], [], [], []
    for arm in ARMS_M:
        r = v6["arms"][arm]
        labels.append(f"V6 D+2 re-scalings\n{arm}\n(n={r['n_vertices']} vertices)"); vals.append(r["cv_between_models"]); los.append(r["cv_between_models_ci95"][0]); his.append(r["cv_between_models_ci95"][1])
    for arm in ARMS_M:
        cv = np.array([x["cv_between_models"] for x in vc[arm]]) ** 2
        labels.append(f"M specs (8 models)\n{arm}\n(n=12 curves)"); vals.append(float(np.sqrt(cv.mean()))); 
        rb = rng_for(60, ARMS_M.index(arm)); bb = np.sqrt(cv[rb.integers(0, 12, (REPS, 12))].mean(1)); los.append(float(np.percentile(bb, 2.5))); his.append(float(np.percentile(bb, 97.5)))
    cols = [C["ic"], C["neg"], C["bsgs"]] * 2
    ax.bar(range(6), vals, color=cols, alpha=0.85); ax.errorbar(range(6), vals, yerr=[np.array(vals) - np.array(los), np.array(his) - np.array(vals)], fmt="none", color="k", capsize=3)
    ax.set_xticks(range(6)); ax.set_xticklabels(labels, fontsize=6.5); ax.set_ylabel("between-model CV of mean S (method of moments)")
    ax.set_title("Model effect size by arm: V6 Q1 data (3 models) and H-M (8 models); 95% CI by cluster bootstrap", fontsize=8.5)
    save(fig, "fig10_model_effect_by_arm")


def input_hashes():
    import glob
    h = {"analysis/analyze.py": sha256(f"{ROOT}/analysis/analyze.py"), "analysis/results.json": sha256(f"{ROOT}/analysis/results.json"),
         "runs/progress.jsonl": sha256(f"{ROOT}/runs/progress.jsonl"), "runs/transfer_cost.jsonl": sha256(f"{ROOT}/runs/transfer_cost.jsonl"),
         "REPORT.md": sha256(f"{ROOT}/REPORT.md"), "AMENDMENTS.md": sha256(f"{ROOT}/AMENDMENTS.md"), "ANALYSIS_PLAN.md": sha256(f"{ROOT}/ANALYSIS_PLAN.md"),
         "census/V5/M.json": sha256(f"{ROOT}/census/V5/M.json"), "census/V6/M.json": sha256(f"{ROOT}/census/V6/M.json"),
         "census/V5/vertices.jsonl": sha256(f"{ROOT}/census/V5/vertices.jsonl"), "census/V6/vertices.jsonl": sha256(f"{ROOT}/census/V6/vertices.jsonl"),
         "deepdive.py": sha256(f"{HERE}/deepdive.py")}
    for sub, pat in (("runs/extract", "*"), ("runs/uncharged", "*"), ("runs/slugmap", "*"), ("census/V5/tcr", "*"), ("sessions (M archives only)", None)):
        fs = sorted(glob.glob(f"{ROOT}/{sub}/{pat}")) if pat else sorted(glob.glob(f"{ROOT}/sessions/*-M-L*-c*.tar.xz"))
        agg = hashlib.sha256()
        for f in fs:
            agg.update((os.path.basename(f) + " " + sha256(f) + "\n").encode())
        h[f"{sub}/ (digest over '<name> <sha256>' lines of {len(fs)} files)"] = agg.hexdigest()
    return h


def clean(o):
    if isinstance(o, dict):
        return {str(k): clean(v) for k, v in o.items() if not str(k).startswith("_")}
    if isinstance(o, (list, tuple)):
        return [clean(v) for v in o]
    if isinstance(o, (np.floating,)):
        return None if not np.isfinite(o) else float(o)
    if isinstance(o, float):
        return None if not math.isfinite(o) else o
    if isinstance(o, (np.integer,)):
        return int(o)
    if isinstance(o, (np.bool_,)):
        return bool(o)
    return o


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--cache", default=None)
    ap.add_argument("--quick", action="store_true")
    ap.add_argument("--no-figs", action="store_true")
    a = ap.parse_args()
    t0 = time.time()
    d, tcr, VERT, rt = load(a.cache)
    log("loaded", len(d), "rows")
    R["meta"] = {"label": "EXPLORATORY, POST HOC: nothing here changes a preregistered verdict or makes a speedup claim", "seed": SEED, "reps": REPS, "quick": a.quick,
                 "preregistered_verdicts_quoted_from_REPORT_md": {"H-T": "SUPPORTED", "H-M": "INCONCLUSIVE by the frozen-seed count (ic significant on 5 of 12; rho-neg 0, bsgs 0), on the Monte Carlo boundary with 'supported for IC' (erratum F1)"}}
    ht, Aa, level, Mv, vm = ht_section(d, tcr, rt)
    R["H-T_decomposition"] = {k: v for k, v in ht.items() if k != "q2_gap"}
    R["H-T_gap"] = ht["q2_gap"]
    log("H-T done")
    R["crater_rows"] = crater_section(d, ht)
    pv, pvout = per_vertex_section(ht, rt, Aa, level, Mv, vm)
    R["H-T_vertices"] = pvout
    log("per vertex done")
    hm, store = hm_section(d, rt, a.quick)
    M = hm["_M"]
    R["H-M"] = hm
    log("H-M done")
    mech, tmech = mechanism_section(d, M, VERT)
    R["mechanism"] = mech
    R["V6_model_effect"] = v6_model_effect(d)
    log("V6 model effect done")
    R["H-M_power"] = power_section(M, a.quick)
    log("power done")
    R["inputs_sha256"] = input_hashes()
    R["runtime_s"] = round(time.time() - t0, 1)
    out = f"{HERE}/results_deepdive.json" if not a.quick else f"{HERE}/results_deepdive_quick.json"
    json.dump(clean(R), open(out, "w"), indent=1, sort_keys=True)
    log("wrote", out)
    if not a.no_figs:
        figures(clean_for_fig(R), pv, M, tmech, f"{HERE}/figures")
        log("figures done")


def clean_for_fig(R):
    # figures read the same structures; keep numpy private keys (_M) out except via explicit args
    return R


if __name__ == "__main__":
    main()
