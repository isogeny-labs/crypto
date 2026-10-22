#!/usr/bin/env python3
"""Phase C analysis of the volcano ECDLP census: implements ANALYSIS_PLAN.md s1-s9.

Run:  python3 -I analysis/analyze.py [--reps N] [--out analysis/results.json]

Everything here follows the frozen plan; interpretations the plan leaves open
are listed in AMENDMENTS.md (A2) and marked `interpretation:` in results.json.
Seeds: bootstrap 20261022, permutation 20261023 (a fresh generator per
statistic, so every arm of a volcano sees the same resampling indices).
"""
import argparse, collections, glob, hashlib, io, json, lzma, math, os, sys, time, warnings
import numpy as np
import pandas as pd
from scipy import stats

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SEED_BOOT, SEED_PERM = 20261022, 20261023
ARMS_MAIN = ["rho-neg", "rho-plain", "bsgs", "bsgs-neg", "ic"]
ISO = ["D", "Ua", "Ub", "S", "N"]
MARGIN = {"rho-neg": 0.95, "rho-plain": 0.95, "bsgs": 0.95, "bsgs-neg": 0.95, "ic": 0.90}
PRICES = [0.0, 0.1, 1.0]
SRC2FAM = {"D": "D", "floor-a": "Ua", "floor-b": "Ub", "crater-b": "S"}


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""):
            h.update(b)
    return h.hexdigest()


# ----------------------------------------------------------------- data
def jinv(p, a, b):
    t = 4 * pow(a, 3, p) % p
    return 1728 * t * pow((t + 27 * b * b) % p, -1, p) % p


def load():
    prog = [json.loads(l) for l in open(f"{ROOT}/runs/progress.jsonl")]
    done = [d for d in prog if d.get("status") == "done" and d.get("audit_ok") is True and "spec" in d]
    specs = sorted(set(d["spec"] for d in done))
    assert len(specs) == len(done), "a session counted twice"
    frames, unc = [], []
    for s in specs:
        frames.append(pd.read_csv(f"{ROOT}/runs/extract/{s}.csv.xz"))
        unc.append(pd.read_csv(f"{ROOT}/runs/uncharged/{s}.csv.xz"))
    d = pd.concat(frames, ignore_index=True)
    u = pd.concat(unc, ignore_index=True)
    n_all = len(d)
    d = d[~d.warmup].copy()
    key = ["spec", "workload_id", "target_index", "arm", "round"]
    assert not d.duplicated(key).any()
    assert not u.duplicated(key).any()
    d = d.merge(u[key + ["total_gae", "s", "unc_total", "unc_detail"]], on=key, how="left", validate="1:1")
    chk = {
        "rows_all_incl_warmup": int(n_all),
        "rows_measured": int(len(d)),
        "uncharged_merge_missing": int(d.total_gae.isna().sum()),
        "max_abs_S_diff": float((d.S - d.s).abs().max()),
        "max_abs_gae_diff": float((d.ops_gae - d.total_gae).abs().max()),
    }
    # vertex map
    J, VL, VERT = {}, {}, {}
    for V in ["V1", "V2", "V3", "V4"]:
        fn = f"{ROOT}/census/{V}/vertices.jsonl"
        if not os.path.exists(fn):
            continue
        for l in open(fn):
            v = json.loads(l)
            J[(V, v["j"])] = v["id"]
            VL[(V, v["id"])] = v["level"]
            VERT.setdefault(V, []).append(v)
    slug2v = {}
    for s in specs:
        V = s.split("-")[0]
        for slug, c in json.load(open(f"{ROOT}/runs/slugmap/{s}.json")).items():
            slug2v[(V, slug)] = J[(V, jinv(c["p"], c["a"], c["b"]))]
    d["vertex"] = [slug2v[(v, s)] for v, s in zip(d.volcano, d.curve_slug)]
    d["vlevel"] = [VL[(v, i)] for v, i in zip(d.volcano, d.vertex)]
    chk["level_mismatch_with_session_level"] = int((d.vlevel != d.level.astype(int)).sum())
    d["level"] = d.level.astype(int)
    d["vid"] = d.volcano + ":" + d.vertex.astype(str)
    return d, chk, VERT, specs, done, prog


