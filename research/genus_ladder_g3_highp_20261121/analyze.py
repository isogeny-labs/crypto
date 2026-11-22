#!/usr/bin/env python3
"""Paired ratios, expected-rho reference, exponents, LA share, P1 check. Protocol: PROTOCOL.md."""
import json, math, random, sys, gzip, os
R = "/home/user/crypto/research/"
NEW = {p: f"run_{p}.jsonl" for p in (65537, 131101, 262147)}
OLD_EXT = {65537: R+"genus_ladder_fast_order_20261121/ext_g3.jsonl", 131101: R+"genus_ladder_fast_order_20261121/ext_g3.jsonl",
           262147: R+"genus_ladder_fast_order_20261121/ext_g3_262147.jsonl"}
LOW_RHO = R+"genus_ladder_scale_20261115/scale_g3.jsonl"
LOW_SGE = R+"genus_ladder_sge_scale_20261115/sge_g3.jsonl"
SGE, UNM, RHO = "C_double_lp+pre+sge(8,3)", "C_double_lp+pre", "R_rho_neg"
def load(f): return [json.loads(l) for l in open(f)]
def fpm(d):
    return d["cantor_mul"] + 10*d["cantor_inv"] + d.get("rel_mul", 0) + 9*d.get("la_mul", 0)
def la9(d): return 9*d.get("la_mul", 0)
rows = {}; inst = {}
for p, f in NEW.items():
    rr = load(f)
    rows[p] = [d for d in rr if d["kind"] == "run"]
    inst[p] = [d for d in rr if d["kind"] == "instance"]
low = {}
for p in (8209, 16411, 32771): low[p] = []
for d in load(LOW_RHO):
    if d["kind"] == "run" and d["arm"] == RHO and d["p"] in low: low[d["p"]].append(d)
for d in load(LOW_SGE):
    if d["kind"] == "run" and d["arm"] in (SGE, UNM) and d["p"] in low: low[d["p"]].append(d)
lowinst = {p: [] for p in low}
for d in load(LOW_RHO):
    if d["kind"] == "instance" and d["p"] in low: lowinst[d["p"]].append(d)
allrows = {**low, **rows}; allinst = {**lowinst, **inst}
out = {}

# ---- P1 ----
p1 = {"all_certified": all(d["certified"] and d["ok"] for p in rows for d in rows[p]),
      "n_runs": {p: len(rows[p]) for p in rows}, "arms": sorted({d["arm"] for p in rows for d in rows[p]})}
def strip(d): return {k: v for k, v in d.items() if k not in ("wall_ns", "order_search_wall_s")}
p1["repro"] = {}
for p in rows:
    old = [d for d in load(OLD_EXT[p]) if d.get("p") == p]
    oi = [d for d in old if d["kind"] == "instance"]
    ni = inst[p][:len(oi)]
    inst_eq = [strip(a) == strip(b) for a, b in zip(oi, ni)]
    seeds = [d["curve_seed"] for d in oi]
    orow = {(d["curve_seed"], d["seed"], d["arm"]): strip(d) for d in old if d["kind"] == "run"}
    nrow = {(d["curve_seed"], d["seed"], d["arm"]): strip(d) for d in rows[p] if d["curve_seed"] in seeds}
    NEWF = ("la_fallbacks", "ferrari_undecided", "target_seed")  # fields the rebuilt binary adds
    run_eq = {k: nrow.get(k) is not None and all(nrow[k].get(x) == v for x, v in orow[k].items()) for k in orow}
    added = sorted({x for k in orow for x in nrow[k] if x not in orow[k]}) if all(k in nrow for k in orow) else None
    p1.setdefault("added_fields", {})[p] = added
    p1["repro"][p] = {"old_seeds": seeds, "new_first_seeds": [d["curve_seed"] for d in ni],
                      "inst_equal": inst_eq, "runs_equal": sum(run_eq.values()), "runs_old": len(orow),
                      "mismatch_keys": [list(k) for k, v in run_eq.items() if not v][:10]}
p1["new_fields"] = {f: sorted({repr(d.get(f)) for p in rows for d in rows[p]}) for f in ("la_fallbacks", "ferrari_undecided", "target_seed")}
out["P1"] = p1

# ---- per-instance tables ----
def by_inst(rs, arm):
    m = {}
    for d in rs:
        if d["arm"] == arm: m.setdefault(d["curve_seed"], []).append(d)
    return m
rng = random.Random(20261121)
B = 4000
def cell_stat(p, arm_sets, Efun=None, draw=None):
    pass

