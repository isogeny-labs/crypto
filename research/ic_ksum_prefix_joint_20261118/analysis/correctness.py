#!/usr/bin/env python3
"""Registered T2 correctness check (PROTOCOL.md section 3). usage: python3 analysis/correctness.py <sessions dir> <out json> [names...]
For every (session, round, target) it compares the `joint` record with the `inc` record (and `prefix-joint` with `prefix`):
outcome status, recovered log, targets tried, relations found, matrix rank, every relation-phase native counter, and the
oracle-setup phase; it also lists the relation-phase (adds + doubles) and scalar-multiplication differences. Reads records
only; computes no cost ratio."""
import json, sys, os
D, OUT = sys.argv[1], sys.argv[2]
names = sys.argv[3:] or sorted(n for n in os.listdir(D) if os.path.isdir(os.path.join(D, n)))
PAIRS = [("inc", "joint"), ("prefix", "prefix-joint")]
res = {}
for n in names:
    recs = {}
    for line in open(os.path.join(D, n, "records.jsonl")):
        r = json.loads(line)
        recs[(r["arm"], r["round"], r["workload"]["target_index"])] = r
    row = {}
    for a, b in PAIRS:
        keys = sorted(k[1:] for k in recs if k[0] == a)
        bad, dgae, dsm = [], [], set()
        for k in keys:
            x, y = recs[(a,) + k], recs[(b,) + k]
            ph = lambda r, name: next(p for p in r["phases"] if p["name"] == name)
            same = (x["outcome"]["status"] == y["outcome"]["status"] and x["outcome"]["recovered"] == y["outcome"]["recovered"]
                    and x["counters"].get("targets_tried") == y["counters"].get("targets_tried")
                    and x["counters"].get("relations_found") == y["counters"].get("relations_found")
                    and x["counters"].get("matrix_rank") == y["counters"].get("matrix_rank")
                    and ph(x, "relations")["native"] == ph(y, "relations")["native"]
                    and [ph(x, "oracle_setup")[f] for f in ("adds", "doubles")] == [ph(y, "oracle_setup")[f] for f in ("adds", "doubles")])
            if not same:
                bad.append(list(k))
            rx, ry = ph(x, "relations"), ph(y, "relations")
            dgae.append((rx["adds"] + rx["doubles"]) - (ry["adds"] + ry["doubles"]))
            dsm.add((rx["scalar_mults"], ry["scalar_mults"]))
        row[f"{b}_vs_{a}"] = {"pairs": len(keys), "mismatches": len(bad), "first_mismatches": bad[:5],
                              "setup_gae_saved_min": min(dgae), "setup_gae_saved_max": max(dgae),
                              "setup_gae_saved_mean": sum(dgae) / len(dgae), "scalar_mults": sorted(dsm)}
    res[n] = row
json.dump(res, open(OUT, "w"), indent=1)
for n, row in res.items():
    print(n, *(f"{k}: {v['pairs']} pairs, {v['mismatches']} mismatches, saved {v['setup_gae_saved_min']}..{v['setup_gae_saved_max']} GAE (mean {v['setup_gae_saved_mean']:.1f}), sm {v['scalar_mults']}" for k, v in row.items()))