# ------------------------------------------------------ bootstrap engine
def boot_sums(strata, clusters, X, reps, seed, chunk=200):
    """Two-stage cluster bootstrap, stratified.

    Within each stratum: resample clusters with replacement, then resample the
    rows of each chosen cluster with replacement (as many as the cluster has).
    Returns (sums[reps, S, k], cnts[reps, S]) with strata remapped to 0..S-1 in
    sorted order of the input stratum codes.
    """
    strata = np.asarray(strata)
    su = np.unique(strata)
    smap = {s: i for i, s in enumerate(su)}
    st = np.array([smap[s] for s in strata])
    order = np.lexsort((clusters, st))
    st, cl, X = st[order], np.asarray(clusters)[order], np.asarray(X, float)[order]
    if X.ndim == 1:
        X = X[:, None]
    uc, start, size = np.unique(cl, return_index=True, return_counts=True)
    cl_st = st[start]
    S, k = len(su), X.shape[1]
    ids = [np.where(cl_st == s)[0] for s in range(S)]
    rng = np.random.default_rng(seed)
    sums = np.zeros((reps, S, k))
    cnts = np.zeros((reps, S))
    done = 0
    while done < reps:
        c = min(chunk, reps - done)
        for s in range(S):
            m = len(ids[s])
            sel = ids[s][rng.integers(0, m, (c, m))]
            sz = size[sel]
            tot_rep = sz.sum(1)
            flat = sz.ravel()
            tot = int(flat.sum())
            idx = np.repeat(start[sel].ravel(), flat) + (rng.random(tot) * np.repeat(flat, flat)).astype(np.int64)
            vals = X[idx]
            bounds = np.concatenate([[0], np.cumsum(tot_rep)[:-1]])
            sums[done:done + c, s, :] = np.add.reduceat(vals, bounds, axis=0)
            cnts[done:done + c, s] = tot_rep
        done += c
    obs_sum = np.zeros((S, k))
    obs_cnt = np.zeros(S)
    np.add.at(obs_sum, st, X)
    np.add.at(obs_cnt, st, 1)
    return sums, cnts, obs_sum, obs_cnt


def pct(x, lo, hi):
    return [float(np.percentile(x, lo)), float(np.percentile(x, hi))]


def boot_p(x, null=1.0):
    n = len(x)
    a = (np.sum(x <= null) + 1) / (n + 1)
    b = (np.sum(x >= null) + 1) / (n + 1)
    return float(min(1.0, 2 * min(a, b)))


def ratio_summary(obs, rep, margin):
    """obs point ratio, rep array of replicate ratios, margin lower bound m (upper 1/m)."""
    ci95, ci90 = pct(rep, 2.5, 97.5), pct(rep, 5, 95)
    lo, hi = margin, 1 / margin
    return {
        "ratio": float(obs), "ci95": ci95, "ci90": ci90,
        "half_width95": (ci95[1] - ci95[0]) / 2,
        "rel_half_width95": (ci95[1] - ci95[0]) / 2 / float(obs) if obs else None,
        "p_boot": boot_p(rep),
        "tost_equivalent": bool(ci90[0] >= lo and ci90[1] <= hi),
        "ci95_excludes_1": bool(ci95[0] > 1 or ci95[1] < 1),
        "margin": [lo, hi],
        "boot_sd": float(np.std(rep, ddof=1)),
    }


def holm(pvals, alpha=0.05):
    """Holm-Bonferroni adjusted p-values (monotone), same order as input."""
    p = np.asarray(pvals, float)
    n = len(p)
    o = np.argsort(p)
    adj = np.empty(n)
    run = 0.0
    for r, i in enumerate(o):
        run = max(run, (n - r) * p[i])
        adj[i] = min(1.0, run)
    return adj.tolist()


def verdict(rs, holm_p, control_ok):
    if rs["tost_equivalent"]:
        v = "equivalent"
        if not control_ok:
            v = "uninformative"
    elif rs["ci95_excludes_1"] and holm_p < 0.05:
        v = "different"
    else:
        v = "inconclusive"
    return v


def clopper_pearson(k, n, a=0.05):
    lo = 0.0 if k == 0 else stats.beta.ppf(a / 2, k, n - k + 1)
    hi = 1.0 if k == n else stats.beta.ppf(1 - a / 2, k + 1, n - k)
    return [float(lo), float(hi)]


