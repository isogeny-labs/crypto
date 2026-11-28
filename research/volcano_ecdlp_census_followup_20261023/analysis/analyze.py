#!/usr/bin/env python3
"""Phase C analysis of the volcano ECDLP census follow-up: implements ANALYSIS_PLAN.md s1-s10.

Run:  python3 -I analysis/analyze.py [--reps N] [--out analysis/results.json]

Adapted from the census's analysis/analyze.py (research/volcano_ecdlp_census_20261017/).
Interpretations the plan leaves open are declared in AMENDMENTS.md (A1 item 4, and A3,
written after the data were collected and before the final run) and marked
`interpretation:` in results.json.
Seeds (ANALYSIS_PLAN): level/paired/transfer/IC bootstrap 20261022; curve-effect
permutation and H-T bootstrap 20261023; H-M permutation and effect-size bootstrap
20261024.  Each statistic gets a fresh generator, so every arm of a volcano sees the
same resampling indices.
"""
import argparse, collections, glob, hashlib, io, json, lzma, math, os, sys, time, warnings
import numpy as np
import pandas as pd
from scipy import stats

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SEED_BOOT, SEED_PERM, SEED_HT, SEED_M = 20261022, 20261023, 20261023, 20261024
ARMS_MAIN = ["rho-neg", "rho-plain", "bsgs", "bsgs-neg", "ic"]
ISO = ["D", "Ua", "Ub", "S", "N"]
MARGIN = {"rho-neg": 0.95, "rho-plain": 0.95, "bsgs": 0.95, "bsgs-neg": 0.95, "ic": 0.90}
PRICES = [0.0, 0.1, 1.0]
SRC2FAM = {"D": "D", "floor-a": "Ua", "floor-b": "Ub", "crater-b": "S"}
VOLC = ["V5", "V6"]
ELL = {"V5": 2, "V6": 3}


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""):
            h.update(b)
    return h.hexdigest()


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


def jinv(p, a, b):
    t = 4 * pow(a, 3, p) % p
    return 1728 * t * pow((t + 27 * b * b) % p, -1, p) % p


# ----------------------------------------------------------------- data
def load():
    prog = [json.loads(l) for l in open(f"{ROOT}/runs/progress.jsonl")]
    done = [d for d in prog if d.get("status") == "done" and d.get("audit_ok") is True and "spec" in d]
    names = [os.path.basename(d["out"]) for d in done]
    assert len(set(names)) == len(names), "a session counted twice"
    assert len(set(d["spec"] for d in done)) == len(done), "a spec counted twice"
    frames, unc = [], []
    for s in names:
        frames.append(pd.read_csv(f"{ROOT}/runs/extract/{s}.csv.xz"))
        unc.append(pd.read_csv(f"{ROOT}/runs/uncharged/{s}.csv.xz"))
    d = pd.concat(frames, ignore_index=True)
    u = pd.concat(unc, ignore_index=True)
    n_all = len(d)
    d = d[~d.warmup].copy()
    key = ["spec", "workload_id", "target_index", "arm", "round"]
    assert not d.duplicated(key).any() and not u.duplicated(key).any()
    d = d.merge(u[key + ["total_gae", "s", "unc_total", "unc_detail"]], on=key, how="left", validate="1:1")
    chk = {
        "rows_all_incl_warmup": int(n_all), "rows_measured": int(len(d)),
        "uncharged_merge_missing": int(d.total_gae.isna().sum()),
        "max_abs_S_diff": float((d.S - d.s).abs().max()),
        "max_abs_gae_diff": float((d.ops_gae - d.total_gae).abs().max()),
    }
    J, VL, VERT = {}, {}, {}
    for V in VOLC:
        VERT[V] = [json.loads(l) for l in open(f"{ROOT}/census/{V}/vertices.jsonl")]
        for v in VERT[V]:
            assert (V, v["j"]) not in J, "two census vertices share a j"
            J[(V, v["j"])] = v["id"]
            VL[(V, v["id"])] = v["level"]
    chk["vertices_with_j_0_or_1728"] = {V: [v["id"] for v in VERT[V] if v["j"] in (0, 1728)] for V in VOLC}
    slug2ab, slug2v = {}, {}
    for s in names:
        V = s.split("-")[0]
        for slug, c in json.load(open(f"{ROOT}/runs/slugmap/{s}.json")).items():
            slug2ab[(V, slug)] = (c["a"], c["b"])
            slug2v[(V, slug)] = J[(V, jinv(c["p"], c["a"], c["b"]))]
    # M models: (curve id, model) from the (a, b) of the model's curve
    mm = {}
    for V in VOLC:
        for c in json.load(open(f"{ROOT}/census/{V}/M.json"))["curves"]:
            for m in c["models"]:
                mm[(V, c["id"], m["a"], m["b"])] = m["m"]
    vert, model = [], []
    for V, fam, ch, slug in zip(d.volcano, d.family, d.chunk, d.curve_slug):
        if fam in ("Tcr", "M"):
            v = int(ch)
        else:
            v = slug2v[(V, slug)]
        vert.append(v)
        model.append(mm[(V, v) + slug2ab[(V, slug)]] if fam == "M" else -1)
    d["vertex"], d["model"] = vert, model
    d["vlevel"] = [VL[(v, i)] for v, i in zip(d.volcano, d.vertex)]
    d["level"] = d.level.astype(int)
    chk["level_mismatch_with_session_level"] = int((d.vlevel != d.level).sum())
    chk["tcr_distinct_slugs"] = int(d[d.family == "Tcr"].curve_slug.nunique())
    d["vid"] = d.volcano + ":" + d.vertex.astype(str)
    return d, chk, VERT, names, done, prog


