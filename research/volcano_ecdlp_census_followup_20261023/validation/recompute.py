#!/usr/bin/env python3
"""Validator recompute of the follow-up study's headline numbers.

Written by the validator from ANALYSIS_PLAN.md and AMENDMENTS.md only; it does
not import, read or run analysis/*.py, and it reads none of the producer's
derived files (runs/slugmap, analysis/results.json). Input: the validator's own
raw-record re-derivation (rederive.py -> ROWS_DIR) mapped to census vertices by
vfu_data.py, plus census/ and runs/transfer_cost.jsonl.

Bootstraps and permutations use this file's own generators (numpy PCG64 with
the plan's seeds, but a different draw order from the producer's), so CI
endpoints and permutation p-values agree with the producer's only up to Monte
Carlo error; point estimates and F statistics are deterministic and must agree
to rounding.

Usage: python3 -I recompute.py STUDY_DIR ROWS_DIR OUT_JSON
"""
import json
import math
import os
import sys

import numpy as np
from scipy import stats

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vfu_data  # noqa: E402

STUDY, ROWS, OUT = sys.argv[1:4]
B = 10000
PRICES = (0.0, 0.1, 1.0)
ISO = {"V5": ["D", "Ua", "Ub", "N"], "V6": ["D", "Ua", "Ub", "S", "N"]}
ARMS5 = ["rho-neg", "rho-plain", "bsgs", "bsgs-neg", "ic"]
OUTD = {}


def put(key, val):
    OUTD[key] = val


def pct(a, q):
    return [float(np.percentile(a, q[0])), float(np.percentile(a, q[1]))]


def holm(ps):
    """Holm-adjusted p-values, input order preserved."""
    idx = sorted(range(len(ps)), key=lambda i: ps[i])
    m = len(ps)
    adj = [0.0] * m
    run = 0.0
    for rank, i in enumerate(idx):
        run = max(run, min(1.0, (m - rank) * ps[i]))
        adj[i] = run
    return adj


df, verts, tcr, checks = vfu_data.load(STUDY, ROWS)
put("data_checks", checks)
df["S"] = df["s"].astype(float)
df["gae"] = df["total_gae"].astype(float)
df["unc"] = df["unc"].astype(float)
df["verified"] = df["status"] == "verified"

# ------------------------------------------------------------------ counts
put("counts.rows_all_incl_warmup", checks["rows_all"])
put("counts.rows_measured", checks["rows_measured"])
put("counts.rows_warmup", checks["rows_warmup"])
put("counts.unverified_measured", int((~df["verified"]).sum()))
put("counts.scalar_check_failures_measured", int((df["scalar_ok"] == 0).sum()))
cells = df.groupby(["V", "fam", "arm"]).agg(n=("verified", "size"), k=("verified", "sum"))
put("counts.recovery_cells", int(len(cells)))
put("counts.recovery_all_verified", bool((cells["n"] == cells["k"]).all()))
nmin = int(cells["n"].min())
put("counts.recovery_smallest_cell_n", nmin)
put("counts.recovery_smallest_cp_lower95", float(stats.beta.ppf(0.025, nmin, 1)))
put("counts.sessions", int(df["session"].nunique()))
sess_fam = df.groupby(["V", "fam"])["spec"].nunique()
put("counts.sessions_by_V_family", {f"{a}|{b}": int(v) for (a, b), v in sess_fam.items()})

# ------------------------------------------------------- strongest rho facts
p5 = verts["V5"][0]["p"]
r5 = verts["V5"][0]["r"]
p6 = verts["V6"][next(iter(verts["V6"]))]["p"]
put("rho_reference.V5_p_mod_3", p5 % 3)
put("rho_reference.V5_r_mod_3", r5 % 3)
put("rho_reference.V5_crater_j", verts["V5"][0]["j"])
put("rho_reference.V6_p_mod_3", p6 % 3)
put("rho_reference.j0_or_1728_vertices", {"V5": checks["V5_j0_or_1728_vertices"],
                                          "V6": checks["V6_j0_or_1728_vertices"]})

# --------------------------------------------------------- positive control
rng = np.random.default_rng(20261022)
pc = df[(df["V"] == "V5") & (df["vertex"] == 0) & df["fam"].isin(["D", "N", "ctl"])
        & df["arm"].isin(["rho-cubic", "rho-neg"])]
