#!/usr/bin/env python3
"""Fit the n=3 cost terms from raw/*.json and extrapolate the crossover with matched rho.
Usage: python3 analyze.py   (writes results.json, tables.md, figure.svg)"""
import json, glob, math, os, hashlib
import numpy as np
rng = np.random.default_rng(20261119)
HERE = os.path.dirname(os.path.abspath(__file__))
FP_PER_ADD = 63.0
LA_EQ = 9.0

def load():
    rows = []
    for f in sorted(glob.glob(os.path.join(HERE, "raw", "*.json"))):
        arm = os.path.basename(f).split("_")[0]
        for r in json.load(open(f)):
            g, ho = r["gaudry"], r["rho"]
            if not g.get("solved") or not g.get("correct"):
                continue
            R = g["residuals"]
            rows.append(dict(arm=arm, p=g["p"], seed=int(f.split("_s")[-1].split(".")[0]),
                bits=g["bits"], base=g["base"], small=g.get("small_base", 0), R=R,
                dec=g["decompositions"], rate=g["decomposition_rate"],
                c3=(g["oracle_fp_muls"] + g["precompute_fp_muls"]) / R,
                grp=g["group_ops"] / R, la=max(g["la_ops"], 1), S=g["s"], total=g["total_ops"],
                rhoS=ho["s"], wall=g["wall_ms"] / 1e3, wt=g.get("la_avg_weight", 0)))
    return rows

def ols(x, y):
    A = np.vstack([x, np.ones_like(x)]).T
    sl, ic = np.linalg.lstsq(A, y, rcond=None)[0]
    return sl, ic

def fit_terms(rows, forced=None):
    """returns dict term -> (exponent, log2 const) for R, cpr (cost per residual in GAE), la."""
    lp = np.log2([r["p"] for r in rows])
    out = {}
    cpr = [ (r["c3"] / FP_PER_ADD + r["grp"]) for r in rows]
    for name, y in (("R", [r["R"] for r in rows]), ("cpr", cpr), ("la", [r["la"] for r in rows])):
        ly = np.log2(y)
        if forced and name in forced:
            e = forced[name]
            out[name] = (e, float(np.mean(ly - e * lp)))
        else:
            out[name] = ols(lp, ly)
    return out

def ratio_log2(t, lpgrid, kappa, kfac=1.0, c3div=1.0):
    """log2 of IC/rho on a grid of log2 p."""
    R = t["R"][1] + t["R"][0] * lpgrid
    cpr = t["cpr"][1] + t["cpr"][0] * lpgrid
    la = t["la"][1] + t["la"][0] * lpgrid
    ic = 2.0 ** R * 2.0 ** cpr / c3div + LA_EQ / FP_PER_ADD * 2.0 ** la
    rho = kappa * kfac * 2.0 ** (1.5 * lpgrid)
    return np.log2(ic / rho)

GRID = np.arange(8.0, 600.0, 0.05)

def crossover(t, kappa, kfac=1.0, c3div=1.0):
    lr = ratio_log2(t, GRID, kappa, kfac, c3div)
    # first grid point at/after which ratio<=1, only if it stays reachable (first crossing)
    idx = np.where(lr <= 0)[0]
    mn = float(lr.min())
    if len(idx) == 0:
        return None, mn, float(GRID[lr.argmin()])
    return float(GRID[idx[0]]), mn, float(GRID[lr.argmin()])

def boot(rows, arm, forced, kappa, kfac=1.0, c3div=1.0, B=4000):
    rs = [r for r in rows if r["arm"] == arm]
    groups = {}
    for r in rs:
        groups.setdefault(r["p"], []).append(r)
    xs, miss = [], 0
    for _ in range(B):
        samp = []
        for p, g in groups.items():
            samp += [g[i] for i in rng.integers(0, len(g), len(g))]
        if len({r["p"] for r in samp}) < 3:
            continue
        t = fit_terms(samp, forced)
        x, mn, _ = crossover(t, kappa, kfac, c3div)
        if x is None:
            miss += 1
        else:
            xs.append(3 * x)  # group bits = 3 log2 p
    xs = np.array(xs)
    if len(xs) == 0:
        return dict(n_cross=0, n_none=miss)
    return dict(n_cross=int(len(xs)), n_none=miss, q025=float(np.percentile(xs, 2.5)),
                q50=float(np.percentile(xs, 50)), q975=float(np.percentile(xs, 97.5)))

