#!/usr/bin/env python3
"""R12 analysis (frozen before data). Reads run_P.jsonl here and ../genus_ladder_bounded_core_20261122/run_P.jsonl (R11).
Writes analysis.json. Every figure [pcc:unverified]. Usage: analyze.py [dir] [r11dir]; env R12_PRIMES overrides primes (code test only)."""
import json, math, os, random, sys, collections
D = sys.argv[1] if len(sys.argv) > 1 else "."
R11 = sys.argv[2] if len(sys.argv) > 2 else "../genus_ladder_bounded_core_20261122"
PS = [int(x) for x in os.environ.get("R12_PRIMES", "32771,65537,131101,262147").split(",")]
UNM = "C_double_lp+pre"; PLAIN = UNM + "+sge(8,3)"
WS = [8.5, 9, 9.5, 10]
def wn(w): return "w%g" % w
ARMS = {"unmerged": UNM, "plain": PLAIN}
for w in WS: ARMS[wn(w)] = PLAIN + "+bd(%g,0.1,3,1)" % w
R11ARM = PLAIN + "+bd(8,0.1,3,1)"
def load(fn):
    return [json.loads(l) for l in open(fn) if l.strip()]
runs = collections.defaultdict(list); inst = {}; r11 = collections.defaultdict(list); r11inst = {}
for p in PS:
    for r in load(os.path.join(D, "run_%d.jsonl" % p)):
        if r.get("kind") == "run": runs[(p, r["arm"])].append(r)
        elif r.get("kind") == "instance": inst[(p, r["curve_seed"])] = r
    for r in load(os.path.join(R11, "run_%d.jsonl" % p)):
        if r.get("kind") == "run": r11[(p, r["arm"])].append(r)
        elif r.get("kind") == "instance": r11inst[(p, r["curve_seed"])] = r
def fpm(r, k=3): return r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + k * r["la_mul"]
def relph(r): return r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"]
def wout(r): return r["sge_nnz_out"] / r["sge_rows_out"] if "sge_nnz_out" in r else 0.0
M = {"fpm3": fpm, "relphase": relph, "la_mul": lambda r: r["la_mul"], "rel_mul": lambda r: r["rel_mul"],
     "relations": lambda r: r["relations"], "bd_extra": lambda r: r.get("bd_extra", 0.0),
     "bd_probes": lambda r: r.get("bd_probes", 0), "bd_capped": lambda r: float(r.get("bd_capped", False)),
     "w_pre": lambda r: r.get("mean_weight_before", 0.0), "w_sel": lambda r: r.get("mean_weight_after", 0.0),
     "w_out": wout, "rows_out": lambda r: r.get("sge_rows_out", 0), "cols_out": lambda r: r.get("sge_cols_out", 0),
     "nnz_out": lambda r: r.get("sge_nnz_out", 0), "core_rows": lambda r: r["core_rows"], "core_cols": lambda r: r["core_cols"],
     "rows_pre": lambda r: r.get("core_rows_pre", r["core_rows"]), "cols_pre": lambda r: r.get("core_cols_pre", r["core_cols"]),
     "la_sel_add": lambda r: r.get("la_sel_add", 0), "certified": lambda r: float(r["certified"] and r["ok"])}
def grid(rs):  # curve -> list of rows ordered by seed
    g = collections.defaultdict(list)
    for r in rs: g[r["curve_seed"]].append(r)
    return {c: sorted(v, key=lambda r: r["seed"]) for c, v in g.items()}
G = {k: grid(v) for k, v in runs.items()}; G11 = {k: grid(v) for k, v in r11.items()}
curves = {p: sorted(G[(p, PLAIN)]) for p in PS}
def cm(g, cs, f, idx=None):
    # idx: dict curve-slot -> seed index list (bootstrap); None: all seeds
    tot = 0.0
    for i, c in enumerate(cs):
        rows = g[c]
        if idx is None: v = [f(r) for r in rows]
        else: v = [f(rows[j]) for j in idx[i]]
        tot += sum(v) / len(v)
    return tot / len(cs)
A = {"primes": PS, "ws": WS, "cells": {}, "n_runs": {}, "all_certified": True}
def cellrow(g, p, f=None):
    return {m: cm(g, curves[p], fn) for m, fn in M.items()}
