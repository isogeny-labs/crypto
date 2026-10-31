#!/usr/bin/env python3
"""R1 analysis: per-arm per-p table and fitted exponents of S vs p. Reads sessions/p*/records.jsonl.
Usage: analyse.py [--out analysis.json]. Bootstrap over curves within each p (seed fixed)."""
import glob, json, sys, math, collections
import numpy as np

HERE = __import__("os").path.dirname(__import__("os").path.abspath(__file__))
rows = []
for f in sorted(glob.glob(f"{HERE}/sessions/p*/records.jsonl")):
    for line in open(f):
        r = json.loads(line)
        if r.get("warmup"):
            continue
        cs = r["workload"]["curve_spec"]
        c = r["counters"] or {}
        ph = {p["name"]: p["gae"] for p in r["phases"]}
        det = r.get("detail") or {}
        rows.append(dict(arm=r["arm"], p=cs["p"], seed=cs["seed"], tgt=r["workload"]["target_index"],
                         status=r["outcome"]["status"], ok=bool(r["outcome"]["matches_planted"]) and r["outcome"]["status"] == "verified",
                         S=r["cost"]["s"], gae=r["cost"]["total_gae"], rel=ph.get("relations", 0.0),
                         la=ph.get("linear_algebra", 0.0), fb=ph.get("factor_base", 0.0),
                         solver=det.get("solver_charged_gae", 0.0), c4=det.get("c4_fp_mults"),
                         trials=c.get("trials"), full=c.get("full_relations"), used=c.get("relations_used"),
                         cols=c.get("columns"), small=c.get("small_columns"), core=c.get("core_unknowns"),
                         wied=c.get("wiedemann_modr_mults"), merge=c.get("merge_modr_mults"),
                         attempts=c.get("la_attempts"), rsq=r["boundaries"].get("floor_s")))
ARMS = ["rho-neg", "plain", "d4", "2lp", "d4-2lp"]
PS = sorted({r["p"] for r in rows})
out = {"n_records": len(rows), "n_verified": sum(r["ok"] for r in rows), "p": PS}
bad = [r for r in rows if not r["ok"]]
out["unverified"] = [(r["arm"], r["p"], r["seed"], r["tgt"], r["status"]) for r in bad]

def curve_means(arm, key, c4scale=None):
    d = collections.defaultdict(list)
    for r in rows:
        if r["arm"] != arm or not r["ok"]:
            continue
        if key == "S_rep":  # reprice S at c4*c4scale
            v = r["S"] * (r["gae"] - r["solver"] + r["solver"] * c4scale) / r["gae"] if r["gae"] else None
        else:
            v = r[key]
        if v is None:
            continue
        d[(r["p"], r["seed"])].append(v)
    return {k: float(np.mean(v)) for k, v in d.items()}

def ols(x, y):
    x = np.asarray(x); y = np.asarray(y)
    xm = x.mean()
    return float(((x - xm) * (y - y.mean())).sum() / ((x - xm) ** 2).sum())

def fit(cm_by_arm, arms, B=4000, seed=20261031):
    """slope of log2 value vs log2 p for each arm and pairwise differences, bootstrap over curves within p."""
    keys = {a: cm_by_arm[a] for a in arms}
    seeds_by_p = {p: sorted({s for (pp, s) in keys[arms[0]] if pp == p}) for p in PS}
    # restrict to curves present in all arms
    for p in PS:
        seeds_by_p[p] = [s for s in seeds_by_p[p] if all((p, s) in keys[a] for a in arms)]
    def slope_for(draw):
        res = {}
        for a in arms:
            xs, ys = [], []
            for p in PS:
                for s in draw[p]:
                    v = keys[a].get((p, s))
                    if v and v > 0:
                        xs.append(math.log2(p)); ys.append(math.log2(v))
            res[a] = ols(xs, ys) if len(set(xs)) > 1 else float("nan")
        return res
    base = slope_for(seeds_by_p)
    rng = np.random.default_rng(seed)
    boots = []
    for _ in range(B):
        draw = {p: list(rng.choice(seeds_by_p[p], size=len(seeds_by_p[p]))) if seeds_by_p[p] else [] for p in PS}
        boots.append(slope_for(draw))
    ci = {}
    for a in arms:
        v = np.array([b[a] for b in boots]); ci[a] = (base[a], float(np.nanpercentile(v, 2.5)), float(np.nanpercentile(v, 97.5)))
    diffs = {}
    for i, a in enumerate(arms):
        for b in arms[i + 1:]:
            v = np.array([bb[a] - bb[b] for bb in boots])
            diffs[f"{a}-{b}"] = (base[a] - base[b], float(np.nanpercentile(v, 2.5)), float(np.nanpercentile(v, 97.5)))
    return ci, diffs

