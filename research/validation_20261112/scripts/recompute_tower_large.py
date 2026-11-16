#!/usr/bin/env python3
"""Recompute the tower_large headline numbers (TOWER_LARGE.md tables and fits) from the
per-instance means in tower_large.json (validator's own code). Usage: recompute_tower_large.py tower_large.json OUT.json"""
import json, sys, math
d = json.load(open(sys.argv[1]))
W = [0.0, 0.1, 1.0]
errs = []; out = {"rows": [], "fits": {}}
def close(a, b, tol=1e-9): return abs(a - b) <= tol * max(1.0, abs(a), abs(b))
U_IC_KEYS = ["la_mulmods", "la_inversions", "oracle_fp_muls", "factor_base_fp_muls", "factor_base_lookups", "canonicalisations", "step_draws"]
inst = {}
for i in d["instances"]: inst.setdefault(i["p"], []).append(i)
for row in d["rows"]:
    p = row["p"]; L = inst[p]
    n = len(L)
    if n != row["instances"]: errs.append(f"p={p}: instances {n} != {row['instances']}")
    mean = lambda f: sum(f(i) for i in L) / n
    rho = mean(lambda i: i["rho_ops"]); steps = mean(lambda i: i["rho_steps"]); dps = mean(lambda i: i["rho_dps"])
    for name, a, b in (("rho_ops", rho, row["rho_ops"]), ("rho_steps", steps, row["rho_steps"]), ("rho_dps", dps, row["rho_dps"]),
                       ("r_mean", mean(lambda i: i["r"]), row["r_mean"])):
        if not close(a, b): errs.append(f"p={p} {name}: recomputed {a} vs {b}")
    rec = {"p": p, "r_mean": row["r_mean"], "rho_ops": rho, "rho_steps": steps}
    for g in (0, 1):
        ic = {k: mean(lambda i: i["ic"][g][k]) for k in row["ic"][g]}
        for k, v in ic.items():
            if not close(v, row["ic"][g][k]): errs.append(f"p={p} g={g+2} {k}: {v} vs {row['ic'][g][k]}")
        rl = [(ic["ops"] + w * ic["la_mulmods"]) / rho for w in W]
        rw = [(ic["ops"] + w * ic["la_mulmods"]) / steps for w in W]
        uic = sum(ic[k] for k in U_IC_KEYS)
        ra = [(ic["ops"] + w * uic) / (rho + w * (steps + dps)) for w in W]
        for name, a, b in (("ratio_la", rl, row["ratio_la"][g]), ("ratio_la_vs_walk", rw, row["ratio_la_vs_walk"][g]), ("ratio_all", ra, row["ratio_all"][g])):
            for x, y, w in zip(a, b, W):
                if not close(x, y, 1e-6): errs.append(f"p={p} g={g+2} {name} w={w}: {x} vs {y}")
        trpr = ic["trials"] / ic["relations"]
        if not close(trpr, row["trials_per_relation"][g], 1e-6): errs.append(f"p={p} g={g+2} trials/rel {trpr} vs {row['trials_per_relation'][g]}")
        above = all(x > 1 for x in rl); below = all(x < 1 for x in rl)
        v = "above rho at every weight" if above else ("below rho at every weight" if below else "unresolved (order changes inside the band)")
        if v != row["verdict_la"][g]: errs.append(f"p={p} g={g+2} verdict_la {v} vs {row['verdict_la'][g]}")
        rec[f"ic{g+2}_ops"] = ic["ops"]; rec[f"la{g+2}"] = ic["la_mulmods"]; rec[f"ratio_la_g{g+2}"] = rl
        rec[f"ratio_all_g{g+2}"] = ra; rec[f"ratio_walk_g{g+2}"] = rw; rec[f"verdict_g{g+2}"] = v
        rec[f"U_ic{g+2}"] = uic
    i32 = row["ic"][1]["ops"] / row["ic"][0]["ops"]
    if not close(i32, row["ic3_over_ic2"], 1e-9): errs.append(f"p={p} ic3/ic2 {i32} vs {row['ic3_over_ic2']}")
    rec["ic3_over_ic2"] = i32
    if not all(i["all_recovered"] for i in L) or not row["all_recovered"]: errs.append(f"p={p}: not all recovered")
    out["rows"].append(rec)
def slope(xs, ys):
    lx = [math.log(x) for x in xs]; ly = [math.log(y) for y in ys]
    mx = sum(lx) / len(lx); my = sum(ly) / len(ly)
    return sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)
ps = [r["p"] for r in d["rows"]]
series = {"rho on E (group ops)": [r["rho_ops"] for r in d["rows"]],
          "rho on E walk steps only": [r["rho_steps"] for r in d["rows"]],
          "mean r": [r["r_mean"] for r in d["rows"]],
          "IC2 group ops": [r["ic"][0]["ops"] for r in d["rows"]],
          "IC3 group ops": [r["ic"][1]["ops"] for r in d["rows"]],
          "IC2 linalg mulmods": [r["ic"][0]["la_mulmods"] for r in d["rows"]],
          "IC3 linalg mulmods": [r["ic"][1]["la_mulmods"] for r in d["rows"]]}
fits = {f["series"]: f for f in d["fits"]}
for name, ys in series.items():
    s_all = slope(ps, ys); idx = [k for k, p in enumerate(ps) if p >= 1000]
    s_big = slope([ps[k] for k in idx], [ys[k] for k in idx])
    out["fits"][name] = [s_all, s_big]
    if name in fits:
        if not close(s_all, fits[name]["slope_all_points"], 1e-6) or not close(s_big, fits[name]["slope_p_ge_1000"], 1e-6):
            errs.append(f"fit {name}: {s_all},{s_big} vs {fits[name]['slope_all_points']},{fits[name]['slope_p_ge_1000']}")
    else: errs.append(f"fit {name} missing in json")
out["certified_solves_total"] = sum(i["certified_solves"] for i in d["instances"])
out["errors"] = errs
json.dump(out, open(sys.argv[2], "w"), indent=1)
for r in out["rows"]:
    print(r["p"], round(r["rho_ops"], 1), round(r["ic2_ops"], 1), round(r["ic3_ops"], 1), [round(x, 2) for x in r["ratio_la_g2"]], [round(x, 2) for x in r["ratio_la_g3"]], r["verdict_g2"][:10], r["verdict_g3"][:10])
for k, v in out["fits"].items(): print(f"  slope {k}: {v[0]:.3f} / {v[1]:.3f}")
print("certified solves in json:", out["certified_solves_total"], "; errors:", len(errs))
for e in errs[:20]: print("  ", e)
