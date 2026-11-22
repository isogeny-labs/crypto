#!/usr/bin/env python3
"""analyze.py: grid.jsonl -> joined.jsonl (grid + reused rho rows), analysis.json, scorecard.json, tables.md.
Reads recorded counts only. FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul (rho: no rel/la terms).
Ratio to rho = mean over the runs of the cell / mean rho over the same instances' rho runs (ratio of means);
interval: hierarchical bootstrap (curves, then runs within a curve; 2000 resamples, seed 20261121) of
the ratio of means, rho resampled with its own curves, as paired.py does for ratios of arms."""
import json, random, sys
from collections import defaultdict
BOOT, SEED = 2000, 20261121
HERE = "/home/user/crypto/research/"
EARLIER = {32771: ("genus_ladder_sge_scale_20261115/sge_g3.jsonl", "genus_ladder_scale_20261115/scale_g3.jsonl"),
           65537: ("genus_ladder_fast_order_20261121/ext_g3.jsonl",) * 2,
           131101: ("genus_ladder_fast_order_20261121/ext_g3.jsonl",) * 2}
UNM, PLAIN = "C_double_lp+pre", "C_double_lp+pre+sge(8,3)"
INP, OUTJ, JOIN = (sys.argv[1], sys.argv[2], sys.argv[3]) if len(sys.argv) > 3 else ("grid.jsonl", "analysis.json", "joined.jsonl")
SETS = sys.argv[4].split(";") if len(sys.argv) > 4 else ["+ex(0.2,0.85)", "+ex(0.5,0.7)", "+ex(1,0.5)"]
ARMS = [UNM, PLAIN] + [PLAIN + s for s in SETS]
PS = [32771, 65537, 131101]

def fpm(r):
    g = r["g"]
    return r["cantor_mul"] + 10 * r["cantor_inv"] + r.get("rel_mul", 0) + g * g * r.get("la_mul", 0)
def relcost(r):
    return r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"]
def load(fn):
    return [json.loads(l) for l in open(fn)]
def mean(x): return sum(x) / len(x)

def boot_ratio(num, den):
    """num, den: dict curve -> list of values. Ratio of means with hierarchical bootstrap."""
    est = mean([v for c in num for v in num[c]]) / mean([v for c in den for v in den[c]])
    rng = random.Random(SEED)
    nc, dc = sorted(num), sorted(den)
    out = []
    for _ in range(BOOT):
        def m(d, cs):
            s = n = 0
            for _ in cs:
                v = d[rng.choice(cs)]
                for _ in v:
                    s += rng.choice(v); n += 1
            return s / n
        out.append(m(num, nc) / m(den, dc))
    out.sort()
    return {"ratio": est, "lo": out[int(0.025 * (BOOT - 1) + .5)], "hi": out[int(0.975 * (BOOT - 1) + .5)]}