# ------------------------------------------------------------- analysis
def level_effect(d, V, arm, reps, depth):
    """PLAN s3 for one (volcano, arm): R(L) on S, ops_gae and wall, cluster bootstrap."""
    x = d[(d.volcano == V) & (d.arm == arm) & d.family.isin(ISO)]
    cl = x.vertex.values
    sums, cnts, osum, ocnt = boot_sums(x.level.values, cl, x[["S", "ops_gae", "solve_wall_ns"]].values, reps, SEED_BOOT)
    means = sums / cnts[:, :, None]
    omean = osum / ocnt[:, None]
    out = {"n_rows": int(len(x)), "n_curves": int(x.vertex.nunique()), "levels": {}}
    for L in range(depth + 1):
        xl = x[x.level == L]
        out["levels"][str(L)] = {
            "n_curves": int(xl.vertex.nunique()), "n_rows": int(len(xl)),
            "mean_S": float(omean[L, 0]), "mean_S_ci95": pct(means[:, L, 0], 2.5, 97.5),
            "mean_ops_gae": float(omean[L, 1]), "mean_wall_ns": float(omean[L, 2]),
            "cv_S_per_row": float(xl.S.std(ddof=1) / xl.S.mean()),
            "mean_S_rel_half_width95": float((np.percentile(means[:, L, 0], 97.5) - np.percentile(means[:, L, 0], 2.5)) / 2 / omean[L, 0]),
            "frac_L2_rows": float((xl.isolation == "L2").mean()),
        }
    out["R"] = {}
    for L in range(1, depth + 1):
        r = {}
        for ci, name in enumerate(["S", "ops_gae", "wall"]):
            obs = omean[L, ci] / omean[0, ci]
            rep = means[:, L, ci] / means[:, 0, ci]
            r[name] = ratio_summary(obs, rep, MARGIN[arm])
        out["R"][str(L)] = r
    # per-family descriptive R(L)
    fam = {}
    for f in ISO:
        xf = x[x.family == f]
        m0 = xf[xf.level == 0].S.mean()
        fam[f] = {str(L): float(xf[xf.level == L].S.mean() / m0) for L in range(1, depth + 1)}
    out["R_by_family_descriptive"] = fam
    return out


def mixed_model(d, V, arm):
    import statsmodels.formula.api as smf
    x = d[(d.volcano == V) & (d.arm == arm) & d.family.isin(ISO)].copy()
    x["logS"] = np.log(x.S)
    res = {"formula": "logS ~ C(level) + C(family) + (1 | curve)", "method": "MixedLM REML"}
    try:
        with warnings.catch_warnings(record=True) as w:
            warnings.simplefilter("always")
            m = smf.mixedlm("logS ~ C(level) + C(family)", x, groups=x["vertex"]).fit(reml=True)
        res["converged"] = bool(m.converged)
        res["warnings"] = sorted(set(str(i.message)[:160] for i in w))
        var_c = float(m.cov_re.iloc[0, 0])
        var_e = float(m.scale)
        res["var_curve"], res["var_resid"] = var_c, var_e
        res["icc"] = var_c / (var_c + var_e) if var_c + var_e > 0 else None
        ci = m.conf_int()
        res["level_ratio"] = {}
        for name in m.params.index:
            if name.startswith("C(level)"):
                L = name.split("[T.")[1].rstrip("]")
                res["level_ratio"][L] = {"ratio": float(np.exp(m.params[name])),
                                         "ci95": [float(np.exp(ci.loc[name, 0])), float(np.exp(ci.loc[name, 1]))],
                                         "p": float(m.pvalues[name])}
    except Exception as e:  # reported, never overrides the bootstrap
        res["converged"] = False
        res["error"] = repr(e)[:300]
    return res


def curve_effect(d, V, arm, reps):
    """PLAN s4. Variance-ratio permutation test + per-curve flags."""
    x = d[(d.volcano == V) & (d.arm == arm)]
    m = x[x.family == "D"].groupby("vertex").S.mean()
    cx = x[x.family == "ctl"]
    slugs = cx.groupby("vertex").curve_slug.unique()
    cm = cx.groupby(["vertex", "curve_slug"]).S.mean()
    verts = [v for v in m.index if v in slugs.index and len(slugs[v]) == 2]
    lev = np.array([int(x[(x.family == "D") & (x.vertex == v)].level.iloc[0]) for v in verts])
    vals = np.array([[m[v], *[cm[(v, s)] for s in sorted(slugs[v])]] for v in verts])  # (n,3): D, c1, c2
    n = len(verts)
    within = float(np.mean(vals.var(axis=1, ddof=1)))  # invariant under label shuffles
    ctl_sd = float(np.sqrt(np.mean((vals[:, 1] - vals[:, 2]) ** 2 / 2)))

    def between(mv):
        num, den = 0.0, 0
        for L in np.unique(lev):
            t = mv[lev == L]
            if len(t) > 1:
                num += ((t - t.mean()) ** 2).sum()
                den += len(t) - 1
        return num / den

    out = {"n_vertices": n, "within_var": within, "control_sd_pooled": ctl_sd,
           "between_var_pooled_within_level": float(between(vals[:, 0]))}
    if within == 0 or ctl_sd == 0:
        out.update({"F": None, "p_perm": None, "degenerate": "zero within-vertex variance: D and both re-scalings give identical S",
                    "flagged": [], "n_flagged": 0})
        return out, None
    F = between(vals[:, 0]) / within
    rng = np.random.default_rng(SEED_PERM)
    ge, znull = 0, []
    rows = np.arange(n)
    Fs = np.empty(reps)
    absz_all = []
    for r in range(reps):
        ch = rng.integers(0, 3, n)
        mv = vals[rows, ch]
        Fs[r] = between(mv) / within
        other = (vals.sum(1) - mv) / 2.0  # mean of the two labels left as controls
        absz_all.append(np.abs((mv - other) / ctl_sd))
    absz_all = np.concatenate(absz_all)
    out["F"] = float(F)
    out["p_perm"] = float((np.sum(Fs >= F) + 1) / (reps + 1))
    z = (vals[:, 0] - vals[:, 1:].mean(1)) / ctl_sd
    srt = np.sort(absz_all)
    N = len(srt)
    pv = (N - np.searchsorted(srt, np.abs(z), side="left") + 1) / (N + 1)
    adj = holm(pv)
    flagged = [{"vertex": int(verts[i]), "level": int(lev[i]), "z": float(z[i]), "p": float(pv[i]), "p_holm": float(adj[i])}
               for i in range(n) if adj[i] < 0.01]
    out.update({"flagged": flagged, "n_flagged": len(flagged),
                "interpretation": "per-curve p = share of the pooled |z*| (all vertices, all permutations) >= |z_v|; within-vertex permutations alone have only 3 outcomes",
                "vertices": [{"vertex": int(verts[i]), "level": int(lev[i]), "m": float(vals[i, 0]), "c1": float(vals[i, 1]), "c2": float(vals[i, 2]),
                              "z": float(z[i]), "p": float(pv[i]), "p_holm": float(adj[i])} for i in range(n)]})
    return out, np.array(out["p_perm"])


