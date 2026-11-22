#!/usr/bin/env python3
"""Round-2 validator: recompute the follow-up's headline statistics from the re-derived rows.

Validator-written from ANALYSIS_PLAN.md and AMENDMENTS.md only; it shares no code with
analysis/*.py (not opened) or with the round-1 validator's scripts. Input: the rows that
rederive.py streamed from the committed archives (checked equal to runs/extract and
runs/uncharged by compare_extracts.py), census/<V>/vertices.jsonl, census/V5/tcr/*.json
and runs/transfer_cost.jsonl. Bootstrap and permutation draws use the plan's seeds with
this script's own draw order, so CI endpoints and permutation p-values agree with the
producer's within Monte Carlo error, not bit for bit.

Usage: recompute.py STUDY_DIR ROWS_CSV_GZ OUT_JSON
"""
import json, math, os, sys
import numpy as np
import pandas as pd
from scipy import stats

study, rows_path, out_path = sys.argv[1:4]
B = 10000
PRICES = [0.0, 0.1, 1.0]
ISO = {"V6": ["D", "Ua", "Ub", "S", "N"], "V5": ["D", "Ua", "Ub", "N"]}
ARMS5 = ["rho-neg", "rho-plain", "bsgs", "bsgs-neg", "ic"]
OUT = {}

df = pd.read_csv(rows_path)
OUT["counts"] = {
    "rows_all_incl_warmup": int(len(df)),
    "rows_measured": int((df.warmup == 0).sum()),
    "rows_warmup_dropped": int((df.warmup == 1).sum()),
    "unverified_rows_total": int(((df.warmup == 0) & (df.verified == 0)).sum()),
    "unverified_rows_incl_warmup": int((df.verified == 0).sum()),
    "sessions": int(df.session.nunique()),
}
m = df[df.warmup == 0].copy()
m["key"] = m.workload_id + "|" + m.target_index.astype(str) + "|" + m["round"].astype(str)


def priced(d, p):
    return d.S * (d.total_gae + p * d.unc) / d.total_gae


def holm(ps):
    ps = np.asarray(ps, float)
    order = np.argsort(ps, kind="stable")
    mm = len(ps)
    adj = np.empty(mm)
    run = 0.0
    for i, j in enumerate(order):
        run = max(run, min(1.0, (mm - i) * ps[j]))
        adj[j] = run
    return adj


def cluster_boot(groups, B, rng, batch=100):
    """Two-stage bootstrap: clusters with replacement, then rows (all columns jointly)
    with replacement within each chosen cluster. groups: list of 2-D arrays (rows x cols).
    Returns (B x cols) column sums and (B,) row counts per replicate."""
    k = len(groups)
    sizes = np.array([len(g) for g in groups])
    offs = np.concatenate([[0], np.cumsum(sizes)[:-1]])
    flat = np.concatenate(groups, axis=0)
    ncol = flat.shape[1]
    sums = np.empty((B, ncol))
    ns = np.empty(B)
    for b0 in range(0, B, batch):
        b = min(batch, B - b0)
        ch = rng.integers(0, k, size=(b, k))
        sz = sizes[ch]
        tot = sz.sum(1)
        rep_off = np.repeat(offs[ch].ravel(), sz.ravel())
        rep_sz = np.repeat(sz.ravel(), sz.ravel())
        idx = rep_off + (rng.random(rep_sz.size) * rep_sz).astype(np.int64)
        vals = flat[idx]
        bounds = np.concatenate([[0], np.cumsum(tot)[:-1]])
        sums[b0:b0 + b] = np.add.reduceat(vals, bounds, axis=0)
        ns[b0:b0 + b] = tot
    return sums, ns


def pct(x, q):
    return [float(np.percentile(x, q[0])), float(np.percentile(x, q[1]))]


# ---------------------------------------------------------------- census facts
cen = {}
for V in ("V5", "V6"):
    vs = [json.loads(l) for l in open(os.path.join(study, "census", V, "vertices.jsonl"))]
    cen[V] = {v["id"]: v for v in vs}
