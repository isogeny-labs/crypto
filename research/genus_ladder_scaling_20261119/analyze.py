#!/usr/bin/env python3
"""Readings, gates and statistics of PROTOCOL.md (genus_ladder_scaling_20261119).
Written before any data of this study existed. Every number it prints is [pcc:unverified]."""
import glob, gzip, json, math, os, random

HERE = os.path.dirname(os.path.abspath(__file__))
PO = os.path.join(HERE, "..", "genus4_prime_order_20261118")
NB, SEED = 2000, 20261126
WS = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)]


def rows(pattern):
    fs = sorted(glob.glob(pattern))
    keep = [f for f in fs if not (f.endswith(".jsonl") and f + ".gz" in fs)]
    out = []
    for f in keep:
        op = gzip.open if f.endswith(".gz") else open
        out += [json.loads(l) for l in op(f, "rt") if l.strip()]
    return out


def runs_and_inst(pattern, g, p):
    d, inst = {}, {}
    for r in rows(pattern):
        if r.get("p") != p or r.get("g") != g:
            continue
        if r.get("kind") == "instance":
            inst[r["curve_seed"]] = r
        elif r.get("kind") == "run":
            d[(r["curve_seed"], r["target_seed"], r["seed"])] = r
    return d, inst


curves = json.load(open(os.path.join(HERE, "curves.json")))
tune = json.load(open(os.path.join(HERE, "tune.json")))
x5star_is_x5 = tune["choice"]["cc"] == "2.0" and tune["choice"]["w_max"] == 4 and tune["choice"]["fill_max"] == 1.5

# (genus, p, arm) -> (runs, instances, confirmation curves)
cells = {}
for p in (16411, 32771):
    for arm, name in (("x", "x"), ("x*", "xstar")):
        d, inst = runs_and_inst(os.path.join(PO, f"b_p{p}_{name}.jsonl*"), 4, p)
        cells[(4, p, arm)] = (d, inst, sorted({k[0] for k in d}))
for arm, cc in (("x", "2.0"), ("x*", "1.4")):
    d, inst = runs_and_inst(os.path.join(HERE, f"b4_p60013_cc{cc}.jsonl*"), 4, 60013)
    cells[(4, 60013, arm)] = (d, inst, curves["g4_p60013"]["confirm"])
for p in (2357, 4099):
    for arm, name in (("x5", "x5"), ("x5*", "x5" if x5star_is_x5 else "x5star")):
        d, inst = runs_and_inst(os.path.join(HERE, f"b5_p{p}_{name}.jsonl*"), 5, p)
        cells[(5, p, arm)] = (d, inst, curves[f"g5_p{p}"]["confirm"])

gates = {"G1": [], "G2": [], "G3": []}
for (g, p, arm), (d, inst, cs) in cells.items():
    for k, r in d.items():
        if not (r.get("certified") and r.get("ok")):
            gates["G1"].append(f"g{g} p{p} {arm} {k}")
    if not d:
        gates["G1"].append(f"g{g} p{p} {arm}: no runs")
for key_, c in curves.items():
    if not c["enough"]:
        gates["G2"].append(f"{key_}: not enough admitted seeds over a complete range")
for (g, p, arm), (d, inst, cs) in cells.items():
    for c in cs:
        if c not in inst or inst[c]["h"] != 1:
            gates["G2"].append(f"g{g} p{p} curve {c}")
Q = [r for r in rows(os.path.join(HERE, "rho5_p2357.jsonl*")) if r.get("kind") == "run"]
inst5 = cells[(5, 2357, "x5")][1]
q_ratio = (sum(r["group_ops"] for r in Q) / len(Q)) / (sum(math.sqrt(math.pi * inst5[r["curve_seed"]]["r"] / 2) for r in Q) / len(Q)) if Q else None
if q_ratio is None or not (0.5 <= q_ratio <= 1.8) or not all(r.get("certified") for r in Q):
    gates["G3"].append(f"rho mean / sqrt(pi r/2) = {q_ratio}, runs {len(Q)}")
gate_ok = {g: not v for g, v in gates.items()}

native = lambda r: r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]
cost = lambda r, w: r["group_ops"] + w * native(r)
mean = lambda v: sum(v) / len(v)


