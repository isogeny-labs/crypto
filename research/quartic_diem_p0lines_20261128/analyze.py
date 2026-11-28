#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md (research/quartic_diem_p0lines_20261128).

Usage: python3 analyze.py raw/*.jsonl > analysis.json

Arms (paired by workload: instance and solver seed, same binary):
  Q_diem_2lp_sfb_sge_nofb   baseline (research/quartic_diem_nofb_20261126)
  Q_diem_2lp_sfb_sge_p0     + the lines through P0 and every factor-base point
  Q_diem_2lp_sfb_sge_p0i3   as p0, 3 linked decomposition rows instead of 4
Reference: ideal negation rho sqrt(pi r / 4) group additions, nothing charged.
Primary panel: held-out solver seeds 5, 6. Secondary: all seeds 1-6.
C0: the nofb rows for seeds 1-4 against the parent study's nofb rows.
Every number is [pcc:unverified].
"""
import json
import math
import sys
from collections import defaultdict

import numpy as np

NOFB = "Q_diem_2lp_sfb_sge_nofb"
P0 = "Q_diem_2lp_sfb_sge_p0"
P0I3 = "Q_diem_2lp_sfb_sge_p0i3"
ARMS = [NOFB, P0, P0I3]
WS = [0.0, 0.1, 1.0]
B = 4000
BOOT_SEED = 20261128
PRIMARY_SEEDS = {5, 6}
ALL_SEEDS = {1, 2, 3, 4, 5, 6}
PARENT = ["../quartic_diem_nofb_20261126/raw/run.jsonl",
          "../quartic_diem_nofb_20261126/raw/run131101.jsonl"]
C0_FIELDS = ["certified", "ok", "recovered", "group_ops", "cantor_mul", "cantor_inv",
             "rel_mul", "rel_add", "la_mul", "la_add", "relations", "rels_by_lp",
             "decomp_trials", "attempts", "core_rows", "core_cols", "core_nnz",
             "idx_lookups", "idx_inserts", "idx_points", "fb_samples",
             "la_fallbacks", "sq_dim", "sq_nnz"]
KILL_LO = 0.98
COMPARISONS = {"p0_over_nofb": (P0, NOFB), "p0i3_over_nofb": (P0I3, NOFB),
               "p0i3_over_p0": (P0I3, P0)}


def native_all(r):
    return (r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]
            + r["idx_lookups"] + r["idx_inserts"])


def gae(r, w):
    return r["group_ops"] + w * native_all(r)


def gm(v):
    return math.exp(sum(math.log(x) for x in v) / len(v))


def pct(a):
    return [float(np.percentile(a, 2.5)), float(np.percentile(a, 97.5))]


def load(paths, arms):
    runs = {}
    for path in paths:
        for line in open(path):
            r = json.loads(line)
            if r.get("kind") == "run" and r["arm"] in arms:
                key = (r["arm"], r["p"], r["curve_seed"], r["seed"])
                if key in runs:
                    raise SystemExit(f"duplicate run {key}")
                runs[key] = r
    return runs


def boot_prime(vals, rng):
    v = np.log(np.asarray(vals, float))
    idx = rng.integers(0, len(v), size=(B, len(v)))
    return np.exp(v[idx].mean(axis=1))


def summarize(per_curve, rng):
    per_prime, boots = {}, []
    if not per_curve:
        return {"per_prime": {}, "pooled": None}
    for p in sorted(per_curve):
        b = boot_prime(per_curve[p], rng)
        boots.append(np.log(b))
        per_prime[str(p)] = {"gm": gm(per_curve[p]), "ci95": pct(b), "n_curves": len(per_curve[p])}
    pooled_point = math.exp(sum(math.log(x["gm"]) for x in per_prime.values()) / len(per_prime))
    pooled_boot = np.exp(np.mean(boots, axis=0))
    return {"per_prime": per_prime, "pooled": {"gm": pooled_point, "ci95": pct(pooled_boot)}}


def verdict(prim):
    if prim["gae_1.0"]["pooled"] is None:
        return None
    up = {w: prim[f"gae_{w}"]["pooled"]["ci95"][1] for w in WS}
    lo1 = prim["gae_1.0"]["pooled"]["ci95"][0]
    if up[1.0] < 1 and up[0.1] < 1 and up[0.0] <= 1:
        return "IMPROVEMENT (order holds across the band)"
    if lo1 >= KILL_LO:
        return "KILL"
    return "INCONCLUSIVE"


def main():
    runs = load(sys.argv[1:], ARMS)
    out = {"protocol": "research/quartic_diem_p0lines_20261128/PROTOCOL.md",
           "label": "[pcc:unverified]"}
    keys = sorted({k[1:] for k in runs})
    bad = [k for k, r in runs.items() if not (r["certified"] and r["recovered"] == r["t_secret"])]
    unpaired = [k for k in keys if any((a,) + k not in runs for a in ARMS)]
    same_log = all(len({runs[(a,) + k]["recovered"] for a in ARMS}) == 1
                   for k in keys if k not in unpaired)
    p0_ok = all(runs[k].get("p0_line_relations", 0) > 0 for k in runs if k[0] in (P0, P0I3))
    nofb_ok = all(runs[k].get("p0_lines", 0) == 0 for k in runs if k[0] == NOFB)
    out["n_runs"] = {a: sum(1 for k in runs if k[0] == a) for a in ARMS}
    out["uncertified_or_wrong_log"] = [list(k) for k in bad]
    out["unpaired"] = [list(k) for k in unpaired]
    out["same_log_all_arms"] = same_log
    out["p0_rows_have_p0_relations"] = p0_ok
    out["nofb_rows_have_no_p0_lines"] = nofb_ok

    # C0: nofb rows for seeds 1-4 reproduce the parent study's nofb rows
    parent = load(PARENT, [NOFB])
    c0 = {"compared": 0, "identical": 0, "mismatches": []}
    for k, ref in parent.items():
        a = runs.get(k)
        if a is None:
            c0["mismatches"].append({"key": list(k), "missing": True})
            continue
        c0["compared"] += 1
        diff = [f for f in C0_FIELDS if ref.get(f) != a.get(f)]
        if diff:
            c0["mismatches"].append({"key": list(k), "fields": diff})
        else:
            c0["identical"] += 1
    out["C0_reproduction"] = c0

    # second attempts per arm, prime and panel
    att = {}
    for pname, seeds in (("primary_seeds_5_6", PRIMARY_SEEDS), ("all_seeds_1_6", ALL_SEEDS)):
        att[pname] = {}
        for a in ARMS:
            rows = [r for k, r in runs.items() if k[0] == a and k[3] in seeds]
            per_p = defaultdict(lambda: [0, 0])
            for r in rows:
                per_p[r["p"]][0] += r["attempts"] > 1
                per_p[r["p"]][1] += 1
            att[pname][a] = {"multi_attempt": sum(v[0] for v in per_p.values()),
                             "runs": sum(v[1] for v in per_p.values()),
                             "per_prime": {str(p): v for p, v in sorted(per_p.items())},
                             "p0_line_relations_mean": (sum(r.get("p0_line_relations", 0) for r in rows)
                                                        / max(1, len(rows)))}
    out["attempts"] = att

    rng = np.random.default_rng(BOOT_SEED)
    panels = {"primary_seeds_5_6": PRIMARY_SEEDS, "all_seeds_1_6": ALL_SEEDS}
    metrics = {f"gae_{w}": (lambda r, w=w: gae(r, w)) for w in WS}
    metrics["la_total"] = lambda r: r["la_add"] + r["la_mul"]
    metrics["rel_total"] = lambda r: r["rel_add"] + r["rel_mul"]
    metrics["native_all"] = native_all
    paired = {}
    for cname, (num, den) in COMPARISONS.items():
        paired[cname] = {}
        for pname, seeds in panels.items():
            res = {}
            for mname, f in metrics.items():
                per_curve = defaultdict(list)
                for p, cs in sorted({(k[0], k[1]) for k in keys}):
                    ss = [s for s in sorted(seeds) if (p, cs, s) in keys and (p, cs, s) not in unpaired]
                    if ss:
                        per_curve[p].append(gm([f(runs[(num, p, cs, s)]) / f(runs[(den, p, cs, s)])
                                                for s in ss]))
                res[mname] = summarize(per_curve, rng)
            paired[cname][pname] = res
    out["paired"] = paired

    icr, ver = {}, {}
    for a in ARMS:
        icr[a], ver[a] = {}, {}
        for w in WS:
            per_curve = defaultdict(list)
            for p, cs in sorted({(k[0], k[1]) for k in keys}):
                vals = [gae(runs[(a, p, cs, s)], w) / math.sqrt(math.pi * runs[(a, p, cs, s)]["r"] / 4)
                        for s in sorted(ALL_SEEDS) if (a, p, cs, s) in runs]
                if vals:
                    per_curve[p].append(gm(vals))
            icr[a][f"all_{w}"] = summarize(per_curve, rng)
        for p in sorted({k[0] for k in keys}):
            ups = [icr[a][f"all_{w}"]["per_prime"][str(p)]["ci95"][1] for w in WS]
            los = [icr[a][f"all_{w}"]["per_prime"][str(p)]["ci95"][0] for w in WS]
            v = ("BELOW-RHO CANDIDATE" if max(ups) < 1 else
                 "ABOVE RHO" if min(los) > 1 else "UNRESOLVED (order changes inside the band)")
            ver[a][str(p)] = {"verdict": v, "upper95": ups, "lower95": los}
    out["ic_over_rho"] = icr
    out["ic_over_rho_verdict"] = ver

    integrity = (not bad and not unpaired and same_log and not c0["mismatches"]
                 and c0["compared"] > 0 and p0_ok and nofb_ok)
    prim = paired["p0_over_nofb"]["primary_seeds_5_6"]
    sec = paired["p0i3_over_nofb"]["primary_seeds_5_6"]
    v1, v2 = verdict(prim), verdict(sec)
    if v1 is None:
        integrity = False
    out["decision"] = {"rule": "PROTOCOL.md section 5", "integrity": integrity,
                       "verdict_p0": v1 if integrity else "VOID",
                       "verdict_p0i3_secondary": v2 if integrity else "VOID",
                       **{f"p0_pooled_gae_{w}": prim[f"gae_{w}"]["pooled"] for w in WS},
                       **{f"p0i3_pooled_gae_{w}": sec[f"gae_{w}"]["pooled"] for w in WS}}
    json.dump(out, sys.stdout, indent=1, default=float)
    print()


if __name__ == "__main__":
    main()
