#!/usr/bin/env python3
"""Independent recompute of every reading in analysis.json / tables.md / RESULTS.md from the raw rows.

Written by the validator; shares no code with analyze.py.  Keys are read with [] so a missing field
raises instead of comparing None == None (lesson L-99ee952350b2).  Point estimates are exact
recomputations; intervals come from the validator's own hierarchical bootstrap (curves, then targets
within curve, then seeds within target; numpy PCG64, 4000 resamples, its own seed), so they are
compared with the producer's within Monte Carlo error, not bit for bit.
Writes validation/out/recompute.json.
"""
import gzip, json, math, os
import numpy as np

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
PO = os.path.join(os.path.dirname(S), "genus4_prime_order_20261118")
WS = [0.0, 0.1, 1.0]
NB = 4000
rng = np.random.Generator(np.random.PCG64(0x5EED_6C5C))


def read(path):
    op = gzip.open if path.endswith(".gz") else open
    return [json.loads(l) for l in op(path, "rt") if l.strip()]


def cell(path, g, p, curves, cc, sge, ntargets=6, nseeds=2):
    rows = read(path)
    cfg = [r for r in rows if r["kind"] == "config"]
    assert len(cfg) == 1, path
    cfg = cfg[0]
    inst = {r["curve_seed"]: r for r in rows if r["kind"] == "instance"}
    runs = [r for r in rows if r["kind"] == "run"]
    checks = {
        "config_cc": cfg["cc"] == cc, "config_sge": cfg["sge"] == [{"w_max": sge[0], "fill_max": sge[1]}],
        "config_g_p": cfg["genera"] == [g] and cfg["primes"] == [p] and cfg["max_h"] == 1 and cfg["arms"] == "x",
        "arm_label": all(r["arm"] == f"C_double_lp+pre+coord+unitT+ferrari+sgecoord+sge({sge[0]},{sge[1]})" for r in runs),
        "row_g_p": all(r["g"] == g and r["p"] == p and r["h"] == 1 for r in runs),
        "row_r_eq_instance": all(r["r"] == inst[r["curve_seed"]]["r"] for r in runs),
        "row_sge_fields": all(r["w_max"] == sge[0] and r["fill_max"] == sge[1] for r in runs),
    }
    keys = sorted((r["curve_seed"], r["target_seed"], r["seed"]) for r in runs)
    design = sorted((c, c + k * 1_000_003, s) for c in curves for k in range(ntargets) for s in range(1, nseeds + 1))
    checks["keys_eq_design"] = keys == design
    checks["no_duplicate_keys"] = len(set(keys)) == len(keys)
    checks["all_certified_ok"] = all(r["certified"] is True and r["ok"] is True for r in runs)
    checks["none_exhausted"] = all(r["exhausted"] is False for r in runs)
    checks["fb_cols_single_value_per_curve"] = all(len({r["fb_cols"] for r in runs if r["curve_seed"] == c}) == 1 for c in curves)
    by = {(r["curve_seed"], r["target_seed"], r["seed"]): r for r in runs}
    return {"by": by, "inst": inst, "curves": curves, "checks": checks, "keys": design}


def native(r):
    return r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]


def cost(r, w):
    return r["group_ops"] + w * native(r)


def Rneg(r):
    return math.sqrt(math.pi * r / 4)


def arrays(c):
    """Per-key arrays in a fixed order with the hierarchy indices."""
    keys = c["keys"]
    G = np.array([c["by"][k]["group_ops"] for k in keys], dtype=float)
    N = np.array([native(c["by"][k]) for k in keys], dtype=float)
    LA = np.array([c["by"][k]["la_mul"] + c["by"][k]["la_add"] for k in keys], dtype=float)
    R = np.array([Rneg(c["inst"][k[0]]["r"]) for k in keys])
    return keys, G, N, LA, R


