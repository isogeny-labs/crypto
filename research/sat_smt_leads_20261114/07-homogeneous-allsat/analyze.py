#!/usr/bin/env python3
import json, glob, math, random
from collections import defaultdict
import numpy as np
rows = []
for f in sorted(glob.glob("raw_*.jsonl")):
    rows += [json.loads(x) for x in open(f)]
import hallsat as H
_diag = {}
def diag(k, l, n, seed):
    """number of trivial all-equal curve tuples (a,a,a,a) in the k=4 chain instance (P+P-P-P); recomputed from the seed"""
    if k != 4: return 0
    key = (k, l, n, seed)
    if key not in _diag:
        F = H.GF(n); C = H.Curve(F); base = H.Base(F, l, random.Random(1000 * l + seed + 100000 * k)); pts = H.base_points(C, base)
        _diag[key] = sum(1 for t in H.truth_homog(C, base, pts, 4) if len(set(t)) == 1)
    return _diag[key]
cells = defaultdict(lambda: defaultdict(list))
for r in rows:
    if r["arm"] in ("H0", "H1"):
        r["n_curve_trivial_diag"] = diag(r["k"], r["l"], r["n"], r["seed"])
        r["n_curve_nontrivial"] = r["n_curve"] - r["n_curve_trivial_diag"]
    cells[(r["k"], r["l"], r["n"])][r["seed"]].append(r)

def agg(seedrows):
    """seedrows: list of per-seed row lists -> metrics"""
    out = {}
    for arm in ("H0", "H1"):
        hs = [r for rs in seedrows for r in rs if r["arm"] == arm]
        for u in ("conflicts", "decisions", "propagations"):
            tot = sum(h[u] for h in hs); rel = sum(h["n_curve_nontrivial"] for h in hs)
            out[f"{arm}_{u}_per_rel"] = tot / rel if rel else float("inf")
        out[f"{arm}_conflicts_per_instance"] = sum(h["conflicts"] for h in hs) / len(hs)
        out[f"{arm}_curve_rel"] = sum(h["n_curve_nontrivial"] for h in hs); out[f"{arm}_diag"] = sum(h["n_curve_trivial_diag"] for h in hs)
        out[f"{arm}_twist"] = sum(h["n_twist"] for h in hs); out[f"{arm}_other"] = sum(h["n_other"] for h in hs)
        out[f"{arm}_wall"] = sum(h["wall"] for h in hs)
        out[f"{arm}_complete"] = all(h["complete"] for h in hs)
    pt = [r for rs in seedrows for r in rs if r["arm"] == "PT"]
    sat = [r for r in pt if r["sat"]]; uns = [r for r in pt if r["sat"] is False]
    for u in ("conflicts", "decisions", "propagations"):
        out[f"PT_{u}_per_rel"] = sum(r[u] for r in pt) / len(sat) if sat else float("inf")
    out.update(PT_targets=len(pt), PT_sat=len(sat), PT_unsat=len(uns),
               PT_conf_sat_mean=float(np.mean([r["conflicts"] for r in sat])) if sat else None,
               PT_conf_unsat_mean=float(np.mean([r["conflicts"] for r in uns])) if uns else None,
               PT_wall=sum(r["wall"] for r in pt))
    out["Q0"] = out["PT_conflicts_per_rel"] / out["H0_conflicts_per_rel"]
    out["Q0_dec"] = out["PT_decisions_per_rel"] / out["H0_decisions_per_rel"]; out["Q0_prop"] = out["PT_propagations_per_rel"] / out["H0_propagations_per_rel"]
    out["Q1"] = out["PT_conflicts_per_rel"] / out["H1_conflicts_per_rel"]
    return out

summary = []
rng = random.Random(7)
for (k, l, n) in sorted(cells):
    seeds = sorted(cells[(k, l, n)])
    sr = [cells[(k, l, n)][s] for s in seeds]
    a = agg(sr)
    bs = []
    for _ in range(400):
        b = agg([sr[rng.randrange(len(sr))] for _ in sr])
        if math.isfinite(b["Q0"]) and b["Q0"] > 0: bs.append(math.log2(b["Q0"]))
    m = k - 1
    N = 2 ** l
    tup_pt = math.comb(N + m - 1, m); tup_h = math.comb(N + k - 1, k)
    a.update(k=k, m=m, l=l, n=n, bases=len(seeds),
             log2Q0_boot_lo=float(np.percentile(bs, 2.5)) if bs else None, log2Q0_boot_hi=float(np.percentile(bs, 97.5)) if bs else None,
             enum_tuples_per_rel_PT=tup_pt * a["PT_targets"] / max(a["PT_sat"], 1),
             enum_tuples_per_rel_H=tup_h * len(seeds) / max(a["H0_curve_rel"], 1),
             leaf_solves_per_rel_PT=math.comb(N + m - 3, m - 2) * a["PT_targets"] / max(a["PT_sat"], 1),
             leaf_solves_per_rel_H=math.comb(N + k - 3, k - 2) * len(seeds) / max(a["H0_curve_rel"], 1))
    summary.append(a)

def slope(k, key):
    pts = [(s["l"], math.log2(s[key])) for s in summary if s["k"] == k and s[key] and math.isfinite(s[key]) and s[key] > 0]
    if len(pts) < 2: return None
    x, y = zip(*pts)
    return float(np.polyfit(x, y, 1)[0])
slopes = {}
for k in (3, 4):
    slopes[k] = {key: slope(k, key) for key in ("Q0", "Q1", "Q0_dec", "Q0_prop", "H0_conflicts_per_rel", "H1_conflicts_per_rel", "PT_conflicts_per_rel",
                 "H0_conflicts_per_instance", "PT_conf_unsat_mean", "PT_conf_sat_mean", "enum_tuples_per_rel_PT", "enum_tuples_per_rel_H",
                 "leaf_solves_per_rel_PT", "leaf_solves_per_rel_H")}
json.dump({"summary": summary, "slopes_bits_per_l": slopes, "rows": rows}, open("results.json", "w"), indent=1)
f = lambda v: "-" if v is None else (f"{v:.3g}" if isinstance(v, float) else str(v))
print("| k | l | n | bases | curve rel | twist | H0 confl/rel | H1 confl/rel | PT sat/targets | PT confl SAT | PT confl UNSAT | PT confl/rel | Q0=PT/H0 [95% boot, log2] | Q1=PT/H1 | enum tuples/rel PT | enum tuples/rel H | leaf solves/rel PT |")
print("|" + "---|" * 17)
for s in summary:
    print("| " + " | ".join(f(v) for v in [s["k"], s["l"], s["n"], s["bases"], s["H0_curve_rel"], s["H0_twist"], s["H0_conflicts_per_rel"], s["H1_conflicts_per_rel"],
          f'{s["PT_sat"]}/{s["PT_targets"]}', s["PT_conf_sat_mean"], s["PT_conf_unsat_mean"], s["PT_conflicts_per_rel"],
          f'{s["Q0"]:.3g} [{f(s["log2Q0_boot_lo"])}, {f(s["log2Q0_boot_hi"])}]', s["Q1"], s["enum_tuples_per_rel_PT"], s["enum_tuples_per_rel_H"], s["leaf_solves_per_rel_PT"]]) + " |")
print(json.dumps(slopes, indent=1))
print("walls", {(s["k"], s["l"]): (round(s["H0_wall"], 1), round(s["H1_wall"], 1), round(s["PT_wall"], 1)) for s in summary}, "complete", all(s["H0_complete"] and s["H1_complete"] for s in summary))
