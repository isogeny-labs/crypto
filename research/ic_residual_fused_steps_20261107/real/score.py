"""Score the real-code session against real/PREDICTIONS.json.

Frozen metric: total walk steps (`trials`), fused / base, per base size,
non-warm-up records, cluster bootstrap over workloads (95%).  Secondary,
added after the predictions were frozen: steps per table hit (collisions plus
self-collisions), which removes the difference in how many hits each arm
needed."""
import json, random, sys
from collections import defaultdict

recs = [json.loads(l) for l in open(sys.argv[1])]
P = json.load(open(sys.argv[2]))
by = defaultdict(lambda: defaultdict(lambda: [0, 0, 0]))  # arm -> workload -> [trials, hits, n]
for r in recs:
    if r.get("warmup"):
        continue
    nat = next(p["native"] for p in r["phases"] if "trials" in p.get("native", {}))
    w = by[r["arm"]][r["workload"]["workload_id"]]
    w[0] += nat["trials"]
    w[1] += nat.get("residual_collisions", 0) + nat.get("residual_self_collisions", 0)
    w[2] += 1

def boot(num, den, f, B=4000, seed=11):
    ws = sorted(set(num) & set(den)); rng = random.Random(seed); out = []
    point = sum(f(num[w]) for w in ws) / sum(f(den[w]) for w in ws)
    for _ in range(B):
        s = [ws[rng.randrange(len(ws))] for _ in ws]
        out.append(sum(f(num[w]) for w in s) / sum(f(den[w]) for w in s))
    out.sort(); return point, out[int(.025 * B)], out[int(.975 * B)], len(ws)

rows = []
for p in P["predictions"]:
    b, fz = by[f"res{p['B']}"], by[f"res{p['B']}f"]
    v, lo, hi, n = boot(fz, b, lambda x: x[0])
    hb = sum(x[1] for x in b.values()); hf = sum(x[1] for x in fz.values())
    sb = sum(x[0] for x in b.values()) / hb; sf = sum(x[0] for x in fz.values()) / hf
    rows.append({**p, "workloads": n, "steps_ratio": v, "ci": [lo, hi], "pass": lo <= p["prediction"] <= hi,
                 "secondary_steps_per_hit_ratio": sf / sb, "hits_base": hb, "hits_fused": hf})
json.dump(rows, sys.stdout, indent=1)
