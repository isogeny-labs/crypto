#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md (research/quartic_diem_nofb_20261126).

Usage: python3 analyze.py raw/*.jsonl > analysis.json

Arms (paired on the same instance and solver seed, same binary):
  Q_diem_2lp_sfb_sge       baseline: trim + SGE + square coordinate Wiedemann,
                           full-core Wiedemann fallback when the square solve
                           does not certify (research/quartic_diem_la_20261123)
  Q_diem_2lp_sfb_sge_nofb  identical, but the fallback is skipped and the loop
                           goes straight on to the next attempt
Reference: ideal negation rho sqrt(pi r / 4) group additions, nothing charged.
Primary panel: held-out solver seeds 3, 4. Secondary: all seeds 1-4.
C0: the baseline rows for seeds 1, 2 against the parent study's rows.
Every number is [pcc:unverified].
"""
import json
import math
import sys
from collections import defaultdict

import numpy as np

SGE = "Q_diem_2lp_sfb_sge"
NOFB = "Q_diem_2lp_sfb_sge_nofb"
ARMS = [SGE, NOFB]
WS = [0.0, 0.1, 1.0]
B = 4000
BOOT_SEED = 20261126
PRIMARY_SEEDS = {3, 4}
PARENT = ["../quartic_diem_la_20261123/raw/run.jsonl",
          "../quartic_diem_la_20261123/raw/run131101.jsonl"]
C0_FIELDS = ["certified", "ok", "recovered", "group_ops", "cantor_mul", "cantor_inv",
             "rel_mul", "rel_add", "la_mul", "la_add", "relations", "rels_by_lp",
             "decomp_trials", "attempts", "core_rows", "core_cols", "core_nnz",
             "idx_lookups", "idx_inserts", "idx_points", "fb_samples",
             "la_fallbacks", "sq_dim", "sq_nnz"]
KILL_LO = 0.98  # pooled paired GAE_1 lower 95% at or above this: no material saving


def native_all(r):
    return (r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]
            + r["idx_lookups"] + r["idx_inserts"])


def gae(r, w):
    return r["group_ops"] + w * native_all(r)


def gm(v):
    return math.exp(sum(math.log(x) for x in v) / len(v))


def pct(a):
    return [float(np.percentile(a, 2.5)), float(np.percentile(a, 97.5))]


def load(paths):
    runs = {}
    for path in paths:
        for line in open(path):
            r = json.loads(line)
            if r.get("kind") == "run" and r["arm"] in ARMS:
                key = (r["arm"], r["p"], r["curve_seed"], r["seed"])
                if key in runs:
                    raise SystemExit(f"duplicate run {key}")
                runs[key] = r
    return runs


def boot_prime(vals, rng):
    """curve-cluster bootstrap of the geometric mean of curve-level values"""
    v = np.log(np.asarray(vals, float))
    idx = rng.integers(0, len(v), size=(B, len(v)))
    return np.exp(v[idx].mean(axis=1))


def summarize(per_curve, rng):
    """per_curve: {p: [curve-level values]} -> per-prime gm + boot95, pooled equal weight per prime"""
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


def main():
    runs = load(sys.argv[1:])
    out = {"protocol": "research/quartic_diem_nofb_20261126/PROTOCOL.md",
           "label": "[pcc:unverified]"}
    keys = sorted({k[1:] for k in runs})
    bad = [k for k, r in runs.items() if not (r["certified"] and r["recovered"] == r["t_secret"])]
    unpaired = [k for k in keys if any((a,) + k not in runs for a in ARMS)]
    same_log = all(runs[(SGE,) + k]["recovered"] == runs[(NOFB,) + k]["recovered"]
                   for k in keys if k not in unpaired)
    out["n_runs"] = {a: sum(1 for k in runs if k[0] == a) for a in ARMS}
    out["uncertified_or_wrong_log"] = [list(k) for k in bad]
    out["unpaired"] = [list(k) for k in unpaired]
    out["same_log_both_arms"] = same_log

    # C0: baseline rows for seeds 1, 2 reproduce the parent study's rows
    parent = load(PARENT)
    c0 = {"compared": 0, "identical": 0, "mismatches": []}
    for k, ref in parent.items():
        if k[0] != SGE:
            continue
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

    # the rescue rate the skip gives up, and the skipped fallbacks
    out["rescues"] = {
        "baseline_fallbacks": sum(runs[k]["la_fallbacks"] for k in runs if k[0] == SGE),
        "baseline_rescues": sum(runs[k]["la_fallback_rescues"] for k in runs if k[0] == SGE),
        "nofb_skipped": sum(runs[k]["la_fallbacks_skipped"] for k in runs if k[0] == NOFB),
        "nofb_fallbacks": sum(runs[k]["la_fallbacks"] for k in runs if k[0] == NOFB),
        "attempts_differ": [list(k) for k in keys if k not in unpaired
                            and runs[(SGE,) + k]["attempts"] != runs[(NOFB,) + k]["attempts"]],
    }

    # pair identities (PROTOCOL section 2): without a rescue the arms see the
    # same attempts and relation phase; nofb does the baseline's work minus the
    # skipped fallbacks (equal with no fallback, strictly less with one)
    ident = []
    for k in keys:
        if k in unpaired:
            continue
        a, b = runs[(SGE,) + k], runs[(NOFB,) + k]
        if a["la_fallback_rescues"] != 0:
            continue
        same_rel = all(a[f] == b[f] for f in ("attempts", "relations", "decomp_trials",
                                                  "rel_mul", "rel_add", "core_rows", "core_cols"))
        la_a, la_b = (a["la_add"], a["la_mul"]), (b["la_add"], b["la_mul"])
        if a["la_fallbacks"] == 0:
            ok = same_rel and la_a == la_b and a["group_ops"] == b["group_ops"] \
                and b["la_fallbacks_skipped"] == 0
        else:
            ok = same_rel and la_b[0] <= la_a[0] and la_b[1] <= la_a[1] \
                and sum(la_b) < sum(la_a) and b["group_ops"] <= a["group_ops"] \
                and b["la_fallbacks_skipped"] == a["la_fallbacks"]
        if not ok:
            ident.append(list(k))
    out["pair_identity_violations"] = ident

    rng = np.random.default_rng(BOOT_SEED)
    panels = {"primary_seeds_3_4": PRIMARY_SEEDS, "all_seeds_1_4": {1, 2, 3, 4}}
    paired = {}
    for pname, seeds in panels.items():
        res = {}
        metrics = {f"gae_{w}": (lambda r, w=w: gae(r, w)) for w in WS}
        metrics["la_total"] = lambda r: r["la_add"] + r["la_mul"]
        metrics["native_all"] = native_all
        for mname, f in metrics.items():
            per_curve = defaultdict(list)
            for p, cs in sorted({(k[0], k[1]) for k in keys}):
                ss = [s for s in seeds if (p, cs, s) in keys and (p, cs, s) not in unpaired]
                if ss:
                    per_curve[p].append(gm([f(runs[(NOFB, p, cs, s)]) / f(runs[(SGE, p, cs, s)])
                                            for s in ss]))
            res[mname] = summarize(per_curve, rng)
        paired[pname] = res
    out["paired_nofb_over_sge"] = paired

    # IC / ideal negation rho per arm, prime and w (all seeds)
    icr, verdict = {}, {}
    for a in ARMS:
        icr[a], verdict[a] = {}, {}
        for w in WS:
            per_curve = defaultdict(list)
            for p, cs in sorted({(k[0], k[1]) for k in keys}):
                vals = []
                for s in (1, 2, 3, 4):
                    r = runs.get((a, p, cs, s))
                    if r is not None:
                        vals.append(gae(r, w) / math.sqrt(math.pi * r["r"] / 4))
                if vals:
                    per_curve[p].append(gm(vals))
            icr[a][f"all_{w}"] = summarize(per_curve, rng)
        for p in sorted({k[0] for k in keys}):
            ups = [icr[a][f"all_{w}"]["per_prime"][str(p)]["ci95"][1] for w in WS]
            los = [icr[a][f"all_{w}"]["per_prime"][str(p)]["ci95"][0] for w in WS]
            v = ("BELOW-RHO CANDIDATE" if max(ups) < 1 else
                 "ABOVE RHO" if min(los) > 1 else "UNRESOLVED (order changes inside the band)")
            verdict[a][str(p)] = {"verdict": v, "upper95": ups, "lower95": los}
    out["ic_over_rho"] = icr
    out["ic_over_rho_verdict"] = verdict

    # decision (section 6), on the primary held-out panel
    prim = paired["primary_seeds_3_4"]
    integrity = (not bad and not unpaired and same_log and not c0["mismatches"]
                 and not ident)
    if prim["gae_1.0"]["pooled"] is None:
        integrity = False
    else:
        up = {w: prim[f"gae_{w}"]["pooled"]["ci95"][1] for w in WS}
        lo1 = prim["gae_1.0"]["pooled"]["ci95"][0]
    if not integrity:
        dec = "VOID"
    elif up[1.0] < 1 and up[0.1] < 1 and up[0.0] <= 1:
        dec = "IMPROVEMENT (order holds across the band)"
    elif lo1 >= KILL_LO:
        dec = "KILL"
    else:
        dec = "INCONCLUSIVE"
    out["decision"] = {"rule": "PROTOCOL.md section 6", "integrity": integrity, "verdict": dec,
                       **{f"pooled_gae_{w}": prim[f"gae_{w}"]["pooled"] for w in WS}}
    json.dump(out, sys.stdout, indent=1, default=float)
    print()


if __name__ == "__main__":
    main()
