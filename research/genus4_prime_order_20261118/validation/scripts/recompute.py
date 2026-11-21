#!/usr/bin/env python3
"""C6/C7: the validator's recompute of every reading of PROTOCOL.md sections 3 and 6 from the raw rows
(git blobs at COMMIT), written from the protocol text, not from analyze.py / tune.py (read, never run).

Readings: IC cost(w) = group_ops + w (rel_mul + rel_add + la_mul + la_add); R_neg = sqrt(pi r / 4),
R_plain = sqrt(pi r / 2) per curve; ratio = mean IC cost / mean R over the 60 keys of a (prime, arm).
Bootstrap: hierarchical (curves, then targets within curve, then seeds within target), 2000 resamples,
seed 20261124, one fresh generator per statistic; the D3 paired ratio uses 20261124 + 1 (the frozen
analysis's choice).  Percentile ends: the 51st and 1950th order statistics of 2000 (the frozen
script's indices) and, as a cross-check, linearly interpolated 2.5 / 97.5 percentiles.
Every field is indexed with [] (lesson L-99ee952350b2).  Writes out/recompute.json."""
import math, random, statistics, sys, os
sys.path.insert(0, os.path.dirname(__file__))
from common import *

NB, SEED = 2000, 20261124
WS = (("0", 0.0), ("0.1", 0.1), ("1", 1.0))
PR = (16411, 32771)
CHOSEN = {16411: [373, 455, 548, 752, 875], 32771: [355, 882, 1461, 1623, 2374]}
NATIVE = ("rel_mul", "rel_add", "la_mul", "la_add")
fails = []
pins = {}


def load(name):
    rows, h = rows_of(f"{D}/{name}")
    pins[name] = h
    return rows


def native(r):
    return sum(r[k] for k in NATIVE)


def cost(r, w):
    return r["group_ops"] + w * native(r)


def mean(v):
    v = list(v)
    return math.fsum(v) / len(v)


def resample(rng, keys, order="levelwise"):
    """Hierarchical draw: curves with replacement, then targets of each drawn curve, then seeds of each target.
    order "levelwise": all curve draws of a resample first, then all target draws of each drawn curve, then the
    seed draws of each drawn target (the order in which the frozen analyze.py consumes its generator; the
    protocol does not fix one).  order "interleaved": each drawn curve's targets and each target's seeds are
    drawn as soon as it is drawn (the validator's first implementation)."""
    curves = sorted({k[0] for k in keys})
    out = []
    if order == "levelwise":
        for c in [curves[rng.randrange(len(curves))] for _ in curves]:
            tgts = sorted({k[1] for k in keys if k[0] == c})
            for t in [tgts[rng.randrange(len(tgts))] for _ in tgts]:
                sds = sorted(k for k in keys if k[0] == c and k[1] == t)
                out += [sds[rng.randrange(len(sds))] for _ in sds]
        return out
    for _ in curves:
        c = curves[rng.randrange(len(curves))]
        tgts = sorted({k[1] for k in keys if k[0] == c})
        for _ in tgts:
            t = tgts[rng.randrange(len(tgts))]
            sds = sorted(k for k in keys if k[0] == c and k[1] == t)
            for _ in sds:
                out.append(sds[rng.randrange(len(sds))])
    return out


def interp(sorted_v, q):
    x = q * (len(sorted_v) - 1)
    i = int(math.floor(x))
    return sorted_v[i] + (x - i) * (sorted_v[min(i + 1, len(sorted_v) - 1)] - sorted_v[i])


def boot(stat, keys, seed, order="levelwise"):
    rng = random.Random(seed)
    reps = [stat(resample(rng, keys, order)) for _ in range(NB)]
    s = sorted(reps)
    b = {"ratio": stat(keys), "lo": s[int(0.025 * NB)], "hi": s[int(0.975 * NB) - 1],
         "lo_interp": interp(s, 0.025), "hi_interp": interp(s, 0.975)}
    if order == "levelwise":
        _, alt = boot(stat, keys, seed, "interleaved")
        sa = sorted(alt)
        b["lo_interleaved"], b["hi_interleaved"] = sa[int(0.025 * NB)], sa[int(0.975 * NB) - 1]
    return b, reps


