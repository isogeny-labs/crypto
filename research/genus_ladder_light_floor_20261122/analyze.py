#!/usr/bin/env python3
"""analyze.py: run_P.jsonl (P = 65537, 131101, 262147) + R2 rho rows -> analysis.json, joined.jsonl. Protocol: PROTOCOL.md.
Run in this directory (decompressed run files). Reads recorded counts only."""
import json, math, random
from collections import defaultdict
R2 = "/home/user/crypto/research/genus_ladder_g3_highp_20261121/"
PS = [65537, 131101, 262147]
SEEDS = {65537: [158, 217, 250, 254, 255, 283], 131101: [41, 58, 61, 79, 84, 101], 262147: [69, 113, 206, 207, 241, 261]}
UNM, PLAIN, RHO = "C_double_lp+pre", "C_double_lp+pre+sge(8,3)", "R_rho_neg"
ES = ["0.1", "0.2", "0.3"]
EXARM = {e: PLAIN + "+ex(%s,0.01)" % e for e in ES}
B, SEED = 4000, 20261122
def load(f): return [json.loads(l) for l in open(f)]
def fpm(r): return r["cantor_mul"] + 10 * r["cantor_inv"] + r.get("rel_mul", 0) + 9 * r.get("la_mul", 0)
def relcost(r): return r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"]
def mean(x): x = list(x); return sum(x) / len(x)
def pct(s, q): return s[int(q * (B - 1) + .5)]

def paired(ex, pl, f):
    rat = defaultdict(list)
    for k in ex:
        if k in pl: rat[k[0]].append(f(ex[k]) / f(pl[k]))
    est = mean(x for v in rat.values() for x in v)
    rng = random.Random(SEED); cs = sorted(rat); ms = []
    for _ in range(B):
        t = n = 0
        for _ in cs:
            v = rat[rng.choice(cs)]
            for _ in v: t += rng.choice(v); n += 1
        ms.append(t / n)
    ms.sort()
    return {"mean": est, "lo": pct(ms, .025), "hi": pct(ms, .975), "pairs": sum(len(v) for v in rat.values())}

def ratio_rho(num, den):
    est = mean(v for c in num for v in num[c]) / mean(v for c in den for v in den[c])
    rng = random.Random(SEED); nc, dc = sorted(num), sorted(den); out = []
    for _ in range(B):
        def m(d, cs):
            s = n = 0
            for _ in cs:
                v = d[rng.choice(cs)]
                for _ in v: s += rng.choice(v); n += 1
            return s / n
        out.append(m(num, nc) / m(den, dc))
    out.sort()
    return {"ratio": est, "lo": pct(out, .025), "hi": pct(out, .975)}

def fit(xs, ys):
    x = [math.log(a) for a in xs]; y = [math.log(a) for a in ys]; mx, my = mean(x), mean(y)
    return sum((a - mx) * (b - my) for a, b in zip(x, y)) / sum((a - mx) ** 2 for a in x)