# ------------------------------------------------------- level effect (V5 and V6)
def level_effect_fam(d, V, arm, depth):
    x = d[(d.volcano == V) & (d.arm == arm) & d.family.isin(ISO)]
    out = {}
    pooled0 = x[x.level == 0].S.mean()
    for f in ISO:
        xf = x[x.family == f]
        if xf.empty:
            continue
        m0 = xf[xf.level == 0].S.mean()
        fin = lambda v: float(v) if np.isfinite(v) else None
        out[f] = {"R_vs_own_level0": {str(L): fin(xf[xf.level == L].S.mean() / m0) for L in range(1, depth + 1)},
                  "R_vs_pooled_level0": {str(L): fin(xf[xf.level == L].S.mean() / pooled0) for L in range(1, depth + 1)},
                  "has_level0_rows": bool(len(xf[xf.level == 0]))}
    return out


# ------------------------------------------------------- positive control (V5 crater)
def positive_control(d, crater, reps):
    x = d[(d.volcano == "V5") & (d.vertex == crater) & d.family.isin(["D", "N", "ctl"]) & d.arm.isin(["rho-cubic", "rho-neg"])]
    key = ["spec", "workload_id", "target_index", "round"]
    a = x[x.arm == "rho-cubic"].set_index(key)[["S", "curve_slug"]]
    b = x[x.arm == "rho-neg"].set_index(key)[["S"]]
    j = a.join(b, how="inner", lsuffix="_cubic", rsuffix="_neg").reset_index()
    out = {"vertex": int(crater), "n_pairs": int(len(j)), "n_slugs": int(j.curve_slug.nunique()),
           "families": sorted(x.family.unique().tolist()), "theory_ratio": 1 / math.sqrt(3), "pass_threshold_ci_upper_below": 0.85}
    res = {}
    for name in ["row_bootstrap", "slug_cluster_sensitivity"]:
        cl = np.zeros(len(j), int) if name == "row_bootstrap" else pd.factorize(j.curve_slug)[0]
        sums, cnts, osum, ocnt = boot_sums(np.zeros(len(j), int), cl, j[["S_cubic", "S_neg"]].values, reps, SEED_BOOT)
        rep = sums[:, 0, 0] / sums[:, 0, 1]
        res[name] = {"ratio": float(osum[0, 0] / osum[0, 1]), "ci95": pct(rep, 2.5, 97.5)}
    out.update(res)
    out["pass"] = bool(res["row_bootstrap"]["ci95"][1] < 0.85)
    out["census_V3_for_comparison_not_pooled"] = "census report: mean S(rho-cubic)/mean S(rho-neg) on the V3 crater, see research/volcano_ecdlp_census_20261017/REPORT.md"
    return out


def crater_rho_cubic_gap(d, crater, reps):
    """IC over the strongest rho on the V5 j = 0 crater (rho-cubic), D + N pairs, 0 / 0.1 / 1 GAE; row bootstrap (one curve)."""
    x = d[(d.volcano == "V5") & (d.vertex == crater) & d.family.isin(["D", "N"]) & d.arm.isin(["ic", "rho-cubic"])]
    key = ["spec", "workload_id", "target_index", "round"]
    a = x[x.arm == "ic"].set_index(key)
    b = x[x.arm == "rho-cubic"].set_index(key)
    j = a[["S", "total_gae", "unc_total"]].join(b[["S", "total_gae", "unc_total"]], how="inner", lsuffix="_ic", rsuffix="_rho").reset_index()
    cols = [j.S_ic * (1 + p * j.unc_total_ic / j.total_gae_ic) for p in PRICES] + [j.S_rho * (1 + p * j.unc_total_rho / j.total_gae_rho) for p in PRICES]
    sums, cnts, osum, ocnt = boot_sums(np.zeros(len(j), int), np.zeros(len(j), int), np.column_stack(cols), reps, SEED_BOOT)
    out = {"n_pairs": int(len(j)), "families": ["D", "N"], "prices": {}}
    for pi, p in enumerate(PRICES):
        rep = sums[:, 0, pi] / sums[:, 0, pi + 3]
        out["prices"][str(p)] = {"ratio_ic_over_rho_cubic": float(osum[0, pi] / osum[0, pi + 3]), "ci95": pct(rep, 2.5, 97.5),
                                 "mean_priced_S_ic": float(osum[0, pi] / ocnt[0]), "mean_priced_S_rho_cubic": float(osum[0, pi + 3] / ocnt[0])}
    pr = [out["prices"][str(p)] for p in PRICES]
    out["ranking"] = ("rho-cubic ahead of ic at all three prices (95% CI above 1 at each)" if all(c["ci95"][0] > 1 for c in pr) else
                      "ic ahead of rho-cubic at all three prices" if all(c["ci95"][1] < 1 for c in pr) else "order not established across the band")
    return out


