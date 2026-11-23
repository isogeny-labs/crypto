#!/usr/bin/env python3
"""L10 analysis, frozen with PROTOCOL.md: applies the preregistered rules only.

Usage: python3 -I analyze.py <lead dir>   -> <lead dir>/analysis/summary.json
E_w(k,t,G,s) = columns * r * (mpt + w*nat) / covered_points   (F4 mul-adds per modelled solve)
"""
import glob, json, math, os, sys

R = {16: 8209, 18: 32719, 20: 131203, 22: 524731, 24: 2097349}
KS = (16, 18, 20, 22, 24)
RHO_S = 1.5            # rho-neg S taken as 1.5 (brief); GAE = RHO_S * sqrt(r)
T10 = {  # k = 20 cells shared with wave-3 T10 (same binary, same targets): P0 reproduction
    "k20_t5_g4_s0": 52816.0, "k20_t5_g4_s1307": 52816.0, "k20_t5_g3_s1307": 177112.0,
    "k20_t6_g5_s0": 146464.0, "k20_t6_g5_s1308": 146464.0, "k20_t6_g4_s1308": 710095.08,
    "k20_t7_g6_s1594": 417161.31}


def nat_of(d):
    n = d.get("native") or {}
    return sum(v for key, v in n.items() if key.endswith("_uncharged") and isinstance(v, (int, float)))