piv = pc.pivot_table(index=["slug", "wid", "tidx", "round"], columns="arm", values="S", aggfunc="first").dropna()
xc, xn = piv["rho-cubic"].to_numpy(), piv["rho-neg"].to_numpy()
put("positive_control.n_pairs", int(len(piv)))
put("positive_control.n_slugs", int(piv.index.get_level_values(0).nunique()))
put("positive_control.n_rows_cubic", int((pc["arm"] == "rho-cubic").sum()))
put("positive_control.ratio", float(pc[pc.arm == "rho-cubic"]["S"].mean() / pc[pc.arm == "rho-neg"]["S"].mean()))
reps = np.empty(B)
n = len(xc)
for i0 in range(0, B, 500):
    ii = rng.integers(0, n, size=(min(500, B - i0), n))
    reps[i0:i0 + len(ii)] = xc[ii].sum(1) / xn[ii].sum(1)
put("positive_control.ci95", pct(reps, (2.5, 97.5)))
put("positive_control.boot_sd", float(reps.std(ddof=1)))
put("positive_control.pass", bool(np.percentile(reps, 97.5) < 0.85))
# slug-cluster sensitivity: slugs with replacement, then pairs within slug
slugs = piv.index.get_level_values(0).to_numpy()
us = sorted(set(slugs))
grp = [np.where(slugs == s)[0] for s in us]
reps2 = np.empty(B)
for b in range(B):
    ch = rng.integers(0, len(us), len(us))
    sc = sn = 0.0
    for c in ch:
        g = grp[c]
        ii = g[rng.integers(0, len(g), len(g))]
        sc += xc[ii].sum()
        sn += xn[ii].sum()
    reps2[b] = sc / sn
put("positive_control.slug_cluster_ci95", pct(reps2, (2.5, 97.5)))


# ------------------------------------------------- level effect (s3, Q1/Q2b)
def level_boot(sub, value, rng, B):
    """Two-stage bootstrap stratified by level: vertices with replacement
    within level, then rows with replacement within each chosen vertex.
    Returns {level: replicate means} and point means."""
    out, point = {}, {}
    for L, g in sub.groupby("level"):
        g = g.sort_values("vertex")
        x = g[value].to_numpy(dtype=float)
        vv = g["vertex"].to_numpy()
        uv, off, cnt = np.unique(vv, return_index=True, return_counts=True)
        nv = len(uv)
        point[L] = float(x.mean())
        reps = np.empty(B)
        ch = 200
        for i0 in range(0, B, ch):
            b = min(ch, B - i0)
            chosen = rng.integers(0, nv, size=(b, nv)).ravel()
            lens = cnt[chosen]
            rep_id = np.repeat(np.repeat(np.arange(b), nv), lens)
            dv = np.repeat(chosen, lens)
            idx = off[dv] + (rng.random(len(dv)) * cnt[dv]).astype(np.int64)
            sums = np.bincount(rep_id, weights=x[idx], minlength=b)
            ns = np.bincount(rep_id, minlength=b)
            reps[i0:i0 + b] = sums / ns
        out[L] = reps
    return out, point


lev = {}
holm_keys, holm_ps = [], []
for V in ("V6", "V5"):
    for arm in ARMS5:
        rng = np.random.default_rng(20261022)
        sub = df[(df["V"] == V) & df["fam"].isin(ISO[V]) & (df["arm"] == arm)]
        reps, point = level_boot(sub, "S", rng, B)
        pg = sub.groupby("level")["gae"].mean()
        for L in sorted(point):
            if L == 0:
                continue
            R = point[L] / point[0]
            Rs = reps[L] / reps[0]
            margin = (0.90, 1 / 0.90) if arm == "ic" else (0.95, 1 / 0.95)
            c90 = pct(Rs, (5, 95))
            c95 = pct(Rs, (2.5, 97.5))
            pb = min(1.0, 2 * min((1 + (Rs <= 1).sum()) / (B + 1), (1 + (Rs >= 1).sum()) / (B + 1)))
            descriptive = (V == "V5" and L <= 3)
            key = f"{V}|{arm}|{L}"
            lev[key] = {"R": R, "R_ops_gae": float(pg[L] / pg[0]), "ci90": c90, "ci95": c95,
                        "boot_sd": float(Rs.std(ddof=1)), "half_width95": (c95[1] - c95[0]) / 2,
                        "tost_equivalent": bool(margin[0] < c90[0] and c90[1] < margin[1]),
                        "ci95_excludes_1": bool(c95[0] > 1 or c95[1] < 1), "p_boot": pb,
                        "n_vertices": int(sub[sub.level == L]["vertex"].nunique()),
                        "n_rows": int((sub.level == L).sum()), "descriptive": descriptive}
            if not descriptive:
                holm_keys.append(key)
                holm_ps.append(pb)
