#!/usr/bin/env python3
"""Validator's independent recomputation for quartic_diem_precheck2_20261121.

Written from PROTOCOL.md sections 6-8 without reusing analyze.py code.
Reads the raw JSONL files, indexes every record field with [] (lesson
L-99ee952350b2), and prints a JSON summary to stdout.

Usage: python3 -I recompute.py STUDY_DIR PREV_RUN_JSONL > recompute.json
"""
import json
import math
import random
import sys
from collections import defaultdict

STUDY = sys.argv[1]
PREV = sys.argv[2]
RAW = ["raw/run.jsonl", "raw/run65537_odd.jsonl", "raw/run65537_even.jsonl"]
ARMS = ["Q_diem_2lp", "Q_diem_2lp_alg", "Q_diem_2lp_kick", "Q_diem_2lp_new"]
OLD, ALG, KICK, NEW = ARMS
B = 4000
rng = random.Random(777)   # validator's own generator (not the producer's)


def load(path):
    out = []
    with open(path) as fh:
        for line in fh:
            if line.strip():
                out.append(json.loads(line))
    return out


rows = []
inst = {}
cells = []
for rp in RAW:
    for r in load(f"{STUDY}/{rp}"):
        if r["kind"] == "run" and r["family"] == "quartic":
            rows.append(r)
        elif r["kind"] == "instance" and r["family"] == "quartic":
            inst[(r["p"], r["curve_seed"])] = r
        elif r["kind"] == "cell" and r["family"] == "quartic":
            cells.append((rp, r))


def met(r):
    cm, ci, rel, la, laa, go = (r["cantor_mul"], r["cantor_inv"], r["rel_mul"],
                                r["la_mul"], r["la_add"], r["group_ops"])
    return {
        "la_mul": la, "la_mul_add": la + laa,
        "fpm_mid": cm + 10 * ci + rel + 9 * la,
        "fpm_low": cm + ci + rel + la,
        "fpm_high": cm + 30 * ci + rel + 9 * la,
        "gae_0": go, "gae_0.1": go + 0.1 * (rel + la), "gae_1": go + rel + la,
        "rel_mul": rel,
        "decomp_trials": max(r["decomp_trials"], 1), "attempts": max(r["attempts"], 1),
    }


def gmean(v):
    return math.exp(sum(math.log(x) for x in v) / len(v))


def slope(xs, ys):
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx, my, mx


def pct(v, q):
    v = sorted(v)
    k = (len(v) - 1) * q
    f = math.floor(k)
    c = min(f + 1, len(v) - 1)
    return v[f] + (v[c] - v[f]) * (k - f)


TT = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447}


def fit(curves_by_p, key):
    """curves_by_p: {p: [metric dicts per curve]}"""
    ps = sorted(curves_by_p)
    xs = [math.log(p) for p in ps for _ in curves_by_p[p]]
    ys = [math.log(c[key]) for p in ps for c in curves_by_p[p]]
    b, _, _ = slope(xs, ys)
    boots = []
    for _ in range(B):
        bx, by = [], []
        for p in ps:
            v = curves_by_p[p]
            for _ in range(len(v)):
                c = v[rng.randrange(len(v))]
                bx.append(math.log(p))
                by.append(math.log(c[key]))
        boots.append(slope(bx, by)[0])
    px = [math.log(p) for p in ps]
    py = [sum(math.log(c[key]) for c in curves_by_p[p]) / len(curves_by_p[p]) for p in ps]
    bm, my, mx = slope(px, py)
    am = my - bm * mx
    res = [y - (am + bm * x) for x, y in zip(px, py)]
    df = len(ps) - 2
    se = math.sqrt(sum(e * e for e in res) / df / sum((x - mx) ** 2 for x in px))
    t = TT[df]
    local = [round((py[i + 1] - py[i]) / (px[i + 1] - px[i]), 3) for i in range(len(ps) - 1)]
    return {"ols": b, "boot95": [pct(boots, 0.025), pct(boots, 0.975)],
            "means_ols": bm, "means_t95": [bm - t * se, bm + t * se], "local": local}


out = {"n_runs": len(rows)}
by_arm = defaultdict(list)
for r in rows:
    by_arm[r["arm"]].append(r)
