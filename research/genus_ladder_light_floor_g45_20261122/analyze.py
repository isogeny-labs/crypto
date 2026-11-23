#!/usr/bin/env python3
"""analyze.py: g4_2053 / g4_4099 / g5_1031 .jsonl (decompressed) + earlier studies' rho and plain rows -> analysis.json, joined.jsonl. Protocol: PROTOCOL.md.
Reads recorded counts only. Run in this directory."""
import json, math, random
from collections import defaultdict
R = "/home/user/crypto/research/"
C2 = R + "genus_ladder_cc_sweep_20261121/raw/"; SD = R + "genus_ladder_sge_depth_20261121/"
CELLS = [  # (g, p, file, curve seeds, g2, earlier C/plain rows, earlier rho rows)
    (4, 2053, "g4_2053.jsonl", [40, 60, 109], 16, C2 + "g4_cc2.0.jsonl", C2 + "rho_g4.jsonl"),
    (4, 4099, "g4_4099.jsonl", [72, 247, 418], 16, C2 + "g4_cc2.0.jsonl", C2 + "rho_g4.jsonl"),
    (5, 1031, "g5_1031.jsonl", [3, 39, 42], 25, SD + "g5.jsonl", SD + "rho5.jsonl"),
]
UNM, PLAIN, RHO = "C_double_lp+pre", "C_double_lp+pre+sge(8,3)", "R_rho_neg"
ES = ["0.1", "0.2", "0.3"]; EXARM = {e: PLAIN + "+ex(%s,0.01)" % e for e in ES}
B, SEED = 4000, 20261122
def load(f): return [json.loads(l) for l in open(f)]
def fpm(r, g2): return r["cantor_mul"] + 10 * r["cantor_inv"] + r.get("rel_mul", 0) + g2 * r.get("la_mul", 0)
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

def lfit(xs, ys):  # least squares y = a + b x
    mx, my = mean(xs), mean(ys); b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    return my - b * mx, b