p5 = cen["V5"][0]["p"]; r5 = cen["V5"][0]["r"]
facts = {}
for V in ("V5", "V6"):
    vs = cen[V]
    p = vs[0]["p"]
    js = [v["j"] for v in vs.values()]
    facts[V] = {
        "p": p, "r": vs[0]["r"], "n_vertices": len(vs),
        "per_level": {str(L): sum(1 for v in vs.values() if v["level"] == L) for L in sorted({v["level"] for v in vs.values()})},
        "vertices_j0": [v["id"] for v in vs.values() if v["j"] % p == 0],
        "vertices_j1728": [v["id"] for v in vs.values() if v["j"] % p == 1728 % p],
        "distinct_j": len(set(js)) == len(js),
        "p_mod_3": p % 3, "p_mod_4": p % 4, "r_mod_3": vs[0]["r"] % 3,
        "is_prime_field": True,
    }
OUT["census_facts"] = facts

# ---------------------------------------------------------------- level effect (s3)
rng = np.random.default_rng(20261022)
lev = {}
holm_keys, holm_ps = [], []
for V in ("V6", "V5"):
    for arm in ARMS5:
        d = m[(m.V == V) & (m.fam.isin(ISO[V])) & (m.arm == arm)]
        levels = sorted(d.level.unique())
        boots = {}
        means = {}
        for L in levels:
            dl = d[d.level == L]
            means[L] = dl.S.mean()
            groups = [g.S.to_numpy()[:, None] for _, g in dl.groupby("vertex")]
            s, n = cluster_boot(groups, B, rng)
            boots[L] = s[:, 0] / n
        for L in levels:
            if L == 0:
                continue
            R = means[L] / means[0]
            Rb = boots[L] / boots[0]
            margin = [0.90, 1 / 0.90] if arm == "ic" else [0.95, 1 / 0.95]
            c90 = pct(Rb, (5, 95)); c95 = pct(Rb, (2.5, 97.5))
            pb = min(1.0, 2 * min((1 + (Rb <= 1).sum()) / (1 + B), (1 + (Rb >= 1).sum()) / (1 + B)))
            descr = (V == "V5" and L <= 3)
            cell = {"ratio": R, "ci90": c90, "ci95": c95, "boot_sd": float(Rb.std(ddof=1)), "p_boot": pb,
                    "margin": margin, "tost": bool(margin[0] < c90[0] and c90[1] < margin[1]),
                    "n_vertices": int(d[d.level == L].vertex.nunique()), "n_rows": int((d.level == L).sum()),
                    "descriptive": descr}
            lev[f"{V}|{arm}|{L}"] = cell
            if not descr:
                holm_keys.append(f"{V}|{arm}|{L}"); holm_ps.append(pb)
adj = holm(holm_ps)
for k, a in zip(holm_keys, adj):
    lev[k]["p_holm"] = float(a)
for k, c in lev.items():
    if c["descriptive"]:
        c["verdict"] = "descriptive"
    elif c["tost"]:
        c["verdict"] = "equivalent"
    elif (c["ci95"][0] > 1 or c["ci95"][1] < 1) and c["p_holm"] < 0.05:
        c["verdict"] = "different"
    else:
        c["verdict"] = "inconclusive"
OUT["level_effect"] = lev
OUT["level_effect_holm_family_size"] = len(holm_keys)
vc = {}
for k, c in lev.items():
    V = k.split("|")[0]
    vc.setdefault(V, {}).setdefault(c["verdict"], 0)
    vc[V][c["verdict"]] += 1
OUT["level_effect_verdict_counts"] = vc

# F7: family-mix explanation of the constant V5 BSGS ratios
f7 = {}
for arm in ("bsgs", "bsgs-neg"):
    d = m[(m.V == "V5") & (m.arm == arm)]
    x = d[d.fam == "N"].S.mean() / d[d.fam == "D"].S.mean()
    x0 = d[(d.fam == "N") & (d.level == 0)].S.mean() / d[(d.fam == "D") & (d.level == 0)].S.mean()
    f7[arm] = {"mean_S_N_over_D_all_levels": x, "mean_S_N_over_D_crater": x0,
               "formula_(3+x)/(2(1+x))": (3 + x0) / (2 * (1 + x0)),
               "R_by_level": {str(L): lev[f"V5|{arm}|{L}"]["ratio"] for L in range(1, 7)},
               "k_identical_D_Ua_Ub_per_target": None}