def paired_source(d, V, arm, src, reps, depth):
    """Ratio of means src/D on shared (vertex, target_index, round) triples; strata = level."""
    x = d[(d.volcano == V) & (d.arm == arm) & d.family.isin(["D", src])]
    key = ["vertex", "target_index", "round"]
    a = x[x.family == "D"].set_index(key)[["S", "level"]]
    b = x[x.family == src].set_index(key)[["S"]]
    j = a.join(b, how="inner", lsuffix="_D", rsuffix="_X").reset_index()
    if j.empty:
        return None
    sums, cnts, osum, ocnt = boot_sums(j.level.values, j.vertex.values, j[["S_X", "S_D"]].values, reps, SEED_BOOT)
    obs = osum[:, 0].sum() / osum[:, 1].sum()
    rep = sums[:, :, 0].sum(1) / sums[:, :, 1].sum(1)
    r = ratio_summary(obs, rep, MARGIN[arm])
    r.update({"n_pairs": int(len(j)), "n_curves": int(j.vertex.nunique()), "ratio_src_over_D": r.pop("ratio")})
    r["n_rows_D"] = int((x.family == "D").sum()); r["n_rows_src"] = int((x.family == src).sum())
    return r


def transport_then_solve(d, V, arm, fam, tr, reps, depth, rows_fam=None, zero_E=False):
    """Ratio (mean ops_gae at destination w + p * velu(w)) / mean ops_gae at the source, per destination level."""
    t = tr[(tr.volcano == V) & (tr.fam == fam)]
    if t.empty:
        return None
    src_rows = t[t.path_len == 0]
    assert len(src_rows) == 1
    sv = int(src_rows.vertex.iloc[0])
    x = d[(d.volcano == V) & (d.arm == arm) & (d.family == (rows_fam or fam))].copy()
    ev = t.set_index("vertex").velu_point_maps
    x["E"] = 0 if zero_E else x.vertex.map(ev)
    dest = x[x.vertex != sv]
    src = x[x.vertex == sv]
    if dest.empty or src.empty:
        return None
    X = np.column_stack([dest.ops_gae + p * dest.E for p in PRICES] + [dest.S])
    srcX = np.column_stack([src.ops_gae.values] * len(PRICES) + [src.S.values])
    strata = np.concatenate([dest.level.values, np.full(len(src), depth + 1)])
    clusters = np.concatenate([dest.vertex.values, np.full(len(src), -1)])
    allX = np.vstack([X, srcX])
    sums, cnts, osum, ocnt = boot_sums(strata, clusters, allX, reps, SEED_BOOT)
    su = list(np.unique(strata))
    si = su.index(depth + 1)
    out = {"source_vertex": sv, "levels": {}}
    mean_e = {}
    for L in su:
        if L == depth + 1:
            continue
        li = su.index(L)
        e = dest[dest.level == L].E
        lv = {"n_dest_vertices": int(dest[dest.level == L].vertex.nunique()), "mean_velu_evals": float(e.mean()), "prices": {}}
        for pi, p in enumerate(PRICES):
            rep = (sums[:, li, pi] / cnts[:, li]) / (sums[:, si, pi] / cnts[:, si])
            obs = (osum[li, pi] / ocnt[li]) / (osum[si, pi] / ocnt[si])
            rs = ratio_summary(obs, rep, MARGIN[arm])
            lv["prices"][str(p)] = {k: rs[k] for k in ("ratio", "ci95", "boot_sd")}
        all3 = [lv["prices"][str(p)] for p in PRICES]
        lv["ci95_excludes_1_all_prices_same_side"] = bool(all(c["ci95"][0] > 1 for c in all3) or all(c["ci95"][1] < 1 for c in all3))
        out["levels"][str(L)] = lv
    return out