def main(lead):
    cells = {}
    sel = json.load(open(os.path.join(lead, "data", "seeds.used.json")))["cells"]
    for p in sorted(glob.glob(os.path.join(lead, "data", "cells", "k*_t*_g*_s*.json"))):
        name = os.path.basename(p)[:-5]
        if ".try" in name:
            continue
        try:
            d = json.loads(open(p).read().strip().splitlines()[-1])
        except Exception as e:  # F2 failed cell
            cells[name] = {"failed": str(e)}
            continue
        k, t, g, s = (int(x[1:]) for x in name.split("_"))
        cols, cov = d["columns"], d["covered_points"]
        mpt, nat = d["muladds_per_target"], nat_of(d)
        ok = (not d.get("capped")) and d.get("undecided", 0) == 0
        c = {"k": k, "t": t, "G": g, "seed": s, "columns": cols, "covered": cov, "mpt": mpt, "nat": nat,
             "hits": d.get("hits"), "capped": d.get("capped"), "undecided": d.get("undecided"),
             "solving_degree_max": d.get("solving_degree_max"), "conclusive": ok}
        for w in (0, 0.1, 1):
            c[f"E_{w}"] = cols * R[k] * (mpt + w * nat) / cov
        ref = sel[f"k{k}_t{t}"]["selected" if s else "seed0"]
        c["census_match"] = (ref["columns"] == cols and ref["covered_points"] == cov)
        cells[name] = c
    out = {"cells": cells, "predictions": {}}
    P = out["predictions"]
    good = {n: c for n, c in cells.items() if "failed" not in c}

    # P0: T10 reproduction (k = 20 shared cells) within 1e-3 relative.
    p0 = {n: (good[n]["mpt"], ref, abs(good[n]["mpt"] - ref) / ref) for n, ref in T10.items() if n in good}
    P["P0"] = {"cells": p0, "held": len(p0) == len(T10) and all(v[2] <= 1e-3 for v in p0.values())}
    # P1: census consistency in every cell.
    mism = [n for n, c in good.items() if not c["census_match"]]
    P["P1"] = {"mismatches": mism, "held": len(good) == 40 and not mism}
    # P2: k-invariance of mpt at fixed (t, G) over the 5 k (S* series), 4/4 series within 10 %.
    series = {}
    for (t, g) in ((4, 3), (5, 4), (6, 5), (7, 6), (5, 3), (6, 4)):
        vals = {c["k"]: c["mpt"] for c in good.values() if c["t"] == t and c["G"] == g and c["seed"] != 0}
        if vals:
            series[f"t{t}_g{g}"] = {"mpt_by_k": vals, "n": len(vals),
                                     "max_over_min": max(vals.values()) / min(vals.values())}
    prim = [series.get(f"t{t}_g{t-1}") for t in (4, 5, 6, 7)]
    P["P2"] = {"series": series, "held": all(s and s["n"] == 5 and s["max_over_min"] <= 1.10 for s in prim)}
    # P3: seed invariance (seed 0 vs S*) at G = t-1, t in {5,6}, all 10 pairs within 10 %.
    pairs = {}
    for k in KS:
        for t in (5, 6):
            a = next((c for c in good.values() if c["k"] == k and c["t"] == t and c["G"] == t - 1 and c["seed"] == 0), None)
            b = next((c for c in good.values() if c["k"] == k and c["t"] == t and c["G"] == t - 1 and c["seed"] != 0), None)
            if a and b:
                pairs[f"k{k}_t{t}"] = b["mpt"] / a["mpt"]
    P["P3"] = {"ratios": pairs, "held": len(pairs) == 10 and all(0.9 <= v <= 1.1 for v in pairs.values())}
    # E_best(k) over the 4 (t, G = t-1, S*) cells; argmin t; P4 (t = 4 everywhere, second-best t = 5 by >= 1.5x).
    best, p4 = {}, {}
    for k in KS:
        es = {c["t"]: c for c in good.values() if c["k"] == k and c["G"] == c["t"] - 1 and c["seed"] != 0 and c["conclusive"]}
        if len(es) < 4:
            p4[k] = {"inconclusive": True, "have": sorted(es)}
            continue
        order = sorted(es.values(), key=lambda c: c["E_1"])
        best[k] = order[0]
        p4[k] = {"argmin_t": order[0]["t"], "second_t": order[1]["t"],
                 "second_over_best_E1": order[1]["E_1"] / order[0]["E_1"],
                 "E_1_by_t": {c["t"]: c["E_1"] for c in es.values()}}
    P["P4"] = {"by_k": p4, "held": len(best) == 5 and all(p4[k]["argmin_t"] == 4 and p4[k]["second_t"] == 5
                                                        and p4[k]["second_over_best_E1"] >= 1.5 for k in KS)}
    # P5: exponent of E_best in r (least squares on ln E_best = a + b ln r, w = 1), b in [0.85, 1.15].
    fit = {}
    if len(best) == 5:
        xs = [math.log(R[k]) for k in KS]; ys = [math.log(best[k]["E_1"]) for k in KS]
        mx, my = sum(xs) / 5, sum(ys) / 5
        b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
        a = my - b * mx
        res = [y - (a + b * x) for x, y in zip(xs, ys)]
        fit = {"a": a, "b": b, "rmse_ln": math.sqrt(sum(e * e for e in res) / 5),
               "local_slopes": {f"{KS[i]}-{KS[i+1]}": (ys[i + 1] - ys[i]) / (xs[i + 1] - xs[i]) for i in range(4)}}
        # secondary: exponent at fixed t (each t series, S*, G = t-1)
        fit["b_by_t"] = {}
        for t in (4, 5, 6, 7):
            pts = [(math.log(R[c["k"]]), math.log(c["E_1"])) for c in good.values() if c["t"] == t and c["G"] == t - 1 and c["seed"] != 0]
            if len(pts) == 5:
                mx2 = sum(p[0] for p in pts) / 5; my2 = sum(p[1] for p in pts) / 5
                fit["b_by_t"][t] = sum((x - mx2) * (y - my2) for x, y in pts) / sum((x - mx2) ** 2 for x, _ in pts)
    P["P5"] = {"fit": fit, "held": bool(fit) and 0.85 <= fit["b"] <= 1.15}
    # P6: E_best / (rho-neg GAE) strictly increasing over the five k (reported at w = 1 and w = 0.1).
    ratios = {}
    for k in KS:
        if k in best:
            gae = RHO_S * math.sqrt(R[k])
            # priced F4 work at weight w is w * E_w (F4 mul-adds are unpriced native work in ecbench)
            ratios[k] = {"rho_gae": gae, "ratio_w1": 1.0 * best[k]["E_1"] / gae,
                         "ratio_w0.1": 0.1 * best[k]["E_0.1"] / gae, "ratio_w0": 0.0}
    seq = [ratios[k]["ratio_w1"] for k in KS if k in ratios]
    P["P6"] = {"ratios": ratios, "held": len(seq) == 5 and all(seq[i] < seq[i + 1] for i in range(4))}
    # Derived (reporting, not a prediction): per-t-step growth g(t+1)/g(t) of cols*mpt/cov at S*, and the
    # t-growth that would be needed for exponent 1/2.
    g = {}
    for k in KS:
        for t in (4, 5, 6, 7):
            c = next((c for c in good.values() if c["k"] == k and c["t"] == t and c["G"] == t - 1 and c["seed"] != 0), None)
            if c:
                g[f"k{k}_t{t}"] = c["columns"] * c["mpt"] / c["covered"]
    out["derived"] = {"g_cols_mpt_over_cov": g}
    os.makedirs(os.path.join(lead, "analysis"), exist_ok=True)
    json.dump(out, open(os.path.join(lead, "analysis", "summary.json"), "w"), indent=1, sort_keys=True, default=str)
    print(json.dumps(P, indent=1, sort_keys=True, default=str))


if __name__ == "__main__":
    main(sys.argv[1])