adj = holm(holm_ps)
for k, a in zip(holm_keys, adj):
    lev[k]["p_holm"] = a
for k, d in lev.items():
    if d["descriptive"]:
        d["verdict"] = "descriptive"
    elif d["tost_equivalent"]:
        d["verdict"] = "equivalent"
    elif d["ci95_excludes_1"] and d["p_holm"] < 0.05:
        d["verdict"] = "different"
    else:
        d["verdict"] = "inconclusive"
put("level_effect", lev)
put("level_effect.holm_family_size", len(holm_keys))
vc = {}
for V in ("V5", "V6"):
    c = {}
    for k, d in lev.items():
        if k.startswith(V + "|"):
            c[d["verdict"]] = c.get(d["verdict"], 0) + 1
    vc[V] = c
put("level_effect.verdict_counts", vc)

# ------------------------------------------------ curve effect (s4, V6 only)
ce = {}
for arm in ARMS5:
    sub = df[(df["V"] == "V6") & df["fam"].isin(["D", "ctl"]) & (df["arm"] == arm)]
    md = sub[sub.fam == "D"].groupby("vertex")["S"].mean()
    cs = sub[sub.fam == "ctl"].groupby(["vertex", "slug"])["S"].mean()
    rows = []
    for v, m in md.items():
        if v not in cs.index.get_level_values(0):
            continue
        c = cs.loc[v].to_numpy()
        if len(c) != 2:
            continue
        rows.append((v, verts["V6"][v]["level"], m, c[0], c[1]))
    arr = np.array([[r[2], r[3], r[4]] for r in rows])
    levs = np.array([r[1] for r in rows])
    vid = np.array([r[0] for r in rows])
    mu = arr.mean(1, keepdims=True)
    W = ((arr - mu) ** 2).sum() / (2 * len(arr))

    def between(m):
        ss = 0.0
        df_ = 0
        for L in np.unique(levs):
            x = m[levs == L]
            ss += ((x - x.mean()) ** 2).sum()
            df_ += len(x) - 1
        return ss / df_

    bw = between(arr[:, 0])
    d = {"n_vertices": int(len(arr)), "within_var": float(W), "between_var": float(bw)}
    if W <= 1e-20:
        d["F"] = None
        d["degenerate"] = True
    else:
        F = bw / W
        rng = np.random.default_rng(20261023)
        cnt = 0
        for b in range(B):
            pick = rng.integers(0, 3, len(arr))
            if between(arr[np.arange(len(arr)), pick]) >= F * W:
                cnt += 1
        sd = math.sqrt(W)
        z = (arr[:, 0] - arr[:, 1:].mean(1)) / sd
        # exact expectation of the pooled-|z*| reference distribution: each
        # vertex contributes its three label assignments with weight 1/3
        allz = np.concatenate([np.abs((arr[:, k] - (arr.sum(1) - arr[:, k]) / 2) / sd) for k in range(3)])
        allz.sort()
        pv = 1 - np.searchsorted(allz, np.abs(z), side="left") / len(allz)
        ph = holm(list(pv))
        d.update({"F": float(F), "p_perm": (1 + cnt) / (B + 1), "control_sd_pooled": float(sd),
                  "max_abs_z": float(np.abs(z).max()), "min_p_curve": float(pv.min()),
                  "min_p_curve_holm": float(min(ph)), "n_flagged": int(sum(x < 0.01 for x in ph)),
                  "min_attainable_holm_p_approx": float(len(arr) / len(allz))})
    ce[arm] = d
tested = [a for a in ARMS5 if ce[a].get("F") is not None]
for a, h in zip(tested, holm([ce[a]["p_perm"] for a in tested])):
    ce[a]["p_perm_holm"] = h
put("curve_effect", ce)

