#!/usr/bin/env python3
"""Analyse the RunPod wallfix ABAB sessions.

Inputs: a directory of session dirs named pr<PR>-<spec>-r<round>-<A|B>, runlog.tsv, verify.txt.
Per session, sums time.solve_wall_ns and time.schedstat_solve.cpu_ns over the measured
(non-warmup) records, per ecbench arm and in total. Per (PR, spec) it reports:
  - A/A: max/min of the A-arm totals over rounds (the noise floor on this host),
  - B/A: per-round paired ratio B_r / A_r, and the geometric mean over rounds,
  - per ecbench arm, the geometric-mean paired B/A ratio of summed solve cpu time.
Writes results.json and prints a table. Timings are [pcc:unverified] RunPod tier.
"""
import json, math, os, re, sys
from collections import defaultdict

root, out = sys.argv[1], sys.argv[2]
pat = re.compile(r"pr(\d+)-(\w+)-r(\d+)-([AB])$")
S = {}
for n in sorted(os.listdir(root)):
    m = pat.match(n)
    if not m or not os.path.isdir(os.path.join(root, n)):
        continue
    pr, spec, r, arm = m.group(1), m.group(2), int(m.group(3)), m.group(4)
    per = defaultdict(lambda: [0, 0, 0])
    with open(os.path.join(root, n, "records.jsonl")) as f:
        for line in f:
            rec = json.loads(line)
            if rec.get("warmup"):
                continue
            t = rec.get("time", {})
            w = t.get("solve_wall_ns") or 0
            c = (t.get("schedstat_solve") or {}).get("cpu_ns") or 0
            a = per[rec["arm"]]
            a[0] += w; a[1] += c; a[2] += 1
    S[(pr, spec, r, arm)] = dict(per)

def gm(xs):
    xs = [x for x in xs if x > 0]
    return math.exp(sum(map(math.log, xs)) / len(xs)) if xs else float("nan")

res = []
for pr, spec in sorted({(k[0], k[1]) for k in S}):
    rounds = sorted({k[2] for k in S if k[:2] == (pr, spec)})
    tot = lambda r, arm, i: sum(v[i] for v in S[(pr, spec, r, arm)].values())
    row = {"pr": pr, "spec": spec, "rounds": rounds}
    for i, name in ((0, "solve_wall"), (1, "solve_cpu")):
        A = [tot(r, "A", i) for r in rounds]; B = [tot(r, "B", i) for r in rounds]
        ratios = [b / a for a, b in zip(A, B)]
        row[name] = {"A_ns": A, "B_ns": B, "aa_spread": max(A) / min(A),
                     "bb_spread": max(B) / min(B), "ba_per_round": ratios,
                     "ba_geomean": gm(ratios), "ba_min": min(ratios), "ba_max": max(ratios)}
    arms = sorted(S[(pr, spec, rounds[0], "A")])
    row["per_arm_cpu"] = {}
    for e in arms:
        # schedstat cpu_ns advances on scheduler ticks: solves under ~1 ms read 0. A per-arm
        # ratio is reported only when every round of both arms has a nonzero sum; otherwise null.
        pa = [S[(pr, spec, r, "A")][e][1] for r in rounds]
        pb = [S[(pr, spec, r, "B")][e][1] for r in rounds]
        n = S[(pr, spec, rounds[0], "A")][e][2]
        if min(pa + pb) <= 0:
            row["per_arm_cpu"][e] = {"ba_geomean": None, "n": n, "note": "a round sums to 0 cpu_ns (sub-tick solves)"}
            continue
        rs = [b / a for a, b in zip(pa, pb)]
        row["per_arm_cpu"][e] = {"ba_geomean": gm(rs), "ba_min": min(rs), "ba_max": max(rs),
                                 "aa_spread": max(pa) / min(pa), "share_of_A_cpu": sum(pa) / sum(tot(r, "A", 1) for r in rounds), "n": n}
    res.append(row)

json.dump({"schema": "wallfix.runpod.analysis/v1", "standing": "pcc:unverified",
           "metric_note": "sums over measured (non-warmup) records of time.solve_wall_ns and time.schedstat_solve.cpu_ns",
           "rows": res}, open(out, "w"), indent=1)
print(f"{'PR':>5} {'spec':8} {'metric':10} {'A/A':>6} {'B/B':>6} {'B/A gm':>7} {'B/A min..max':>14}")
for row in res:
    for name in ("solve_cpu", "solve_wall"):
        m = row[name]
        print(f"{row['pr']:>5} {row['spec']:8} {name:10} {m['aa_spread']:6.3f} {m['bb_spread']:6.3f} {m['ba_geomean']:7.3f} {m['ba_min']:6.3f}..{m['ba_max']:.3f}")