def instances(rows, p):
    return {r["curve_seed"]: r for r in rows if r["kind"] == "instance" and r["p"] == p}


def runs(rows, p):
    d = {}
    for r in rows:
        if r["kind"] != "run":
            continue
        k = (r["curve_seed"], r["target_seed"], r["seed"])
        if k in d:
            a = {x: y for x, y in d[k].items() if x != "wall_ns"}
            b = {x: y for x, y in r.items() if x != "wall_ns"}
            if a != b:
                fails.append(f"duplicate key differs {p} {k}")
        d[k] = r
    return d


out = {"commit": COMMIT, "design": __doc__, "checks": {}, "partA": {}, "primes": {}, "slope": {}}

# ---------------- part A ----------------
SGES = ((4, 1.5), (8, 2), (16, 3), (32, 4))
grid = {}
for cc in ("1.4", "2.0", "2.8"):
    rows = load(f"a_cc{cc}.jsonl.gz")
    cfg = [r for r in rows if r["kind"] == "config"][0]
    cfg_ok = (cfg["cc"] == float(cc) and cfg["primes"] == [16411] and cfg["max_h"] == 2 and cfg["seeds"] == 2
              and cfg["targets"] == 3 and cfg["arms"] == "x"
              and [(s["w_max"], s["fill_max"]) for s in cfg["sge"]] == [(4, 1.5), (8, 2.0), (16, 3.0), (32, 4.0)])
    inst = instances(rows, 16411)
    if not cfg_ok or sorted(inst) != [172, 180]:
        fails.append(f"partA config {cc}")
    allruns = [r for r in rows if r["kind"] == "run"]
    for wm, fm in SGES:
        sel = [r for r in allruns if r["w_max"] == wm and r["fill_max"] == fm]
        keys = sorted((r["curve_seed"], r["target_seed"], r["seed"]) for r in sel)
        want = sorted((c, c + k * 1000003, s) for c in (172, 180) for k in range(3) for s in (1, 2))
        label_ok = all(r["arm"] == f"C_double_lp+pre+coord+unitT+ferrari+sgecoord+sge({wm},{fm:g})" for r in sel)
        elig = keys == want and all(r["certified"] and r["ok"] and not r["exhausted"] for r in sel)
        grid[f"{cc}|{wm},{fm:g}"] = {"cc": cc, "w_max": wm, "fill_max": fm, "runs": len(sel), "eligible": elig,
                                     "label_ok": label_ok, "keys_ok": keys == want,
                                     "mean_cost_w1": mean(cost(r, 1.0) for r in sel),
                                     "h": sorted({r["h"] for r in sel})}
base = grid["2.0|4,1.5"]
dist = lambda v: abs(float(v["cc"]) - 2.0) + abs(v["w_max"] - 4) / 4 + abs(v["fill_max"] - 1.5)
elig = [v for v in grid.values() if v["eligible"]]
choice = min(elig, key=lambda v: (v["mean_cost_w1"], dist(v)))
srt = sorted(elig, key=lambda v: v["mean_cost_w1"])
out["partA"] = {"grid": grid, "choice": {"cc": choice["cc"], "w_max": choice["w_max"], "fill_max": choice["fill_max"],
                "mean_cost_w1": choice["mean_cost_w1"]}, "choice_over_baseline": choice["mean_cost_w1"] / base["mean_cost_w1"],
                "runner_up": {"cell": f"{srt[1]['cc']}|{srt[1]['w_max']},{srt[1]['fill_max']:g}",
                              "over_choice": srt[1]["mean_cost_w1"] / srt[0]["mean_cost_w1"]},
                "best_cc2.0_over_choice": min(v["mean_cost_w1"] for v in elig if v["cc"] == "2.0") / choice["mean_cost_w1"],
                "choice_at_grid_edge_cc": choice["cc"] in ("1.4", "2.8")}