def crater_ic_by_model(d, crater):
    """V5 crater: IC (and rho-neg, rho-cubic) mean S on every Weierstrass model present (D/N model, ctl re-scalings) - the isomorphic control for census F2."""
    out = {}
    for arm in ["ic", "rho-neg", "rho-cubic"]:
        x = d[(d.volcano == "V5") & (d.vertex == crater) & (d.arm == arm) & d.family.isin(["D", "N", "ctl"])]
        out[arm] = {slug: {"families": sorted(g.family.unique().tolist()), "n_rows": int(len(g)), "mean_S": float(g.S.mean()),
                           "se": float(g.S.std(ddof=1) / np.sqrt(len(g)))} for slug, g in x.groupby("curve_slug")}
    return out


# ------------------------------------------------------- IC against matched rho (s9)
def ic_vs_rho(d, V, reps, depth):
    """PLAN s9: per level mean S(ic)/mean S(rho-neg) at 0 / 0.1 / 1 GAE per uncharged operation (V5 crater: see crater_rho_cubic_gap)."""
    x = d[(d.volcano == V) & d.family.isin(ISO) & d.arm.isin(["ic", "rho-neg"])]
    key = ["spec", "workload_id", "target_index", "round"]
    a = x[x.arm == "ic"].set_index(key)
    b = x[x.arm == "rho-neg"].set_index(key)
    j = a[["vertex", "level", "S", "total_gae", "unc_total"]].join(b[["S", "total_gae", "unc_total"]], how="inner", lsuffix="_ic", rsuffix="_rho").reset_index()
    cols = [j.S_ic * (1 + p * j.unc_total_ic / j.total_gae_ic) for p in PRICES] + [j.S_rho * (1 + p * j.unc_total_rho / j.total_gae_rho) for p in PRICES]
    sums, cnts, osum, ocnt = boot_sums(j.level.values, j.vertex.values, np.column_stack(cols), reps, SEED_BOOT)
    out = {"n_pairs": int(len(j)), "levels": {}, "pooled": {}}
    for L in range(depth + 1):
        lv = {"n_curves": int(j[j.level == L].vertex.nunique()), "n_pairs": int((j.level == L).sum()), "prices": {}}
        for pi, p in enumerate(PRICES):
            rep = sums[:, L, pi] / sums[:, L, pi + 3]
            lv["prices"][str(p)] = {"ratio_ic_over_rho": float(osum[L, pi] / osum[L, pi + 3]), "ci95": pct(rep, 2.5, 97.5),
                                    "mean_S_ic": float(osum[L, pi] / ocnt[L]), "mean_S_rho": float(osum[L, pi + 3] / ocnt[L])}
        pr = [lv["prices"][str(p)] for p in PRICES]
        lv["ranking"] = ("rho-neg ahead of ic at all three prices (95% CI above 1 at each)" if all(c["ci95"][0] > 1 for c in pr) else
                         "ic ahead of rho-neg at all three prices (95% CI below 1 at each)" if all(c["ci95"][1] < 1 for c in pr) else
                         "order not established across the band")
        if V == "V5" and L == 0:
            lv["matched_rho_note"] = "j = 0 crater: the matched, strongest rho is rho-cubic; use crater_rho_cubic, not this rho-neg ratio"
        out["levels"][str(L)] = lv
    lo = 1 if V == "V5" else 0
    for tag, rng_ in [("pooled", range(0, depth + 1)), ("pooled_levels_ge1", range(1, depth + 1))]:
        if tag == "pooled_levels_ge1" and V != "V5":
            continue
        out[tag] = {}
        for pi, p in enumerate(PRICES):
            rep = sums[:, list(rng_), pi].sum(1) / sums[:, list(rng_), pi + 3].sum(1)
            obs = osum[list(rng_), pi].sum() / osum[list(rng_), pi + 3].sum()
            out[tag][str(p)] = {"ratio_ic_over_rho": float(obs), "ci95": pct(rep, 2.5, 97.5)}
    u = {}
    for arm in ["ic", "rho-neg"]:
        y = d[(d.volcano == V) & d.family.isin(ISO) & (d.arm == arm)]
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


def ic_vs_rho_M(d, reps):
    """Descriptive: per M curve, mean priced S(ic)/mean priced S(rho-neg) over its 8 models (row bootstrap)."""
    out = {}
    key = ["spec", "workload_id", "target_index", "round"]
    for (V, v), g in d[d.family == "M"].groupby(["volcano", "vertex"]):
        a = g[g.arm == "ic"].set_index(key)
        b = g[g.arm == "rho-neg"].set_index(key)
        j = a[["S", "total_gae", "unc_total", "level"]].join(b[["S", "total_gae", "unc_total"]], how="inner", lsuffix="_ic", rsuffix="_rho").reset_index()
        cols = [j.S_ic * (1 + p * j.unc_total_ic / j.total_gae_ic) for p in PRICES] + [j.S_rho * (1 + p * j.unc_total_rho / j.total_gae_rho) for p in PRICES]
        sums, cnts, osum, ocnt = boot_sums(np.zeros(len(j), int), np.zeros(len(j), int), np.column_stack(cols), reps, SEED_BOOT)
        pr = {}
        for pi, p in enumerate(PRICES):
            rep = sums[:, 0, pi] / sums[:, 0, pi + 3]
            pr[str(p)] = {"ratio_ic_over_rho_neg": float(osum[0, pi] / osum[0, pi + 3]), "ci95": pct(rep, 2.5, 97.5)}
        out[f"{V}:{v}"] = {"level": int(j.level.iloc[0]), "n_pairs": int(len(j)), "prices": pr,
                           "note": "matched rho on the V5 crater is rho-cubic (not run on M specs); this ratio is against rho-neg" if (V == "V5" and v == 0) else ""}
    return out