def ratio_boot(num, den, B=B):
    """num, den: dict curve -> list of values per seed (den may be a per-curve scalar list). Hierarchical bootstrap."""
    curves = sorted(num)
    def mean_pick(pick_c, pick_s, d):
        return sum(sum(d[c][i] for i in idx)/len(idx) for c, idx in zip(pick_c, pick_s)) / len(pick_c)
    pt = (sum(sum(num[c])/len(num[c]) for c in curves)/len(curves)) / (sum(sum(den[c])/len(den[c]) for c in curves)/len(curves))
    bs = []
    for _ in range(B):
        pc = [rng.choice(curves) for _ in curves]
        ps = []
        for c in pc:
            n = len(num[c]); ps.append([rng.randrange(n) for _ in range(n)])
        # den seeds: same positional indices when lengths match
        ps_d = [[i % len(den[c]) for i in idx] if len(den[c]) != len(num[c]) else idx for c, idx in zip(pc, ps)]
        bs.append(mean_pick(pc, ps, num) / mean_pick(pc, ps_d, den))
    bs.sort()
    return pt, bs[int(0.025*B)], bs[int(0.975*B)-1]

def per_curve(rs, arm, f):
    # seed-indexed values per curve
    m = {}
    for d in sorted(rs, key=lambda d: (d["curve_seed"], d["seed"])):
        if d["arm"] == arm: m.setdefault(d["curve_seed"], []).append(f(d))
    return m

def meanl(x): return sum(x)/len(x)
res = {}
for p in sorted(allrows):
    rs = allrows[p]; rr = [d for d in rs if d["arm"] == RHO]
    cm = sum(d["cantor_mul"] for d in rr)/sum(d["steps"] for d in rr)
    ci = sum(d["cantor_inv"] for d in rr)/sum(d["steps"] for d in rr)
    rmap = {i["curve_seed"]: i["r"] for i in allinst[p]}
    E = {c: [0.886*math.sqrt(rmap[c])*(cm+10*ci)] for c in rmap}
    cell = {"n_curves": len(rmap), "cm_step": cm, "ci_step": ci, "rho_steps_mean": meanl([d["steps"] for d in rr]),
            "exp_steps_mean": meanl([0.886*math.sqrt(r) for r in rmap.values()])}
    rho = per_curve(rs, RHO, fpm)
    for name, arm in (("sge", SGE), ("unm", UNM)):
        a = per_curve(rs, arm, fpm)
        if not a: continue
        runs = [d for d in rs if d["arm"] == arm]
        cell[name] = {"n_runs": len(runs), "cantor_mul": meanl([d["cantor_mul"] for d in runs]),
                      "rel_mul": meanl([d["rel_mul"] for d in runs]), "la_mul": meanl([d["la_mul"] for d in runs]),
                      "fpm": meanl([fpm(d) for d in runs]), "la_share": meanl([la9(d)/fpm(d) for d in runs]),
                      "ratio_rho": ratio_boot(a, rho), "ratio_exp": ratio_boot(a, E)}
    cell["rho"] = {"n_runs": len(rr), "cantor_mul": meanl([d["cantor_mul"] for d in rr]), "cantor_inv": meanl([d["cantor_inv"] for d in rr]),
                   "fpm": meanl([fpm(d) for d in rr]), "exp_fpm": meanl([e[0] for e in E.values()])}
    res[p] = cell
out["cells"] = res

# ---- exponents ----
def fit(ps, y):
    x = [math.log(p) for p in ps]; ly = [math.log(v) for v in y]
    mx, my = meanl(x), meanl(ly)
    return sum((a-mx)*(b-my) for a, b in zip(x, ly)) / sum((a-mx)**2 for a in x)
def cellmeans(p, arm, curves):
    # curves: list (with repeats) of curve seeds; per-curve mean over seeds, then mean
    m = per_curve(allrows[p], arm, fpm)
    return meanl([meanl(m[c]) for c in curves])
exps = {}
for label, ps in (("six", sorted(allrows)), ("five", [p for p in sorted(allrows) if p <= 131101])):
    for name, arm in (("rho", RHO), ("sge", SGE), ("unm", UNM)):
        if any(not per_curve(allrows[p], arm, fpm) for p in ps): continue
        cs = {p: sorted(per_curve(allrows[p], arm, fpm)) for p in ps}
        pt = fit(ps, [cellmeans(p, arm, cs[p]) for p in ps])
        bs = sorted(fit(ps, [cellmeans(p, arm, [rng.choice(cs[p]) for _ in cs[p]]) for p in ps]) for _ in range(B))
        exps[f"{label}_{name}"] = [pt, bs[int(0.025*B)], bs[int(0.975*B)-1]]
# expected-rho exponent (descriptive)
exps["six_exp_rho"] = fit(sorted(res), [res[p]["rho"]["exp_fpm"] for p in sorted(res)])
out["exponents"] = exps
json.dump(out, open("paired.json", "w"), indent=1, default=str)
print(json.dumps(out["P1"], indent=1))
for p, c in res.items():
    s = c["sge"]
    print(p, c["n_curves"], "SGE/rho %.3f [%.3f,%.3f]" % s["ratio_rho"], "SGE/Erho %.3f [%.3f,%.3f]" % s["ratio_exp"],
          "LA %.3f" % s["la_share"], "rho/Erho %.3f" % (c["rho"]["fpm"]/c["rho"]["exp_fpm"]))
print(json.dumps(exps, indent=1))