# --------------------------------------------- paired source ratios (s5, V6)
ps_ = {}
pkeys, pps = [], []
for arm in ["rho-neg", "rho-plain", "ic"]:
    sub = df[(df["V"] == "V6") & df["fam"].isin(ISO["V6"]) & (df["arm"] == arm)]
    piv = sub.pivot_table(index=["vertex", "tidx", "round"], columns="fam", values="S", aggfunc="first")
    for fam in ["Ua", "Ub", "S", "N"]:
        pp = piv[["D", fam]].dropna()
        ps_[f"V6|{arm}|{fam}/D"] = {"ratio": float(pp[fam].mean() / pp["D"].mean()), "n_pairs": int(len(pp))}
put("paired_source", ps_)

# ------------------------------------------------------- BSGS identity (s5)
bi = {}
for V in ("V5", "V6"):
    for arm in ["bsgs", "bsgs-neg"]:
        fams = [f for f in ISO[V] if f != "N"]
        sub = df[(df["V"] == V) & df["fam"].isin(fams) & (df["arm"] == arm)]
        g = sub.groupby(["vertex", "tidx", "round"])["gae"].agg(["min", "max", "size"])
        g2 = g[g["size"] >= 2]
        bi[f"{V}|{arm}"] = {"n_triples": int(len(g2)), "fraction_identical": float((g2["min"] == g2["max"]).mean())}
put("bsgs_identity", bi)

# --------------------------------------------------------------- H-T (s5b)
tc = {}
for l in open(os.path.join(STUDY, "runs/transfer_cost.jsonl")):
    d = json.loads(l)
    if str(d.get("src", "")).startswith("tcr-v"):
        tc[int(d["tcr_source_id"])] = d
Mv, path_bad = {}, []
for v, d in tcr.items():
    Mv[v] = tc[v]["velu_point_maps"] + 2
    if Mv[v] != 2 * d["path_len_to_crater"]:
        path_bad.append(v)
tcr_rows = df[(df["fam"] == "Tcr")]
n_rows = df[(df["V"] == "V5") & (df["fam"] == "N") & (df["arm"] == "rho-neg")]
ht_v = {}
ks_bad = seed_bad = unpaired = 0
for v in sorted(tcr):
    t = tcr_rows[tcr_rows.vertex == v]
    a = t[t.arm == "rho-cubic"].set_index(["tidx", "round"])
    b = t[t.arm == "rho-neg"].set_index(["tidx", "round"])
    c = n_rows[n_rows.vertex == v].set_index(["tidx", "round"])
    keys = sorted(set(a.index) | set(b.index) | set(c.index))
    common = sorted(set(a.index) & set(b.index) & set(c.index))
    unpaired += len(keys) - len(common)
    # shared k_i: planted scalars equal across Tcr_v and N_v and the census tcr json
    for k in common:
        if not (a.loc[k, "planted"] == b.loc[k, "planted"] == c.loc[k, "planted"] == tcr[v]["ks"][k[0]]):
            ks_bad += 1
    if set(t["tseed"]) != set(n_rows[n_rows.vertex == v]["tseed"]):
        seed_bad += 1
    cols = np.stack([a.loc[common, "gae"].to_numpy(), a.loc[common, "unc"].to_numpy(),
                     b.loc[common, "gae"].to_numpy(), b.loc[common, "unc"].to_numpy(),
                     c.loc[common, "gae"].to_numpy(), c.loc[common, "unc"].to_numpy(),
                     a.loc[common, "S"].to_numpy(), b.loc[common, "S"].to_numpy()], axis=1)
    ht_v[v] = {"level": verts["V5"][v]["level"], "M": Mv[v], "cols": cols}


def ht_stats(means_by_v, Ms):
    """means_by_v: (nv, 8) column means; returns dict of pooled values."""
    m = means_by_v.mean(0)
    Mbar = np.mean(Ms)
    res = {}
    for p in PRICES:
        num = m[0] + p * m[1] + p * Mbar
        num0 = m[2] + p * m[3] + p * Mbar
        den = m[4] + p * m[5]
        res[f"T|{p}"] = num / den
        res[f"T0|{p}"] = num0 / den
        res[f"share|{p}"] = p * Mbar / num
    res["cubic_over_neg_S_on_Tcr"] = m[6] / m[7]
    return res