def resample(rnd, keys):
    cs = sorted({k[0] for k in keys})
    out = []
    for c in [rnd.choice(cs) for _ in cs]:
        kc = [k for k in keys if k[0] == c]
        tg = sorted({k[:2] for k in kc})
        for t in [rnd.choice(tg) for _ in tg]:
            kt = [k for k in kc if k[:2] == t]
            out += [rnd.choice(kt) for _ in kt]
    return out


stream = {}
out = {"gates": gate_ok, "gate_failures": gates, "tune": tune["choice"], "x5star_is_x5": x5star_is_x5,
       "rho_check": {"runs": len(Q), "mean_over_sqrt_pi_r_2": q_ratio}, "cells": {}, "claims": "[pcc:unverified]"}
for i, ((g, p, arm), (d, inst, cs)) in enumerate(sorted(cells.items(), key=lambda kv: (kv[0][0], kv[0][1], kv[0][2]))):
    keys = sorted(k for k in d if k[0] in cs)
    if not keys:
        continue
    R = lambda kk: mean([math.sqrt(math.pi * inst[k[0]]["r"] / 4) for k in kk])
    c = {"keys": len(keys), "curves": cs, "mean_r": mean([inst[c_]["r"] for c_ in cs]), "neg": {}}
    for n, w in WS:
        fn = lambda kk, w=w: mean([cost(d[k], w) for k in kk]) / R(kk)
        rnd = random.Random(SEED + 101 * i + int(w * 10))
        raw = [fn(resample(rnd, keys)) for _ in range(NB)]
        s = sorted(raw)
        c["neg"][n] = {"ratio": fn(keys), "lo": s[int(0.025 * NB)], "hi": s[int(0.975 * NB) - 1]}
        stream[(g, p, arm, n)] = raw
    c["below_all_w"] = all(c["neg"][n]["hi"] < 1 for n, _ in WS)
    c["mean_cost_w1"] = mean([cost(d[k], 1) for k in keys])
    c["la_share_w1"] = mean([d[k]["la_mul"] + d[k]["la_add"] for k in keys]) / c["mean_cost_w1"]
    c["per_curve_w1"] = {str(c_): mean([cost(d[k], 1) for k in keys if k[0] == c_]) / mean([math.sqrt(math.pi * inst[c_]["r"] / 4)]) for c_ in cs}
    out["cells"][f"g{g}_p{p}_{arm}"] = c


def ols_slope(xs, ys):
    mx, my = mean(xs), mean(ys)
    return sum((a - mx) * (b - my) for a, b in zip(xs, ys)) / sum((a - mx) ** 2 for a in xs)


def q(v):
    s = sorted(v)
    return [s[int(0.025 * len(s))], s[int(0.975 * len(s)) - 1]]


# C1: three-point genus-4 fit
P4 = [16411, 32771, 60013]
out["C1"] = {}
for arm in ("x", "x*"):
    if not all(f"g4_p{p}_{arm}" in out["cells"] for p in P4):
        continue
    xs = [math.log(p) for p in P4]
    ys = [math.log(out["cells"][f"g4_p{p}_{arm}"]["neg"]["1"]["ratio"]) for p in P4]
    slope = ols_slope(xs, ys)
    bs = [ols_slope(xs, [math.log(stream[(4, p, arm, "1")][j]) for p in P4]) for j in range(NB)]
    ce = ols_slope(xs, [math.log(out["cells"][f"g4_p{p}_{arm}"]["mean_cost_w1"]) for p in P4])
    a = mean(ys) - slope * mean(xs)
    out["C1"][arm] = {"slope_ratio_vs_neg_w1": slope, "slope_ci": q(bs), "cost_exponent": ce,
                      "middle_residual": ys[1] - (a + slope * xs[1]),
                      "consistent_with_1p5": None}
# cost exponent CI by bootstrapping mean cost via ratio streams times R (R fixed per key set)
for arm in out["C1"]:
    Rm = {p: out["cells"][f"g4_p{p}_{arm}"]["mean_cost_w1"] / out["cells"][f"g4_p{p}_{arm}"]["neg"]["1"]["ratio"] for p in P4}
    xs = [math.log(p) for p in P4]
    bs = [ols_slope(xs, [math.log(stream[(4, p, arm, "1")][j] * Rm[p]) for p in P4]) for j in range(NB)]
    ci = q(bs)
    out["C1"][arm]["cost_exponent_ci"] = ci
    out["C1"][arm]["consistent_with_1p5"] = ci[0] <= 1.6 and ci[1] >= 1.4