def main():
    A = {"cells": [], "paired": {}, "L1": {}, "rho": {}, "extrap": {}}
    joined = []; cell = {}
    for g, p, f, seeds, g2, oldf, rhof in CELLS:
        key = "g%d_%d" % (g, p)
        d = load(f)
        runs = [r for r in d if r["kind"] == "run"]; insts = [r for r in d if r["kind"] == "instance"]
        joined += [json.dumps(r, sort_keys=True) for r in d if r["kind"] in ("instance", "run", "cell")]
        o = load(oldf)
        old = {(r["curve_seed"], r["seed"], r["arm"]): r for r in o if r["kind"] == "run" and r["p"] == p}
        oldinst = {r["curve_seed"]: r for r in o if r["kind"] == "instance" and r["p"] == p}
        rr = load(rhof)
        rhoruns = [r for r in rr if r["kind"] == "run" and r["arm"] == RHO and r["p"] == p and r["curve_seed"] in seeds]
        joined += [json.dumps(r, sort_keys=True) for r in rhoruns]
        ni = {r["curve_seed"]: r for r in insts}
        rho_mismatch = [r["curve_seed"] for r in rhoruns if r["curve_seed"] not in ni or any(r.get(k) != ni[r["curve_seed"]][k] for k in ("r", "h", "g", "p"))]
        rb = defaultdict(list)
        for r in rhoruns: rb[r["curve_seed"]].append(fpm(r, 0))
        nseeds = max(r["seed"] for r in runs)
        A["rho"][key] = {"runs": len(rhoruns), "fpm_mean": mean(x for v in rb.values() for x in v) if rb else None, "curves_with_rho": sorted(rb), "rho_instance_mismatch": rho_mismatch}
        by = defaultdict(dict)
        for r in runs: by[r["arm"]][(r["curve_seed"], r["seed"])] = r
        bad = []; ibad = []
        for r in runs:
            if r["arm"] in (UNM, PLAIN):
                ol = old.get((r["curve_seed"], r["seed"], r["arm"]))
                if ol is None: bad.append(["missing", r["curve_seed"], r["seed"], r["arm"]]); continue
                for k in ol:
                    if k != "wall_ns" and ol[k] != r.get(k): bad.append([k, r["curve_seed"], r["seed"], r["arm"], ol[k], r.get(k)])
        for r in insts:
            ol = oldinst.get(r["curve_seed"])
            if ol is None or any(ol[k] != r.get(k) for k in ol if "wall" not in k): ibad.append(r["curve_seed"])
        arms = sorted(by)
        A["L1"][key] = {"plain_unm_runs_compared": sum(1 for r in runs if r["arm"] in (UNM, PLAIN)), "field_mismatches": len(bad), "examples": bad[:6],
                        "instances": len(insts), "instance_seed_list_ok": sorted(ni) == sorted(seeds), "instance_mismatches": ibad,
                        "runs": {a: len(by[a]) for a in arms}, "certified": {a: sum(1 for r in by[a].values() if r["certified"] and r["ok"]) for a in arms},
                        "fallbacks": sum(r.get("ex_fallback", 0) for r in runs), "attempts_gt1": sum(1 for r in runs if r["attempts"] > 1),
                        "expected_runs_per_arm": len(seeds) * nseeds}
        for a in arms:
            rs = list(by[a].values()); c = {"g": g, "p": p, "key": key, "arm": a, "runs": len(rs), "g2": g2}
            for k in ("relations", "trials", "core_rows", "core_cols", "la_mul", "rel_mul", "cantor_mul", "cantor_inv", "attempts"):
                c["mean_" + k] = mean(r[k] for r in rs)
            c["mean_fpm"] = mean(fpm(r, g2) for r in rs); c["mean_relcost"] = mean(relcost(r) for r in rs)
            c["la_share"] = mean(g2 * r["la_mul"] / fpm(r, g2) for r in rs)
            if "sge_nnz_in" in rs[0] and "excess_rows_collected" not in rs[0]:
                for k in ("sge_nnz_in", "sge_nnz_out", "la_sge_mul", "la_wied_mul"): c["mean_" + k] = mean(r[k] for r in rs)
                c["mean_wt_before"] = c["mean_wt_after"] = mean(r["sge_nnz_in"] / r["sge_rows_in"] for r in rs)
            if "excess_rows_collected" in rs[0]:
                for k in ("excess_rows_collected", "rows_kept", "mean_weight_before", "mean_weight_after", "core_nnz_before_sge", "core_nnz_after_sge", "core_rows_pre", "core_cols_pre", "la_sel_mul", "ex_fallback", "sge_nnz_out", "la_sge_mul", "la_wied_mul"):
                    c["mean_" + k] = mean(r[k] for r in rs)
                c["mean_wt_before"] = c["mean_mean_weight_before"]; c["mean_wt_after"] = c["mean_mean_weight_after"]
            num = defaultdict(list)
            for (cs, s), r in by[a].items():
                if cs in rb: num[cs].append(fpm(r, g2))
            c["fpm_over_rho"] = ratio_rho(num, rb) if rb and num else None
            c["rho_curves_used"] = sorted(num)
            cell[(key, a)] = c; A["cells"].append(c)
        for e in ES:
            ex = by[EXARM[e]]; pl = by[PLAIN]
            A["paired"]["%s|%s" % (key, e)] = {"fpm": paired(ex, pl, lambda r: fpm(r, g2)), "la_mul": paired(ex, pl, lambda r: r["la_mul"]), "relcost": paired(ex, pl, relcost),
                                               "wt_after_over_plain": cell[(key, EXARM[e])]["mean_wt_after"] / cell[(key, PLAIN)]["mean_wt_before"]}
        # extrapolation in E (outside the scorecard, not run): log la_mul linear in E over the three E; relation phase (1+E) x plain's (factor k measured)
        pl = cell[(key, PLAIN)]; rel0 = pl["mean_relcost"]; xs = [float(e) for e in ES]
        a0, b0 = lfit(xs, [math.log(cell[(key, EXARM[e])]["mean_la_mul"]) for e in ES])
        k = mean(cell[(key, EXARM[e])]["mean_relcost"] / (rel0 * (1 + float(e))) for e in ES)
        ext = {}
        for E in (0.1, 0.2, 0.3, 0.4, 0.5, 0.7, 1.0):
            la = math.exp(a0 + b0 * E); fp = k * rel0 * (1 + E) + g2 * la
            ext[str(E)] = {"la_mul": la, "fpm": fp, "fpm_over_plain": fp / pl["mean_fpm"]}
        A["extrap"][key] = {"la_slope_per_E": b0, "rel_k": k, "model": ext}
    L = {}
    keys = ["g%d_%d" % (g, p) for g, p, *_ in CELLS]
    L["G1"] = {"all_equal": all(A["L1"][k]["field_mismatches"] == 0 and not A["L1"][k]["instance_mismatches"] and A["L1"][k]["instance_seed_list_ok"] for k in keys),
               "all_certified": all(all(v == A["L1"][k]["runs"][a] == A["L1"][k]["expected_runs_per_arm"] for a, v in A["L1"][k]["certified"].items()) for k in keys),
               "rho_instances_match": all(not A["rho"][k]["rho_instance_mismatch"] for k in keys)}
    best = {}
    for k in keys:
        f = [A["paired"]["%s|%s" % (k, e)]["fpm"]["mean"] for e in ES]; best[k] = ES[f.index(min(f))]
    L["best_E"] = best
    L["G2"] = {k: {"best_E": best[k], "fpm": A["paired"]["%s|%s" % (k, best[k])]["fpm"], "bound": 0.80 if k in ("g4_4099", "g5_1031") else None,
                   "held": (A["paired"]["%s|%s" % (k, best[k])]["fpm"]["mean"] <= 0.80) if k in ("g4_4099", "g5_1031") else None} for k in keys}
    L["G3"] = {k: {"best_E": best[k], "la_mul_best": A["paired"]["%s|%s" % (k, best[k])]["la_mul"]["mean"], "la_mul_E0.3": A["paired"]["%s|0.3" % k]["la_mul"]["mean"],
                   "held": A["paired"]["%s|%s" % (k, best[k])]["la_mul"]["mean"] <= 0.65} for k in keys}
    L["G4"] = {k: {"best_E": best[k], "held": best[k] == "0.3"} for k in keys}
    arms_all = [UNM, PLAIN] + [EXARM[e] for e in ES]
    r4099 = {a: cell[("g4_4099", a)]["fpm_over_rho"] for a in arms_all}
    ba = min(arms_all, key=lambda a: r4099[a]["ratio"])
    L["G5"] = {"best_arm": ba, "ratio": r4099[ba], "plain": r4099[PLAIN], "bound": 0.016, "held": r4099[ba]["ratio"] <= 0.016}
    L["la_share_plain"] = {k: cell[(k, PLAIN)]["la_share"] for k in keys}
    A["scorecard"] = L
    open("joined.jsonl", "w").write("\n".join(joined) + "\n")
    json.dump(A, open("analysis.json", "w"), indent=1, sort_keys=True, default=str)
    print(json.dumps(L, indent=1))
if __name__ == "__main__": main()
