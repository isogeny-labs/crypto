#!/usr/bin/env python3
"""Independent recompute of round 39's holdout statistic from raw ecbench records.

Reads sessions/holdout-k28/records.jsonl, pairs `inc` and `sel` by (workload, round)
on counted S at w = 0, and prints the ratio of means with a workload-clustered and a
plain pair bootstrap. Uses no ecbench code. Run: python3 paired_ratio.py <records.jsonl>
"""
import collections, json, random, sys

records = [json.loads(line) for line in open(sys.argv[1])]
by_key = collections.defaultdict(dict)
for rec in records:
    if rec["warmup"] or rec["outcome"]["status"] != "verified":
        continue
    by_key[(rec["workload"]["workload_id"], rec["round"])][rec["arm"]] = rec["cost"]["s"]

def ratio_of_means(pairs):
    return sum(b for a, b in pairs) / sum(a for a, b in pairs)

pairs = [(v["inc"], v["sel"]) for v in by_key.values() if "inc" in v and "sel" in v]
aa = [(v["inc"], v["inc-aa"]) for v in by_key.values() if "inc" in v and "inc-aa" in v]
by_workload = collections.defaultdict(list)
for (workload, _), v in by_key.items():
    if "inc" in v and "sel" in v:
        by_workload[workload].append((v["inc"], v["sel"]))
workloads = list(by_workload)
random.seed(1)
clustered = sorted(
    ratio_of_means([q for w in random.choices(workloads, k=len(workloads)) for q in by_workload[w]])
    for _ in range(4000)
)
plain = sorted(ratio_of_means(random.choices(pairs, k=len(pairs))) for _ in range(4000))
print(f"pairs {len(pairs)} workloads {len(workloads)}")
print(f"ratio of means sel/inc {ratio_of_means(pairs):.4f}   A/A inc-aa/inc {ratio_of_means(aa):.4f}")
print(f"workload-clustered bootstrap 95% [{clustered[100]:.4f}, {clustered[3899]:.4f}]")
print(f"plain pair bootstrap 95%        [{plain[100]:.4f}, {plain[3899]:.4f}]")