for p in PS:
    for nm, arm in ARMS.items():
        rs = runs.get((p, arm), [])
        A["n_runs"]["%d|%s" % (p, nm)] = len(rs)
        if not rs: continue
        A["all_certified"] &= all(r["certified"] and r["ok"] for r in rs)
        A["cells"]["%d|%s" % (p, nm)] = cellrow(G[(p, arm)], p)
    if (p, R11ARM) in G11:
        A["n_runs"]["%d|r11w8" % p] = len(r11[(p, R11ARM)])
        A["cells"]["%d|r11w8" % p] = cellrow(G11[(p, R11ARM)], p)
logp = [math.log(p) for p in PS]
def fit(ys):
    n = len(logp); mx = sum(logp) / n; my = sum(ys) / n
    return sum((x - mx) * (y - my) for x, y in zip(logp, ys)) / sum((x - mx) ** 2 for x in logp)
def pct(v, q):
    v = sorted(v); i = q * (len(v) - 1); lo = int(math.floor(i)); hi = min(lo + 1, len(v) - 1)
    return v[lo] + (v[hi] - v[lo]) * (i - lo)
iv = lambda v: {"lo": pct(v, 0.025), "hi": pct(v, 0.975)}
NW = [wn(w) for w in WS]
def select(cells_p):
    """cells_p: name -> metrics for one prime. returns (best W name by fpm3, const-excess W name by |extra-0.2|)"""
    best = min(NW, key=lambda n: (cells_p[n]["fpm3"], n))
    ce = min(NW, key=lambda n: (abs(cells_p[n]["bd_extra"] - 0.2), WS[NW.index(n)]))
    return best, ce
def derived(cells):  # cells: (p,name)->metrics ; returns exponents + selections
    out = {"sel": {}, "exp": {}}
    for p in PS: out["sel"][p] = select({n: cells[(p, n)] for n in NW})
    return out
def exps(cells, sel):
    e = {}
    for n in ["plain", "unmerged"] + NW:
        for m in ("fpm3", "relphase", "la_mul"):
            e["%s|%s" % (n, m)] = fit([math.log(cells[(p, n)][m]) for p in PS])
    for m in ("fpm3", "relphase", "la_mul"):
        e["env|%s" % m] = fit([math.log(cells[(p, sel[p][0])][m]) for p in PS])
        e["ce|%s" % m] = fit([math.log(cells[(p, sel[p][1])][m]) for p in PS])
        # secondary envelope including plain
        e["envp|%s" % m] = fit([math.log(min((cells[(p, n)] for n in NW + ["plain"]), key=lambda c: c["fpm3"])[m]) for p in PS])
    return e
pt_cells = {(p, n): A["cells"]["%d|%s" % (p, n)] for p in PS for n in ["plain", "unmerged"] + NW}
pt = derived(pt_cells); pe = exps(pt_cells, pt["sel"])
A["selection"] = {str(p): {"best_w": pt["sel"][p][0], "const_excess_w": pt["sel"][p][1]} for p in PS}
A["exponent_point"] = pe
rng = random.Random(20261122); NB = 4000
bd = collections.defaultdict(list); br = collections.defaultdict(list); bsel = collections.defaultdict(collections.Counter)
names = ["plain", "unmerged"] + NW
for _ in range(NB):
    bc = {}
    for p in PS:
        cs = [rng.choice(curves[p]) for _ in curves[p]]
        idx = {i: [rng.randrange(len(G[(p, PLAIN)][c])) for _ in G[(p, PLAIN)][c]] for i, c in enumerate(cs)}
        for n in names:
            bc[(p, n)] = {m: cm(G[(p, ARMS[n])], cs, M[m], idx) for m in ("fpm3", "relphase", "la_mul", "bd_extra")}
    sel = {p: select({n: bc[(p, n)] for n in NW}) for p in PS}
    for p in PS: bsel[(p, "best")][sel[p][0]] += 1; bsel[(p, "ce")][sel[p][1]] += 1
    e_re = exps(bc, sel); e_fx = exps(bc, pt["sel"])
    for k in e_re: bd["re|" + k].append(e_re[k]); bd["fx|" + k].append(e_fx[k])
    for tag, e in (("re", e_re), ("fx", e_fx)):
        for a in ("env", "envp", "ce") + tuple(NW):
            for m in ("fpm3", "relphase", "la_mul"):
                bd["%s|d|%s|%s" % (tag, a, m)].append(e["%s|%s" % (a, m)] - e["plain|%s" % m])
    for p in PS:
        for n in NW + ["unmerged"]:
            for m in ("fpm3", "la_mul", "relphase"):
                br["%d|%s|%s" % (p, n, m)].append(bc[(p, n)][m] / bc[(p, "plain")][m])
        for tag, key in (("best", sel[p][0]), ("ce", sel[p][1])):
            br["%d|%s|fpm3" % (p, tag)].append(bc[(p, key)]["fpm3"] / bc[(p, "plain")]["fpm3"])
        br["%d|best|minratio" % p].append(min(bc[(p, n)]["fpm3"] for n in NW) / bc[(p, "plain")]["fpm3"])
        for n in NW: br["%d|%s|extra" % (p, n)].append(bc[(p, n)]["bd_extra"])