OUT["F7_family_mix"] = f7

# ---------------------------------------------------------------- positive control (s6)
rng = np.random.default_rng(20261022)
d = m[(m.V == "V5") & (m.vertex == 0) & (m.fam.isin(["D", "N", "ctl"]))]
c = d[d.arm == "rho-cubic"].set_index(["slug", "key"]).S
n_ = d[d.arm == "rho-neg"].set_index(["slug", "key"]).S
pr = pd.concat([c.rename("c"), n_.rename("n")], axis=1).dropna()
ratio = pr.c.mean() / pr.n.mean()
X = pr[["c", "n"]].to_numpy()
idx = rng.integers(0, len(X), size=(B, len(X)))
sb = X[idx].sum(1)
rb = sb[:, 0] / sb[:, 1]
groups = [g[["c", "n"]].to_numpy() for _, g in pr.groupby(level="slug")]
s2, _ = cluster_boot(groups, B, rng)
OUT["positive_control"] = {"ratio": ratio, "n_pairs": int(len(pr)), "n_slugs": int(pr.index.get_level_values(0).nunique()),
                           "ci95_row": pct(rb, (2.5, 97.5)), "boot_sd_row": float(rb.std(ddof=1)),
                           "ci95_slug_cluster": pct(s2[:, 0] / s2[:, 1], (2.5, 97.5)),
                           "unpaired_rows": int(len(c) + len(n_) - 2 * len(pr)),
                           "pass": bool(np.percentile(rb, 97.5) < 0.85), "theory": 1 / math.sqrt(3)}

# ---------------------------------------------------------------- H-T (s5b)
tc = {}
for l in open(os.path.join(study, "runs", "transfer_cost.jsonl")):
    t = json.loads(l)
    if str(t.get("src", "")).startswith("tcr-v"):
        tc.setdefault(t["tcr_source_id"], []).append(t)
tcr_json = {}
for f in os.listdir(os.path.join(study, "census", "V5", "tcr")):
    t = json.load(open(os.path.join(study, "census", "V5", "tcr", f)))
    tcr_json[t["id"]] = t
ht_rows = {}
Mv = {}
path_fail = []
unpaired = 0
for v in sorted(cen["V5"]):
    if v == 0:
        continue
    assert len(tc[v]) == 1
    Mv[v] = tc[v][0]["velu_point_maps"] + 2
    if Mv[v] != 2 * tcr_json[v]["path_len_to_crater"]:
        path_fail.append(v)
    t = m[(m.fam == "Tcr") & (m.chunk == v)]
    nv = m[(m.V == "V5") & (m.fam == "N") & (m.vertex == v) & (m.arm == "rho-neg")]
    tcub = t[t.arm == "rho-cubic"].assign(k=lambda x: x.target_index.astype(str) + "|" + x["round"].astype(str)).set_index("k")
    tneg = t[t.arm == "rho-neg"].assign(k=lambda x: x.target_index.astype(str) + "|" + x["round"].astype(str)).set_index("k")
    nn = nv.assign(k=lambda x: x.target_index.astype(str) + "|" + x["round"].astype(str)).set_index("k")
    cols = {}
    for p in PRICES:
        cols[f"cub{p}"] = tcub.total_gae + p * tcub.unc
        cols[f"tneg{p}"] = tneg.total_gae + p * tneg.unc
        cols[f"n{p}"] = nn.total_gae + p * nn.unc
    cols["cubS"] = tcub.S; cols["tnegS"] = tneg.S
    J = pd.concat(cols, axis=1)
    unpaired += int(J.isna().any(axis=1).sum())
    ht_rows[v] = J.dropna()
colnames = list(ht_rows[1].columns)


def ht_stats(sum_by_vertex, verts):
    """sum_by_vertex: dict v -> per-vertex mean vector; returns T, T0 per price (vertex-weighted)."""
    res = {}
    for p in PRICES:
        num = np.mean([sum_by_vertex[v][colnames.index(f"cub{p}")] + p * Mv[v] for v in verts])
        num0 = np.mean([sum_by_vertex[v][colnames.index(f"tneg{p}")] + p * Mv[v] for v in verts])
        den = np.mean([sum_by_vertex[v][colnames.index(f"n{p}")] for v in verts])
        res[p] = (num / den, num0 / den)
    return res