allv = sorted(ht_v)
means = np.array([ht_v[v]["cols"].mean(0) for v in allv])
Ms = np.array([ht_v[v]["M"] for v in allv], dtype=float)
point = ht_stats(means, Ms)
by_level_pt = {}
for L in range(1, 7):
    sel = [i for i, v in enumerate(allv) if ht_v[v]["level"] == L]
    by_level_pt[L] = ht_stats(means[sel], Ms[sel])
# bootstrap: vertices within level, then (tidx, round) pairs within vertex jointly
rng = np.random.default_rng(20261023)
lv_groups = {L: [v for v in allv if ht_v[v]["level"] == L] for L in range(1, 7)}
rep_overall = {k: np.empty(B) for k in point}
rep_level = {L: {k: np.empty(B) for k in point} for L in range(1, 7)}
for b in range(B):
    allm, allM = [], []
    for L in range(1, 7):
        g = lv_groups[L]
        ch = rng.integers(0, len(g), len(g))
        ml, Ml = [], []
        for c in ch:
            cols = ht_v[g[c]]["cols"]
            ii = rng.integers(0, len(cols), len(cols))
            ml.append(cols[ii].mean(0))
            Ml.append(ht_v[g[c]]["M"])
        st = ht_stats(np.array(ml), np.array(Ml, dtype=float))
        for k in st:
            rep_level[L][k][b] = st[k]
        allm += ml
        allM += Ml
    st = ht_stats(np.array(allm), np.array(allM, dtype=float))
    for k in st:
        rep_overall[k][b] = st[k]
ht = {"n_vertices": len(allv), "path_check_failures": path_bad, "unpaired_keys": unpaired,
      "shared_k_mismatch": ks_bad, "target_seed_mismatch_vertices": seed_bad,
      "n_pairs_per_vertex_by_level": {L: int(len(ht_v[lv_groups[L][0]]["cols"])) for L in range(1, 7)},
      "M_range": [int(Ms.min()), int(Ms.max())],
      "overall": {k: {"value": float(point[k]), "ci95": pct(rep_overall[k], (2.5, 97.5)),
                      "boot_sd": float(rep_overall[k].std(ddof=1))} for k in point},
      "by_level": {L: {k: {"value": float(by_level_pt[L][k]), "ci95": pct(rep_level[L][k], (2.5, 97.5))}
                       for k in point} for L in range(1, 7)}}
# row-pooled sensitivity (not the A3.2 estimand)
allc = np.concatenate([ht_v[v]["cols"] for v in allv])
allM_rows = np.concatenate([np.full(len(ht_v[v]["cols"]), ht_v[v]["M"]) for v in allv])
ht["row_pooled_sensitivity"] = {f"T|{p}": float((allc[:, 0].mean() + p * allc[:, 1].mean() + p * allM_rows.mean())
                                                 / (allc[:, 4].mean() + p * allc[:, 5].mean())) for p in PRICES}
# per-vertex T_v range at 1 GAE (point)
tv1 = [(means[i, 0] + means[i, 1] + Ms[i]) / (means[i, 4] + means[i, 5]) for i in range(len(allv))]
ht["T_v_at_1_range"] = [float(min(tv1)), float(max(tv1))]
o = ht["overall"]
T_below = all(o[f"T|{p}"]["ci95"][1] < 1 for p in PRICES)
T0_incl = all(o[f"T0|{p}"]["ci95"][0] <= 1 <= o[f"T0|{p}"]["ci95"][1] for p in PRICES)
T_refute = any(o[f"T|{p}"]["ci95"][1] >= 1 for p in PRICES)
ht["verdict"] = "supported" if (T_below and T0_incl) else ("refuted" if T_refute else "inconclusive")
ht["verdict_inputs"] = {"T_ci_upper_below_1_all_prices": T_below, "T0_ci_includes_1_all_prices": T0_incl,
                        "T_ci_includes_or_above_1_any_price": T_refute}
put("H-T", ht)