# ---------------- part B ----------------
X, inst_all = {}, {}
for p in PR:
    for arm, fn, cc in (("x", f"b_p{p}_x.jsonl.gz", 2.0), ("x*", f"b_p{p}_xstar.jsonl.gz", 1.4)):
        rows = load(fn)
        cfg = [r for r in rows if r["kind"] == "config"][0]
        cfg_ok = (cfg["cc"] == cc and cfg["primes"] == [p] and cfg["max_h"] == 1 and cfg["seeds"] == 2 and cfg["targets"] == 6
                  and cfg["curves"] == 5 and cfg["arms"] == "x" and [(s["w_max"], s["fill_max"]) for s in cfg["sge"]] == [(4, 1.5)])
        inst = instances(rows, p)
        rr = runs(rows, p)
        want = sorted((c, c + k * 1000003, s) for c in CHOSEN[p] for k in range(6) for s in (1, 2))
        keys = sorted(rr)
        labels = {r["arm"] for r in rr.values()}
        out["checks"][f"{p} {arm}"] = {
            "config_ok": cfg_ok, "keys_exact_60": keys == want, "instances": sorted(inst),
            "instances_h1_r_eq_order": all(inst[c]["h"] == 1 and inst[c]["r"] == inst[c]["order"] for c in CHOSEN[p]),
            "run_r_h_match_instance": all(r["r"] == inst[r["curve_seed"]]["r"] and r["h"] == 1 for r in rr.values()),
            "G1_certified_ok_not_exhausted": all(r["certified"] and r["ok"] and not r["exhausted"] for r in rr.values()),
            "arm_labels": sorted(labels), "fb_cols_set": sorted({r["fb_cols"] for r in rr.values()}),
            "sge_on_rows": sorted({(r["w_max"], r["fill_max"]) for r in rr.values()})}
        c = out["checks"][f"{p} {arm}"]
        if not (cfg_ok and c["keys_exact_60"] and c["instances_h1_r_eq_order"] and c["run_r_h_match_instance"]
                and c["G1_certified_ok_not_exhausted"] and labels == {"C_double_lp+pre+coord+unitT+ferrari+sgecoord+sge(4,1.5)"}):
            fails.append(f"part B checks {p} {arm}")
        X[(p, arm)] = rr
        for cs, r in inst.items():
            if cs in inst_all.get(p, {}) and inst_all[p][cs] != r:
                fails.append(f"instance rows differ {p} {cs}")
            inst_all.setdefault(p, {})[cs] = r

# G3: measured plain rho Q
qrows = load("b_p16411_q.jsonl.gz")
Q = [r for r in qrows if r["kind"] == "run"]
qinst = instances(qrows, 16411)
q_ratio = mean(r["group_ops"] for r in Q) / mean(math.sqrt(math.pi * inst_all[16411][r["curve_seed"]]["r"] / 2) for r in Q)
q_per_run = [r["group_ops"] / math.sqrt(math.pi * r["r"] / 2) for r in Q]
out["checks"]["Q"] = {"runs": len(Q), "keys": sorted((r["curve_seed"], r["target_seed"], r["seed"]) for r in Q),
                      "all_certified_ok": all(r["certified"] and r["ok"] for r in Q), "arm": sorted({r["arm"] for r in Q}),
                      "compares_uncharged": [r["compares_uncharged"] for r in Q],
                      "mean_over_sqrt_pi_r_2": q_ratio, "per_run": q_per_run, "G3_in_0.6_1.6": 0.6 <= q_ratio <= 1.6,
                      "instances_match_part_B": all(qinst[c] == inst_all[16411][c] for c in qinst)}
gates = {"G1": all(out["checks"][f"{p} {a}"]["G1_certified_ok_not_exhausted"] for p in PR for a in ("x", "x*")),
         "G2": all(out["checks"][f"{p} {a}"]["instances_h1_r_eq_order"] for p in PR for a in ("x", "x*")),
         "G3": out["checks"]["Q"]["G3_in_0.6_1.6"] and out["checks"]["Q"]["all_certified_ok"]}