vmeans = {v: ht_rows[v].mean().to_numpy() for v in ht_rows}
lvl = {v: cen["V5"][v]["level"] for v in ht_rows}
point = ht_stats(vmeans, list(ht_rows))
perlevel_point = {L: ht_stats(vmeans, [v for v in ht_rows if lvl[v] == L]) for L in range(1, 7)}
# row-pooled alternative (N3) and same-curve automorphism gain
allr = pd.concat(ht_rows.values())
rowpooled = {p: ((allr[f"cub{p}"].sum() + p * sum(Mv[v] * len(ht_rows[v]) for v in ht_rows)) / allr[f"n{p}"].sum()) for p in PRICES}
gain_vw = np.mean([vmeans[v][colnames.index("cubS")] for v in ht_rows]) / np.mean([vmeans[v][colnames.index("tnegS")] for v in ht_rows])
gain_row = allr.cubS.mean() / allr.tnegS.mean()
# cluster bootstrap: vertices within level, then pairs within vertex (all columns jointly)
rng = np.random.default_rng(20261023)
Tb = {p: np.empty(B) for p in PRICES}; T0b = {p: np.empty(B) for p in PRICES}
gb = np.empty(B)
TbL = {L: {p: np.empty(B) for p in PRICES} for L in range(1, 7)}
byL = {L: [v for v in sorted(ht_rows) if lvl[v] == L] for L in range(1, 7)}
arrs = {v: ht_rows[v].to_numpy() for v in ht_rows}
ic_cub = {p: colnames.index(f"cub{p}") for p in PRICES}
ic_tn = {p: colnames.index(f"tneg{p}") for p in PRICES}
ic_n = {p: colnames.index(f"n{p}") for p in PRICES}
for b in range(B):
    num = {p: [] for p in PRICES}; num0 = {p: [] for p in PRICES}; den = {p: [] for p in PRICES}
    gc, gn = [], []
    for L in range(1, 7):
        vs = byL[L]
        pick = rng.integers(0, len(vs), size=len(vs))
        lnum = {p: [] for p in PRICES}; lden = {p: [] for p in PRICES}
        for i in pick:
            v = vs[i]
            a = arrs[v]
            mu = a[rng.integers(0, len(a), size=len(a))].mean(0)
            for p in PRICES:
                x = mu[ic_cub[p]] + p * Mv[v]
                num[p].append(x); num0[p].append(mu[ic_tn[p]] + p * Mv[v]); den[p].append(mu[ic_n[p]])
                lnum[p].append(x); lden[p].append(mu[ic_n[p]])
            gc.append(mu[colnames.index("cubS")]); gn.append(mu[colnames.index("tnegS")])
        for p in PRICES:
            TbL[L][p][b] = np.mean(lnum[p]) / np.mean(lden[p])
    for p in PRICES:
        Tb[p][b] = np.mean(num[p]) / np.mean(den[p]); T0b[p][b] = np.mean(num0[p]) / np.mean(den[p])
    gb[b] = np.mean(gc) / np.mean(gn)
ht = {"overall": {}, "by_level": {}, "M_v": {str(v): Mv[v] for v in sorted(Mv)}, "path_check_failures": path_fail,
      "unpaired_rows": unpaired, "n_vertices": len(ht_rows),
      "pairs_per_vertex_by_level": {str(L): sorted({len(ht_rows[v]) for v in byL[L]}) for L in range(1, 7)}}
for p in PRICES:
    ht["overall"][str(p)] = {"T": point[p][0], "T_ci95": pct(Tb[p], (2.5, 97.5)), "T_boot_sd": float(Tb[p].std(ddof=1)),
                             "T0": point[p][1], "T0_ci95": pct(T0b[p], (2.5, 97.5)), "T0_boot_sd": float(T0b[p].std(ddof=1)),
                             "T_row_pooled": rowpooled[p]}
