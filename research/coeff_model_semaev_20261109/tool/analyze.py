#!/usr/bin/env python3
"""Frozen analysis for the coefficient/model Semaev study (PROTOCOL.md section 5).

usage: analyze.py <rows.jsonl> <meta.json> <out.json>
"""
import json, sys, math
import numpy as np

SEED = 20261109
NBOOT = 10000
NPERM = 10000
MARGIN = {"C40-S3": 0.10, "C48-S3": 0.15}


def load(path):
    base, rows = {}, []
    for l in open(path):
        r = json.loads(l)
        if r["kind"] == "base":
            base[r["arm"]] = r
        else:
            rows.append(r)
    return base, rows


def holm(ps):
    order = sorted(range(len(ps)), key=lambda i: ps[i])
    adj = [0.0] * len(ps)
    run = 0.0
    m = len(ps)
    for rank, i in enumerate(order):
        run = max(run, min(1.0, (m - rank) * ps[i]))
        adj[i] = run
    return adj


def main():
    rows_path, meta_path, out_path = sys.argv[1:4]
    meta = json.load(open(meta_path))
    cell = meta["cell"]
    base, rows = load(rows_path)
    mode = meta["mode"]
    rng = np.random.default_rng(SEED)

    # cross-check
    xc = [r for r in rows if r["y_gl"] is not None and r["y_rf"] is not None]
    mism = [r for r in xc if r["y_gl"] != r["y_rf"]]

    arms = sorted(base)
    per = {}
    vals = {}
    for a in arms:
        rs = sorted([r for r in rows if r["arm"] == a], key=lambda r: r["t"])
        b = base[a]
        ng, fp = b["ngroup"], b["fpts"]
        if mode == "s4":
            y = np.array([r["y4"] for r in rs], float)
            norm = ng / fp**3
        else:
            y = np.array([(r["y_rf"] if r["y_rf"] is not None else r["y_gl"]) for r in rs], float)
            norm = ng / fp**2
        yn = y * norm
        vals[a] = yn
        muls = np.array([r["muls"] for r in rs], float)
        ns = np.array([r["ns"] for r in rs], float)
        per[a] = {
            "targets": len(rs), "planted_targets": sum(r["planted"] for r in rs),
            "fx": b["fx"], "fpts": fp, "expected_y": fp**(3 if mode == "s4" else 2) / ng,
            "y_total": int(y.sum()), "y_mean": float(y.mean()) if len(y) else None,
            "ynorm_mean": float(yn.mean()) if len(y) else None,
            "ynorm_se": float(yn.std(ddof=1) / math.sqrt(len(yn))) if len(yn) > 1 else None,
            "muls_per_target_mean": float(muls.mean()) if len(rs) else None,
            "muls_per_target_min": float(muls.min()) if len(rs) else None,
            "muls_per_target_max": float(muls.max()) if len(rs) else None,
            "muls_per_base_element": float(muls.mean() / b["fx"]) if len(rs) and mode == "s3" else None,
            "sqrt_per_target_mean": float(np.mean([r["sqrt"] for r in rs])) if rs else None,
            "jacobi_per_target_mean": float(np.mean([r["jacobi"] for r in rs])) if rs else None,
            "ns_per_target_median_descriptive": float(np.median(ns)) if len(rs) else None,
            "ns_per_base_element_median_descriptive": float(np.median(ns) / b["fx"]) if len(rs) and mode == "s3" else None,
        }

    boots = {a: np.array([rng.choice(vals[a], len(vals[a])).mean() for _ in range(NBOOT)]) for a in arms if len(vals[a])}

    def group_mean(names):
        return float(np.mean([vals[a].mean() for a in names]))

    def contrast(num, den):
        est = group_mean(num) / group_mean(den)
        bn = np.mean([boots[a] for a in num], axis=0)
        bd = np.mean([boots[a] for a in den], axis=0)
        with np.errstate(divide='ignore', invalid='ignore'):
            r = bn / bd
        r = r[np.isfinite(r)]
        lo, hi = np.quantile(r, [0.025, 0.975])
        p = 2 * min((r <= 1).mean(), (r >= 1).mean())
        return {"num": num, "den": den, "ratio": est, "ci95": [float(lo), float(hi)], "p_boot": float(min(1.0, p))}

    out = {"cell": cell, "mode": mode, "rows": len(rows), "crosscheck_targets": len(xc),
           "crosscheck_mismatches": len(mism), "cell_void": len(mism) > 0, "per_arm": per}

    if "side" in cell or "W" in arms:
        ref = [a for a in ("W", "W_AA") if a in arms]
        fam = {a: contrast([a], ["W"] if a == "W_AA" else ref) for a in arms if a not in ref or a == "W_AA"}
    else:
        rand = [a for a in arms if a.startswith("RAND")]
        small = [a for a in arms if a.startswith("SMALL") and not a.endswith("_ISO")]
        out["P1_small_vs_rand"] = contrast(small, rand) if small and rand else None
        fam = {}
        for a in arms:
            if a.startswith(("SMALL", "A3_", "E0_ISO")) or a == "E0":
                fam[a + "_vs_RAND"] = contrast([a], rand)
        for a in arms:
            if a.startswith("SMALL") and not a.endswith("_ISO") and a + "_ISO" in arms:
                fam["P3_" + a + "_vs_ISO"] = contrast([a], [a + "_ISO"])
        out["AA"] = contrast(["E0_AA"], ["E0"]) if "E0_AA" in arms else None
        out["positive_control"] = contrast(["PLANTED"], ["E0"]) if "PLANTED" in arms else None
        if out["positive_control"]:
            out["positive_control"]["detected"] = out["positive_control"]["ci95"][0] > 1.0
    keys = list(fam)
    adj = holm([fam[k]["p_boot"] for k in keys])
    for k, pa in zip(keys, adj):
        fam[k]["p_holm"] = pa
    out["family"] = fam

    # heterogeneity permutation test over non-planted arms
    het_arms = [a for a in arms if a != "PLANTED" and len(vals[a])]
    allv = np.concatenate([vals[a] for a in het_arms])
    sizes = [len(vals[a]) for a in het_arms]
    def between_var(v):
        ms, i = [], 0
        for s in sizes:
            ms.append(v[i:i + s].mean()); i += s
        return float(np.var(ms))
    obs = between_var(allv)
    cnt = 0
    for _ in range(NPERM):
        if between_var(rng.permutation(allv)) >= obs:
            cnt += 1
    out["heterogeneity"] = {"arms": het_arms, "between_arm_var": obs, "p_perm": (cnt + 1) / (NPERM + 1)}

    # counted cost per base element (exact): spread across arms
    if mode == "s3":
        mpe = {a: per[a]["muls_per_base_element"] for a in arms if per[a]["muls_per_base_element"]}
        if mpe:
            out["muls_per_element_range"] = [min(mpe.values()), max(mpe.values())]

    # decision (main cells only)
    if cell in MARGIN and out.get("P1_small_vs_rand"):
        m = MARGIN[cell]
        p1 = out["P1_small_vs_rand"]
        flat = (p1["ci95"][0] >= 1 - m and p1["ci95"][1] <= 1 + m
                and all(v["p_holm"] >= 0.05 for v in fam.values())
                and out["positive_control"]["detected"] and not out["cell_void"])
        effect = (p1["ci95"][0] > 1 or p1["ci95"][1] < 1) and not out["cell_void"]
        out["decision"] = "flat" if flat else ("effect (needs replication)" if effect else "inconclusive")
        out["margin"] = m
    json.dump(out, open(out_path, "w"), indent=1)
    print(json.dumps({k: out[k] for k in out if k not in ("per_arm", "family")}, indent=1))


if __name__ == "__main__":
    main()
