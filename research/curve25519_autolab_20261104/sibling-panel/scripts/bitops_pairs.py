#!/usr/bin/env python3
"""Paired bit-op ratios for the sibling-points panel (frozen instrument, PROTOCOL.md section 2).

Usage: bitops_pairs.py <session_dir> <A> <B> [more A B pairs...]
Per record: work.bitops / sqrt(r) (bitop/v1, as written on every record) and cost.s (GAE, w = 0).
Pairs runs by (workload_id, round); ratio = sum(B) / sum(A); 95 percent percentile bootstrap over
workload blocks (each block carries its paired rounds), 4000 resamples, seed 20261121. Warm-ups skipped,
only records whose outcome status is 'verified' are used, and the count of dropped records is reported.
"""
import json, math, random, sys
from collections import defaultdict

def load(d):
    recs = defaultdict(dict)  # (wid, round) -> arm -> (bits_per_sqrt_r, s_gae, verified)
    dropped = 0
    for line in open(f"{d}/records.jsonl"):
        r = json.loads(line)
        if r.get("warmup"):
            continue
        ok = r["outcome"]["status"] == "verified"
        if not ok:
            dropped += 1
            continue
        rr = r["workload"]["curve_spec"]["r"]
        key = (r["workload"]["workload_id"], r["round"])
        recs[key][r["arm"]] = (int(r["work"]["bitops"]) / math.sqrt(rr), r["cost"]["s"])
    return recs, dropped

def ratio(recs, a, b, idx):
    keys = [k for k in recs if a in recs[k] and b in recs[k]]
    blocks = defaultdict(list)
    for k in keys:
        blocks[k[0]].append(k)
    wids = sorted(blocks)
    def summ(sel):
        sa = sb = ga = gb = 0.0
        for w in sel:
            for k in blocks[w]:
                sa += recs[k][a][idx]; sb += recs[k][b][idx]
        return sb / sa
    point = summ(wids)
    rng = random.Random(20261121)
    boots = []
    for _ in range(4000):
        sel = [wids[rng.randrange(len(wids))] for _ in wids]
        boots.append(summ(sel))
    boots.sort()
    return point, boots[int(0.025 * 4000)], boots[int(0.975 * 4000) - 1], len(keys), len(wids)

def main():
    d = sys.argv[1]
    recs, dropped = load(d)
    arms = sorted({a for v in recs.values() for a in v})
    out = {"session": d, "dropped_unverified": dropped, "means": {}, "pairs": []}
    for a in arms:
        vals = [v[a] for v in recs.values() if a in v]
        out["means"][a] = {"bitops_per_sqrt_r": sum(x[0] for x in vals) / len(vals), "s_gae_w0": sum(x[1] for x in vals) / len(vals), "n": len(vals)}
    pairs = sys.argv[2:]
    for a, b in zip(pairs[::2], pairs[1::2]):
        pb, lo, hi, n, nw = ratio(recs, a, b, 0)
        pg, glo, ghi, _, _ = ratio(recs, a, b, 1)
        out["pairs"].append({"a": a, "b": b, "bitops_ratio": pb, "bitops_ci95": [lo, hi], "gae_w0_ratio": pg, "gae_w0_ci95": [glo, ghi], "pairs": n, "workloads": nw})
    print(json.dumps(out, indent=1))

if __name__ == "__main__":
    main()