for L in range(1, 7):
    ht["by_level"][str(L)] = {str(p): {"T": perlevel_point[L][p][0], "T0": perlevel_point[L][p][1],
                                       "T_ci95": pct(TbL[L][p], (2.5, 97.5))} for p in PRICES}
ht["cubic_over_neg_S_on_Tcr_vertex_weighted"] = gain_vw
ht["cubic_over_neg_S_on_Tcr_row_pooled"] = gain_row
ht["cubic_over_neg_S_ci95"] = pct(gb, (2.5, 97.5))
ht["per_vertex_T_at_1"] = {str(v): (vmeans[v][ic_cub[1.0]] + Mv[v]) / vmeans[v][ic_n[1.0]] for v in sorted(ht_rows)}
ht["transport_share_of_numerator_at_1"] = float(np.mean([Mv[v] for v in ht_rows]) / np.mean([vmeans[v][ic_cub[1.0]] + Mv[v] for v in ht_rows]))
below = all(ht["overall"][str(p)]["T_ci95"][1] < 1 for p in PRICES)
t0inc = all(ht["overall"][str(p)]["T0_ci95"][0] <= 1 <= ht["overall"][str(p)]["T0_ci95"][1] for p in PRICES)
refut = any(ht["overall"][str(p)]["T_ci95"][1] >= 1 for p in PRICES)
ht["verdict"] = "supported" if (below and t0inc) else ("refuted" if refut else "inconclusive")
ht["T0_ci_upper_margin_above_1"] = {str(p): ht["overall"][str(p)]["T0_ci95"][1] - 1 for p in PRICES}
OUT["H-T"] = ht

# ---------------------------------------------------------------- H-M (s5c)
def oneway_F(Y):  # Y: (..., g, n)
    g, n = Y.shape[-2], Y.shape[-1]
    mu = Y.mean(-1)
    grand = mu.mean(-1, keepdims=True)
    msb = n * ((mu - grand) ** 2).sum(-1) / (g - 1)
    msw = ((Y - mu[..., None]) ** 2).sum((-2, -1)) / (g * (n - 1))
    return msb, msw


hm = {"curves": {}}
Mrows = m[m.fam == "M"]
curves = sorted({(r.V, r.chunk) for r in Mrows[["V", "chunk"]].drop_duplicates().itertuples()})
pv = {a: [] for a in ("rho-neg", "bsgs", "ic")}
for (V, v) in curves:
    cname = f"{V}:{v}"
    hm["curves"][cname] = {"level": int(cen[V][v]["level"])}
    for arm in ("rho-neg", "bsgs", "ic"):
        d = Mrows[(Mrows.V == V) & (Mrows.chunk == v) & (Mrows.arm == arm)].sort_values(["model", "target_index", "round"])
        Y = np.stack([g.S.to_numpy() for _, g in d.groupby("model")])
        g, n = Y.shape
        msb, msw = oneway_F(Y)
        cell = {"n_models": g, "rows_per_model": n, "model_means": Y.mean(1).tolist()}
        # bsgs instance check: identical ops across models for each (target, round)
        G = np.stack([gg.total_gae.to_numpy() for _, gg in d.groupby("model")])
        cell["identical_ops_across_models_fraction"] = float((G == G[0]).all(0).mean())
        if msw <= 1e-12 * max(1.0, abs(Y.mean())) ** 2 or np.ptp(Y.mean(1)) <= 1e-12 * abs(Y.mean()):
            cell.update({"F": float(msb / msw) if msw > 0 else None, "degenerate": True, "p_perm": 1.0})
        else:
            F = msb / msw
            rng = np.random.default_rng(20261024)
            flat = Y.ravel()
            cnt = 0
            for b0 in range(0, B, 2000):
                bb = min(2000, B - b0)
                P = rng.permuted(np.broadcast_to(flat, (bb, flat.size)), axis=1).reshape(bb, g, n)
                a_, w_ = oneway_F(P)
                cnt += int((a_ / w_ >= F * (1 - 1e-12)).sum())
            cell.update({"F": float(F), "degenerate": False, "p_perm": (1 + cnt) / (1 + B),
                         "p_parametric": float(stats.f.sf(F, g - 1, g * (n - 1)))})
        mx = Y.mean(1)
        rng2 = np.random.default_rng(20261024)
        bm = np.stack([Y[i][rng2.integers(0, n, size=(B, n))].mean(1) for i in range(g)], 1)
        mm_ = bm.max(1) / bm.min(1)
        cell["max_over_min"] = float(mx.max() / mx.min())
        cell["max_over_min_ci95"] = pct(mm_, (2.5, 97.5))
        hm["curves"][cname][arm] = cell
        pv[arm].append((cname, cell["p_perm"], cell.get("p_parametric", 1.0)))