def main():
    A = {"cells": [], "paired": {}, "L1": {}, "rho": {}}
    joined = []; runs = {}; insts = {}
    for p in PS:
        d = load("run_%d.jsonl" % p)
        runs[p] = [r for r in d if r["kind"] == "run"]; insts[p] = [r for r in d if r["kind"] == "instance"]
        joined += [json.dumps(r, sort_keys=True) for r in d if r["kind"] in ("instance", "run", "cell")]
    rho = {}; old = {}; oldinst = {}
    for p in PS:
        o = load(R2 + "run_%d.jsonl" % p)
        old[p] = {(r["curve_seed"], r["seed"], r["arm"]): r for r in o if r["kind"] == "run"}
        oldinst[p] = {r["curve_seed"]: r for r in o if r["kind"] == "instance"}
        rho[p] = [r for r in o if r["kind"] == "run" and r["arm"] == RHO and r["curve_seed"] in SEEDS[p]]
        joined += [json.dumps(r, sort_keys=True) for r in rho[p]]
    open("joined.jsonl", "w").write("\n".join(joined) + "\n")
    cell = {}
    for p in PS:
        rb = defaultdict(list)
        for r in rho[p]: rb[r["curve_seed"]].append(fpm(r))
        A["rho"][p] = {"runs": len(rho[p]), "fpm_mean": mean(x for v in rb.values() for x in v)}
        by = defaultdict(dict)
        for r in runs[p]: by[r["arm"]][(r["curve_seed"], r["seed"])] = r
        # L1
        bad = []; ibad = []
        for r in runs[p]:
            if r["arm"] in (UNM, PLAIN):
                o = old[p].get((r["curve_seed"], r["seed"], r["arm"]))
                if o is None: bad.append(["missing", r["curve_seed"], r["seed"], r["arm"]]); continue
                for k in o:
                    if k != "wall_ns" and o[k] != r.get(k): bad.append([k, r["curve_seed"], r["seed"], r["arm"], o[k], r.get(k)])
        for r in insts[p]:
            o = oldinst[p].get(r["curve_seed"])
            if o is None or any(o[k] != r.get(k) for k in o if k != "order_search_wall_s"): ibad.append(r["curve_seed"])
        arms = sorted(by)
        A["L1"][p] = {"plain_unm_runs_compared": sum(1 for r in runs[p] if r["arm"] in (UNM, PLAIN)), "field_mismatches": len(bad), "examples": bad[:6],
                      "instances": len(insts[p]), "instance_seed_list_ok": sorted(r["curve_seed"] for r in insts[p]) == sorted(SEEDS[p]),
                      "instance_mismatches": ibad, "runs": {a: len(by[a]) for a in arms},
                      "certified": {a: sum(1 for r in by[a].values() if r["certified"] and r["ok"]) for a in arms},
                      "fallbacks": sum(r.get("ex_fallback", 0) for r in runs[p]), "attempts_gt1": sum(1 for r in runs[p] if r["attempts"] > 1)}
        for a in arms:
            rs = list(by[a].values()); c = {"p": p, "arm": a, "runs": len(rs)}
            for k in ("relations", "trials", "core_rows", "core_cols", "la_mul", "rel_mul", "cantor_mul", "cantor_inv", "attempts"):
                c["mean_" + k] = mean(r[k] for r in rs)
            c["mean_fpm"] = mean(fpm(r) for r in rs); c["mean_relcost"] = mean(relcost(r) for r in rs)
            c["la_share"] = mean(9 * r["la_mul"] / fpm(r) for r in rs)
            if "sge_nnz_in" in rs[0]:
                for k in ("sge_nnz_in", "sge_nnz_out", "la_sge_mul", "la_wied_mul"): c["mean_" + k] = mean(r[k] for r in rs)
                c["mean_wt_before"] = c["mean_wt_after"] = mean(r["sge_nnz_in"] / r["sge_rows_in"] for r in rs)
            if "excess_rows_collected" in rs[0]:
                for k in ("excess_rows_collected", "rows_kept", "mean_weight_before", "mean_weight_after", "core_nnz_before_sge", "core_nnz_after_sge", "core_rows_pre", "core_cols_pre", "la_sel_mul", "ex_fallback", "sge_nnz_out", "la_sge_mul", "la_wied_mul"):
                    c["mean_" + k] = mean(r[k] for r in rs)
                c["mean_wt_before"] = c["mean_mean_weight_before"]; c["mean_wt_after"] = c["mean_mean_weight_after"]
            if a != RHO:
                num = defaultdict(list)
                for (cs, s), r in by[a].items(): num[cs].append(fpm(r))
                c["fpm_over_rho"] = ratio_rho(num, rb)
            cell[(p, a)] = c; A["cells"].append(c)
        for e in ES:
            ex = by[EXARM[e]]; pl = by[PLAIN]
            A["paired"]["%d|%s" % (p, e)] = {"fpm": paired(ex, pl, fpm), "la_mul": paired(ex, pl, lambda r: r["la_mul"]), "relcost": paired(ex, pl, relcost),
                                             "wt_after_over_plain": cell[(p, EXARM[e])]["mean_wt_after"] / cell[(p, PLAIN)]["mean_wt_before"]}
    # scorecard inputs
    L = {}
    L["L1"] = {"all_equal": all(A["L1"][p]["field_mismatches"] == 0 and not A["L1"][p]["instance_mismatches"] and A["L1"][p]["instance_seed_list_ok"] for p in PS),
               "all_certified": all(all(v == A["L1"][p]["runs"][a] for a, v in A["L1"][p]["certified"].items()) for p in PS)}
    thr = {65537: 0.90, 131101: 0.87, 262147: 0.85}
    L["L2"] = {str(p): {"fpm": A["paired"]["%d|0.2" % p]["fpm"]["mean"], "bound": thr[p], "la_mul": A["paired"]["%d|0.2" % p]["la_mul"]["mean"], "held": A["paired"]["%d|0.2" % p]["fpm"]["mean"] <= thr[p] and A["paired"]["%d|0.2" % p]["la_mul"]["mean"] <= 0.70} for p in PS}
    L["L3"] = {}
    for p in PS:
        rc = [cell[(p, EXARM[e])]["mean_relcost"] for e in ES]; la = [cell[(p, EXARM[e])]["mean_la_mul"] for e in ES]
        L["L3"][str(p)] = {"relcost": rc, "la_mul": la, "rel_order": rc[0] < rc[1] < rc[2], "la_order": la[2] <= la[1] <= la[0],
                           "fpm_ratio_by_E": [A["paired"]["%d|%s" % (p, e)]["fpm"]["mean"] for e in ES]}
    f262 = L["L3"]["262147"]["fpm_ratio_by_E"]; bestE = ES[f262.index(min(f262))]
    L["L3"]["best_E_262147"] = bestE
    L["L4"] = {str(p): {"wt_ratio_0.2": A["paired"]["%d|0.2" % p]["wt_after_over_plain"], "all_E": [A["paired"]["%d|%s" % (p, e)]["wt_after_over_plain"] for e in ES]} for p in PS}
    best = cell[(262147, EXARM[bestE])]
    L["L5"] = {"best_E": bestE, "ratio_to_rho": best["fpm_over_rho"], "plain_ratio_to_rho": cell[(262147, PLAIN)]["fpm_over_rho"], "bound": 0.33}
    ex = {}
    for name, arm in (("plain", lambda p: PLAIN), ("best", lambda p: EXARM[bestE])):
        ex[name + "_la_mul"] = fit(PS, [cell[(p, arm(p))]["mean_la_mul"] for p in PS])
        ex[name + "_fpm"] = fit(PS, [cell[(p, arm(p))]["mean_fpm"] for p in PS])
    for e in ES: ex["E%s_la_mul" % e] = fit(PS, [cell[(p, EXARM[e])]["mean_la_mul"] for p in PS])
    L["exponents"] = ex; A["scorecard"] = L
    json.dump(A, open("analysis.json", "w"), indent=1, sort_keys=True, default=str)
    print(json.dumps(L, indent=1))
if __name__ == "__main__": main()