def boot_index(keys, nb):
    """nb hierarchical resamples, each a list of key indices (curves, then targets, then seeds)."""
    curves = sorted({k[0] for k in keys})
    tg = {c: sorted({k[1] for k in keys if k[0] == c}) for c in curves}
    sd = {(c, t): [i for i, k in enumerate(keys) if k[0] == c and k[1] == t] for c in curves for t in tg[c]}
    out = []
    for _ in range(nb):
        idx = []
        for c in rng.choice(curves, size=len(curves)):
            c = int(c)
            for t in rng.choice(tg[c], size=len(tg[c])):
                lst = sd[(c, int(t))]
                idx += [lst[j] for j in rng.integers(0, len(lst), size=len(lst))]
        out.append(np.array(idx))
    return out


def summarize(c):
    keys, G, N, LA, R = arrays(c)
    bi = boot_index(keys, NB)
    res = {"keys": len(keys), "curves": c["curves"], "mean_r": float(np.mean([c["inst"][x]["r"] for x in c["curves"]])),
           "neg": {}, "boot": {}}
    for w in WS:
        C = G + w * N
        ratio = C.mean() / R.mean()
        bs = np.array([C[i].mean() / R[i].mean() for i in bi])
        res["neg"][str(w)] = {"ratio": float(ratio), "lo": float(np.quantile(bs, 0.025)), "hi": float(np.quantile(bs, 0.975)),
                              "paired_key_ratio_median": float(np.median(C / R)),
                              "paired_key_ratio_min": float((C / R).min()), "paired_key_ratio_max": float((C / R).max())}
        res["boot"][str(w)] = bs
    res["below_all_w"] = all(res["neg"][str(w)]["hi"] < 1 for w in WS)
    res["below_all_w_point"] = all(res["neg"][str(w)]["ratio"] < 1 for w in WS)
    res["mean_cost_w1"] = float((G + N).mean())
    res["la_share_w1"] = float(LA.mean() / (G + N).mean())
    res["mean_R"] = float(R.mean())
    res["per_curve_w1"] = {str(cv): float(np.mean([(G + N)[i] for i, k in enumerate(keys) if k[0] == cv])
                                          / Rneg(c["inst"][cv]["r"])) for cv in c["curves"]}
    res["gae_band_mean"] = {str(w): {"ic": float((G + w * N).mean()), "rho_ideal_negation": float(R.mean())} for w in WS}
    # sensitivity: also charge one per-step partition hash per walk step at w (the IC rows carry walk_ops)
    H = np.array([c["by"][k]["walk_ops"] for k in keys], dtype=float)
    res["hash_priced_w1_ratio"] = float((G + N + H).mean() / R.mean())
    return res


curves = json.load(open(os.path.join(S, "curves.json")))
po_curves = json.load(open(os.path.join(PO, "curves.json")))
cells = {}
for p, nm_x, nm_s in ((16411, "b_p16411_x", "b_p16411_xstar"), (32771, "b_p32771_x", "b_p32771_xstar")):
    cs = sorted({r["curve_seed"] for r in read(os.path.join(PO, nm_x + ".jsonl.gz")) if r["kind"] == "run"})
    cells[(4, p, "x")] = cell(os.path.join(PO, nm_x + ".jsonl.gz"), 4, p, cs, 2.0, (4, 1.5))
    cells[(4, p, "x*")] = cell(os.path.join(PO, nm_s + ".jsonl.gz"), 4, p, cs, 1.4, (4, 1.5))
c4 = curves["g4_p54293"]["confirm"]
cells[(4, 54293, "x")] = cell(os.path.join(S, "b4_p54293_cc2.0.jsonl"), 4, 54293, c4, 2.0, (4, 1.5))
cells[(4, 54293, "x*")] = cell(os.path.join(S, "b4_p54293_cc1.4.jsonl"), 4, 54293, c4, 1.4, (4, 1.5))
for p in (2357, 4099):
    c5 = curves[f"g5_p{p}"]["confirm"]
    cells[(5, p, "x5")] = cell(os.path.join(S, f"b5_p{p}_x5.jsonl"), 5, p, c5, 2.0, (4, 1.5))
    cells[(5, p, "x5*")] = cell(os.path.join(S, f"b5_p{p}_x5star.jsonl"), 5, p, c5, 2.8, (8, 2))

out = {"cells": {}, "checks": {}}
summ = {}
for k, c in cells.items():
    name = f"g{k[0]}_p{k[1]}_{k[2]}"
    out["checks"][name] = c["checks"]
    s = summarize(c)
    summ[k] = s
    out["cells"][name] = {x: v for x, v in s.items() if x != "boot"}