out["gates"] = gates

reps = {}
for p in PR:
    keys = sorted(X[(p, "x")])
    assert keys == sorted(X[(p, "x*")])
    r_of = {c: inst_all[p][c]["r"] for c in CHOSEN[p]}
    ref = lambda k, neg: math.sqrt(math.pi * r_of[k[0]] / (4 if neg else 2))
    cell = {"keys": len(keys), "curves": CHOSEN[p], "arms": {}}
    for arm in ("x", "x*"):
        d = X[(p, arm)]
        ac = {}
        for nm, neg in (("neg", True), ("plain", False)):
            for wn, w in WS:
                st = lambda kk, d=d, w=w, neg=neg: mean(cost(d[k], w) for k in kk) / mean(ref(k, neg) for k in kk)
                b, rp = boot(st, keys, SEED)
                ac[f"{nm}_w{wn}"] = b
                reps[(p, arm, nm, wn)] = rp
        ac["below_neg_all_w"] = all(ac[f"neg_w{wn}"]["hi"] < 1 for wn, _ in WS)
        ac["mean_cost_w1"] = mean(cost(d[k], 1.0) for k in keys)
        ac["mean_cost_w0"] = mean(cost(d[k], 0.0) for k in keys)
        ac["mean_cost_w0.1"] = mean(cost(d[k], 0.1) for k in keys)
        ac["mean_native"] = mean(native(d[k]) for k in keys)
        ac["la_share_w1"] = mean(d[k]["la_mul"] + d[k]["la_add"] for k in keys) / ac["mean_cost_w1"]
        ac["la_share_of_native"] = mean(d[k]["la_mul"] + d[k]["la_add"] for k in keys) / ac["mean_native"]
        ac["fb_cols"] = mean(d[k]["fb_cols"] for k in keys)
        ac["per_curve_neg_w1"] = {str(c): mean(cost(d[k], 1.0) for k in keys if k[0] == c) / mean(ref(k, True) for k in keys if k[0] == c)
                                  for c in CHOSEN[p]}
        ac["mean_trials"] = mean(d[k]["trials"] for k in keys)
        ac["mean_walk_ops"] = mean(d[k]["walk_ops"] for k in keys)
        ac["mean_cantor_mul_per_group_op"] = mean(d[k]["cantor_mul"] for k in keys) / mean(d[k]["group_ops"] for k in keys)
        # break-even w: price per native op at which mean IC cost equals mean R_neg
        Rn = mean(ref(k, True) for k in keys)
        ac["mean_R_neg"] = Rn
        ac["mean_R_plain"] = mean(ref(k, False) for k in keys)
        ac["break_even_w_vs_neg"] = (Rn - mean(d[k]["group_ops"] for k in keys)) / ac["mean_native"]
        G0 = mean(d[k]["group_ops"] for k in keys)
        REL = mean(d[k]["rel_mul"] + d[k]["rel_add"] for k in keys)
        LA = mean(d[k]["la_mul"] + d[k]["la_add"] for k in keys)
        ac["mean_rel_native"], ac["mean_la_native"] = REL, LA
        # price per mod-r linear-algebra operation at which the arm reaches R_neg, F_p relation work at 1 or 0
        ac["break_even_w_la_with_rel_w1"] = (Rn - G0 - REL) / LA
        ac["break_even_w_la_with_rel_w0"] = (Rn - G0) / LA
        ac["mean_cantor_inv_per_group_op"] = mean(d[k]["cantor_inv"] for k in keys) / G0
        # IC-side per-step work the walk does not count (one mix64 hash + two addm mod r per walk step)
        ac["neg_w1_with_walk_hash_coef"] = (ac["mean_cost_w1"] + 3 * ac["mean_walk_ops"]) / Rn
        ac["neg_w0.1_with_walk_hash_coef"] = (ac["mean_cost_w0.1"] + 0.3 * ac["mean_walk_ops"]) / Rn
        cell["arms"][arm] = ac
    st = lambda kk: mean(cost(X[(p, "x*")][k], 1.0) for k in kk) / mean(cost(X[(p, "x")][k], 1.0) for k in kk)
    b, _ = boot(st, keys, SEED + 1)
    cell["xstar_over_x_w1"] = b
    b0, _ = boot(st, keys, SEED)
    cell["xstar_over_x_w1_seed20261124"] = b0
    pk = [cost(X[(p, "x*")][k], 1.0) / cost(X[(p, "x")][k], 1.0) for k in keys]
    cell["xstar_over_x_w1_paired_per_key"] = {"mean": mean(pk), "median": statistics.median(pk), "min": min(pk), "max": max(pk),
                                              "xstar_cheaper_keys": sum(v < 1 for v in pk)}
    cell["mean_r"] = mean(r_of[c] for c in CHOSEN[p])
    cell["log2_r"] = {str(c): math.log2(r_of[c]) for c in CHOSEN[p]}
    out["primes"][str(p)] = cell