# ------------------------------------------------------- transfer cost
def transfer_cost(VERT):
    tc = pd.DataFrame([json.loads(l) for l in open(f"{ROOT}/runs/transfer_cost.jsonl")])
    tcr = tc[tc.src.str.startswith("tcr-v")].copy()
    tc = tc[~tc.src.str.startswith("tcr-v")].copy()
    tc["fam"] = tc.src.map(SRC2FAM)
    rows = []
    for V in VOLC:
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
    return tc, tcr


def transfer_summary(d, tc):
    out = {"all_matches_transported": bool(tc.matches_transported.all()), "n_rows": int(len(tc)), "by_volcano_path_len": {}, "fraction_of_solve": {}}
    for V in VOLC:
        t = tc[tc.volcano == V]
        per = {}
        for dlen, g in t[t.path_len > 0].groupby("path_len"):
            q = lambda s: [float(s.median()), float(s.quantile(0.25)), float(s.quantile(0.75))]
            per[str(int(dlen))] = {"n_paths": int(len(g)), "ns_per_edge_median_q1_q3": q(g.ns_per_edge_mean),
                                   "ns_per_path_median_q1_q3": q(g.ns_mean), "velu_point_maps_per_path": float(g.velu_point_maps.median()),
                                   "field_muls_model_per_path": float(g.field_muls_model.median())}
        out["by_volcano_path_len"][V] = per
        fr = {}
        for arm in ARMS_MAIN:
            y = d[(d.volcano == V) & d.family.isin(ISO) & (d.arm == arm)]
            medwall = y.groupby("level").solve_wall_ns.median()
            meangae = y.groupby("level").ops_gae.mean()
            fa = {}
            for L, g in t[t.path_len > 0].groupby("level"):
                if L not in medwall.index:
                    continue
                f = g.ns_mean / medwall[L]
                fa[str(int(L))] = {"median_solve_wall_ns": float(medwall[L]), "fraction_median_q1_q3": [float(f.median()), float(f.quantile(.25)), float(f.quantile(.75))],
                                   "fraction_max": float(f.max()),
                                   "velu_evals_over_mean_ops_gae": {str(p): float((p * g.velu_point_maps / meangae[L]).median()) for p in PRICES}}
            fr[arm] = fa
        out["fraction_of_solve"][V] = fr
    return out


# ------------------------------------------------------- H-T (s5b)
def ht_arrays(d, tcr_tc):
    """Per source vertex v: array of (target_index, round) pairs x 11 columns:
    0-2 priced ops_gae of rho-cubic on Tcr_v at p = 0, 0.1, 1; 3-5 rho-neg on Tcr_v; 6-8 rho-neg on N_v; 9 S(rho-cubic, Tcr_v); 10 S(rho-neg, Tcr_v)."""
    t = d[(d.volcano == "V5") & (d.family == "Tcr")]
    n = d[(d.volcano == "V5") & (d.family == "N") & (d.arm == "rho-neg")]
    key = ["target_index", "round"]
    A, level, Mv, chk = {}, {}, {}, {"unpaired_rows": 0, "path_check_failures": []}
    meta = {int(r.tcr_source_id): r for r in tcr_tc.itertuples()}
    for v in sorted(t.vertex.unique()):
        a = t[(t.vertex == v) & (t.arm == "rho-cubic")].set_index(key)
        b = t[(t.vertex == v) & (t.arm == "rho-neg")].set_index(key)
        c = n[n.vertex == v].set_index(key)
        j = a[["ops_gae", "unc_total", "S"]].join(b[["ops_gae", "unc_total", "S"]], how="inner", lsuffix="_c", rsuffix="_t").join(
            c[["ops_gae", "unc_total"]].rename(columns={"ops_gae": "ops_gae_n", "unc_total": "unc_total_n"}), how="inner")
        chk["unpaired_rows"] += int(max(len(a), len(b), len(c)) - len(j))
        cols = [j.ops_gae_c + p * j.unc_total_c for p in PRICES] + [j.ops_gae_t + p * j.unc_total_t for p in PRICES] + \
               [j.ops_gae_n + p * j.unc_total_n for p in PRICES] + [j.S_c, j.S_t]
        A[int(v)] = np.column_stack(cols).astype(float)
        level[int(v)] = int(a.level.iloc[0])
        m = meta[int(v)]
        Mv[int(v)] = float(m.velu_point_maps) + 2.0
        pl = json.load(open(f"{ROOT}/census/V5/tcr/{int(v)}.json"))
        if pl["path_len_to_crater"] != int(m.path_len) + 1 or 2 * pl["path_len_to_crater"] != Mv[int(v)]:
            chk["path_check_failures"].append(int(v))
    return A, level, Mv, chk