def ols(xs, ys):
    xs, ys = np.asarray(xs, float), np.asarray(ys, float)
    mx, my = xs.mean(), ys.mean()
    b = ((xs - mx) * (ys - my)).sum() / ((xs - mx) ** 2).sum()
    return float(b), float(my - b * mx)


def ci(v):
    return [float(np.quantile(v, 0.025)), float(np.quantile(v, 0.975))]


P4 = [16411, 32771, 54293]
lx = np.log(P4)
out["C1"] = {}
for arm in ("x", "x*"):
    ys = [math.log(summ[(4, p, arm)]["neg"]["1.0"]["ratio"]) for p in P4]
    b, a = ols(lx, ys)
    bs = [ols(lx, [math.log(summ[(4, p, arm)]["boot"]["1.0"][j]) for p in P4])[0] for j in range(NB)]
    ce, _ = ols(lx, [math.log(summ[(4, p, arm)]["mean_cost_w1"]) for p in P4])
    Rm = {p: summ[(4, p, arm)]["mean_R"] for p in P4}
    ces = [ols(lx, [math.log(summ[(4, p, arm)]["boot"]["1.0"][j] * Rm[p]) for p in P4])[0] for j in range(NB)]
    # two-point slopes and the slope that 1.5 would imply
    out["C1"][arm] = {"slope_ratio_vs_neg_w1": b, "slope_ci": ci(bs), "cost_exponent": ce, "cost_exponent_ci": ci(ces),
                      "middle_residual": ys[1] - (a + b * lx[1]),
                      "consistent_with_1p5_rule": ci(ces)[0] <= 1.6 and ci(ces)[1] >= 1.4,
                      "ci_contains_1p5": ci(ces)[0] <= 1.5 <= ci(ces)[1],
                      "two_point_cost_exponents": [float((math.log(summ[(4, P4[i + 1], arm)]["mean_cost_w1"]) - math.log(summ[(4, P4[i], arm)]["mean_cost_w1"]))
                                                         / (lx[i + 1] - lx[i])) for i in range(2)],
                      "ln_r_slope_in_ln_p": ols(lx, [math.log(summ[(4, p, arm)]["mean_R"]) for p in P4])[0]}

out["C2"] = {}
for g5p, g4p in ((2357, 16411), (4099, 32771)):
    a5, a4 = summ[(5, g5p, "x5*")], summ[(4, g4p, "x*")]
    r = a5["neg"]["1.0"]["ratio"] / a4["neg"]["1.0"]["ratio"]
    bs = a5["boot"]["1.0"] / a4["boot"]["1.0"]
    out["C2"][f"N_g5p{g5p}_vs_g4p{g4p}"] = {"x5star_over_xstar_w1": r, "ci": ci(bs),
                                             "verdict": "genus 5 ahead" if ci(bs)[1] < 1 else "genus 5 behind" if ci(bs)[0] > 1 else "unresolved",
                                             "N_g5": a5["mean_r"], "N_g4": a4["mean_r"], "N_rel_diff": a5["mean_r"] / a4["mean_r"] - 1,
                                             "x5_over_xstar_w1": summ[(5, g5p, "x5")]["neg"]["1.0"]["ratio"] / a4["neg"]["1.0"]["ratio"],
                                             "at_w": {str(w): summ[(5, g5p, "x5*")]["neg"][str(w)]["ratio"] / a4["neg"][str(w)]["ratio"] for w in WS}}
for g, (pa, pb), arm in ((5, (2357, 4099), "x5*"), (4, (16411, 32771), "x*"), (5, (2357, 4099), "x5"), (4, (16411, 32771), "x")):
    A, B = summ[(g, pa, arm)], summ[(g, pb, arm)]
    dn = math.log(B["mean_r"]) - math.log(A["mean_r"])
    sl = (math.log(B["neg"]["1.0"]["ratio"]) - math.log(A["neg"]["1.0"]["ratio"])) / dn
    bs = (np.log(B["boot"]["1.0"]) - np.log(A["boot"]["1.0"])) / dn
    out["C2"][f"slope_in_lnN_g{g}_{arm}"] = {"slope": sl, "ci": ci(bs)}
