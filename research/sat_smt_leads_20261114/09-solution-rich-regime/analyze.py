#!/usr/bin/env python3
import json, glob, math, statistics as st
from collections import defaultdict
rows = []
for f in sorted(glob.glob("rows_n*.jsonl")):
    rows += [json.loads(x) for x in open(f)]
CAP = 400000
cells = defaultdict(list)
for r in rows:
    cells[(r["n"], r["l"])].append(r)

def cpr(rs):
    """conflicts per relation; relation = SAT and lifted."""
    rel = sum(1 for r in rs if r["result"] == "SAT" and r["lifted"])
    tot = sum(r["conflicts"] for r in rs)
    return (tot / rel if rel else None), rel, tot

def fit(xs, ys):
    pts = [(x, math.log2(y)) for x, y in zip(xs, ys) if y and y > 0]
    if len(pts) < 3: return None
    mx = sum(p[0] for p in pts) / len(pts); my = sum(p[1] for p in pts) / len(pts)
    return round(sum((p[0]-mx)*(p[1]-my) for p in pts) / sum((p[0]-mx)**2 for p in pts), 3)

summary = []
for (n, l) in sorted(cells):
    rs = cells[(n, l)]
    A = [r for r in rs if r["arm"] == "a_glucose"]
    full = len(A)
    c = dict(n=n, l=l, targets=full, balanced_l=round(n/3, 2))
    c["frac_decomposable_truth"] = round(sum(1 for r in A if r["nsol"] > 0) / full, 3)
    c["mean_nsol_ordered"] = round(sum(r["nsol"] for r in A) / full, 2)
    c["mismatch_sat_vs_truth"] = sum(1 for r in A if r["result"] != "CENSORED" and (r["result"] == "SAT") != (r["nsol"] > 0))
    c["sat_not_lifted"] = sum(1 for r in A if r["result"] == "SAT" and not r["lifted"])
    c["censored"] = sum(1 for r in A if r["result"] == "CENSORED")
    for arm, key in (("a_glucose", "glucose"), ("a_cadical", "cadical")):
        X = [r for r in rs if r["arm"] == arm]
        if not X: continue
        s = [r["conflicts"] for r in X if r["result"] == "SAT"]; u = [r["conflicts"] for r in X if r["result"] == "UNSAT"]
        c[key + "_median_conf_sat"] = st.median(s) if s else None
        c[key + "_median_conf_unsat"] = st.median(u) if u else None
        c[key + "_n_sat"] = len(s); c[key + "_n_unsat"] = len(u)
        v, rel, tot = cpr(X)
        c[key + "_conf_per_rel"] = round(v, 1) if v else None
        c[key + "_relations"] = rel
        c[key + "_props_per_conflict"] = round(sum(r["propagations"] for r in X) / max(1, tot), 1)
        c[key + "_decisions_per_rel"] = round(sum(r["decisions"] for r in X) / rel, 1) if rel else None
        c[key + "_wall_per_rel_s"] = round(sum(r["wall"] for r in X) / rel, 4) if rel else None
    # arm b: budget sweep, post hoc from arm a (in-sample choice of B)
    sweep = {}
    for e in range(2, 19):
        B = 1 << e
        rel = sum(1 for r in A if r["result"] == "SAT" and r["lifted"] and r["conflicts"] <= B)
        tot = sum(min(r["conflicts"], B) for r in A)
        sweep[B] = (round(tot / rel, 1) if rel else None, rel)
    c["budget_sweep"] = {str(B): v for B, v in sweep.items()}
    ok = [(v[0], B) for B, v in sweep.items() if v[0] is not None]
    c["budget_best_conf_per_rel"], c["budget_best_B"] = min(ok) if ok else (None, None)
    c["budget_best_relations"] = sweep[c["budget_best_B"]][1] if ok else 0
    # arm c
    for k in sorted(set(r["k"] for r in rs if r["arm"] == "c_streamline")):
        X = [r for r in rs if r["arm"] == "c_streamline" and r["k"] == k]
        v, rel, tot = cpr(X)
        c[f"xor{k}_conf_per_rel"] = round(v, 1) if v else None
        c[f"xor{k}_relations"] = rel
        c[f"xor{k}_frac_sat"] = round(sum(1 for r in X if r["result"] == "SAT") / len(X), 3)
    # baselines from ground truth
    dec = sum(1 for r in A if r["nsol"] > 0)
    lin = sum(((2**l + 1) / (r["k3"] + 1)) if r["k3"] else 2**l for r in A)
    enu = sum(((4**l + 1) / (r["k2"] + 1)) if r["k2"] else 4**l for r in A)
    c["linscan_leaves_per_rel"] = round(lin / dec, 2) if dec else None
    c["enum_pairs_per_rel"] = round(enu / dec, 2) if dec else None
    c["linscan_noexit_per_rel"] = round(full * 2**l / dec, 2) if dec else None
    c["enum_noexit_per_rel"] = round(full * 4**l / dec, 2) if dec else None
    sat_arms = {k: c[k] for k in c if k.endswith("_conf_per_rel") and c[k] is not None}
    c["best_sat_arm"], c["best_sat_conf_per_rel"] = (min(sat_arms.items(), key=lambda kv: kv[1]) if sat_arms else (None, None))
    b = c["best_sat_conf_per_rel"]
    c["best_sat_over_linscan"] = round(b / c["linscan_leaves_per_rel"], 2) if b and dec else None
    c["best_sat_over_enum"] = round(b / c["enum_pairs_per_rel"], 3) if b and dec else None
    for name, key in (("glucose", "glucose_conf_per_rel"), ("best_sat", "best_sat_conf_per_rel"), ("linscan", "linscan_leaves_per_rel"), ("enum", "enum_pairs_per_rel")):
        c["T_" + name] = round(2**l * c[key] + 3 * 4**l, 1) if c.get(key) else None
    summary.append(c)

