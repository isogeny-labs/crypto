#!/usr/bin/env python3
"""Independent recompute of the small_de headline numbers (validator's own code, not render.py).
Inputs: small_de.json (replayed), dump of instances (val_sd_dump), dump of rho runs (val_sd_rho),
the committed small_de_derived.json (producer-derived, compared against).
Usage: recompute_small_de.py small_de.json sd_dump.jsonl sd_rho.jsonl small_de_derived.json OUT.json"""
import json, sys, os, math, itertools
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import EC, is_prime
raw = json.load(open(sys.argv[1]))
dump = {(d["bits"], d["d"]): d for d in map(json.loads, open(sys.argv[2])) if not d.get("none")}
rho_dump = [json.loads(l) for l in open(sys.argv[3]) if l.strip()]
derived = json.load(open(sys.argv[4]))
W = (0.0, 0.1, 1.0)
errs = []; out = {}
def nat(n): return n["muls"] + n["invs"] + n["tests"] + n["la_ops"]
def mse(xs):
    n = len(xs); m = sum(xs) / n
    return (m, math.sqrt(sum((x - m) ** 2 for x in xs) / (n - 1) / n) if n > 1 else 0.0, n)
# --- rho runs: independent certificate of every recovered log, and cost equality with small_de.json
rho_idx = {}
ncert_rho = 0
for d in rho_dump:
    inst = dump[(d["bits"], d["d"])]
    E = EC(inst["p"], 0, inst["a"], inst["b"]); G = (inst["g"][0], inst["g"][1])
    T = None if d["tinf"] else (d["tx"], d["ty"])
    ok = d["recovered"] is not None and d["recovered"] == d["t"] and E.mul(d["recovered"], G) == T and E.mul(d["t"], G) == T
    ncert_rho += ok
    if not ok: errs.append(("rho run not independently certified", d["bits"], d["m"], d["d"], d["kind"], d["k"]))
    rho_idx.setdefault((d["bits"], d["m"], d["d"], d["kind"]), []).append(d)
# --- cells
cells_out = []; ncert_ic = 0
for c in raw["cells"]:
    key = (c["bits"], c["m"], c["d"])
    inst = dump[(c["bits"], c["d"])]
    E = EC(inst["p"], 0, inst["a"], inst["b"]); G = (inst["g"][0], inst["g"][1])
    for f in ("p", "a", "b", "r", "n", "h"):
        if c["inst"][f] != inst[f]: errs.append(("instance field mismatch", key, f))
    # rho: per kind, 16 runs, cost equal to the validator's replay of the same calls
    rho = {}
    for rk in c["rho"]:
        runs = rk["runs"]
        mine = rho_idx.get(key + (rk["kind"],), [])
        if len(runs) != 16 or len(mine) != 16: errs.append(("rho run count", key, rk["kind"], len(runs), len(mine)))
        for a, b in zip(runs, mine):
            if a["gae"] != b["gae"] or a["native"] != b["native"]: errs.append(("rho cost differs from validator replay", key, rk["kind"]))
            if not (a["certified"] and a["verified"]): errs.append(("rho flag false", key, rk["kind"]))
        rho[rk["kind"]] = {w: mse([x["gae"] + w * x["native"] for x in runs]) for w in W}
        rho[rk["kind"]]["sens"] = mse([x["gae"] + 2 * x["native"] for x in runs])  # DP-table sensitivity at w=1: canonicalisations counted twice
        rho[rk["kind"]]["gae_only"] = mse([x["gae"] for x in runs])
    best = {w: min(rho.values(), key=lambda v: v[w][0])[w][0] for w in W}
    best_kind = {w: min(rho, key=lambda k: rho[k][w][0]) for w in W}
    arms = {}
    for a in c["arms"]:
        # independent target certificates
        for k, ts in enumerate(a["targets"]):
            t = inst["targets"][k]; T = None if t["inf"] else (t["x"], t["y"])
            ok = ts["recovered"] is not None and ts["recovered"] == t["t"] and E.mul(ts["recovered"], G) == T
            ncert_ic += ok
            if not ok: errs.append(("IC target not certified", key, a["arm"], k))
            if ok != ts["certified"]: errs.append(("certified flag disagrees", key, a["arm"], k))
        rel = {w: mse([r["gae"] + a["build_gae"] + w * (nat(r["native"]) + nat(a["build_native"])) for r in a["reps"]]) for w in W}
        tgt = {w: mse([t["gae"] + w * nat(t["native"]) for t in a["targets"]]) for w in W}
        single = {w: rel[w][0] + tgt[w][0] for w in W}
        amort = {w: rel[w][0] / len(a["targets"]) + tgt[w][0] for w in W}
        # field-arithmetic native (muls + invs + la_ops) vs probes (tests), for the calibrated reading
        f_rel = sum(r["native"]["muls"] + r["native"]["invs"] + r["native"]["la_ops"] for r in a["reps"]) / len(a["reps"]) \
            + a["build_native"]["muls"] + a["build_native"]["invs"] + a["build_native"]["la_ops"]
        t_rel = sum(r["native"]["tests"] for r in a["reps"]) / len(a["reps"]) + a["build_native"]["tests"]
        f_tgt = sum(t["native"]["muls"] + t["native"]["invs"] + t["native"]["la_ops"] for t in a["targets"]) / len(a["targets"])
        t_tgt = sum(t["native"]["tests"] for t in a["targets"]) / len(a["targets"])
        priced = rel[0.0][0] + tgt[0.0][0]
        arms[a["arm"]] = {"rel": rel, "tgt": tgt, "single": single, "amort": amort, "columns": a["columns"],
                          "priced": priced, "field_native": f_rel + f_tgt, "probe_native": t_rel + t_tgt,
                          "rule_met": all(r["rule_met"] for r in a["reps"]), "reps": len(a["reps"]),
                          "ops_per_trial": [r["sweep_field_ops_per_trial"] for r in a["reps"]],
                          "certified": sum(1 for t in a["targets"] if t["certified"])}
    # worst case for IC over any per-class weights in [0,1]: IC natives at 0, rho canonicalisations at 1 and a
    # further 1 per canonicalisation for undeclared DP-table work
    rho_worst = min(v["sens"][0] for v in rho.values())
    cells_out.append({"key": key, "rho_best": best, "rho_best_kind": best_kind, "rho_worst_case_for_ic": rho_worst, "arms": arms})