def main():
    grid = load(INP)
    runs = [r for r in grid if r.get("kind") == "run"]
    rho = {}
    joined = [json.dumps(r, sort_keys=True) for r in grid if r.get("kind") in ("instance", "run", "cell")]
    for p in PS:
        for r in load(HERE + EARLIER[p][1]):
            if r.get("kind") == "run" and r["arm"] == "R_rho_neg" and r["p"] == p and r["g"] == 3:
                rho.setdefault(p, []).append(r)
                joined.append(json.dumps(r, sort_keys=True))
    open(JOIN, "w").write("\n".join(joined) + "\n")
    # instance identity
    seeds = {p: {r["curve_seed"] for r in grid if r.get("kind") == "instance" and r["p"] == p} for p in PS}
    A = {"cells": [], "paired": [], "x1": {}, "instances": {str(p): sorted(seeds[p]) for p in PS}}
    for p in PS:
        rho_by = defaultdict(list)
        for r in rho[p]:
            if r["curve_seed"] in seeds[p]:
                rho_by[r["curve_seed"]].append(fpm(r))
        A["rho_runs_%d" % p] = sum(len(v) for v in rho_by.values())
        A["rho_fpm_mean_%d" % p] = mean([x for v in rho_by.values() for x in v])
        by = defaultdict(list)
        for r in runs:
            if r["p"] == p: by[r["arm"]].append(r)
        for arm in ARMS:
            rs = by[arm]
            num = defaultdict(list)
            for r in rs: num[r["curve_seed"]].append(fpm(r))
            cell = {"p": p, "arm": arm, "runs": len(rs), "certified": sum(1 for r in rs if r["certified"] and r["ok"])}
            cell["fpm_over_rho"] = boot_ratio(num, rho_by)
            for k in ("la_mul", "la_add", "rel_mul", "relations", "trials", "core_rows", "core_cols", "attempts", "cantor_mul", "cantor_inv"):
                cell["mean_" + k] = mean([r[k] for r in rs])
            cell["mean_fpm"] = mean([fpm(r) for r in rs])
            cell["mean_relcost"] = mean([relcost(r) for r in rs])
            cell["la_share_fpm"] = mean([9 * r["la_mul"] / fpm(r) for r in rs])
            if "excess_rows_collected" in rs[0]:
                for k in ("excess_rows_collected", "rows_kept", "mean_weight_before", "mean_weight_after", "core_nnz_before_sge", "core_nnz_after_sge", "core_rows_pre", "core_cols_pre", "la_sel_mul", "la_sel_add", "ex_fallback", "sge_nnz_out", "la_sge_mul", "la_sge_add", "la_wied_mul"):
                    cell[k if k.startswith("mean_") else "mean_" + k] = mean([r[k] for r in rs])
            elif "sge_nnz_in" in rs[0]:
                for k in ("sge_nnz_in", "sge_nnz_out", "sge_rows_in", "la_sge_mul", "la_sge_add", "la_wied_mul"):
                    cell["mean_" + k] = mean([r[k] for r in rs])
                cell["mean_weight_after"] = cell["mean_weight_before"] = mean([r["sge_nnz_in"] / r["sge_rows_in"] for r in rs])
                cell["mean_core_nnz_before_sge"] = cell["mean_sge_nnz_in"]
                cell["mean_core_nnz_after_sge"] = cell["mean_sge_nnz_out"]
                cell["mean_rows_kept"] = cell["mean_sge_rows_in"]
            A["cells"].append(cell)
        # paired excess / plain per (curve, seed)
        key = lambda r: (r["curve_seed"], r["seed"])
        pl = {key(r): r for r in by[PLAIN]}
        for s in SETS:
            ex = {key(r): r for r in by[PLAIN + s]}
            for stat, f in (("fpm", fpm), ("la_mul", lambda r: r["la_mul"]), ("relcost", relcost)):
                num, den = defaultdict(list), defaultdict(list)
                pairs = [(k, ex[k], pl[k]) for k in ex if k in pl]
                rat = defaultdict(list)
                for k, e, b in pairs: rat[k[0]].append(f(e) / f(b))
                # mean of per-pair ratios with hierarchical bootstrap
                est = mean([x for v in rat.values() for x in v])
                rng = random.Random(SEED); cs = sorted(rat); ms = []
                for _ in range(BOOT):
                    t = n = 0
                    for _ in cs:
                        v = rat[rng.choice(cs)]
                        for _ in v: t += rng.choice(v); n += 1
                    ms.append(t / n)
                ms.sort()
                A["paired"].append({"p": p, "setting": s, "stat": stat, "mean": est, "lo": ms[int(.025 * (BOOT - 1) + .5)], "hi": ms[int(.975 * (BOOT - 1) + .5)], "pairs": len(pairs)})
    # X1: plain arm and unmerged arm rows against the earlier studies, every field but wall_ns
    for p in PS:
        old = [r for r in load(HERE + EARLIER[p][0]) if r.get("kind") == "run" and r["p"] == p and r["g"] == 3 and r["arm"] in (UNM, PLAIN)]
        oldk = {(r["curve_seed"], r["seed"], r["arm"]): r for r in old}
        new = [r for r in runs if r["p"] == p and r["arm"] in (UNM, PLAIN)]
        bad = []
        for r in new:
            o = oldk.get((r["curve_seed"], r["seed"], r["arm"]))
            if o is None: bad.append(("missing", r["curve_seed"], r["seed"], r["arm"])); continue
            for k in o:
                if k != "wall_ns" and o[k] != r.get(k): bad.append((k, r["curve_seed"], r["seed"], r["arm"], o[k], r.get(k)))
        inst_old = {(r["curve_seed"]): r for r in load(HERE + EARLIER[p][0]) if r.get("kind") == "instance" and r["p"] == p}
        inst_bad = []
        for r in grid:
            if r.get("kind") == "instance" and r["p"] == p:
                o = inst_old.get(r["curve_seed"])
                if o is None or any(o[k] != r[k] for k in ("f", "order", "h", "r", "curve_seed")): inst_bad.append(r["curve_seed"])
        A["x1"][str(p)] = {"runs_compared": len(new), "field_mismatches": len(bad), "examples": bad[:5], "instance_mismatches": inst_bad,
                           "all_certified": all(r["certified"] and r["ok"] for r in runs if r["p"] == p)}
    json.dump(A, open(OUTJ, "w"), indent=1, sort_keys=True)

if __name__ == "__main__": main()