def positive_control(d, V3_crater, reps):
    x = d[(d.volcano == "V3") & (d.vertex == V3_crater) & d.family.isin(["D", "N", "ctl"]) & d.arm.isin(["rho-cubic", "rho-neg"])]
    key = ["spec", "workload_id", "target_index", "round"]
    a = x[x.arm == "rho-cubic"].set_index(key)[["S", "curve_slug"]]
    b = x[x.arm == "rho-neg"].set_index(key)[["S"]]
    j = a.join(b, how="inner", lsuffix="_cubic", rsuffix="_neg").reset_index()
    out = {"vertex": int(V3_crater), "n_pairs": int(len(j)), "n_slugs": int(j.curve_slug.nunique()),
           "families": sorted(x.family.unique().tolist()), "theory_ratio": 1 / math.sqrt(3), "pass_threshold_ci_upper_below": 0.85}
    res = {}
    for name, cl in [("row_bootstrap", np.arange(len(j))), ("slug_cluster_sensitivity", pd.factorize(j.curve_slug)[0])]:
        if name == "row_bootstrap":
            # one cluster, row resampling only (the crater is a single curve)
            sums, cnts, osum, ocnt = boot_sums(np.zeros(len(j), int), np.zeros(len(j), int), j[["S_cubic", "S_neg"]].values, reps, SEED_BOOT)
        else:
            sums, cnts, osum, ocnt = boot_sums(np.zeros(len(j), int), cl, j[["S_cubic", "S_neg"]].values, reps, SEED_BOOT)
        rep = sums[:, 0, 0] / sums[:, 0, 1]
        obs = osum[0, 0] / osum[0, 1]
        res[name] = {"ratio": float(obs), "ci95": pct(rep, 2.5, 97.5)}
    out.update(res)
    out["pass"] = bool(res["row_bootstrap"]["ci95"][1] < 0.85)
    return out


def ic_vs_rho(d, V, reps, depth):
    """PLAN s9: per level mean S(ic)/mean S(rho-neg) at 0 / 0.1 / 1 GAE per uncharged operation."""
    x = d[(d.volcano == V) & d.family.isin(ISO) & d.arm.isin(["ic", "rho-neg"])]
    key = ["spec", "workload_id", "target_index", "round"]
    a = x[x.arm == "ic"].set_index(key)
    b = x[x.arm == "rho-neg"].set_index(key)
    j = a[["vertex", "level", "S", "total_gae", "unc_total"]].join(b[["S", "total_gae", "unc_total"]], how="inner", lsuffix="_ic", rsuffix="_rho").reset_index()
    cols = []
    for p in PRICES:
        cols.append(j.S_ic * (1 + p * j.unc_total_ic / j.total_gae_ic))
    for p in PRICES:
        cols.append(j.S_rho * (1 + p * j.unc_total_rho / j.total_gae_rho))
    X = np.column_stack(cols)
    sums, cnts, osum, ocnt = boot_sums(j.level.values, j.vertex.values, X, reps, SEED_BOOT)
    out = {"n_pairs": int(len(j)), "levels": {}, "pooled": {}}
    for L in range(depth + 1):
        lv = {"n_curves": int(j[j.level == L].vertex.nunique()), "n_pairs": int((j.level == L).sum()), "prices": {}}
        for pi, p in enumerate(PRICES):
            rep = sums[:, L, pi] / sums[:, L, pi + 3]
            obs = osum[L, pi] / osum[L, pi + 3]
            lv["prices"][str(p)] = {"ratio_ic_over_rho": float(obs), "ci95": pct(rep, 2.5, 97.5),
                                    "mean_S_ic": float(osum[L, pi] / ocnt[L]), "mean_S_rho": float(osum[L, pi + 3] / ocnt[L])}
        pr = [lv["prices"][str(p)] for p in PRICES]
        if all(c["ci95"][0] > 1 for c in pr):
            lv["ranking"] = "rho-neg ahead of ic at all three prices (95% CI above 1 at each)"
        elif all(c["ci95"][1] < 1 for c in pr):
            lv["ranking"] = "ic ahead of rho-neg at all three prices (95% CI below 1 at each)"
        else:
            lv["ranking"] = "order not established across the band"
        out["levels"][str(L)] = lv
    for pi, p in enumerate(PRICES):
        rep = sums[:, :, pi].sum(1) / sums[:, :, pi + 3].sum(1)
        obs = osum[:, pi].sum() / osum[:, pi + 3].sum()
        out["pooled"][str(p)] = {"ratio_ic_over_rho": float(obs), "ci95": pct(rep, 2.5, 97.5)}
    # uncharged counts per arm
    u = {}
    for arm in ["ic", "rho-neg"]:
        y = d[(d.volcano == V) & (d.family.isin(ISO)) & (d.arm == arm)]
        agg = collections.Counter()
        for s in y.unc_detail.dropna():
            for kv in s.split(";"):
                if kv:
                    k_, v_ = kv.split("=")
                    agg[k_] += int(v_)
        u[arm] = {"mean_uncharged_per_run": {k_: v_ / len(y) for k_, v_ in agg.items()},
                  "mean_unc_total_over_gae": float(y.unc_total.sum() / y.total_gae.sum()), "n_runs": int(len(y))}
    out["uncharged"] = u
    return out