out["theory"] = {"g4_cost_exp_in_p": 2 - 2 / 4, "g5_cost_exp_in_p": 2 - 2 / 5,
                 "g4_exp_in_N": (2 - 2 / 4) / 4, "g5_exp_in_N": (2 - 2 / 5) / 5,
                 "g4_ratio_slope_in_lnN": (2 - 2 / 4) / 4 - 0.5, "g5_ratio_slope_in_lnN": (2 - 2 / 5) / 5 - 0.5}

# tuning (PROTOCOL section 3), recomputed from the tune rows
tune = {}
tune_keys_all = set()
for cc in ("1.0", "1.4", "2.0", "2.8"):
    rows = read(os.path.join(S, f"tune5_cc{cc}.jsonl"))
    cfg = [r for r in rows if r["kind"] == "config"][0]
    runs = [r for r in rows if r["kind"] == "run"]
    for w_, fm in ((4, 1.5), (8, 2)):
        lbl = f"C_double_lp+pre+coord+unitT+ferrari+sgecoord+sge({w_},{fm})"
        rr = [r for r in runs if r["arm"] == lbl]
        keys = sorted((r["curve_seed"], r["target_seed"], r["seed"]) for r in rr)
        tune_keys_all |= {k[0] for k in keys}
        design = sorted((c, c + k * 1_000_003, s) for c in curves["g5_p2357"]["tune"] for k in range(3) for s in (1, 2))
        tune[f"{cc}|{w_},{fm}"] = {"cc": cc, "w_max": w_, "fill_max": fm, "runs": len(rr), "keys_eq_design": keys == design,
                                   "config_cc": cfg["cc"] == float(cc), "eligible": len(rr) == 12 and all(r["certified"] is True and r["ok"] is True for r in rr),
                                   "mean_cost_w1": sum(cost(r, 1) for r in rr) / len(rr)}
dist = lambda v: abs(float(v["cc"]) - 2.0) + abs(v["w_max"] - 4) / 4 + abs(v["fill_max"] - 1.5)
best = min((v for v in tune.values() if v["eligible"]), key=lambda v: (v["mean_cost_w1"], dist(v)))
srt = sorted(tune.values(), key=lambda v: v["mean_cost_w1"])
out["tune"] = {"grid": tune, "choice": best, "runner_up": srt[1], "choice_margin_rel": srt[1]["mean_cost_w1"] / srt[0]["mean_cost_w1"] - 1,
               "curves_seen_by_tuning": sorted(tune_keys_all),
               "tuning_disjoint_from_confirmation": not (tune_keys_all & (set(curves["g5_p2357"]["confirm"]) | set(curves["g5_p4099"]["confirm"]))),
               "best_is_grid_edge_cc": best["cc"] == "2.8", "P3_cc_le_1p4": float(best["cc"]) <= 1.4}

# G3 measured plain rho at genus 5, p = 2357
Q = [r for r in read(os.path.join(S, "rho5_p2357.jsonl")) if r["kind"] == "run"]
inst5 = {r["curve_seed"]: r for r in read(os.path.join(S, "rho5_p2357.jsonl")) if r["kind"] == "instance"}
qr = float(np.mean([r["group_ops"] for r in Q])) / float(np.mean([math.sqrt(math.pi * inst5[r["curve_seed"]]["r"] / 2) for r in Q]))
out["G3"] = {"runs": len(Q), "certified_ok": all(r["certified"] is True and r["ok"] is True for r in Q),
             "keys": sorted((r["curve_seed"], r["target_seed"], r["seed"]) for r in Q),
             "mean_over_sqrt_pi_r_2": float(qr), "in_0p5_1p8": 0.5 <= qr <= 1.8,
             "per_run": [r["group_ops"] / math.sqrt(math.pi * inst5[r["curve_seed"]]["r"] / 2) for r in Q],
             "unpriced_listed": {k: [r[k] for r in Q] for k in ("hash_uncharged", "coef_adds_uncharged", "compares_uncharged")}}