# --------------------------------------------------------------- H-M (s5c)
hm = {}
NPERM = B
HP = int(os.environ.get("HM_PRECISE_PERM", "200000"))
for (V, v), g in df[df["fam"] == "M"].groupby(["V", "vertex"]):
    cur = f"{V}:{v}"
    hm[cur] = {"level": int(verts[V][v]["level"])}
    for arm in ["rho-neg", "bsgs", "ic"]:
        a = g[g.arm == arm].sort_values(["model", "tidx", "round"])
        groups = [a[a.model == m]["S"].to_numpy() for m in sorted(a["model"].unique())]
        k = len(groups)
        nper = [len(x) for x in groups]
        x = np.concatenate(groups)
        N = len(x)
        gm = np.array([y.mean() for y in groups])
        ssb = sum(nn * (mm - x.mean()) ** 2 for nn, mm in zip(nper, gm))
        ssw = sum(((y - y.mean()) ** 2).sum() for y in groups)
        d = {"n_models": k, "rows_per_model": nper, "model_mean_S": gm.tolist()}
        if ssw <= 1e-18 * max(1.0, (x ** 2).sum()):
            d.update({"F": None, "p_perm": 1.0, "degenerate": True})
        else:
            F = (ssb / (k - 1)) / (ssw / (N - k))
            d["F"] = float(F)
            d["p_anova_parametric"] = float(stats.f.sf(F, k - 1, N - k))
            assert len(set(nper)) == 1
            npm = nper[0]

            def perm_count(nperm, seed):
                rng = np.random.default_rng(seed)
                cnt = 0
                done = 0
                tot = x.sum()
                while done < nperm:
                    b = min(2000, nperm - done)
                    idx = np.argsort(rng.random((b, N)), axis=1)
                    xs = x[idx].reshape(b, k, npm)
                    gms = xs.mean(2)
                    ssb_ = (npm * (gms - tot / N) ** 2).sum(1)
                    cnt += int((ssb_ >= ssb * (1 - 1e-12)).sum())
                    done += b
                return cnt

            c = perm_count(NPERM, 20261024)
            d["p_perm"] = (1 + c) / (NPERM + 1)
            if arm == "ic" and HP:
                c2 = perm_count(HP, 7_20261024)
                d["p_perm_precise"] = (1 + c2) / (HP + 1)
                d["p_perm_precise_n"] = HP
        # effect size: max/min of model means, rows resampled within model
        rng = np.random.default_rng(20261024 + 1)
        arr = np.array(groups)
        reps = np.empty(B)
        for i0 in range(0, B, 1000):
            b = min(1000, B - i0)
            ii = rng.integers(0, arr.shape[1], size=(b, arr.shape[0], arr.shape[1]))
            mm = np.take_along_axis(np.broadcast_to(arr, (b,) + arr.shape), ii, axis=2).mean(2)
            reps[i0:i0 + b] = mm.max(1) / mm.min(1)
        d["max_over_min"] = float(gm.max() / gm.min())
        d["max_over_min_ci95"] = pct(reps, (2.5, 97.5))
        if arm == "bsgs":
            piv = a.pivot_table(index=["tidx", "round"], columns="model", values="gae", aggfunc="first")
            d["identical_ops_across_models_fraction"] = float((piv.nunique(axis=1) == 1).mean())
        hm[cur][arm] = d
for arm in ["rho-neg", "bsgs", "ic"]:
    curs = sorted(hm)
    ps = [hm[c][arm]["p_perm"] for c in curs]
    for c, h in zip(curs, holm(ps)):
        hm[c][arm]["p_holm"] = h
        hm[c][arm]["significant"] = bool(h < 0.05)
    if arm == "ic":
        psp = [hm[c][arm].get("p_perm_precise", hm[c][arm]["p_perm"]) for c in curs]
        for c, h in zip(curs, holm(psp)):
            hm[c][arm]["p_holm_precise"] = h
        pa = [hm[c][arm].get("p_anova_parametric", 1.0) for c in curs]
        for c, h in zip(curs, holm(pa)):
            hm[c][arm]["p_holm_parametric"] = h
summ = {arm: int(sum(hm[c][arm]["significant"] for c in hm)) for arm in ["rho-neg", "bsgs", "ic"]}
summ["ic_precise_perm"] = int(sum(hm[c]["ic"]["p_holm_precise"] < 0.05 for c in hm))
summ["ic_parametric_anova"] = int(sum(hm[c]["ic"]["p_holm_parametric"] < 0.05 for c in hm))
nc = len(hm)
if summ["ic"] >= nc / 2 and summ["rho-neg"] <= 1 and summ["bsgs"] <= 1:
    verdict = "supported for IC"
elif summ["ic"] <= 1:
    verdict = "refuted"
else:
    verdict = "inconclusive"
put("H-M", {"curves": hm, "n_significant": summ, "verdict": verdict})