A["exponents"] = {}
for tag in ("re", "fx"):
    for k, v in pe.items():
        A["exponents"]["%s|%s" % (tag, k)] = dict(est=v, **iv(bd["%s|%s" % (tag, k)]))
A["exp_diff_vs_plain"] = {}
for tag in ("re", "fx"):
    for a in ("env", "envp", "ce") + tuple(NW):
        for m in ("fpm3", "relphase", "la_mul"):
            est = pe["%s|%s" % (a, m)] - pe["plain|%s" % m]
            A["exp_diff_vs_plain"]["%s|%s|%s" % (tag, a, m)] = dict(est=est, **iv(bd["%s|d|%s|%s" % (tag, a, m)]))
A["ratios"] = {}
for k, v in br.items():
    p, n, m = k.split("|")
    if m == "extra": A["ratios"][k] = iv(v); continue
    if m == "minratio": est = min(A["cells"]["%s|%s" % (p, w)]["fpm3"] for w in NW) / A["cells"]["%s|plain" % p]["fpm3"]
    elif n in ("best", "ce"):
        s = pt["sel"][int(p)][0 if n == "best" else 1]; est = A["cells"]["%s|%s" % (p, s)]["fpm3"] / A["cells"]["%s|plain" % p]["fpm3"]
    else: est = A["cells"]["%s|%s" % (p, n)][m] / A["cells"]["%s|plain" % p][m]
    A["ratios"][k] = dict(est=est, **iv(v))
A["selection_freq"] = {"%d|%s" % k: dict(v) for k, v in bsel.items()}
# S1: identity with R11 (plain and unmerged), instances
SKIP = {"wall_ns", "order_search_wall_s"}
b1 = {"compared": 0, "mismatch": [], "instances_equal": True}
for p in PS:
    for (pp, cs), r in inst.items():
        if pp == p:
            o = r11inst.get((p, cs))
            if o is None or any(r.get(k) != o[k] for k in o if k not in SKIP): b1["instances_equal"] = False
    for arm in (UNM, PLAIN):
        old = {(r["curve_seed"], r["seed"]): r for r in r11[(p, arm)]}
        for r in runs.get((p, arm), []):
            o = old.get((r["curve_seed"], r["seed"]))
            if o is None: b1["mismatch"].append([p, r["curve_seed"], r["seed"], arm, "missing"]); continue
            b1["compared"] += 1
            for k in o:
                if k not in SKIP and r.get(k) != o[k]: b1["mismatch"].append([p, r["curve_seed"], r["seed"], arm, k])
A["S1"] = b1
sc = {}
sc["S1_holds"] = A["all_certified"] and not b1["mismatch"] and b1["instances_equal"] and b1["compared"] > 0
sc["S2_min_ratio"] = {str(p): A["ratios"]["%d|best|minratio" % p] for p in PS}
sc["S2_holds"] = all(sc["S2_min_ratio"][str(p)]["est"] <= 0.99 for p in PS)
d = A["exp_diff_vs_plain"]["re|env|fpm3"]
sc["S3_env_exp"] = A["exponents"]["re|env|fpm3"]; sc["S3_plain_exp"] = pe["plain|fpm3"]; sc["S3_diff"] = d
sc["S3_holds"] = pe["env|fpm3"] <= pe["plain|fpm3"] - 0.02 and d["hi"] < 0
sc["S4_rel_exp"] = A["exponents"]["re|ce|relphase"]; sc["S4_la_exp"] = A["exponents"]["re|ce|la_mul"]
sc["S4_holds"] = pe["ce|relphase"] <= 1.32 and pe["ce|la_mul"] <= 1.47
sc["S4_rel_ok"] = pe["ce|relphase"] <= 1.32; sc["S4_la_ok"] = pe["ce|la_mul"] <= 1.47
sc["all_certified"] = A["all_certified"]
A["scorecard"] = sc
json.dump(A, open(os.path.join(D, "analysis.json"), "w"), indent=1, sort_keys=True)
print(json.dumps({k: v for k, v in sc.items()}, indent=1))