table = {}
for a in ARMS:
    for key in ("S", "rel", "la", "trials", "full", "used", "cols", "small", "core", "wied", "merge", "attempts", "solver"):
        cm = curve_means(a, key)
        for p in PS:
            vs = [v for (pp, s), v in cm.items() if pp == p]
            if vs:
                table.setdefault(a, {}).setdefault(p, {})[key] = (float(np.mean(vs)), len(vs))
out["table"] = {a: {str(p): v for p, v in d.items()} for a, d in table.items()}

# main fits: whole S, relation phase, LA phase (S-scale: gae/sqrt(r) not needed for slopes in p except r depends on p;
# the task asks exponent in p of total counted cost -> use S (= GAE/sqrt r) and also raw GAE).
IC = ["plain", "d4", "2lp", "d4-2lp"]
for name, key in (("S", "S"), ("gae", "gae"), ("relations_phase_gae", "rel"), ("la_phase_gae", "la")):
    cm = {a: curve_means(a, key) for a in ARMS}
    ci, diffs = fit(cm, ARMS if key in ("S", "gae") else IC)
    out[f"fit_{name}"] = {"slope": ci, "diff": diffs}
cmS = {a: curve_means(a, "S") for a in ARMS}
# ratios vs plain, d4 and rho per p (geometric mean over curves)
def gm(a, b, p):
    v = [math.log(cmS[a][k] / cmS[b][k]) for k in cmS[a] if k[0] == p and k in cmS[b]]
    return math.exp(np.mean(v)) if v else None
out["ratio"] = {f"{a}/{b}": {str(p): gm(a, b, p) for p in PS} for a, b in
                (("d4-2lp", "d4"), ("2lp", "plain"), ("d4", "plain"), ("d4-2lp", "plain"), ("d4-2lp", "2lp"),
                 ("plain", "rho-neg"), ("d4", "rho-neg"), ("2lp", "rho-neg"), ("d4-2lp", "rho-neg"))}
# reprice sweep: S at c4 scaled by f (the LA phase stays counted)
sweep = {}
for f in (1.0, 1 / 340, 1e-3, 1e-5, 1e-7):
    cmr = {a: curve_means(a, "S_rep", c4scale=f) for a in IC}
    cmr["rho-neg"] = cmS["rho-neg"]
    ci, diffs = fit(cmr, IC + ["rho-neg"], B=1000)
    sweep[str(f)] = {"slope": ci, "diff": {k: v for k, v in diffs.items() if "d4-2lp" in k or k == "d4-2lp-d4"}}
out["fit_S_reprice_c4_scale"] = sweep
# LA share of whole per arm/p
out["la_share"] = {a: {str(p): (table[a][p]["la"][0] / (table[a][p]["rel"][0] + table[a][p]["la"][0]))
                       for p in PS if a in table and p in table[a] and "la" in table[a][p]} for a in IC}
json.dump(out, open(f"{HERE}/analysis.json", "w"), indent=1, default=str)
print(json.dumps({k: out[k] for k in ("n_records", "n_verified", "unverified")}))
for k in ("fit_S", "fit_relations_phase_gae", "fit_la_phase_gae"):
    print(k)
    for a, v in out[k]["slope"].items():
        print(f"  {a:8s} slope {v[0]:.4f} [{v[1]:.4f}, {v[2]:.4f}]")
    for a, v in out[k]["diff"].items():
        if "d4-2lp" in a:
            print(f"  diff {a:16s} {v[0]:+.4f} [{v[1]:+.4f}, {v[2]:+.4f}]")