out["runs_per_arm"] = {a: len(v) for a, v in by_arm.items()}
out["uncertified"] = [(r["arm"], r["p"], r["curve_seed"], r["seed"]) for r in rows
                      if not (r["certified"] is True and r["ok"] is True)]
out["exhausted"] = sum(1 for r in rows if r["exhausted"])
out["fallbacks_total"] = sum(r["decomp_fallbacks"] for r in rows)
out["la_skipped_total"] = sum(r["la_skipped"] for r in rows)
out["h_values"] = sorted({r["h"] for r in rows})
out["order_eq_r"] = all(i["order"] == i["r"] for i in inst.values())


def curves(arm, maxp=None):
    cur = defaultdict(list)
    for r in by_arm[arm]:
        if maxp and r["p"] > maxp:
            continue
        cur[(r["p"], r["curve_seed"])].append(met(r))
    byp = defaultdict(list)
    for (p, _cs), ms in sorted(cur.items()):
        byp[p].append({k: gmean([m[k] for m in ms]) for k in ms[0]})
    return byp


KEYS = ["la_mul", "la_mul_add", "fpm_mid", "fpm_low", "fpm_high", "gae_0", "gae_0.1",
        "gae_1", "rel_mul"]
out["exponents"] = {}
out["exponents_six"] = {}
for arm in ARMS:
    c = curves(arm)
    out["exponents"][arm] = {k: fit(c, k) for k in KEYS}
    c6 = curves(arm, 32771)
    out["exponents_six"][arm] = {k: fit(c6, k) for k in ["la_mul", "fpm_mid"]}
    out.setdefault("curves_per_prime", {})[arm] = {p: len(v) for p, v in c.items()}

# per-prime table of the primary arm (run-level geometric means, as RESULTS)
pp = {}
for arm in ARMS:
    pp[arm] = {}
    for p in sorted({r["p"] for r in by_arm[arm]}):
        rs = [r for r in by_arm[arm] if r["p"] == p]
        ms = [met(r) for r in rs]
        pp[arm][p] = {
            "la_mul": gmean([m["la_mul"] for m in ms]),
            "fpm_mid": gmean([m["fpm_mid"] for m in ms]),
            "rel_mul": gmean([m["rel_mul"] for m in ms]),
            "rel_enum": gmean([r["rel_enum_mul"] for r in rs]),
            "rel_decomp_mean": sum(r["rel_decomp_mul"] for r in rs) / len(rs),
            "la_share": sum(9 * m["la_mul"] / m["fpm_mid"] for m in ms) / len(ms),
            "rel_enum_share_of_rel_mul_mean": sum(r["rel_enum_mul"] / r["rel_mul"] for r in rs) / len(rs),
            "attempts_mean": sum(r["attempts"] for r in rs) / len(rs),
            "attempts_max": max(r["attempts"] for r in rs),
            "decomp_trials_gm": gmean([max(r["decomp_trials"], 1) for r in rs]),
            "decomp_trials_mean": sum(r["decomp_trials"] for r in rs) / len(rs),
            "runs_attempts_ge2": sum(1 for r in rs if r["attempts"] >= 2),
            "fpm_mid_mean": sum(m["fpm_mid"] for m in ms) / len(ms),
        }
out["per_prime"] = pp

# paired ratios
PK = ["fpm_mid", "la_mul", "rel_mul", "gae_1", "gae_0", "decomp_trials", "attempts"]