summ = {}
for arm in pv:
    names = [x[0] for x in pv[arm]]
    a1 = holm([x[1] for x in pv[arm]]); a2 = holm([x[2] for x in pv[arm]])
    for nme, h1, h2 in zip(names, a1, a2):
        hm["curves"][nme][arm]["p_holm_perm"] = float(h1); hm["curves"][nme][arm]["p_holm_parametric"] = float(h2)
    summ[arm] = {"n_sig_perm_10000": int((a1 < 0.05).sum()), "n_sig_parametric": int((a2 < 0.05).sum())}
hm["summary"] = summ


def hm_verdict(nic, nneg, nb):
    if nic >= 6 and nneg <= 1 and nb <= 1:
        return "supported for IC"
    if nic <= 1:
        return "refuted"
    return "inconclusive"


hm["verdict_perm_10000"] = hm_verdict(summ["ic"]["n_sig_perm_10000"], summ["rho-neg"]["n_sig_perm_10000"], summ["bsgs"]["n_sig_perm_10000"])
hm["verdict_parametric"] = hm_verdict(summ["ic"]["n_sig_parametric"], summ["rho-neg"]["n_sig_parametric"], summ["bsgs"]["n_sig_parametric"])
# high-precision permutation p for every ic curve (1,000,000 shuffles, separate seed) -> Holm count
prec = {}
for (V, v) in curves:
    cname = f"{V}:{v}"
    d = Mrows[(Mrows.V == V) & (Mrows.chunk == v) & (Mrows.arm == "ic")].sort_values(["model", "target_index", "round"])
    Y = np.stack([g.S.to_numpy() for _, g in d.groupby("model")])
    g, n = Y.shape
    F = hm["curves"][cname]["ic"]["F"]
    rng = np.random.default_rng([20261106, v, 1 if V == "V5" else 2])
    flat = Y.ravel(); cnt = 0; NP = 1_000_000
    for b0 in range(0, NP, 5000):
        P = rng.permuted(np.broadcast_to(flat, (5000, flat.size)), axis=1).reshape(5000, g, n)
        a_, w_ = oneway_F(P)
        cnt += int((a_ / w_ >= F * (1 - 1e-12)).sum())
    ph = (1 + cnt) / (1 + NP)
    prec[cname] = {"p": ph, "mc_se": math.sqrt(ph * (1 - ph) / NP)}
names = list(prec)
adj = holm([prec[k]["p"] for k in names])
order = np.argsort([prec[k]["p"] for k in names], kind="stable")
for rank, i in enumerate(order):
    prec[names[i]]["rank"] = rank + 1
    prec[names[i]]["holm_multiplier"] = len(names) - rank
for k, a in zip(names, adj):
    prec[k]["p_holm"] = float(a)
    prec[k]["p_holm_mc_se"] = prec[k]["holm_multiplier"] * prec[k]["mc_se"]
hm["ic_precise_1e6"] = prec
hm["ic_precise_1e6_n_sig"] = int((adj < 0.05).sum())
OUT["H-M"] = hm