# ------------------------------------------------- IC vs matched rho (s9)
def priced(sub, p):
    return sub["S"] * (sub["gae"] + p * sub["unc"]) / sub["gae"]


icr = {}
for V in ("V5", "V6"):
    sub = df[(df["V"] == V) & df["fam"].isin(ISO[V]) & df["arm"].isin(["ic", "rho-neg"])].copy()
    for p in PRICES:
        sub[f"P{p}"] = priced(sub, p)
    for L in sorted(sub["level"].unique()):
        s = sub[sub.level == L]
        icr[f"{V}|{L}"] = {"n_pairs": int((s.arm == "ic").sum()),
                           **{f"{p}": float(s[s.arm == "ic"][f"P{p}"].mean() / s[s.arm == "rho-neg"][f"P{p}"].mean())
                              for p in PRICES}}
    for name, sel in (("pooled_all", sub), ("pooled_L1plus", sub[sub.level >= 1])):
        icr[f"{V}|{name}"] = {f"{p}": float(sel[sel.arm == "ic"][f"P{p}"].mean() / sel[sel.arm == "rho-neg"][f"P{p}"].mean())
                              for p in PRICES}
        icr[f"{V}|{name}"]["n_pairs"] = int((sel.arm == "ic").sum())
# cluster-bootstrap CIs (s3 scheme) on the pooled ratios
for V, Lmin in (("V5", 1), ("V6", 0)):
    sub = df[(df["V"] == V) & df["fam"].isin(ISO[V]) & df["arm"].isin(["ic", "rho-neg"]) & (df["level"] >= Lmin)]
    piv = sub.pivot_table(index=["level", "vertex", "wid", "tidx", "round"], columns="arm",
                          values=["S", "gae", "unc"], aggfunc="first").dropna().reset_index()
    rng = np.random.default_rng(20261022)
    reps = {p: np.zeros(B) for p in PRICES}
    num = {p: np.zeros(B) for p in PRICES}
    den = {p: np.zeros(B) for p in PRICES}
    cnt = np.zeros(B)
    for L, g in piv.groupby("level"):
        g = g.sort_values("vertex")
        vv = g["vertex"].to_numpy()
        uv, off, cn = np.unique(vv, return_index=True, return_counts=True)
        nv = len(uv)
        xi = {p: (g[("S", "ic")] * (g[("gae", "ic")] + p * g[("unc", "ic")]) / g[("gae", "ic")]).to_numpy() for p in PRICES}
        xr = {p: (g[("S", "rho-neg")] * (g[("gae", "rho-neg")] + p * g[("unc", "rho-neg")]) / g[("gae", "rho-neg")]).to_numpy()
              for p in PRICES}
        for i0 in range(0, B, 200):
            b = min(200, B - i0)
            chosen = rng.integers(0, nv, size=(b, nv)).ravel()
            lens = cn[chosen]
            rep_id = np.repeat(np.repeat(np.arange(b), nv), lens)
            dv = np.repeat(chosen, lens)
            idx = off[dv] + (rng.random(len(dv)) * cn[dv]).astype(np.int64)
            for p in PRICES:
                num[p][i0:i0 + b] += np.bincount(rep_id, weights=xi[p][idx], minlength=b)
                den[p][i0:i0 + b] += np.bincount(rep_id, weights=xr[p][idx], minlength=b)
    key = f"{V}|pooled_all" if V == "V6" else f"{V}|pooled_L1plus"
    for p in PRICES:
        icr[key][f"ci95_{p}"] = pct(num[p] / den[p], (2.5, 97.5))
