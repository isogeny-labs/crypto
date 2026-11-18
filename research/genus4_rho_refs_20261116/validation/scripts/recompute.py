#!/usr/bin/env python3
"""Validator's independent recomputation of every gate, ratio and interval in
research/genus4_positive_control_20261116/RESULTS.md (study 1) and
research/genus4_rho_refs_20261116/RESULTS.md (study 2), from the raw JSONL.
Written from the PROTOCOL definitions; every record field is indexed with []
(lesson L-99ee952350b2).  Bootstrap uses the validator's own RNG seed, so
interval ends agree with the producer's only to Monte-Carlo precision."""
import json, random, sys, statistics as st

ROOT = sys.argv[1]
OUT = sys.argv[2]
S1 = f"{ROOT}/research/genus4_positive_control_20261116/run.jsonl"
S2 = f"{ROOT}/research/genus4_rho_refs_20261116/run.jsonl"
BR = f"{ROOT}/research/genus_ladder_bound_r_20261114/bound_r.jsonl"
B = 4000
SEED = 777
WS = [0.0, 0.1, 1.0]


def load(fn):
    return [json.loads(l) for l in open(fn)]


def mean(v):
    return sum(v) / len(v)


def pct(bs, q):
    bs = sorted(bs)
    return bs[int(q * len(bs))] if q < 1 else bs[-1]


res = {}
s1, s2, br = load(S1), load(S2), load(BR)

# ------------------------------------------------------------------ study 1
A1 = {"c": "C_double_lp+pre", "v": "C_double_lp+pre+coord", "w": "C_double_lp+pre+coord+unitT", "R": "R_rho_neg"}
runs1 = [r for r in s1 if r["kind"] == "run"]
r1 = {}
for r in runs1:
    assert r["g"] == 4 and r["family"] == "hyperelliptic"
    k = (r["arm"], r["p"], r["curve_seed"], r["seed"])
    assert k not in r1, k
    r1[k] = r
res["s1_counts"] = {a: sum(1 for r in runs1 if r["arm"] == n) for a, n in A1.items()}
res["s1_runs"] = len(runs1)
res["s1_other_kinds"] = sorted({r["kind"] for r in s1 if r["kind"] != "run"})
res["s1_skips"] = sum(1 for r in s1 if r["kind"] == "skip")
P1 = sorted({r["p"] for r in runs1})
CS1 = {p: sorted({r["curve_seed"] for r in runs1 if r["p"] == p}) for p in P1}
SEEDS1 = sorted({r["seed"] for r in runs1})

# G2: every run certified and ok
res["s1_G2_fail"] = [k for k, r in r1.items() if not (r["certified"] is True and r["ok"] is True)]
res["s1_exhausted"] = [k for k, r in r1.items() if "exhausted" in r and r["exhausted"]]

# G1: instances and seed 1-2 c / R rows equal the r-bound rows, on every shared field but wall time
inst1 = {(r["p"], r["curve_seed"]): r for r in s1 if r["kind"] == "instance"}
instb = {(r["p"], r["curve_seed"]): r for r in br if r["kind"] == "instance" and r["family"] == "hyperelliptic" and r["g"] == 4}
g1 = {"instances_compared": 0, "instance_fail": [], "rows_compared": {}, "row_fail": [], "fields": {}}
for k, o in instb.items():
    n = inst1[k]
    g1["instances_compared"] += 1
    for f in ("order", "h", "r", "f"):
        if n[f] != o[f]:
            g1["instance_fail"].append((k, f))
brun = {(r["arm"], r["p"], r["curve_seed"], r["seed"]): r for r in br
        if r["kind"] == "run" and r["family"] == "hyperelliptic" and r["g"] == 4}
for a in ("c", "R"):
    cnt = 0
    for (arm, p, cs, sd), o in brun.items():
        if arm != A1[a]:
            continue
        n = r1[(arm, p, cs, sd)]
        shared = sorted((set(n) & set(o)) - {"wall_ns"})
        g1["fields"][a] = shared
        cnt += 1
        for f in shared:
            if n[f] != o[f]:
                g1["row_fail"].append(((arm, p, cs, sd), f, n[f], o[f]))
    g1["rows_compared"][a] = cnt
res["s1_G1"] = g1


def ic_tot(r, w):
    return r["group_ops"] + w * (r["rel_mul"] + r["la_mul"])


def rho_tot(r, w):
    return r["group_ops"] + w * r["native"]


def fpm(r):
    if "rel_mul" in r:
        return r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + 16 * r["la_mul"]
    return r["cantor_mul"] + 10 * r["cantor_inv"]