# ---------------------------------------------------------------- IC against matched rho (s9)
rng = np.random.default_rng(20261022)
icr = {}
for V in ("V5", "V6"):
    d = m[(m.V == V) & (m.fam.isin(ISO[V])) & (m.arm.isin(["ic", "rho-neg"]))]
    w = d.pivot_table(index=["vertex", "level", "key"], columns="arm", values=["S", "total_gae", "unc"], aggfunc="first").dropna()
    res = {"levels": {}, "n_pairs": int(len(w))}

    def ratios(ww):
        out = {}
        for p in PRICES:
            ic = (ww["S"]["ic"] * (ww["total_gae"]["ic"] + p * ww["unc"]["ic"]) / ww["total_gae"]["ic"])
            rh = (ww["S"]["rho-neg"] * (ww["total_gae"]["rho-neg"] + p * ww["unc"]["rho-neg"]) / ww["total_gae"]["rho-neg"])
            out[str(p)] = (ic.mean() / rh.mean(), ic.to_numpy(), rh.to_numpy())
        return out

    for L in sorted(w.index.get_level_values("level").unique()):
        wl = w.xs(L, level="level")
        rr = ratios(wl)
        cell = {"n_pairs": int(len(wl))}
        groups = [np.column_stack([ratios(g)[str(p)][1] for p in PRICES] + [ratios(g)[str(p)][2] for p in PRICES])
                  for _, g in wl.groupby(level="vertex")]
        s, _ = cluster_boot(groups, 2000, rng)
        for i, p in enumerate(PRICES):
            rb = s[:, i] / s[:, 3 + i]
            cell[str(p)] = {"ratio": rr[str(p)][0], "ci95_cluster_2000": pct(rb, (2.5, 97.5))}
        res["levels"][str(L)] = cell
    for nm, sel in (("pooled", w), ("pooled_levels_ge1", w[w.index.get_level_values("level") >= 1])):
        rr = ratios(sel)
        res[nm] = {str(p): rr[str(p)][0] for p in PRICES}
    icr[V] = res
d = m[(m.V == "V5") & (m.vertex == 0) & (m.fam.isin(["D", "N"])) & (m.arm.isin(["ic", "rho-cubic"]))]
w = d.pivot_table(index=["slug", "key"], columns="arm", values=["S", "total_gae", "unc"], aggfunc="first").dropna()
cr = {"n_pairs": int(len(w))}
for p in PRICES:
    ic = w["S"]["ic"] * (w["total_gae"]["ic"] + p * w["unc"]["ic"]) / w["total_gae"]["ic"]
    rh = w["S"]["rho-cubic"] * (w["total_gae"]["rho-cubic"] + p * w["unc"]["rho-cubic"]) / w["total_gae"]["rho-cubic"]
    X = np.column_stack([ic, rh])
    idx = rng.integers(0, len(X), size=(B, len(X)))
    sb = X[idx].sum(1)
    cr[str(p)] = {"ratio": ic.mean() / rh.mean(), "ci95_row": pct(sb[:, 0] / sb[:, 1], (2.5, 97.5)),
                  "mean_priced_S_ic": ic.mean(), "mean_priced_S_rho_cubic": rh.mean()}
icr["V5_crater_ic_over_rho_cubic"] = cr
# M curves: IC / rho-neg per curve (descriptive)
mc = {}
for (V, v) in curves:
    d = Mrows[(Mrows.V == V) & (Mrows.chunk == v)]
    w = d.pivot_table(index=["model", "key"], columns="arm", values=["S", "total_gae", "unc"], aggfunc="first").dropna()
    mc[f"{V}:{v}"] = {str(p): float((w["S"]["ic"] * (w["total_gae"]["ic"] + p * w["unc"]["ic"]) / w["total_gae"]["ic"]).mean() /
                                    (w["S"]["rho-neg"] * (w["total_gae"]["rho-neg"] + p * w["unc"]["rho-neg"]) / w["total_gae"]["rho-neg"]).mean())
                      for p in PRICES}
icr["M_curves"] = mc
OUT["ic_vs_rho"] = icr

# ---------------------------------------------------------------- BSGS identity
bi = {}
for V in ("V5", "V6"):
    fams = ["D", "Ua", "Ub", "S"] if V == "V6" else ["D", "Ua", "Ub"]
    for arm in ("bsgs", "bsgs-neg"):
        d = m[(m.V == V) & (m.fam.isin(fams)) & (m.arm == arm)]
        w = d.pivot_table(index=["vertex", "target_index", "round"], columns="fam", values="total_gae", aggfunc="first")
        full = w.dropna()
        ident = (full.nunique(axis=1) == 1)
        bi[f"{V}|{arm}"] = {"n_triples_all_families": int(len(full)), "fraction_identical": float(ident.mean()),
                            "n_triples_any": int(len(w))}
