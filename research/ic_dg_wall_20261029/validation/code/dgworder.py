#!/usr/bin/env python3
"""Arm execution-order positions and memory per arm in the wall sessions (descriptive). Stdlib only."""
import collections, json, os, sys
EV, OUTP = sys.argv[1:3]
out = {}
for b in (24, 28, 32):
    recs = [json.loads(l) for l in open(os.path.join(EV, "P%d/wall/eks-results/session/records.jsonl" % b))]
    by = collections.defaultdict(list)
    for r in recs:
        if not r["warmup"]:
            by[(r["workload"]["workload_id"], r["round"])].append(r)
    pos = collections.defaultdict(collections.Counter)
    prev = collections.defaultdict(collections.Counter)
    for k, rs in by.items():
        rs.sort(key=lambda r: r["seq"])
        contiguous = rs[-1]["seq"] - rs[0]["seq"] == len(rs) - 1
        for i, r in enumerate(rs):
            pos[r["arm"]][i] += 1
            prev[r["arm"]][rs[i - 1]["arm"] if i else "-"] += 1
    mem = {}
    for arm in ("walk", "srenew", "dg", "srenew-aa", "dg-small"):
        rs = [r for r in recs if r["arm"] == arm and not r["warmup"]]
        n = len(rs)
        mem[arm] = {"mean_max_rss_kib": sum(r["time"]["max_rss_kib"] for r in rs) / n,
                    "max_max_rss_kib": max(r["time"]["max_rss_kib"] for r in rs),
                    "mean_minor_faults_solve": sum(r["time"]["solve"]["minor_faults"] for r in rs) / n,
                    "mean_solve_wall_ms": sum(r["time"]["solve_wall_ns"] for r in rs) / n / 1e6,
                    "involuntary_switches_solve_total": sum(r["time"]["solve"]["involuntary_switches"] for r in rs),
                    "run_delay_ns_total": sum(r["time"]["schedstat_solve"]["run_delay_ns"] for r in rs),
                    "cpu_at_start_end_equal_run_cpu": all(r["isolation"]["placement"]["cpu_at_start"] == r["isolation"]["run_cpu"] == r["isolation"]["placement"]["cpu_at_end"] for r in rs)}
    out["P%d" % b] = {"blocks": len(by), "positions_by_arm": {a: dict(sorted(c.items())) for a, c in pos.items()},
                      "mean_position_by_arm": {a: sum(i * n for i, n in c.items()) / sum(c.values()) for a, c in pos.items()},
                      "predecessor_by_arm": {a: dict(c) for a, c in prev.items()}, "memory_and_scheduling": mem}
    print("P%d" % b, json.dumps(out["P%d" % b]["mean_position_by_arm"]), json.dumps(out["P%d" % b]["positions_by_arm"]))
    for a, m in mem.items():
        print("   ", a, {k: (round(v, 3) if isinstance(v, float) else v) for k, v in m.items()})
json.dump(out, open(OUTP, "w"), indent=1)
