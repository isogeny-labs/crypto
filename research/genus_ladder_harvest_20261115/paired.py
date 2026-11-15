#!/usr/bin/env python3
"""Paired harvest/off ratios of a genus_ladder_harvest session (layout only).

usage: paired.py OUT.json IN.jsonl [IN.jsonl ...]

Per pair (instance, seed, base arm): ratio harvest / off of each reading.
Per (genus, p, arm): geometric mean with a 95% hierarchical bootstrap interval
(curves, then seeds within a curve), 4000 resamples, seed 20261115.
Decision rules: PROTOCOL.md section 5.  All figures [pcc:unverified].
"""
import json
import math
import random
import sys
from collections import defaultdict


def readings(r):
    g = r["g"]
    nat = r["rel_mul"] + r["la_mul"]
    return {
        "trials": r["trials"],
        "trials_per_relation": r["trials"] / max(r["relations"], 1),
        "group_ops": r["group_ops"],
        "gae_w0": r["group_ops"],
        "gae_w01": r["group_ops"] + 0.1 * nat,
        "gae_w1": r["group_ops"] + nat,
        "fpm_mid": r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + g * g * r["la_mul"],
        "rel_mul": r["rel_mul"],
        "la_mul": r["la_mul"],
        "relations": r["relations"],
    }


def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def boot(by_curve, rng, n=4000):
    curves = sorted(by_curve)
    out = []
    for _ in range(n):
        logs = []
        for _ in curves:
            c = by_curve[rng.choice(curves)]
            for _ in c:
                logs.append(math.log(rng.choice(c)))
        out.append(math.exp(sum(logs) / len(logs)))
    out.sort()
    return out[int(0.025 * (n - 1))], out[int(0.975 * (n - 1))]


def main():
    out_path, ins = sys.argv[1], sys.argv[2:]
    runs = []
    for fn in ins:
        for line in open(fn):
            r = json.loads(line)
            if r.get("kind") == "run":
                runs.append(r)
    by = {}
    for r in runs:
        by[(r["g"], r["p"], r["curve_seed"], r["seed"], r["arm"])] = r
    pairs = []
    for (g, p, cs, seed, arm), on in sorted(by.items()):
        if not arm.endswith("+harvest"):
            continue
        base = arm[: -len("+harvest")]
        off = by.get((g, p, cs, seed, base))
        if off is None:
            continue
        ro, rf = readings(on), readings(off)
        pairs.append({
            "g": g, "p": p, "curve_seed": cs, "seed": seed, "arm": base,
            "certified": on["certified"] and off["certified"],
            "ok": on["ok"] and off["ok"],
            "ratio": {k: ro[k] / rf[k] for k in ro if rf[k]},
            "on": {k: on[k] for k in ("trials", "relations", "harvest_relations", "intermediates",
                                      "harvest_candidates", "harvest_prefilter_rejects",
                                      "harvest_splits", "harvest_overflow", "candidates", "splits")},
            "off": {k: off[k] for k in ("trials", "relations", "candidates", "splits")},
        })
    cells = defaultdict(list)
    for pr in pairs:
        cells[(pr["g"], pr["p"], pr["arm"])].append(pr)
    rng = random.Random(20261115)
    summary = []
    keys = ["trials", "trials_per_relation", "group_ops", "gae_w01", "gae_w1", "fpm_mid",
            "rel_mul", "la_mul"]
    for (g, p, arm), prs in sorted(cells.items()):
        rec = {"g": g, "p": p, "arm": arm, "pairs": len(prs),
               "all_certified": all(x["certified"] and x["ok"] for x in prs)}
        for k in keys:
            byc = defaultdict(list)
            for x in prs:
                byc[x["curve_seed"]].append(x["ratio"][k])
            vals = [v for vs in byc.values() for v in vs]
            lo, hi = boot(byc, rng)
            rec[k] = {"gm": gm(vals), "lo": lo, "hi": hi, "min": min(vals), "max": max(vals)}
        inter = sum(x["on"]["intermediates"] for x in prs)
        rec["harvest_split_rate"] = sum(x["on"]["harvest_splits"] for x in prs) / max(inter, 1)
        rec["harvest_prefilter_pass"] = 1 - sum(
            x["on"]["harvest_prefilter_rejects"] for x in prs) / max(inter, 1)
        rec["main_split_rate_off"] = sum(x["off"]["splits"] for x in prs) / max(
            sum(x["off"]["candidates"] for x in prs), 1)
        rec["harvest_share_of_relations"] = sum(x["on"]["harvest_relations"] for x in prs) / max(
            sum(x["on"]["relations"] for x in prs), 1)
        rec["intermediates_per_trial"] = inter / max(sum(x["on"]["trials"] for x in prs), 1)
        rec["harvest_overflow"] = sum(x["on"]["harvest_overflow"] for x in prs)
        rec["genus2_all_trials_equal"] = all(x["ratio"]["trials"] == 1.0 for x in prs)
        summary.append(rec)
    json.dump({"schema": "genus_ladder_harvest.paired/v1", "inputs": ins, "pairs": pairs,
               "cells": summary, "label": "[pcc:unverified]"}, open(out_path, "w"), indent=1,
              sort_keys=True)
    for rec in summary:
        print(rec["g"], rec["p"], rec["arm"], rec["pairs"],
              " ".join(f"{k}={rec[k]['gm']:.3f}[{rec[k]['lo']:.3f},{rec[k]['hi']:.3f}]"
                       for k in ("trials", "trials_per_relation", "group_ops", "fpm_mid")),
              f"split_rate={rec['harvest_split_rate']:.4f}")


if __name__ == "__main__":
    main()