lr = math.log(PR[1] / PR[0])
for arm in ("x", "x*"):
    a0 = out["primes"][str(PR[0])]["arms"][arm]
    a1 = out["primes"][str(PR[1])]["arms"][arm]
    est = math.log(a1["neg_w1"]["ratio"] / a0["neg_w1"]["ratio"]) / lr
    s = sorted(math.log(b / a) / lr for a, b in zip(reps[(PR[0], arm, "neg", "1")], reps[(PR[1], arm, "neg", "1")]))
    # independent streams per prime (the protocol's wording): a second generator for p = 32771
    keys1 = sorted(X[(PR[1], arm)])
    r1 = {c: inst_all[PR[1]][c]["r"] for c in CHOSEN[PR[1]]}
    st1 = lambda kk, d=X[(PR[1], arm)]: mean(cost(d[k], 1.0) for k in kk) / mean(math.sqrt(math.pi * r1[k[0]] / 4) for k in kk)
    _, rp_ind = boot(st1, keys1, SEED + 1000)
    si = sorted(math.log(b / a) / lr for a, b in zip(reps[(PR[0], arm, "neg", "1")], rp_ind))
    out["slope"][arm] = {"ratio_vs_neg_w1": {"slope": est, "lo": s[int(0.025 * NB)], "hi": s[int(0.975 * NB) - 1]},
                         "ratio_vs_neg_w1_independent_streams": {"lo": si[int(0.025 * NB)], "hi": si[int(0.975 * NB) - 1]},
                         "ic_cost_w1_in_p": math.log(a1["mean_cost_w1"] / a0["mean_cost_w1"]) / lr,
                         "ic_cost_w0_in_p": math.log(a1["mean_cost_w0"] / a0["mean_cost_w0"]) / lr,
                         "R_neg_in_p": math.log(a1["mean_R_neg"] / a0["mean_R_neg"]) / lr}
out["same_seed_both_primes_gives_identical_index_draws"] = True
out["raw_sha256"] = pins
out["fail"] = fails
dump("recompute.json", out)
print("gates", gates, "fails", fails)
for p in PR:
    for arm in ("x", "x*"):
        a = out["primes"][str(p)]["arms"][arm]
        print(p, arm, {k: (round(a[k]["ratio"], 6), round(a[k]["lo"], 6), round(a[k]["hi"], 6)) for k in ("neg_w0", "neg_w0.1", "neg_w1", "plain_w1")},
              "below", a["below_neg_all_w"], "breakeven", round(a["break_even_w_vs_neg"], 4))
    print(p, "x*/x", out["primes"][str(p)]["xstar_over_x_w1"])
print(json.dumps(out["slope"], indent=1))
print(json.dumps(out["partA"]["choice"]), out["partA"]["choice_over_baseline"], out["partA"]["runner_up"])
print(out["checks"]["Q"]["mean_over_sqrt_pi_r_2"], out["checks"]["Q"]["per_run"])