# C2: genus 5 vs genus 4 at matched size
out["C2"] = {}
for g5p, g4p in ((2357, 16411), (4099, 32771)):
    k5, k4 = f"g5_p{g5p}_x5*", f"g4_p{g4p}_x*"
    if k5 in out["cells"] and k4 in out["cells"]:
        r = out["cells"][k5]["neg"]["1"]["ratio"] / out["cells"][k4]["neg"]["1"]["ratio"]
        bs = [a / b for a, b in zip(stream[(5, g5p, "x5*", "1")], stream[(4, g4p, "x*", "1")])]
        ci = q(bs)
        out["C2"][f"N_g5p{g5p}_vs_g4p{g4p}"] = {"x5star_over_xstar_w1": r, "ci": ci,
                                                 "verdict": "genus 5 ahead" if ci[1] < 1 else "genus 5 behind" if ci[0] > 1 else "unresolved",
                                                 "N_g5": out["cells"][k5]["mean_r"], "N_g4": out["cells"][k4]["mean_r"]}
for g, (pa, pb), arm in ((5, (2357, 4099), "x5*"), (4, (16411, 32771), "x*")):
    ka, kb = f"g{g}_p{pa}_{arm}", f"g{g}_p{pb}_{arm}"
    if ka in out["cells"] and kb in out["cells"]:
        dn = math.log(out["cells"][kb]["mean_r"]) - math.log(out["cells"][ka]["mean_r"])
        sl = (math.log(out["cells"][kb]["neg"]["1"]["ratio"]) - math.log(out["cells"][ka]["neg"]["1"]["ratio"])) / dn
        bs = [(math.log(b) - math.log(a)) / dn for a, b in zip(stream[(g, pa, arm, "1")], stream[(g, pb, arm, "1")])]
        out["C2"][f"slope_in_lnN_g{g}"] = {"slope": sl, "ci": q(bs)}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

f3 = lambda v: f"{v['ratio']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}]"
L = ["# Tables (generated by analyze.py; every number [pcc:unverified])", "",
     "Gates: " + ", ".join(f"{g} {'pass' if ok else 'FAIL'}" for g, ok in gate_ok.items()),
     f"Genus-5 tuning: cc {tune['choice']['cc']}, SGE ({tune['choice']['w_max']}, {tune['choice']['fill_max']})"
     + (" = baseline (x5* = x5)" if x5star_is_x5 else ""),
     f"Measured plain rho, genus 5, p = 2357 ({len(Q)} runs): mean / sqrt(pi r/2) = {q_ratio}", "",
     "| genus | p | arm | mean r | vs ideal negation w=0 | w=0.1 | w=1 | below at all w | LA share |",
     "|--|--|--|--|--|--|--|--|--|"]
for k, c in out["cells"].items():
    g, p, arm = k.split("_", 2)
    L.append(f"| {g[1:]} | {p[1:]} | {arm} | {c['mean_r']:.3e} | {f3(c['neg']['0'])} | {f3(c['neg']['0.1'])} | {f3(c['neg']['1'])} | "
             f"**{c['below_all_w']}** | {c['la_share_w1']:.2f} |")
L += ["", "## C1: genus-4 three-point fit (p = 16411, 32771, 60013)", ""]
for arm, v in out["C1"].items():
    L.append(f"- {arm}: slope of ln(ratio vs ideal negation, w = 1) in ln p = {v['slope_ratio_vs_neg_w1']:.3f} "
             f"[{v['slope_ci'][0]:.3f}, {v['slope_ci'][1]:.3f}]; cost exponent {v['cost_exponent']:.3f} "
             f"[{v['cost_exponent_ci'][0]:.3f}, {v['cost_exponent_ci'][1]:.3f}] (consistent with 1.5: {v['consistent_with_1p5']}); "
             f"middle-point residual {v['middle_residual']:+.3f}")
L += ["", "## C2: genus 5 against genus 4 at matched group size", ""]
for k, v in out["C2"].items():
    if k.startswith("N_"):
        L.append(f"- {k}: x5* / x* (w = 1) = {v['x5star_over_xstar_w1']:.3f} [{v['ci'][0]:.3f}, {v['ci'][1]:.3f}] → **{v['verdict']}** "
                 f"(N {v['N_g5']:.3e} vs {v['N_g4']:.3e})")
    else:
        L.append(f"- {k}: slope of ln(ratio, w = 1) in ln N = {v['slope']:.3f} [{v['ci'][0]:.3f}, {v['ci'][1]:.3f}]")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L))