def wall_admission(d, V, reps):
    """PLAN s8: A/A ratio mean wall(rho-neg-aa)/mean wall(rho-neg), cluster bootstrap by level strata."""
    x = d[(d.volcano == V) & d.family.isin(ISO) & d.arm.isin(["rho-neg", "rho-neg-aa"])]
    key = ["spec", "workload_id", "target_index", "round"]
    a = x[x.arm == "rho-neg-aa"].set_index(key)[["vertex", "level", "solve_wall_ns"]]
    b = x[x.arm == "rho-neg"].set_index(key)[["solve_wall_ns"]]
    j = a.join(b, how="inner", lsuffix="_aa", rsuffix="_neg").reset_index()
    sums, cnts, osum, ocnt = boot_sums(j.level.values, j.vertex.values, j[["solve_wall_ns_aa", "solve_wall_ns_neg"]].values, reps, SEED_BOOT)
    rep = sums[:, :, 0].sum(1) / sums[:, :, 1].sum(1)
    obs = osum[:, 0].sum() / osum[:, 1].sum()
    ci = pct(rep, 2.5, 97.5)
    out = {"aa_ratio": float(obs), "ci95": ci, "n_pairs": int(len(j)), "cond_aa_within_5pct": bool(ci[0] >= 0.95 and ci[1] <= 1.05), "arms": {}}
    for arm in ARMS_MAIN:
        y = d[(d.volcano == V) & d.family.isin(ISO) & (d.arm == arm)]
        allL2 = bool((y.isolation == "L2").all())
        out["arms"][arm] = {"frac_rows_L2": float((y.isolation == "L2").mean()), "all_rows_L2": allL2,
                            "admitted": bool(allL2 and out["cond_aa_within_5pct"])}
    return out


def transfer_cost(d, VERT):
    tc = pd.DataFrame([json.loads(l) for l in open(f"{ROOT}/runs/transfer_cost.jsonl")])
    tc["fam"] = tc.src.map(SRC2FAM)
    # census transport rows give the destination vertex for each walk id
    rows = []
    for V in ["V1", "V2", "V3"]:
        J = {v["j"]: v["id"] for v in VERT[V]}
        for src, fam in SRC2FAM.items():
            fn = f"{ROOT}/census/{V}/transport/{src}.jsonl"
            if not os.path.exists(fn):
                continue
            for l in open(fn):
                r = json.loads(l)
                rows.append({"volcano": V, "fam": fam, "id": r["id"], "vertex": J[r["j"]], "census_level": r["level"], "census_path_len": r["path_len"]})
    cen = pd.DataFrame(rows)
    tc = tc.merge(cen, on=["volcano", "fam", "id"], how="left", validate="1:1")
    assert tc.vertex.notna().all()
    assert (tc.level == tc.census_level).all() and (tc.path_len == tc.census_path_len).all()
    tc["vertex"] = tc.vertex.astype(int)
    return tc


def transfer_summary(d, tc):
    out = {"all_matches_transported": bool(tc.matches_transported.all()), "n_rows": int(len(tc)), "by_volcano_path_len": {}, "fraction_of_solve": {}}
    for V in ["V1", "V2", "V3"]:
        t = tc[tc.volcano == V]
        per = {}
        for dlen, g in t[t.path_len > 0].groupby("path_len"):
            q = lambda s: [float(s.median()), float(s.quantile(0.25)), float(s.quantile(0.75))]
            per[str(int(dlen))] = {"n_paths": int(len(g)), "ns_per_edge_median_q1_q3": q(g.ns_per_edge_mean),
                                   "ns_per_path_median_q1_q3": q(g.ns_mean), "velu_point_maps_per_path": float(g.velu_point_maps.median()),
                                   "field_muls_model_per_path": float(g.field_muls_model.median())}
        out["by_volcano_path_len"][V] = per
        # transport/solve fraction: transport ns / median solve wall of the arm at the destination level
        fr = {}
        for arm in ARMS_MAIN:
            y = d[(d.volcano == V) & d.family.isin(ISO) & (d.arm == arm)]
            medwall = y.groupby("level").solve_wall_ns.median()
            meangae = y.groupby("level").ops_gae.mean()
            fa = {}
            for L, g in t[t.path_len > 0].groupby("level"):
                f = g.ns_mean / medwall[L]
                fa[str(int(L))] = {"median_solve_wall_ns": float(medwall[L]), "fraction_median_q1_q3": [float(f.median()), float(f.quantile(.25)), float(f.quantile(.75))],
                                   "fraction_max": float(f.max()),
                                   "velu_evals_over_mean_ops_gae": {str(p): float((p * g.velu_point_maps / meangae[L]).median()) for p in PRICES}}
            fr[arm] = fa
        out["fraction_of_solve"][V] = fr
    return out