def _ht_stats(cm, mv, den_n):
    """cm: (..., k, 11) per-vertex means; mv: (..., k); returns dict of ratio arrays over the k vertices averaged."""
    out = {}
    cub, negt, nat = cm[..., 0:3].mean(-2), cm[..., 3:6].mean(-2), cm[..., 6:9].mean(-2)
    mbar = mv.mean(-1)[..., None]
    p = np.array(PRICES)
    out["T"] = (cub + p * mbar) / nat
    out["T0"] = (negt + p * mbar) / nat
    out["cubic_over_neg_S"] = cm[..., 9].mean(-1) / cm[..., 10].mean(-1)
    out["transport_share_T"] = p * mbar / (cub + p * mbar)
    return out


def hypothesis_T(d, tcr_tc, reps):
    A, level, Mv, chk = ht_arrays(d, tcr_tc)
    levels = sorted(set(level.values()))
    verts = {L: [v for v in sorted(A) if level[v] == L] for L in levels}
    npairs = {L: A[verts[L][0]].shape[0] for L in levels}
    assert all(A[v].shape[0] == npairs[L] for L in levels for v in verts[L])
    vm = {v: A[v].mean(0) for v in A}                       # per-vertex observed means (11)
    p = np.array(PRICES)

    def pooled(vs):
        cm = np.stack([vm[v] for v in vs])[None]
        mv = np.array([Mv[v] for v in vs])[None]
        return {k: x[0] for k, x in _ht_stats(cm, mv, None).items()}

    obs_all = pooled(sorted(A))
    obs_lvl = {L: pooled(verts[L]) for L in levels}
    # vertex-weighted overall bootstrap: stratified by level, vertices then (target,round) pairs jointly
    rng = np.random.default_rng(SEED_HT)
    nv_tot = sum(len(verts[L]) for L in levels)
    rep_all = {k: [] for k in ["T", "T0", "cubic_over_neg_S", "transport_share_T"]}
    rep_lvl = {L: {k: [] for k in rep_all} for L in levels}
    done, chunk = 0, 100
    Aarr = {L: np.stack([A[v] for v in verts[L]]) for L in levels}            # (m, n, 11)
    Marr = {L: np.array([Mv[v] for v in verts[L]]) for L in levels}
    while done < reps:
        c = min(chunk, reps - done)
        sum_cm = np.zeros((c, 11))
        sum_mv = np.zeros(c)
        for L in levels:
            m, n = Aarr[L].shape[:2]
            vi = rng.integers(0, m, (c, m))
            pi = rng.integers(0, n, (c, m, n))
            vals = Aarr[L][vi[:, :, None], pi].mean(2)                         # (c, m, 11) per chosen vertex mean over resampled pairs
            mvv = Marr[L][vi]                                                   # (c, m)
            st = _ht_stats(vals, mvv, None)
            for k in rep_lvl[L]:
                rep_lvl[L][k].append(st[k])
            sum_cm += vals.sum(1)
            sum_mv += mvv.sum(1)
        cm = (sum_cm / nv_tot)[:, None, :]
        st = _ht_stats(cm, (sum_mv / nv_tot)[:, None], None)
        for k in rep_all:
            rep_all[k].append(st[k])
        done += c
    rep_all = {k: np.concatenate(v) for k, v in rep_all.items()}
    rep_lvl = {L: {k: np.concatenate(v) for k, v in r.items()} for L, r in rep_lvl.items()}
    ci = lambda a: [pct(a[:, i], 2.5, 97.5) for i in range(a.shape[1])]
    res = {"n_vertices": nv_tot, "n_pairs_per_vertex_by_level": {str(L): int(npairs[L]) for L in levels}, "checks": chk,
           "interpretation": "overall and per-level pooled values are ratios of vertex-averaged means (A1 4d, A3.2); the bootstrap draws vertices within level, then (target_index, round) pairs jointly across Tcr_v rho-cubic, Tcr_v rho-neg and N_v rho-neg",
           "theory_ratio": 1 / math.sqrt(3), "prices": PRICES, "overall": {}, "by_level": {}, "by_vertex": {}}
    def pack(obs, rep):
        o = {}
        for k, nm in [("T", "T"), ("T0", "T0"), ("transport_share_T", "transport_share_of_T_numerator")]:
            c_ = ci(rep[k])
            o[nm] = {str(PRICES[i]): {"value": float(obs[k][i]), "ci95": c_[i], "boot_sd": float(rep[k][:, i].std(ddof=1))} for i in range(3)}
        o["cubic_over_neg_S_on_Tcr"] = {"value": float(obs["cubic_over_neg_S"]), "ci95": pct(rep["cubic_over_neg_S"], 2.5, 97.5)}
        return o
    res["overall"] = pack(obs_all, rep_all)
    for L in levels:
        r = pack(obs_lvl[L], rep_lvl[L])
        r["n_vertices"] = len(verts[L])
        res["by_level"][str(L)] = r
    # verdict (PLAN s4, ANALYSIS_PLAN s5b)
    T, T0 = res["overall"]["T"], res["overall"]["T0"]
    below = all(T[str(q)]["ci95"][1] < 1 for q in PRICES)
    ctl_incl = all(T0[str(q)]["ci95"][0] <= 1 <= T0[str(q)]["ci95"][1] for q in PRICES)
    refuted = any(T[str(q)]["ci95"][1] >= 1 for q in PRICES)
    res["verdict"] = "supported" if (below and ctl_incl) else "refuted" if refuted else "inconclusive"
    res["verdict_inputs"] = {"T_ci_upper_below_1_all_prices": bool(below), "T0_ci_includes_1_all_prices": bool(ctl_incl), "T_ci_includes_or_above_1_any_price": bool(refuted)}
    # per-vertex T_v and T0_v with pair-bootstrap CIs (descriptive)
    for v in sorted(A):
        r = np.random.default_rng(np.random.SeedSequence([SEED_HT, v]))
        n = A[v].shape[0]
        idx = r.integers(0, n, (reps, n))
        bm = np.concatenate([A[v][idx[i0:i0 + 1000]].mean(1) for i0 in range(0, reps, 1000)])
        mv_ = Mv[v]
        T_ = (bm[:, 0:3] + p * mv_) / bm[:, 6:9]
        T0_ = (bm[:, 3:6] + p * mv_) / bm[:, 6:9]
        o = vm[v]
        res["by_vertex"][str(v)] = {"level": level[v], "velu_point_maps": Mv[v],
                                    "T": {str(PRICES[i]): {"value": float((o[i] + PRICES[i] * mv_) / o[6 + i]), "ci95": pct(T_[:, i], 2.5, 97.5)} for i in range(3)},
                                    "T0": {str(PRICES[i]): {"value": float((o[3 + i] + PRICES[i] * mv_) / o[6 + i]), "ci95": pct(T0_[:, i], 2.5, 97.5)} for i in range(3)}}
    return res