def main():
    rows = load()
    res = {"runs": rows}
    summ = {}
    for arm in ("NL", "LP"):
        rs = [r for r in rows if r["arm"] == arm]
        if not rs:
            continue
        ps = sorted({r["p"] for r in rs})
        summ[arm] = {"p": ps, "per_p": {}}
        for p in ps:
            g = [r for r in rs if r["p"] == p]
            summ[arm]["per_p"][p] = dict(n=len(g), R=float(np.mean([r["R"] for r in g])),
                c3=float(np.mean([r["c3"] for r in g])), rate=float(np.mean([r["rate"] for r in g])),
                R_over_base=float(np.mean([r["R"] / r["base"] for r in g])),
                S=float(np.mean([r["S"] for r in g])), rhoS=float(np.mean([r["rhoS"] for r in g])),
                la=float(np.mean([r["la"] for r in g])), wall=float(np.mean([r["wall"] for r in g])))
    res["per_p"] = summ
    theory = {"NL": {"R": 1.0, "cpr": 0.0, "la": 2.0}, "LP": {"R": 4 / 3, "cpr": 0.0, "la": 4 / 3}}
    res["fits"] = {}
    for arm in summ:
        rs = [r for r in rows if r["arm"] == arm]
        free = fit_terms(rs)
        forced = fit_terms(rs, theory[arm])
        res["fits"][arm] = dict(free={k: list(v) for k, v in free.items()},
                                forced={k: list(v) for k, v in forced.items()})
        # exponent bootstrap intervals (free fit)
        groups = {}
        for r in rs: groups.setdefault(r["p"], []).append(r)
        ex = {k: [] for k in ("R", "cpr", "la")}
        for _ in range(4000):
            samp = []
            for p, g in groups.items():
                samp += [g[i] for i in rng.integers(0, len(g), len(g))]
            t = fit_terms(samp)
            for k in ex: ex[k].append(t[k][0])
        res["fits"][arm]["exp_ci"] = {k: [float(np.percentile(v, 2.5)), float(np.percentile(v, 97.5))] for k, v in ex.items()}
    # crossovers
    res["crossover"] = {}
    for arm in summ:
        res["crossover"][arm] = {}
        for fname, forced in (("free", None), ("forced_theory", theory[arm])):
            t = {k: tuple(v) for k, v in res["fits"][arm][fname if fname == "free" else "forced"].items()}
            for kname, kappa in (("rho_negation_0.627", 0.627), ("rho_plain_0.886", 0.886)):
                for kfac in (1.0, 1.1, 2.0):
                    for c3div in (1.0, 3.0, 10.0):
                        x, mn, at = crossover(t, kappa, kfac, c3div)
                        key = f"{fname}|{kname}|rho_x{kfac}|C3_div{c3div:g}"
                        ent = dict(log2_p_star=x, group_bits=None if x is None else 3 * x,
                                   min_log2_ratio=mn, min_at_log2_p=at)
                        if c3div == 1.0 and kfac == 1.0:
                            ent["boot"] = boot(rows, arm, forced, kappa, kfac, c3div, B=2000)
                        res["crossover"][arm][key] = ent
    # ratios at measured sizes
    res["measured_ratio"] = {}
    for arm in summ:
        res["measured_ratio"][arm] = {str(p): dict(S_ic=v["S"], S_rho_theory_plain=0.886, ratio_theory=v["S"] / 0.886,
                                       ratio_negation=v["S"] / 0.627, ratio_vs_measured_rho=v["S"] / v["rhoS"])
                                       for p, v in summ[arm]["per_p"].items()}
    # slope of log S against log p per arm (P7)
    res["S_slope"] = {}
    for arm in summ:
        rs = [r for r in rows if r["arm"] == arm]
        sl, _ = ols(np.log2([r["p"] for r in rs]), np.log2([r["S"] for r in rs]))
        res["S_slope"][arm] = sl
    # raw file hashes
    res["raw_sha256"] = {os.path.basename(f): hashlib.sha256(open(f, "rb").read()).hexdigest()
                         for f in sorted(glob.glob(os.path.join(HERE, "raw", "*.json")))}
    json.dump(res, open(os.path.join(HERE, "results.json"), "w"), indent=1)
    print(json.dumps({k: res[k] for k in ("fits", "S_slope", "measured_ratio")}, indent=1))
    for arm in res["crossover"]:
        for k, v in res["crossover"][arm].items():
            if "rho_x1.0" in k and "C3_div1" in k and "C3_div10" not in k:
                print(arm, k, v)

if __name__ == "__main__":
    main()
