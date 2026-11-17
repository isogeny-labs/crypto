"""Score spec-p36.json's session against PREDICTIONS.json.

For every arm: mean counted GAE per solve over non-warm-up records, divided by
the ideal negation-map rho cost sqrt(pi r / 4), with a cluster bootstrap over
workloads.  For res1 also the walk-step group operations alone (F1), and for
the residual arms the table hits per solve."""
import json, math, random, sys
from collections import defaultdict

recs = [json.loads(l) for l in open(sys.argv[1])]
P = json.load(open(sys.argv[2]))
ideal = P["ideal_rho_adds"]
arms = defaultdict(lambda: defaultdict(list))
for r in recs:
    if r.get("warmup"):
        continue
    nat = {}
    for p in r["phases"]:
        nat.update(p.get("native", {}))
    hits = nat.get("residual_collisions", 0) + nat.get("residual_direct_hits", 0)
    arms[r["arm"]][r["workload"]["workload_id"]].append(
        (r["cost"]["total_gae"], nat.get("residual_walk_step_group_ops"), hits, r["outcome"].get("verified") if isinstance(r.get("outcome"), dict) else None))

def boot(per_w, f, B=4000, seed=5):
    ws = sorted(per_w); rng = random.Random(seed)
    def stat(sample):
        vals = [f(x) for w in sample for x in per_w[w] if f(x) is not None]
        return sum(vals) / len(vals) / ideal
    pt = stat(ws); out = sorted(stat([ws[rng.randrange(len(ws))] for _ in ws]) for _ in range(B))
    return pt, out[int(.025 * B)], out[int(.975 * B)]

res = {}
for arm, per_w in arms.items():
    n = sum(len(v) for v in per_w.values())
    row = {"runs": n, "workloads": len(per_w), "gae_over_ideal": boot(per_w, lambda x: x[0])}
    if arm.startswith("res"):
        row["walk_ops_over_ideal"] = boot(per_w, lambda x: x[1])
        hs = [x[2] for v in per_w.values() for x in v]
        row["hits_per_solve"] = {h: hs.count(h) for h in sorted(set(hs))}
    res[arm] = row
json.dump(res, sys.stdout, indent=1)

# Paired ratio of two arms' mean GAE, cluster bootstrap over workloads.
def paired(a, b, B=4000, seed=9):
    ws = sorted(set(arms[a]) & set(arms[b])); rng = random.Random(seed)
    def st(s):
        return sum(x[0] for w in s for x in arms[a][w]) / sum(x[0] for w in s for x in arms[b][w])
    out = sorted(st([ws[rng.randrange(len(ws))] for _ in ws]) for _ in range(B))
    return st(ws), out[int(.025 * B)], out[int(.975 * B)]
print(json.dumps({"res1eq/rho": paired("res1eq", "rho"), "res1/rho": paired("res1", "rho"), "res1/res1eq": paired("res1", "res1eq")}), file=sys.stderr)