# ------------------------------------------------------- H-M (s5c)
def anova_F(y, g, k):
    """One-way ANOVA F on responses y with group codes g in 0..k-1 (vectorised over leading axis of y)."""
    n = y.shape[-1]
    gm = y.mean(-1, keepdims=True)
    oh = (g[None, :] == np.arange(k)[:, None])           # (k, n)
    cnt = oh.sum(1)
    means = (y @ oh.T) / cnt                              # (..., k)
    ssb = (cnt * (means - gm) ** 2).sum(-1)
    ssw = ((y - means[..., g]) ** 2).sum(-1)
    msb, msw = ssb / (k - 1), ssw / (n - k)
    with np.errstate(divide="ignore", invalid="ignore"):
        return msb / msw, ssw


def hypothesis_M(d, reps):
    res = {"curves": {}, "arms": ["rho-neg", "bsgs", "ic"], "alpha": 0.05, "n_perm": reps}
    pvals = {a: [] for a in res["arms"]}
    keys = []
    for (V, v), g in sorted(d[d.family == "M"].groupby(["volcano", "vertex"])):
        key = f"{V}:{v}"
        keys.append(key)
        cur = {"volcano": V, "vertex": int(v), "level": int(g.level.iloc[0]), "arms": {}}
        for arm in res["arms"]:
            x = g[g.arm == arm].sort_values(["model", "workload_id", "target_index", "round"])
            y = x.S.values.astype(float)
            gcode = x.model.values.astype(int)
            k = len(np.unique(gcode))
            ngroup = np.bincount(gcode)
            r = {"n_models": int(k), "rows_per_model": [int(c) for c in ngroup], "model_mean_S": [float(y[gcode == m].mean()) for m in range(k)],
                 "model_se_S": [float(y[gcode == m].std(ddof=1) / np.sqrt((gcode == m).sum())) for m in range(k)]}
            F, ssw = anova_F(y[None, :], gcode, k)
            F, ssw = float(F[0]), float(ssw[0])
            if ssw == 0 or not np.isfinite(F):
                r.update({"F": None, "p_perm": None, "degenerate": "zero within-model variance: no F reported, counted not significant"})
                pvals[arm].append(None)
            else:
                rng = np.random.default_rng(SEED_M)
                ge = 0
                for i0 in range(0, reps, 500):
                    c = min(500, reps - i0)
                    perm = np.argsort(rng.random((c, len(y))), axis=1)
                    Fs, _ = anova_F(y[perm], gcode, k)
                    ge += int(np.sum(Fs >= F))
                r.update({"F": F, "p_perm": (1 + ge) / (1 + reps)})
                pvals[arm].append(r["p_perm"])
            # effect size: max/min ratio of model means, bootstrap rows within each model on a separate generator (seed 20261024)
            mm_ = np.array(r["model_mean_S"])
            rng2 = np.random.default_rng(SEED_M)
            bm = np.empty((reps, k))
            for m in range(k):
                ym = y[gcode == m]
                bm[:, m] = ym[rng2.integers(0, len(ym), (reps, len(ym)))].mean(1)
            rat = bm.max(1) / bm.min(1)
            r["max_over_min_model_mean"] = {"value": float(mm_.max() / mm_.min()), "ci95": pct(rat, 2.5, 97.5)}
            cur["arms"][arm] = r
        res["curves"][key] = cur
    res["summary"] = {}
    for arm in res["arms"]:
        idx = [i for i, p_ in enumerate(pvals[arm]) if p_ is not None]
        adj = holm([pvals[arm][i] for i in idx]) if idx else []
        # Holm over the curves with a defined p; degenerate curves count as not significant (they are not tested)
        nsig = 0
        for i, a_ in zip(idx, adj):
            res["curves"][keys[i]]["arms"][arm]["p_holm"] = a_
            res["curves"][keys[i]]["arms"][arm]["significant"] = bool(a_ < 0.05)
            nsig += int(a_ < 0.05)
        for i, k_ in enumerate(keys):
            if pvals[arm][i] is None:
                res["curves"][k_]["arms"][arm]["significant"] = False
        res["summary"][arm] = {"n_curves": len(keys), "n_significant_holm": nsig, "n_tested": len(idx)}
    s = res["summary"]
    half = len(keys) / 2
    if s["ic"]["n_significant_holm"] >= half and s["rho-neg"]["n_significant_holm"] <= 1 and s["bsgs"]["n_significant_holm"] <= 1:
        res["verdict"] = "supported for IC"
    elif s["ic"]["n_significant_holm"] <= 1:
        res["verdict"] = "refuted"
    else:
        res["verdict"] = "inconclusive"
    res["motivating_observation_not_pooled"] = "census V3 crater, IC mean S 57 / 67 / 73 on three isomorphic models (census A3 F2)"
    # BSGS check that the models carry the same instances: per-curve identical k_i -> identical S per (target, round)?
    chk = {}
    for (V, v), g in d[(d.family == "M") & (d.arm == "bsgs")].groupby(["volcano", "vertex"]):
        gg = g.groupby(["target_index", "round"]).ops_gae.nunique()
        chk[f"{V}:{v}"] = float((gg == 1).mean())
    res["bsgs_same_instance_check_fraction_identical_ops_gae_across_models"] = chk
    return res