OUT["bsgs_identity"] = bi

# ---------------------------------------------------------------- curve effect (s4, V6)
ce = {}
for arm in ("rho-neg", "rho-plain", "ic"):
    d = m[(m.V == "V6") & (m.arm == arm)]
    md = d[d.fam == "D"].groupby("vertex").S.mean()
    cc = d[d.fam == "ctl"].groupby(["vertex", "slug"]).S.mean().unstack()
    cc = cc.apply(lambda r: pd.Series(sorted(r.dropna().tolist())[:2] if r.notna().sum() >= 2 else [np.nan, np.nan]), axis=1)
    tab = pd.concat([md.rename("m"), cc], axis=1).dropna()
    tab["level"] = [cen["V6"][v]["level"] for v in tab.index]
    M3 = tab[["m", 0, 1]].to_numpy()
    lv = tab.level.to_numpy()

    def Fstat(M3):
        mv = M3[..., 0]
        num = 0.0; dfn = 0
        for L in np.unique(lv):
            x = mv[..., lv == L]
            num = num + ((x - x.mean(-1, keepdims=True)) ** 2).sum(-1); dfn += x.shape[-1] - 1
        within = ((M3 - M3.mean(-1, keepdims=True)) ** 2).sum((-1, -2)) / (M3.shape[-2] * 2)
        return (num / dfn) / within

    F = Fstat(M3)
    rng = np.random.default_rng(20261023)
    cnt = 0
    for b0 in range(0, B, 1000):
        P = rng.permuted(np.broadcast_to(M3, (1000,) + M3.shape), axis=2)
        cnt += int((Fstat(P) >= F * (1 - 1e-12)).sum())
    ce[arm] = {"F": float(F), "p_perm": (1 + cnt) / (1 + B), "n_vertices": int(len(tab)),
               "min_attainable_holm_p_per_curve": float(len(tab) / (3 * len(tab)))}
OUT["curve_effect_V6"] = ce

# ---------------------------------------------------------------- wall admission (s8; F4)
wa = {}
for V in ("V5", "V6"):
    for nm, fams in (("isogenous", ISO[V]), ("isogenous+ctl", ISO[V] + ["ctl"])):
        d = m[(m.V == V) & (m.fam.isin(fams)) & (m.arm.isin(["rho-neg", "rho-neg-aa"]))]
        w = d.pivot_table(index=["slug", "key"], columns="arm", values="solve_wall_ns", aggfunc="first").dropna()
        fr = {arm: float((m[(m.V == V) & (m.fam.isin(fams)) & (m.arm == arm)].iso == "L2").mean()) for arm in ARMS5}
        wa[f"{V}|{nm}"] = {"aa_ratio": float(w["rho-neg-aa"].mean() / w["rho-neg"].mean()), "n_pairs": int(len(w)),
                           "frac_L2_per_arm": fr}
    wa[f"{V}|all_rows_measured_frac_L2"] = float((m[m.V == V].iso == "L2").mean())
    wa[f"{V}|all_rows_incl_warmup_frac_L2"] = float((df[df.V == V].iso == "L2").mean())
OUT["wall"] = wa

# ---------------------------------------------------------------- mixed model ICC (s3 secondary; ic only)
try:
    import statsmodels.formula.api as smf
    mmres = {}
    for V in ("V6", "V5"):
        d = m[(m.V == V) & (m.fam.isin(ISO[V])) & (m.arm == "ic")].copy()
        d["logS"] = np.log(d.S)
        fit = smf.mixedlm("logS ~ C(level) + C(fam)", d, groups=d["vertex"]).fit(reml=True)
        vc_ = float(fit.cov_re.iloc[0, 0]); res_ = float(fit.scale)
        mmres[f"{V}|ic"] = {"icc": vc_ / (vc_ + res_), "converged": bool(fit.converged)}
    OUT["mixed_model"] = mmres
except Exception as e:  # reported, never fatal
    OUT["mixed_model"] = {"error": repr(e)}

json.dump(OUT, open(out_path, "w"), indent=1, default=float)
print("done")
