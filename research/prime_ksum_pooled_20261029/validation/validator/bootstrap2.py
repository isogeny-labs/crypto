#!/usr/bin/env python3
"""Two-stage cluster percentile bootstrap (workloads, then rounds within a
workload) of the paired ratio of sums, own RNG (Python random, seed
20261002, 4000 resamples). Monte-Carlo cross-check of the README brackets;
not bit-reproducible against ecbench's RNG by design."""
import json, math, random, sys
from collections import defaultdict
from pathlib import Path
SESS = Path(sys.argv[1]); OUT = Path(sys.argv[2])
want = [(20,'inc-sel','ksum5-sel',0.0),(24,'inc-sel','ksum5-sel',0.0),(28,'inc-sel','ksum5-sel',0.0),(32,'inc-sel','ksum5-sel',0.0),
        (20,'inc-sel','ksum4-sel',0.0),(24,'inc-sel','ksum4-sel',0.0),(28,'inc-sel','ksum4-sel',0.0),(32,'inc-sel','ksum4-sel',0.0),
        (20,'ksum4-sel','ksum5-sel',1.0),(24,'ksum4-sel','ksum5-sel',1.0),(28,'ksum4-sel','ksum5-sel',1.0),(32,'ksum4-sel','ksum5-sel',1.0),
        (20,'ksum4-sel','ksum5-sel',0.0),(20,'inc-rec','inc-sel',0.0),(20,'inc-rec','inc-sel',1.0)]
def U(r):
    t = 0
    for name in r["cost"]["unpriced"]:
        base = name[:-10] if name.endswith("_uncharged") else name
        if name in r.get("counters", {}): t += r["counters"][name]; continue
        t += sum((ph.get("native") or {}).get(name, (ph.get("native") or {}).get(base, 0)) for ph in r["phases"])
    return t
S = defaultdict(dict)
for b in (20, 24, 28, 32):
    for line in open(SESS / f"holdout-P{b}" / "records.jsonl"):
        r = json.loads(line)
        if r["warmup"] or r["outcome"]["status"] != "verified": continue
        S[(b, r["arm"])][(r["workload"]["workload_id"], r["round"])] = (r["cost"]["total_gae"], U(r), math.sqrt(r["workload"]["curve"]["r"]))
out = {}
for b, A, B, w in want:
    a, bb = S[(b, A)], S[(b, B)]
    cl = defaultdict(list)
    for k in sorted(set(a) & set(bb)):
        sa = (a[k][0] + w * a[k][1]) / a[k][2]; sb = (bb[k][0] + w * bb[k][1]) / bb[k][2]
        cl[k[0]].append((sa, sb))
    clusters = list(cl.values()); K = len(clusters)
    rng = random.Random(20261002); st = []
    for _ in range(4000):
        tA = tB = 0.0
        for _ in range(K):
            c = clusters[rng.randrange(K)]
            for _ in range(len(c)):
                x, y = c[rng.randrange(len(c))]; tA += x; tB += y
        st.append(tB / tA)
    st.sort()
    out[f"P{b}|{A}__{B}|w{w:g}"] = [st[99], st[3899]]
    print(b, A, B, w, [round(v, 4) for v in out[f"P{b}|{A}__{B}|w{w:g}"]])
OUT.write_text(json.dumps(out, indent=1))
