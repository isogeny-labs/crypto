#!/usr/bin/env python3
"""Per-phase share, run-to-run spread and Amdahl ceilings from ecbench records.

Input: one or more records.jsonl files (ecbench.record/v1; each line has
`arm` and `phases[]` with `name` and `wall_ns`). Output: JSON on stdout.

Per arm and phase: mean wall, sample standard deviation across runs, mean share
of the summed phase wall, the share's sample standard deviation, and the Amdahl
ceiling 1 / (1 - share) on the summed phase wall (the best whole-solve speedup
if that phase cost nothing). The ceiling is a bound on what removing a phase
could give, never a promised speedup. Phases without `wall_ns` are listed under
`unclocked` and are left out of the shares.

A profile explains a cost; it proves no speedup (profile skill, section 0).
"""
import json
import statistics
import sys
from collections import defaultdict


def load(paths):
    runs = defaultdict(list)  # arm -> [ {phase: wall_ns} ]
    unclocked = defaultdict(set)
    for path in paths:
        with open(path) as handle:
            for line in handle:
                line = line.strip()
                if not line:
                    continue
                rec = json.loads(line)
                if rec.get("warmup"):
                    continue
                walls = {}
                for ph in rec.get("phases", []):
                    if ph.get("wall_ns") is None:
                        unclocked[rec["arm"]].add(ph["name"])
                    else:
                        walls[ph["name"]] = walls.get(ph["name"], 0) + ph["wall_ns"]
                if walls:
                    runs[rec["arm"]].append(walls)
    return runs, unclocked


def sd(xs):
    return statistics.stdev(xs) if len(xs) > 1 else None


def summarise(runs, unclocked):
    out = {}
    for arm, rows in sorted(runs.items()):
        names = sorted({n for r in rows for n in r})
        totals = [sum(r.values()) for r in rows]
        phases = {}
        for n in names:
            walls = [r.get(n, 0) for r in rows]
            shares = [w / t for w, t in zip(walls, totals)]
            share = statistics.fmean(shares)
            phases[n] = {
                "mean_wall_ns": statistics.fmean(walls),
                "sd_wall_ns": sd(walls),
                "share": share,
                "sd_share": sd(shares),
                "amdahl_ceiling": (1.0 / (1.0 - share)) if share < 1 else None,
            }
        out[arm] = {
            "runs": len(rows),
            "mean_total_ns": statistics.fmean(totals),
            "sd_total_ns": sd(totals),
            "phases": phases,
            "unclocked": sorted(unclocked.get(arm, [])),
        }
    return out


def main(argv):
    if not argv:
        sys.exit("usage: analyse.py records.jsonl [...]")
    runs, unclocked = load(argv)
    json.dump(summarise(runs, unclocked), sys.stdout, indent=1, sort_keys=True)
    print()


if __name__ == "__main__":
    main(sys.argv[1:])
