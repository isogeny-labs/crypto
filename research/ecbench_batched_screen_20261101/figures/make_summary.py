#!/usr/bin/env python3
"""summary.json: per-cell, per-arm means from the verified records and the
saved paired comparisons, and the five frozen predictions of PROTOCOL.md
evaluated by code.  Standard library only.  Reads sessions/<cell>/records.jsonl
(in the artifact store: `artifacts.py pull`, see README.md) and sessions/<cell>/comparisons/*.json."""
import json, os, statistics as st, sys
from collections import defaultdict
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(HERE)
CELLS = ["n13a0", "n17a1", "n19a0", "n23a0"]
BATCHES = [1, 2, 4, 8, 16, 32, 64, 128, 256]
PH = ["factor_base", "oracle_setup", "relations", "linear_algebra", "descent_setup", "descent", "verify"]
CT = ["collection_trials", "collection_batches", "relations_collected", "solve_attempts", "descent_trials", "columns"]
UN = ["la_row_ops_uncharged", "pair_filter_probes_uncharged", "pair_bucket_reads_uncharged",
      "frobenius_maps_uncharged", "canonicalisations_uncharged"]
out = {"schema": "crypto.ecbench_batched_screen/v1", "protocol": "PROTOCOL.md", "unit": "counted GAE (group additions and doublings), mean per run over the measured rounds", "cells": {}}
for cell in CELLS:
    acc = defaultdict(lambda: defaultdict(list)); log2r = None
    for line in open(f"{ROOT}/sessions/{cell}/records.jsonl"):
        r = json.loads(line)
        if r.get("warmup"): continue
        a = r["arm"]
        for p in r["phases"]: acc[a]["gae:" + p["name"]].append(p["gae"])
        acc[a]["gae:total"].append(sum(p["gae"] for p in r["phases"]))
        for k in CT + UN: acc[a]["c:" + k].append(r["counters"].get(k, 0))
        acc[a]["status"].append(r["outcome"]["status"])
    arms = {}
    for b in BATCHES + ["64-aa"]:
        a = f"ic-b{b}"; d = acc[a]
        m = lambda k: st.mean(d[k])
        arm = {"runs": len(d["gae:total"]), "all_verified": set(d["status"]) == {"verified"},
               "mean_gae_total": m("gae:total"), "phase_gae": {p: m("gae:" + p) for p in PH},
               "counters": {k: m("c:" + k) for k in CT}, "uncharged": {k: m("c:" + k) for k in UN}}
        if a not in ("ic-b64", "ic-b64-aa"):
            c = json.load(open(f"{ROOT}/sessions/{cell}/comparisons/ic-b64__{a}.json"))["ops"]
            arm["ratio_to_b64"] = {"ratio": c["ratio_b_over_a"], "ci95": c["ci95"], "pairs": c["pairs"]}
        arms[a] = arm
    aa = json.load(open(f"{ROOT}/sessions/{cell}/comparisons/ic-b64__ic-b64-aa.json"))["ops"]
    out["cells"][cell] = {"arms": arms, "aa": {"ratio": aa["ratio_b_over_a"], "ci95": aa["ci95"]}}
g = lambda cell, b: out["cells"][cell]["arms"][f"ic-b{b}"]
tot = lambda cell, b: g(cell, b)["mean_gae_total"]
trials = lambda cell, b: g(cell, b)["counters"]["collection_trials"]
P = {}
# P1: queries drawn = 8 for b<=8 and b for b>=16 (within 0.5%).
bad = [(c, b, round(trials(c, b), 2)) for c in CELLS for b in BATCHES
       if abs(trials(c, b) - (8 if b <= 8 else b)) > 0.005 * (8 if b <= 8 else b)]
P["P1"] = {"held": not bad, "text": "queries drawn are 8 for batch <= 8 and equal to the batch for batch >= 16",
           "evidence": {"mean_queries_drawn": {c: {str(b): round(trials(c, b), 2) for b in BATCHES} for c in CELLS}, "off_prediction": len(bad)}}
# P2: GAE rises monotonically b16..b256, and b128, b256 are the two most expensive arms.
mono = all(all(tot(c, x) < tot(c, y) for x, y in zip(BATCHES[4:], BATCHES[5:])) for c in CELLS)
top2 = all(set(sorted(BATCHES, key=lambda b: -tot(c, b))[:2]) == {128, 256} for c in CELLS)
P["P2"] = {"held": mono and top2, "text": "GAE rises monotonically from batch 16 to 256; batch 128 and 256 are the two most expensive arms", "evidence": {"monotone_16_to_256": mono, "128_and_256_most_expensive": top2}}
# P3: GAE non-increasing in batch from 1 to 8.
non_inc = {c: all(tot(c, x) >= tot(c, y) for x, y in zip([1, 2, 4], [2, 4, 8])) for c in CELLS}
P["P3"] = {"held": all(non_inc.values()), "text": "GAE is non-increasing in batch_trials from 1 to 8",
           "evidence": {"non_increasing_per_cell": non_inc, "gae_b1_b2_b4_b8": {c: [round(tot(c, b), 1) for b in (1, 2, 4, 8)] for c in CELLS}}}
# P4: the minimum over the nine arms lies in {4, 8, 16}.
argmin = {c: min(BATCHES, key=lambda b: tot(c, b)) for c in CELLS}
P["P4"] = {"held": all(v in (4, 8, 16) for v in argmin.values()), "text": "the minimum over the nine arms lies in {4, 8, 16} on every cell", "evidence": {"argmin_batch": argmin}}
# P5: ic-b16 / ic-b64 < 1 with the upper interval bound below 1 on every cell.
r16 = {c: g(c, 16)["ratio_to_b64"] for c in CELLS}
P["P5"] = {"held": all(v["ci95"][1] < 1 for v in r16.values()), "text": "ic-b16 / ic-b64 < 1 (upper 95% bound below 1) on every cell",
           "evidence": {c: {"ratio": v["ratio"], "ci95": v["ci95"]} for c, v in r16.items()}}
out["predictions"] = P
out["a_a_all_within_0.001"] = all(abs(out["cells"][c]["aa"]["ratio"] - 1) <= 0.001 for c in CELLS)
out["all_records_verified"] = all(g(c, b)["all_verified"] for c in CELLS for b in BATCHES)
# every uncharged counter at batch 16 is no larger than at batch 64 (the native work the unit leaves unpriced)
out["b16_native_not_above_b64"] = {c: {k: g(c, 16)["uncharged"][k] <= g(c, 64)["uncharged"][k] for k in UN} for c in CELLS}
json.dump(out, open(f"{HERE}/summary.json", "w"), indent=1)
for k, v in P.items(): print(k, "HELD" if v["held"] else "NOT HELD", "-", v["text"])
print("A/A within 0.001:", out["a_a_all_within_0.001"], "| all records verified:", out["all_records_verified"])