out["G1"] = all(v["all_certified_ok"] and v["keys_eq_design"] for v in out["checks"].values())
out["all_cell_checks_pass"] = {k: all(v.values()) for k, v in out["checks"].items()}
json.dump(out, open(os.path.join(V, "out", "recompute.json"), "w"), indent=1)

for k, v in out["cells"].items():
    n = v["neg"]
    print(f"{k:16s} keys={v['keys']} r={v['mean_r']:.4e} w0={n['0.0']['ratio']:.4f} w.1={n['0.1']['ratio']:.4f} "
          f"w1={n['1.0']['ratio']:.4f} [{n['1.0']['lo']:.4f},{n['1.0']['hi']:.4f}] below={v['below_all_w']} LA={v['la_share_w1']:.3f}")
print(json.dumps(out["C1"], indent=1))
print(json.dumps(out["C2"], indent=1))
print(json.dumps({k: out["tune"][k] for k in out["tune"] if k != "grid"}, indent=1))
print(json.dumps(out["G3"], indent=1))
print("cell checks:", out["all_cell_checks_pass"])

# ---- curvature of the genus-4 fit: bootstrap of the middle residual and of the two local exponents
curv = {}
for arm in ("x", "x*"):
    res_b, e1_b, e2_b = [], [], []
    Rm = {p: summ[(4, p, arm)]["mean_R"] for p in P4}
    for j in range(NB):
        ys = [math.log(summ[(4, p, arm)]["boot"]["1.0"][j]) for p in P4]
        b, a = ols(lx, ys)
        res_b.append(ys[1] - (a + b * lx[1]))
        lc = [math.log(summ[(4, p, arm)]["boot"]["1.0"][j] * Rm[p]) for p in P4]
        e1_b.append((lc[1] - lc[0]) / (lx[1] - lx[0]))
        e2_b.append((lc[2] - lc[1]) / (lx[2] - lx[1]))
    curv[arm] = {"middle_residual_ci": ci(res_b), "local_exponent_16411_32771_ci": ci(e1_b),
                 "local_exponent_32771_54293_ci": ci(e2_b),
                 "local_exponent_difference_ci": ci(np.array(e2_b) - np.array(e1_b))}
out["C1_curvature"] = curv
json.dump(out, open(os.path.join(V, "out", "recompute.json"), "w"), indent=1)
print(json.dumps(curv, indent=1))

# ---- C2 with the genus-5 arm that actually uses large primes (x5, cc 2.0): x5* (cc 2.8) saturates the
# factor base at both genus-5 primes (every degree-1 place, rels_by_lp = [n, 0, 0]), so it is the
# full-factor-base algorithm; this reading keeps the double-large-prime comparison the protocol describes.
sat = {}
for k, c in cells.items():
    if k[0] != 5:
        continue
    lp = [0, 0, 0]
    for r in c["by"].values():
        for i, v in enumerate(r["rels_by_lp"]):
            lp[i] += v
    sat[f"g{k[0]}_p{k[1]}_{k[2]}"] = {"fb_cols": sorted({r["fb_cols"] for r in c["by"].values()}), "rels_by_lp_sum": lp,
                                       "large_prime_relations": lp[1] + lp[2]}
out["g5_factor_base"] = sat
out["C2_x5_dlp"] = {}
for g5p, g4p in ((2357, 16411), (4099, 32771)):
    a5, a4 = summ[(5, g5p, "x5")], summ[(4, g4p, "x*")]
    bs = a5["boot"]["1.0"] / a4["boot"]["1.0"]
    out["C2_x5_dlp"][f"N_g5p{g5p}_vs_g4p{g4p}"] = {
        "x5_over_xstar_w1": a5["neg"]["1.0"]["ratio"] / a4["neg"]["1.0"]["ratio"], "ci": ci(bs),
        "at_w": {str(w): a5["neg"][str(w)]["ratio"] / a4["neg"][str(w)]["ratio"] for w in WS},
        "at_w_ci": {str(w): ci(a5["boot"][str(w)] / a4["boot"][str(w)]) for w in WS}}
json.dump(out, open(os.path.join(V, "out", "recompute.json"), "w"), indent=1)
print(json.dumps(out["g5_factor_base"], indent=1))
print(json.dumps(out["C2_x5_dlp"], indent=1))
