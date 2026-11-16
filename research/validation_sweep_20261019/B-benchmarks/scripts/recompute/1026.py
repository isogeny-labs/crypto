import json, os
BASE = "/home/user/crypto/research/f4_f5_row_build_20261011/records"
def load(f): return [json.loads(l) for l in open(os.path.join(BASE, f))]
base = load("baseline.jsonl"); cand = load("candidate.jsonl")
comp = json.load(open(os.path.join(BASE, "comparison.json")))
def key(r): return (r["arm"], r["case"], r["repeat"], r.get("seed"), r.get("rank"))
bmap = {key(r): r for r in base}; cmap = {key(r): r for r in cand}
assert len(bmap) == len(base) == 72 and len(cmap) == len(cand) == 72
print("keys matched:", len(set(bmap) & set(cmap)))
for arm in ["f4", "f5", "criterion"]:
    bb = sum(r["allocated_bytes"] for r in base if r["arm"] == arm); cb = sum(r["allocated_bytes"] for r in cand if r["arm"] == arm)
    ba = sum(r["allocations"] for r in base if r["arm"] == arm); ca = sum(r["allocations"] for r in cand if r["arm"] == arm)
    pb = 100 * (bb - cb) / bb; pa = 100 * (ba - ca) / ba
    print(f"arm={arm} baseline_bytes={bb} candidate_bytes={cb} byte_red%={pb:.4f} baseline_allocs={ba} candidate_allocs={ca} alloc_red%={pa:.4f}")
for a in comp["arms"]:
    print("comparison.json", a["arm"], round(a["allocated_byte_reduction_percent"], 4), round(a["allocation_reduction_percent"], 4), a["baseline_allocated_bytes"], a["candidate_allocated_bytes"])
# identical-output check: compare a field only where present in both baseline and candidate for that arm
for field in ["output_digest", "word_ops", "rank", "input_digest"]:
    for arm in ["f4", "f5", "criterion"]:
        n = eq = 0
        for k in bmap:
            if k[0] != arm or k not in cmap:
                continue
            bv, cv = bmap[k].get(field, "<absent>"), cmap[k].get(field, "<absent>")
            if bv == "<absent>" or cv == "<absent>":
                continue
            n += 1; eq += (bv == cv)
        print(f"field={field} arm={arm} n_compared={n} n_equal={eq} n_differ={n-eq}")