def boot1(p, num_arm, den_arm, fn_num, fn_den, paired, seed=SEED, curves=None):
    """ratio of means over (curve, seed) at prime p; two-stage bootstrap
    (curves with replacement, then seeds within each curve)."""
    rnd = random.Random(seed)
    cs_all = curves if curves is not None else CS1[p]
    def stat(pairs_num, pairs_den):
        return mean([fn_num(r1[(A1[num_arm], p, c, s)]) for c, s in pairs_num]) / \
               mean([fn_den(r1[(A1[den_arm], p, c, s)]) for c, s in pairs_den])
    allp = [(c, s) for c in cs_all for s in SEEDS1]
    est = stat(allp, allp)
    bs = []
    for _ in range(B):
        cc = [rnd.choice(cs_all) for _ in cs_all]
        pn, pd = [], []
        for c in cc:
            sn = [rnd.choice(SEEDS1) for _ in SEEDS1]
            sd = sn if paired else [rnd.choice(SEEDS1) for _ in SEEDS1]
            pn += [(c, s) for s in sn]
            pd += [(c, s) for s in sd]
        bs.append(stat(pn, pd))
    return [est, pct(bs, 0.025), pct(bs, 0.975)]


s1t = {}
for p in P1:
    e = {}
    e["runs_per_arm"] = {a: sum(1 for k in r1 if k[0] == A1[a] and k[1] == p) for a in A1}
    e["fallbacks"] = {a: sum(r1[(A1[a], p, c, s)]["la_fallbacks"] for c in CS1[p] for s in SEEDS1) for a in "vw"}
    e["LA_v_over_c"] = boot1(p, "v", "c", lambda r: r["la_mul"], lambda r: r["la_mul"], False)
    e["LA_w_over_c"] = boot1(p, "w", "c", lambda r: r["la_mul"], lambda r: r["la_mul"], False)
    e["LA_w_over_c_paired"] = boot1(p, "w", "c", lambda r: r["la_mul"], lambda r: r["la_mul"], True)
    e["rel_w_over_c"] = boot1(p, "w", "c", lambda r: r["rel_mul"], lambda r: r["rel_mul"], False)
    ws = [r1[(A1["w"], p, c, s)] for c in CS1[p] for s in SEEDS1]
    e["w_rel_share_mean_of_runs"] = mean([r["rel_mul"] / (r["rel_mul"] + r["la_mul"]) for r in ws])
    e["w_rel_share_ratio_of_sums"] = sum(r["rel_mul"] for r in ws) / sum(r["rel_mul"] + r["la_mul"] for r in ws)
    e["w_mean_adds"] = mean([r["rel_add"] + r["la_add"] for r in ws])
    e["w_mean_muls"] = mean([r["rel_mul"] + r["la_mul"] for r in ws])
    e["w_adds_over_muls"] = e["w_mean_adds"] / e["w_mean_muls"]
    vs = [r1[(A1["v"], p, c, s)] for c in CS1[p] for s in SEEDS1]
    e["LA_w_over_v_mul"] = sum(r["la_mul"] for r in ws) / sum(r["la_mul"] for r in vs)
    e["LA_w_over_v_mul_plus_add"] = sum(r["la_mul"] + r["la_add"] for r in ws) / sum(r["la_mul"] + r["la_add"] for r in vs)
    # restart cap of the unit-T walk: max(64, floor(sqrt(r)/16)) vs RESTART 4096
    import math
    e["unitT_cap_min_over_curves"] = min(max(64, math.isqrt(inst1[(p, c)]["r"]) // 16) for c in CS1[p])
    for arm in "cvw":
        for w in WS:
            e[f"{arm}/R w={w}"] = boot1(p, arm, "R", lambda r, w=w: ic_tot(r, w), lambda r, w=w: rho_tot(r, w), False)
            e[f"{arm}/R w={w} paired"] = boot1(p, arm, "R", lambda r, w=w: ic_tot(r, w), lambda r, w=w: rho_tot(r, w), True)
        e[f"{arm}/R FPM"] = boot1(p, arm, "R", fpm, fpm, False)
    for arm in "vw":
        for w in (0.1, 1.0):
            e[f"{arm}/c w={w}"] = boot1(p, arm, "c", lambda r, w=w: ic_tot(r, w), lambda r, w=w: ic_tot(r, w), False)
    # red-team F2 numbers: additions priced on the IC, rho as emitted; and rho + hash + 2 coef adds per step
    e["w/R w=1 mul+add (F2)"] = boot1(p, "w", "R", lambda r: r["group_ops"] + r["rel_mul"] + r["la_mul"] + r["rel_add"] + r["la_add"],
                                      lambda r: rho_tot(r, 1.0), False)
    e["w/R w=1 mul+add, rho +3/step (F2 est.)"] = boot1(
        p, "w", "R", lambda r: r["group_ops"] + r["rel_mul"] + r["la_mul"] + r["rel_add"] + r["la_add"],
        lambda r: r["group_ops"] + r["native"] + 3 * r["steps"], False)
    # leave one curve out, w/R at w = 1
    e["LOO w/R w=1"] = {c: boot1(p, "w", "R", lambda r: ic_tot(r, 1.0), lambda r: rho_tot(r, 1.0), False,
                                 curves=[x for x in CS1[p] if x != c]) for c in CS1[p]}
    e["w_below_R_all_prices"] = all(e[f"w/R w={w}"][2] < 1 for w in WS)
    s1t[p] = e
res["s1"] = s1t

# ------------------------------------------------------------------ study 2
runs2 = [r for r in s2 if r["kind"] == "run"]
cal = [r for r in s2 if r["kind"] == "calibration"]
res["s2_runs"] = len(runs2)
res["s2_kinds"] = sorted({r["kind"] for r in s2})
A2 = {"w": "C_double_lp+pre+coord+unitT", "R": "R_rho_neg", "P": "R_rho_plain"}
r2 = {}
for r in runs2:
    assert r["g"] == 4 and r["p"] == 8209
    k = (r["arm"], r["curve_seed"], r["target_seed"], r["seed"])
    assert k not in r2
    r2[k] = r
res["s2_counts"] = {a: sum(1 for r in runs2 if r["arm"] == n) for a, n in A2.items()}
TG = sorted({(r["curve_seed"], r["target_seed"]) for r in runs2})
SEEDS2 = sorted({r["seed"] for r in runs2})
res["s2_targets"] = TG
res["s2_target_seed_rule"] = all(t in (c, c + 1000003, c + 2000006) for c, t in TG)
res["s2_G2_fail"] = [k for k, r in r2.items() if not (r["certified"] is True and r["ok"] is True)]
# G1 against study 1 (first target = curve seed; seeds 1-2), every shared field but wall time
g1b = {"rows_compared": {}, "row_fail": [], "fields": {}}
for a, n1 in (("w", A1["w"]), ("R", A1["R"])):
    cnt = 0
    for (arm, p, cs, sd), o in r1.items():
        if arm != n1 or p != 8209 or sd > 2:
            continue
        n = r2[(arm, cs, cs, sd)]
        shared = sorted((set(n) & set(o)) - {"wall_ns"})
        g1b["fields"][a] = shared
        cnt += 1
        for f in shared:
            if n[f] != o[f]:
                g1b["row_fail"].append(((arm, cs, sd), f, n[f], o[f]))
    g1b["rows_compared"][a] = cnt
res["s2_G1"] = g1b

# calibration
calr = []
for c in cal:
    calr.append({"curve_seed": c["curve_seed"],
                 "mul_p": c["ns_mulmod_p"] / c["ns_cantor_add"], "add_p": c["ns_addmod_p"] / c["ns_cantor_add"],
                 "mul_r": c["ns_mulmod_r"] / c["ns_cantor_add"],
                 "recorded": [c["w_cal_mul_p"], c["w_cal_add_p"], c["w_cal_mul_r"]]})
wm_p = mean([c["mul_p"] for c in calr]); wa_p = mean([c["add_p"] for c in calr]); wm_r = mean([c["mul_r"] for c in calr])
res["s2_cal"] = {"records": calr, "mean": [wm_p, wa_p, wm_r]}


def ic_cost(r, q, w):
    G, M, A = r["group_ops"], r["rel_mul"] + r["la_mul"], r["rel_add"] + r["la_add"]
    if q == "Q1":
        return G + w * M
    if q == "Qcal":
        return G + wm_p * r["rel_mul"] + wm_r * r["la_mul"] + wa_p * A
    if q == "Q3+ICcmp_lo":   # validator sensitivity: IC LA coefficient compares, lower estimate
        return G + w * (M + A + (r["la_add"] - r["la_mul"]))
    if q == "Q3+ICcmp_hi":   # upper estimate: two compares per counted LA addition
        return G + w * (M + A + 2 * r["la_add"])
    if q == "Q4+ICwalk":     # validator sensitivity: IC walk hash + 2 coef adds per trial, as Q4 charges rho
        return G + w * (M + A + 3 * r["trials"])
    return G + w * (M + A)


def rho_cost(r, q, w, plain_floor=True):
    G, C = r["group_ops"], r["native"]
    if not plain_floor and r["arm"] == "R_rho_plain":
        C = 0  # the plain walk performs no canonicalisation; the floor charges steps
    K, H, D = r["compares_uncharged"], r["hash_uncharged"], r["coef_adds_uncharged"]
    if q in ("Q1", "Q2"):
        return G + w * C
    if q in ("Q3", "Q3+ICcmp_lo", "Q3+ICcmp_hi"):
        return G + w * (C + K)
    if q in ("Q4", "Q4+ICwalk"):
        return G + w * (C + K + H + D)
    if q == "Qcal":
        return G
    raise ValueError(q)


def boot2(q, w, keys, plain_floor=True, seed=SEED):
    rnd = random.Random(seed)
    tg = sorted({(c, t) for c, t, s in keys})
    by = {t: [k for k in keys if (k[0], k[1]) == t] for t in tg}
    def stat(ks):
        ic = mean([ic_cost(r2[(A2["w"],) + k], q, w) for k in ks])
        rr = mean([rho_cost(r2[(A2["R"],) + k], q, w, plain_floor) for k in ks])
        pp = mean([rho_cost(r2[(A2["P"],) + k], q, w, plain_floor) for k in ks])
        return ic / min(rr, pp), ("R" if rr <= pp else "P"), pp / rr
    est, which, pr = stat(keys)
    bs, which_bs = [], {"R": 0, "P": 0}
    for _ in range(B):
        ks = []
        for t in [rnd.choice(tg) for _ in tg]:
            ks += [rnd.choice(by[t]) for _ in by[t]]
        sv = stat(ks)
        bs.append(sv[0]); which_bs[sv[1]] += 1
    return {"ratio": est, "lo": pct(bs, 0.025), "hi": pct(bs, 0.975), "cheapest": which, "P_over_R": pr,
            "cheapest_in_resamples": which_bs}


keys2 = sorted({(c, t, s) for (a, c, t, s) in r2})
s2t = {}
for q in ("Q1", "Q2", "Q3", "Q4"):
    s2t[q] = {str(w): boot2(q, w, keys2) for w in WS}
s2t["Qcal"] = {"cal": boot2("Qcal", 0.0, keys2)}
for q in ("Q3+ICcmp_lo", "Q3+ICcmp_hi", "Q4+ICwalk"):
    s2t[q] = {str(w): boot2(q, w, keys2) for w in WS}
for q in ("Q1", "Q2", "Q3", "Q4"):
    s2t[q + " plain unfloored"] = {str(w): boot2(q, w, keys2, plain_floor=False) for w in WS}
curves2 = sorted({k[0] for k in keys2})
s2t["LOO w=1"] = {q: {str(c): boot2(q, 1.0, [k for k in keys2 if k[0] != c]) for c in curves2}
                  for q in ("Q1", "Q2", "Q3", "Q4")}
s2t["verdict"] = {q: all(v["hi"] < 1 for v in s2t[q].values()) for q in ("Q1", "Q2", "Q3", "Q4", "Qcal")}
# per-step quantities
def ps(arm, f, den="steps"):
    rs = [r for r in runs2 if r["arm"] == A2[arm]]
    return sum(r[f] for r in rs) / sum(r[den] for r in rs)
s2t["per_step"] = {a: {"compares": ps(a, "compares_uncharged"), "native": ps(a, "native"),
                       "group_ops": ps(a, "group_ops"), "hash": ps(a, "hash_uncharged"),
                       "coef_adds": ps(a, "coef_adds_uncharged")} for a in ("R", "P")}
s2t["P_over_R_group_ops"] = sum(r2[(A2["P"],) + k]["group_ops"] for k in keys2) / sum(r2[(A2["R"],) + k]["group_ops"] for k in keys2)
# wall time (informational only: shared host, 3 threads, no isolation level)
s2t["wall_ratio_w_over_R"] = mean([r2[(A2["w"],) + k]["wall_ns"] for k in keys2]) / mean([r2[(A2["R"],) + k]["wall_ns"] for k in keys2])
s2t["wall_ratio_P_over_R"] = mean([r2[(A2["P"],) + k]["wall_ns"] for k in keys2]) / mean([r2[(A2["R"],) + k]["wall_ns"] for k in keys2])
ns_cantor = mean([c["ns_cantor_add"] for c in cal])
s2t["rho_wall_ns_per_group_op"] = {a: sum(r["wall_ns"] for r in runs2 if r["arm"] == A2[a]) / sum(r["group_ops"] for r in runs2 if r["arm"] == A2[a]) for a in ("R", "P")}
s2t["ns_cantor_add_mean"] = ns_cantor
res["s2"] = s2t
json.dump(res, open(OUT, "w"), indent=1, default=str)

# ------------------------------------------------------------------ print
f3 = lambda t: f"{t[0]:.3f} [{t[1]:.3f}, {t[2]:.3f}]"
g3 = lambda v: f"{v['ratio']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}] ({v['cheapest']})"
print("S1 runs", res["s1_runs"], res["s1_counts"], "kinds", res["s1_other_kinds"], "skips", res["s1_skips"])
print("S1 G1 instances", g1["instances_compared"], "inst fails", g1["instance_fail"], "rows", g1["rows_compared"], "row fails", g1["row_fail"][:5])
print("S1 G1 fields", g1["fields"])
print("S1 G2 fails", res["s1_G2_fail"], "exhausted", res["s1_exhausted"])
for p, e in s1t.items():
    print(f"\n== p={p} runs {e['runs_per_arm']} fallbacks {e['fallbacks']}")
    print(" LA v/c", f3(e["LA_v_over_c"]), " LA w/c", f3(e["LA_w_over_c"]), "paired", f3(e["LA_w_over_c_paired"]), " rel w/c", f3(e["rel_w_over_c"]))
    print(f" rel share (mean of runs) {e['w_rel_share_mean_of_runs']:.4f}  (ratio of sums) {e['w_rel_share_ratio_of_sums']:.4f}")
    print(f" w adds {e['w_mean_adds']:.4g} muls {e['w_mean_muls']:.4g} adds/muls {e['w_adds_over_muls']:.3f}; LA w/v mul {e['LA_w_over_v_mul']:.3f} mul+add {e['LA_w_over_v_mul_plus_add']:.3f}; unitT cap min {e['unitT_cap_min_over_curves']}")
    for arm in "cvw":
        print(f" {arm}/R", " | ".join(f3(e[f'{arm}/R w={w}']) for w in WS), "| FPM", f3(e[f"{arm}/R FPM"]),
              "| paired w=1", f3(e[f"{arm}/R w=1.0 paired"]))
    for arm in "vw":
        print(f" {arm}/c", " | ".join(f3(e[f'{arm}/c w={w}']) for w in (0.1, 1.0)))
    print(" F2 w/R mul+add", f3(e["w/R w=1 mul+add (F2)"]), " +3/step", f3(e["w/R w=1 mul+add, rho +3/step (F2 est.)"]))
    print(" LOO w/R w=1", {c: f3(v) for c, v in e["LOO w/R w=1"].items()})
    print(" w below R all prices:", e["w_below_R_all_prices"])
print("\nS2 runs", res["s2_runs"], res["s2_counts"], "kinds", res["s2_kinds"], "targets", len(TG), "seed rule", res["s2_target_seed_rule"])
print("S2 G1 rows", g1b["rows_compared"], "fails", g1b["row_fail"][:5], "fields", g1b["fields"])
print("S2 G2 fails", res["s2_G2_fail"])
print("S2 cal", res["s2_cal"])
for q in ("Q1", "Q2", "Q3", "Q4", "Q3+ICcmp_lo", "Q3+ICcmp_hi", "Q4+ICwalk",
          "Q1 plain unfloored", "Q2 plain unfloored", "Q3 plain unfloored", "Q4 plain unfloored"):
    print(f" {q:22s}", " | ".join(g3(s2t[q][str(w)]) for w in WS), "| P/R w=1", f"{s2t[q]['1.0']['P_over_R']:.3f}",
          "| P/R w=0", f"{s2t[q]['0.0']['P_over_R']:.3f}", "| cheapest-in-resamples w=1", s2t[q]["1.0"]["cheapest_in_resamples"])
print(" Qcal", g3(s2t["Qcal"]["cal"]))
for q, d in s2t["LOO w=1"].items():
    print(" LOO", q, {c: g3(v) for c, v in d.items()})
print(" verdict", s2t["verdict"])
print(" per step", s2t["per_step"], "P/R group ops", s2t["P_over_R_group_ops"])
print(" wall w/R", s2t["wall_ratio_w_over_R"], "wall P/R", s2t["wall_ratio_P_over_R"], "rho ns/group op", s2t["rho_wall_ns_per_group_op"], "ns cantor", ns_cantor)