def paired(num, den):
    d = {(r["p"], r["curve_seed"], r["seed"]): r for r in by_arm[den]}
    cur = defaultdict(lambda: defaultdict(list))
    for r in by_arm[num]:
        o = d[(r["p"], r["curve_seed"], r["seed"])]
        mn, md = met(r), met(o)
        for k in PK:
            cur[(r["p"], r["curve_seed"])][k].append(mn[k] / md[k])
    byp = defaultdict(list)
    for (p, _cs), kv in sorted(cur.items()):
        byp[p].append({k: gmean(v) for k, v in kv.items()})
    res = {"pooled": {}, "per_prime": {}, "slope": {}}
    for k in PK:
        allv = [c[k] for p in byp for c in byp[p]]
        boots = []
        for _ in range(B):
            s = []
            for p in byp:
                v = byp[p]
                s += [v[rng.randrange(len(v))][k] for _ in range(len(v))]
            boots.append(gmean(s))
        res["pooled"][k] = [gmean(allv), pct(boots, 0.025), pct(boots, 0.975)]
        for p in sorted(byp):
            v = [c[k] for c in byp[p]]
            bb = [gmean([v[rng.randrange(len(v))] for _ in v]) for _ in range(B)]
            res["per_prime"].setdefault(p, {})[k] = [gmean(v), pct(bb, 0.025), pct(bb, 0.975)]
        f = fit(byp, k)
        res["slope"][k] = [f["ols"], *f["boot95"]]
    return res


out["paired"] = {"new/old": paired(NEW, OLD), "alg/old": paired(ALG, OLD),
                 "kick/alg": paired(KICK, ALG), "new/kick": paired(NEW, KICK)}

# C1: ALG vs OLD on every field except rel_* and wall_ns and arm/descent labels
skip = {"rel_mul", "rel_add", "rel_decomp_mul", "rel_decomp_add", "wall_ns", "arm", "descent"}
old = {(r["p"], r["curve_seed"], r["seed"]): r for r in by_arm[OLD]}
c1 = {"compared": 0, "identical": 0, "diff_fields": defaultdict(int)}
for r in by_arm[ALG]:
    o = old[(r["p"], r["curve_seed"], r["seed"])]
    assert set(r) == set(o), (set(r) ^ set(o))
    c1["compared"] += 1
    dd = [k for k in r if k not in skip and r[k] != o[k]]
    for k in dd:
        c1["diff_fields"][k] += 1
    c1["identical"] += not dd
# relation-phase difference ALG vs OLD should be exactly the descent share
c1["rel_mul_minus_decomp_equal"] = sum(
    1 for r in by_arm[ALG]
    if r["rel_mul"] - r["rel_decomp_mul"] == old[(r["p"], r["curve_seed"], r["seed"])]["rel_mul"]
    - old[(r["p"], r["curve_seed"], r["seed"])]["rel_decomp_mul"])
c1["rel_add_minus_decomp_equal"] = sum(
    1 for r in by_arm[ALG]
    if r["rel_add"] - r["rel_decomp_add"]
    == old[(r["p"], r["curve_seed"], r["seed"])]["rel_add"]
    - old[(r["p"], r["curve_seed"], r["seed"])]["rel_decomp_add"])
out["C1"] = c1

# C0: OLD vs pre-check 1 Q_diem_2lp, every field present in the pre-check-1 record
prev = {(r["p"], r["curve_seed"], r["seed"]): r for r in load(PREV)
        if r["kind"] == "run" and r["arm"] == "Q_diem_2lp"}
c0 = {"compared": 0, "identical": 0, "diff_fields": defaultdict(int), "missing_in_new": set()}
for k, pr in prev.items():
    nr = old[k]
    c0["compared"] += 1
    dd = []
    for f in pr:
        if f == "wall_ns":
            continue
        if f not in nr:
            c0["missing_in_new"].add(f)
            continue
        if nr[f] != pr[f]:
            dd.append(f)
    for f in dd:
        c0["diff_fields"][f] += 1
    c0["identical"] += not dd
c0["missing_in_new"] = sorted(c0["missing_in_new"])
out["C0"] = c0

# 65537 admissions: first two admitted per process
out["cells"] = [(rp, c["p"], c["tried"], c["admitted"], c["rejects"]) for rp, c in cells]
out["instances_65537"] = sorted(k[1] for k in inst if k[0] == 65537)

# NEW: runs needing a second attempt
out["new_attempts_ge2"] = sum(1 for r in by_arm[NEW] if r["attempts"] >= 2)
out["new_attempts_max"] = max(r["attempts"] for r in by_arm[NEW])
out["new_inhom_linked_min"] = min(r["inhom_linked"] for r in by_arm[NEW])
json.dump(out, sys.stdout, indent=1, default=lambda o: dict(o) if isinstance(o, defaultdict) else str(o))
print()