# V5 crater against rho-cubic (D and N pairs)
cr = df[(df["V"] == "V5") & (df["vertex"] == 0) & df["fam"].isin(["D", "N"]) & df["arm"].isin(["ic", "rho-cubic", "rho-neg"])].copy()
piv = cr.pivot_table(index=["wid", "tidx", "round"], columns="arm", values=["S", "gae", "unc"], aggfunc="first").dropna()
rng = np.random.default_rng(20261022)
crd = {"n_pairs": int(len(piv))}
for p in PRICES:
    xi = (piv[("S", "ic")] * (piv[("gae", "ic")] + p * piv[("unc", "ic")]) / piv[("gae", "ic")]).to_numpy()
    xc = (piv[("S", "rho-cubic")] * (piv[("gae", "rho-cubic")] + p * piv[("unc", "rho-cubic")]) / piv[("gae", "rho-cubic")]).to_numpy()
    xn = (piv[("S", "rho-neg")] * (piv[("gae", "rho-neg")] + p * piv[("unc", "rho-neg")]) / piv[("gae", "rho-neg")]).to_numpy()
    ii = rng.integers(0, len(xi), size=(B, len(xi)))
    rr = xi[ii].sum(1) / xc[ii].sum(1)
    crd[f"{p}"] = {"ic": float(xi.mean()), "rho_cubic": float(xc.mean()), "rho_neg": float(xn.mean()),
                   "ic_over_cubic": float(xi.mean() / xc.mean()), "ic_over_neg": float(xi.mean() / xn.mean()),
                   "ci95_ic_over_cubic": pct(rr, (2.5, 97.5))}
crd["order_holds_across_band"] = all(crd[f"{p}"]["ci95_ic_over_cubic"][0] > 1 for p in PRICES)
icr["V5|crater_rho_cubic"] = crd
# M curves IC / rho-neg (descriptive)
mr = {}
for (V, v), g in df[df["fam"] == "M"].groupby(["V", "vertex"]):
    mr[f"{V}:{v}"] = {f"{p}": float(priced(g[g.arm == "ic"], p).mean() / priced(g[g.arm == "rho-neg"], p).mean()) for p in PRICES}
icr["M_descriptive"] = mr
icr["M_range_0"] = [min(x["0.0"] for x in mr.values()), max(x["0.0"] for x in mr.values())]
icr["M_range_1"] = [min(x["1.0"] for x in mr.values()), max(x["1.0"] for x in mr.values())]
# V5 crater IC mean S per model (D/N and ctl re-scalings)
cm = df[(df["V"] == "V5") & (df["vertex"] == 0) & df["fam"].isin(["D", "N", "ctl"]) & (df["arm"] == "ic")]
icr["V5_crater_ic_by_model"] = {s: float(x) for s, x in cm.groupby("slug")["S"].mean().items()}
put("ic_vs_rho", icr)

# --------------------------------------------------------------- wall (s8)
wl = {}
for V in ("V5", "V6"):
    fams = ISO[V] + ["ctl"]
    sub = df[(df["V"] == V) & df["fam"].isin(fams) & df["arm"].isin(["rho-neg", "rho-neg-aa"])]
    piv = sub.pivot_table(index=["fam", "wid", "tidx", "round"], columns="arm", values="solve_wall_ns", aggfunc="first").dropna()
    wl[V] = {"aa_ratio": float(piv["rho-neg-aa"].mean() / piv["rho-neg"].mean()), "n_pairs": int(len(piv))}
    for arm in ARMS5:
        for nm, fs in (("iso", ISO[V]), ("iso+ctl", fams)):
            s = df[(df["V"] == V) & df["fam"].isin(fs) & (df["arm"] == arm)]
            wl[V][f"L2|{arm}|{nm}"] = float((s["iso"] == "L2").mean())
    s = df[(df["V"] == V)]
    wl[V]["L2|all_rows"] = float((s["iso"] == "L2").mean())
    s = df[(df["V"] == V) & df["fam"].isin(fams) & df["arm"].isin(ARMS5)]
    wl[V]["L2|5arms_iso+ctl"] = float((s["iso"] == "L2").mean())
put("wall", wl)

# --------------------------------------------------- transfer cost (s5)
tcl = [json.loads(l) for l in open(os.path.join(STUDY, "runs/transfer_cost.jsonl"))]
trc = {"n_rows": len(tcl), "all_matches_transported": all(d.get("matches_transported") for d in tcl)}
for V in ("V5", "V6"):
    e = [d["ns_per_edge_mean"] for d in tcl if d["volcano"] == V and d["path_len"] == 1 and not str(d["src"]).startswith("tcr")]
    trc[f"{V}|ns_per_edge_median_path_len_1"] = float(np.median(e))
    trc[f"{V}|velu_per_edge"] = sorted({d["velu_point_maps"] / d["path_len"] for d in tcl if d["volcano"] == V and d["path_len"] > 0})
put("transfer_cost", trc)

json.dump(OUTD, open(OUT, "w"), indent=1, default=lambda o: o.item() if hasattr(o, "item") else str(o))
print("wrote", OUT)