per_n = []
for n in sorted(set(c["n"] for c in summary)):
    cs = [c for c in summary if c["n"] == n]
    ls = [c["l"] for c in cs]
    d = dict(n=n, ls=ls, balanced_l=round(n/3, 2))
    for key in ("glucose_conf_per_rel", "best_sat_conf_per_rel", "budget_best_conf_per_rel", "linscan_leaves_per_rel", "enum_pairs_per_rel", "glucose_median_conf_unsat", "glucose_median_conf_sat"):
        d["slope_bits_per_l__" + key] = fit(ls, [c.get(key) for c in cs])
    for name in ("glucose", "best_sat", "linscan", "enum"):
        ok = [(c["T_" + name], c["l"]) for c in cs if c["T_" + name]]
        d["argmin_l_T_" + name] = min(ok)[1] if ok else None
    lo, hi = math.floor(n/3), math.ceil(n/3)
    for name in ("glucose_conf_per_rel", "best_sat_conf_per_rel"):
        bal = [c[name] for c in cs if c["l"] in (lo, hi) and c[name]]
        opt = [c[name] for c in cs if c["l"] == d["argmin_l_T_" + name.split("_conf")[0]]]
        d["balanced_over_opt__" + name] = [round(b / opt[0], 2) for b in bal] if opt and opt[0] else None
    d["cells_sat_below_linscan"] = [c["l"] for c in cs if c["best_sat_over_linscan"] is not None and c["best_sat_over_linscan"] < 1]
    d["cells_sat_below_enum"] = [c["l"] for c in cs if c["best_sat_over_enum"] is not None and c["best_sat_over_enum"] < 1]
    per_n.append(d)

json.dump(dict(note="pilot, [pcc:unverified]", cap=CAP, per_n=per_n, cells=summary, rows=rows), open("results.json", "w"), indent=1)
cols = ["n","l","targets","frac_decomposable_truth","mean_nsol_ordered","glucose_median_conf_sat","glucose_median_conf_unsat","glucose_conf_per_rel","cadical_conf_per_rel","budget_best_conf_per_rel","budget_best_B","xor2_conf_per_rel","xor4_conf_per_rel","xor8_conf_per_rel","linscan_leaves_per_rel","enum_pairs_per_rel","best_sat_over_linscan","best_sat_over_enum","T_best_sat","T_linscan","T_enum","mismatch_sat_vs_truth","sat_not_lifted","censored","glucose_props_per_conflict"]
print("| " + " | ".join(cols) + " |"); print("|" + "---|" * len(cols))
for c in summary:
    print("| " + " | ".join(str(c.get(k, "")) for k in cols) + " |")
print()
for d in per_n: print(json.dumps(d))