def bsgs_identity(d, V):
    out = {}
    for arm in ["bsgs", "bsgs-neg"]:
        x = d[(d.volcano == V) & (d.arm == arm) & d.family.isin(["D", "Ua", "Ub", "S"])]
        g = x.groupby(["vertex", "target_index", "round"]).agg(n=("ops_gae", "size"), nu=("ops_gae", "nunique"))
        g = g[g.n >= 2]
        out[arm] = {"n_triples": int(len(g)), "fraction_identical_ops_gae": float((g.nu == 1).mean()) if len(g) else None}
    return out


def recovery(d):
    out = {}
    for (V, fam, arm), g in d.groupby(["volcano", "family", "arm"]):
        n, k = len(g), int(g.verified.sum())
        out[f"{V}|{fam}|{arm}"] = {"n": n, "verified": k, "fraction": k / n, "cp95": clopper_pearson(k, n), "flag_any_unverified": bool(k < n)}
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--reps", type=int, default=10000)
    ap.add_argument("--out", default=f"{ROOT}/analysis/results.json")
    ap.add_argument("--only", default=None, help="debug: comma list of stages")
    args = ap.parse_args()
    reps = args.reps
    t0 = time.time()
    d, chk, VERT, specs, done, prog = load()
    depth = {V: max(v["level"] for v in VERT[V]) for V in VERT}
    R = {"schema": "volcano-census.analysis/v1", "reps": reps, "seeds": {"bootstrap": SEED_BOOT, "permutation": SEED_PERM},
         "data_checks": chk, "sessions": len(specs), "depth": depth}
    R["inputs_sha256"] = {os.path.relpath(f, ROOT): sha256(f) for pat in
                          ["runs/extract/*.csv.xz", "runs/uncharged/*.csv.xz", "runs/slugmap/*.json", "census/V*/vertices.jsonl", "census/V*/transport/*.jsonl",
                           "runs/transfer_cost.jsonl", "runs/progress.jsonl", "runs/manifest.json", "ANALYSIS_PLAN.md", "AMENDMENTS.md", "FROZEN.json"]
                          for f in sorted(glob.glob(f"{ROOT}/{pat}"))}
    def log(*a):
        print(f"[{time.time()-t0:6.0f}s]", *a, flush=True)

    # s1: failure rates
    R["recovery"] = recovery(d)
    R["unverified_rows_total"] = int((~d.verified).sum())
    d = d[d.verified].copy()
    log("loaded", len(d), "rows;", chk)

    # s6 first: it gates everything
    crater3 = [v["id"] for v in VERT["V3"] if v["level"] == 0][0]
    R["positive_control"] = positive_control(d, crater3, reps)
    control_ok = R["positive_control"]["pass"]
    log("positive control", R["positive_control"])

    # s3
    R["level_effect"], R["mixed_model"] = {}, {}
    pvals, keys = [], []
    for V in ["V1", "V2"]:
        for arm in ARMS_MAIN:
            le = level_effect(d, V, arm, reps, depth[V])
            R["level_effect"][f"{V}|{arm}"] = le
            for L, r in le["R"].items():
                pvals.append(r["S"]["p_boot"]); keys.append((V, arm, L))
            R["mixed_model"][f"{V}|{arm}"] = mixed_model(d, V, arm)
            log("level effect", V, arm)
    adj = holm(pvals)
    for (V, arm, L), p in zip(keys, adj):
        r = R["level_effect"][f"{V}|{arm}"]["R"][L]["S"]
        r["p_holm"] = p
        r["verdict"] = verdict(r, p, control_ok)
    R["multiplicity"] = {"family_a_tests": len(pvals), "method": "Holm-Bonferroni, alpha 0.05, over V x arm x L (S, bootstrap two-sided p)"}
    # V3 descriptive
    R["v3_descriptive"] = {}
    for arm in ARMS_MAIN + ["rho-cubic"]:
        x = d[(d.volcano == "V3") & (d.arm == arm) & d.family.isin(ISO)]
        if x.empty:
            continue
        g = x.groupby("level").S.agg(["mean", "std", "size"])
        R["v3_descriptive"][arm] = {str(L): {"mean_S": float(r["mean"]), "sd": float(r["std"]), "n_rows": int(r["size"]),
                                             "n_curves": int(x[x.level == L].vertex.nunique()), "ratio_to_crater": float(r["mean"] / g.loc[0, "mean"])}
                                    for L, r in g.iterrows()}

    # s4
    R["curve_effect"] = {}
    pf, kf = [], []
    for V in ["V1", "V2"]:
        for arm in ARMS_MAIN:
            ce, p = curve_effect(d, V, arm, reps)
            R["curve_effect"][f"{V}|{arm}"] = ce
            if p is not None:
                pf.append(float(p)); kf.append(f"{V}|{arm}")
            log("curve effect", V, arm, ce.get("F"), ce.get("n_flagged"))
    for k, p in zip(kf, holm(pf)):
        R["curve_effect"][k]["p_perm_holm"] = p
        R["curve_effect"][k]["variance_ratio_different"] = bool(p < 0.05)

    # s5
    R["bsgs_identity"] = {V: bsgs_identity(d, V) for V in ["V1", "V2", "V3"]}
    R["paired_source"] = {}
    pc, kc = [], []
    for V in ["V1", "V2"]:
        for arm in ["rho-neg", "rho-plain", "ic"]:
            for src in ["Ua", "Ub", "S", "N"]:
                r = paired_source(d, V, arm, src, reps, depth[V])
                R["paired_source"][f"{V}|{arm}|{src}/D"] = r
                pc.append(r["p_boot"]); kc.append(f"{V}|{arm}|{src}/D")
        log("paired", V)
    for k, p in zip(kc, holm(pc)):
        r = R["paired_source"][k]
        r["p_holm"] = p
        r["verdict"] = verdict(r, p, control_ok)
    tc = transfer_cost(d, VERT)
    R["transfer_cost"] = transfer_summary(d, tc)
    R["transport_then_solve"] = {}
    for V in ["V1", "V2", "V3"]:
        for fam in ["D", "Ua", "Ub", "S"]:
            for arm in ARMS_MAIN:
                r = transport_then_solve(d, V, arm, fam, tc, reps, depth[V])
                if r:
                    # negative control: same destinations, same source vertex, native (untransported) instances, no Velu cost
                    c = transport_then_solve(d, V, arm, fam, tc, reps, depth[V], rows_fam="N", zero_E=True)
                    for L, lv in r["levels"].items():
                        if c and L in c["levels"]:
                            cc = c["levels"][L]["prices"]["0.0"]
                            lv["control_native_ratio"] = {"ratio": cc["ratio"], "ci95": cc["ci95"]}
                            tp = lv["prices"]["1.0"]
                            lv["exceeds_control"] = bool(tp["ci95"][1] < cc["ci95"][0] or tp["ci95"][0] > cc["ci95"][1])
                            lv["speedup_signal"] = bool(lv["ci95_excludes_1_all_prices_same_side"] and lv["exceeds_control"] and all(lv["prices"][str(q)]["ci95"][1] < 1 for q in PRICES))
                    R["transport_then_solve"][f"{V}|{fam}|{arm}"] = r
        log("transport", V)

    # s8 / s9
    R["wall_admission"] = {V: wall_admission(d, V, reps) for V in ["V1", "V2", "V3"]}
    R["ic_vs_rho"] = {V: ic_vs_rho(d, V, reps, depth[V]) for V in ["V1", "V2", "V3"]}
    log("ic vs rho done")

    # per-vertex means for diagrams, and curve inventory
    vm = {}
    for V in ["V1", "V2", "V3"]:
        vm[V] = {}
        for arm in ["rho-neg", "ic"]:
            x = d[(d.volcano == V) & (d.arm == arm) & d.family.isin(ISO)]
            g = x.groupby("vertex").agg(S=("S", "mean"), wall=("solve_wall_ns", "mean"), n=("S", "size"))
            vm[V][arm] = {str(k): {"mean_S": float(r.S), "mean_wall_ms": float(r.wall / 1e6), "n_rows": int(r.n)} for k, r in g.iterrows()}
    R["vertex_means"] = vm
    R["inventory"] = {f"{V}|{fam}|{arm}": {"n_rows": int(len(g)), "n_vertices": int(g.vertex.nunique())} for (V, fam, arm), g in d.groupby(["volcano", "family", "arm"])}
    R["runtime_s"] = time.time() - t0
    with open(args.out, "w") as f:
        json.dump(R, f, indent=1, sort_keys=True)
    log("wrote", args.out)


if __name__ == "__main__":
    main()