# ------------------------------------------------------- main
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--reps", type=int, default=10000)
    ap.add_argument("--out", default=f"{ROOT}/analysis/results.json")
    args = ap.parse_args()
    reps = args.reps
    t0 = time.time()
    d, chk, VERT, names, done, prog = load()
    depth = {V: max(v["level"] for v in VERT[V]) for V in VERT}
    R = {"schema": "volcano-census-followup.analysis/v1", "reps": reps,
         "seeds": {"bootstrap": SEED_BOOT, "permutation_curve_effect": SEED_PERM, "H-T_bootstrap": SEED_HT, "H-M_permutation_and_effect_bootstrap": SEED_M},
         "data_checks": chk, "sessions": len(names), "depth": depth}
    R["inputs_sha256"] = {os.path.relpath(f, ROOT): sha256(f) for pat in
                          ["runs/extract/*.csv.xz", "runs/uncharged/*.csv.xz", "runs/slugmap/*.json", "census/V*/vertices.jsonl", "census/V*/transport/*.jsonl",
                           "census/V*/M.json", "census/V5/tcr/*.json", "runs/transfer_cost.jsonl", "runs/progress.jsonl", "runs/manifest.json",
                           "ANALYSIS_PLAN.md", "AMENDMENTS.md", "analysis/extract_uncharged.py"]
                          for f in sorted(glob.glob(f"{ROOT}/{pat}"))}
    R["input_notes"] = {"redone_sessions": [os.path.basename(x["out"]) for x in done if x["out"].endswith(".r2")],
                        "orphan_archives_not_analysed": sorted(os.path.basename(f) for f in glob.glob(f"{ROOT}/runs/orphans/*.tar.xz")),
                        "busy_host_refusal_lines_ignored": int(sum(1 for p in prog if p.get("status") == "failed")),
                        "session_wall_s_done": int(sum(x["wall_s"] for x in done)),
                        "pause_lines": int(sum(1 for p in prog if p.get("type") == "pause"))}
    def log(*a):
        print(f"[{time.time()-t0:6.0f}s]", *a, flush=True)

    R["recovery"] = recovery(d)
    R["unverified_rows_total"] = int((~d.verified).sum())
    R["isolation_fraction_L2"] = {V: float((d[d.volcano == V].isolation == "L2").mean()) for V in VOLC}
    d = d[d.verified].copy()
    log("loaded", len(d), "rows;", chk)

    crater5 = [v["id"] for v in VERT["V5"] if v["level"] == 0][0]
    R["positive_control"] = positive_control(d, crater5, reps)
    control_ok = R["positive_control"]["pass"]
    log("positive control", R["positive_control"]["row_bootstrap"])

    R["level_effect"], R["mixed_model"] = {}, {}
    pvals, keys = [], []
    for V in VOLC:
        for arm in ARMS_MAIN:
            le = level_effect(d, V, arm, reps, depth[V])
            le["R_by_family_descriptive"] = level_effect_fam(d, V, arm, depth[V])
            R["level_effect"][f"{V}|{arm}"] = le
            for L, r in le["R"].items():
                if V == "V5" and int(L) <= 3:
                    r["S"]["verdict"] = "descriptive"
                    r["S"]["p_holm"] = None
                    continue
                pvals.append(r["S"]["p_boot"]); keys.append((V, arm, L))
            R["mixed_model"][f"{V}|{arm}"] = mixed_model(d, V, arm)
            log("level effect", V, arm)
    adj = holm(pvals)
    for (V, arm, L), p in zip(keys, adj):
        r = R["level_effect"][f"{V}|{arm}"]["R"][L]["S"]
        r["p_holm"] = p
        r["verdict"] = verdict(r, p, control_ok)
    R["multiplicity"] = {"family_a_tests": len(pvals), "method": "Holm-Bonferroni, alpha 0.05, over V x arm x L (S, bootstrap two-sided p); V6 levels 1-4, V5 levels 4-6 only"}
    R["v5_descriptive_rho_cubic"] = {}
    x = d[(d.volcano == "V5") & (d.arm == "rho-cubic") & d.family.isin(ISO)]
    if not x.empty:
        g = x.groupby("level").S.agg(["mean", "std", "size"])
        R["v5_descriptive_rho_cubic"] = {str(L): {"mean_S": float(r["mean"]), "n_rows": int(r["size"])} for L, r in g.iterrows()}

    R["curve_effect"] = {}
    pf, kf = [], []
    for arm in ARMS_MAIN:
        ce, p = curve_effect(d, "V6", arm, reps)
        R["curve_effect"][f"V6|{arm}"] = ce
        if p is not None:
            pf.append(float(p)); kf.append(f"V6|{arm}")
        log("curve effect", arm, ce.get("F"), ce.get("n_flagged"))
    for k, p in zip(kf, holm(pf)):
        R["curve_effect"][k]["p_perm_holm"] = p
        R["curve_effect"][k]["variance_ratio_different"] = bool(p < 0.05)

    R["bsgs_identity"] = {V: bsgs_identity(d, V) for V in VOLC}
    R["paired_source"] = {}
    pc, kc = [], []
    for arm in ["rho-neg", "rho-plain", "ic"]:
        for src in ["Ua", "Ub", "S", "N"]:
            r = paired_source(d, "V6", arm, src, reps, depth["V6"])
            R["paired_source"][f"V6|{arm}|{src}/D"] = r
            pc.append(r["p_boot"]); kc.append(f"V6|{arm}|{src}/D")
    for k, p in zip(kc, holm(pc)):
        r = R["paired_source"][k]
        r["p_holm"] = p
        r["verdict"] = verdict(r, p, control_ok)
    log("paired")
    tc, tcr_tc = transfer_cost(VERT)
    R["transfer_cost"] = transfer_summary(d, tc)
    R["transport_then_solve"] = {}
    for fam in ["D", "Ua", "Ub", "S"]:
        for arm in ARMS_MAIN:
            r = transport_then_solve(d, "V6", arm, fam, tc, reps, depth["V6"])
            if r:
                c = transport_then_solve(d, "V6", arm, fam, tc, reps, depth["V6"], rows_fam="N", zero_E=True)
                for L, lv in r["levels"].items():
                    if c and L in c["levels"]:
                        cc = c["levels"][L]["prices"]["0.0"]
                        lv["control_native_ratio"] = {"ratio": cc["ratio"], "ci95": cc["ci95"]}
                        tp = lv["prices"]["1.0"]
                        lv["exceeds_control"] = bool(tp["ci95"][1] < cc["ci95"][0] or tp["ci95"][0] > cc["ci95"][1])
                        lv["speedup_signal"] = bool(lv["ci95_excludes_1_all_prices_same_side"] and lv["exceeds_control"] and all(lv["prices"][str(q)]["ci95"][1] < 1 for q in PRICES))
                R["transport_then_solve"][f"V6|{fam}|{arm}"] = r
    log("transport")

    R["H-T"] = hypothesis_T(d, tcr_tc, reps)
    log("H-T", R["H-T"]["verdict"])
    R["H-M"] = hypothesis_M(d, reps)
    log("H-M", R["H-M"]["verdict"], R["H-M"]["summary"])

    R["wall_admission"] = {V: wall_admission(d, V, reps) for V in VOLC}
    R["ic_vs_rho"] = {V: ic_vs_rho(d, V, reps, depth[V]) for V in VOLC}
    R["ic_vs_rho_cubic_crater"] = crater_rho_cubic_gap(d, crater5, reps)
    R["ic_vs_rho_M_descriptive"] = ic_vs_rho_M(d, reps)
    R["crater_models_V5"] = crater_ic_by_model(d, crater5)
    log("ic vs rho done")

    vm = {}
    for V in VOLC:
        vm[V] = {}
        for arm in ["rho-neg", "ic"]:
            x = d[(d.volcano == V) & (d.arm == arm) & d.family.isin(ISO)]
            g = x.groupby("vertex").agg(S=("S", "mean"), wall=("solve_wall_ns", "mean"), n=("S", "size"))
            vm[V][arm] = {str(k): {"mean_S": float(r.S), "mean_wall_ms": float(r.wall / 1e6), "n_rows": int(r.n)} for k, r in g.iterrows()}
    R["vertex_means"] = vm
    R["inventory"] = {f"{V}|{fam}|{arm}": {"n_rows": int(len(g)), "n_vertices": int(g.vertex.nunique())} for (V, fam, arm), g in d.groupby(["volcano", "family", "arm"])}
    R["runtime_s"] = time.time() - t0
    with open(args.out, "w") as f:
        json.dump(R, f, indent=1, sort_keys=True, allow_nan=False)
    log("wrote", args.out)


if __name__ == "__main__":
    main()