# --- headline: ratios of single-instance totals to the best matched rho
ratios = {w: [] for w in W}; below = {w: 0 for w in W}; worst_case = []
for co in cells_out:
    for an, a in co["arms"].items():
        for w in W:
            x = a["single"][w] / co["rho_best"][w]; ratios[w].append((x, co["key"], an))
            below[w] += x < 1
        worst_case.append((a["priced"] / co["rho_worst_case_for_ic"], co["key"], an))
out["arm_cells"] = len(ratios[0.0])
out["min_single_ratio"] = {str(w): min(ratios[w])[0] for w in W}
out["argmin_single_ratio"] = {str(w): [list(min(ratios[w])[1]), min(ratios[w])[2]] for w in W}
out["arm_cells_below_best_rho"] = {str(w): below[w] for w in W}
out["min_ratio_ic_natives_free_rho_natives_and_dp_charged"] = min(worst_case)[0]
out["argmin_worst_case"] = [list(min(worst_case)[1]), min(worst_case)[2]]
# amortised best row at w=0.1
am = [(a["amort"][0.1] / co["rho_best"][0.1], co["key"], an) for co in cells_out for an, a in co["arms"].items()]
out["best_amortised_ratio_w0.1"] = [min(am)[0], list(min(am)[1]), min(am)[2]]
am28 = [x for x in am if x[1][0] == 28]
out["amortised_ratio_w0.1_28bits_range"] = [min(am28)[0], max(am28)[0]]
# solving work per trial identical across arms of a cell
same = 0; tot = 0
for co in cells_out:
    vals = {an: a["ops_per_trial"] for an, a in co["arms"].items()}
    v = list(vals.values()); tot += 1
    if all(x == v[0][0] for vv in v for x in vv): same += 1
out["cells_with_identical_sweep_ops_per_trial"] = [same, tot]
# A2 vs A1 gain rule (two pooled SE on the relation phase, every weight, every cell of the D)
verd = {}
for d in (-3, -4, -7, -8, -11):
    cls = []
    for co in cells_out:
        if co["key"][2] != d: continue
        a2, a1 = co["arms"]["A2"], co["arms"]["A1"]
        for w in W:
            m2, s2, _ = a2["rel"][w]; m1, s1, _ = a1["rel"][w]; se = math.hypot(s2, s1)
            cls.append("win" if m1 - m2 > 2 * se else ("loss" if m2 - m1 > 2 * se else "tie"))
        if not (a2["rule_met"] and a1["rule_met"]): cls.append("rule_not_met")
    verd[str(d)] = "gain" if all(x == "win" for x in cls) else "not a gain in every cell"
out["verdicts"] = verd
if verd != derived["verdicts"]: errs.append(("verdicts differ from committed derived", verd, derived["verdicts"]))
# compare with committed derived per arm-cell (relation-phase and target means at each w)
dc = 0; dmis = []
for dcell in derived["cells"]:
    k = (dcell.get("bits"), dcell.get("m"), dcell.get("d"))
    co = next((x for x in cells_out if x["key"] == k), None)
    if co is None or dcell["arm"] not in co["arms"]: dmis.append(("no match", k, dcell.get("arm"))); continue
    a = co["arms"][dcell["arm"]]
    for w in W:
        for nm in ("rel", "tgt"):
            got = a[nm][w][0]; want = dcell[nm][str(w)][0]
            dc += 1
            if abs(got - want) > 1e-9 * max(1, abs(want)): dmis.append((k, dcell["arm"], nm, w, got, want))
out["derived_values_compared"] = dc; out["derived_mismatches"] = dmis[:20]
if dmis: errs.append(("derived mismatches", len(dmis)))
out["ic_targets_independently_certified"] = ncert_ic
out["rho_runs_independently_certified"] = ncert_rho
out["errors"] = errs
json.dump(out, open(sys.argv[5], "w"), indent=1, default=str)
print(json.dumps({k: v for k, v in out.items() if k != "errors"}, default=str, indent=1)[:4000])
print("errors", len(errs), errs[:10])
