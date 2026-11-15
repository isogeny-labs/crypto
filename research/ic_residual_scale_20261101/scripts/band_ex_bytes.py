#!/usr/bin/env python3
"""Pricing sensitivity (written after the independent validator's finding F1): the band as built prices
`residual_table_bytes_uncharged` (a BYTE count) as operations at weight w. This recomputes paired ratios
(b/a, ratio of totals over (workload, round) pairs, measured runs) at w = 0, 0.1, 1 with and without that counter.
Usage: band_ex_bytes.py SESSION_DIR ARM_A ARM_B [...]   (pairs given as a:b)"""
import json, sys, collections
def load(d):
    rec = []
    for l in open(d + "/records.jsonl"):
        x = json.loads(l)
        if x["warmup"]: continue
        un = 0; by = 0
        for name in x["cost"]["unpriced"]:
            base = name[:-10] if name.endswith("_uncharged") else name
            if name in x["counters"]: v = x["counters"][name]
            else: v = sum(p["native"].get(name, p["native"].get(base, 0)) for p in x["phases"])
            if name == "residual_table_bytes_uncharged": by += v
            else: un += v
        rec.append((x["arm"], (x["workload"]["workload_id"], x["round"]), x["cost"]["total_gae"], un, by))
    return rec
def ratio(rec, a, b, w, ex):
    A = {k: (g, u, y) for arm, k, g, u, y in rec if arm == a}; B = {k: (g, u, y) for arm, k, g, u, y in rec if arm == b}
    ks = sorted(set(A) & set(B))
    cost = lambda t: t[0] + w * (t[1] + (0 if ex else t[2]))
    return sum(cost(B[k]) for k in ks) / sum(cost(A[k]) for k in ks)
if __name__ == "__main__":
    d = sys.argv[1]; rec = load(d); out = {}
    for pair in sys.argv[2:]:
        a, b = pair.split(":")
        out[pair] = {"as_built": [ratio(rec, a, b, w, False) for w in (0, 0.1, 1)], "bytes_excluded": [ratio(rec, a, b, w, True) for w in (0, 0.1, 1)]}
        print(d.split("/")[-1], pair, "as built", ["%.3f" % t for t in out[pair]["as_built"]], "| bytes excluded", ["%.3f" % t for t in out[pair]["bytes_excluded"]])
    json.dump(out, open(d.rstrip("/").split("/")[-1] + "-band-ex-bytes.json", "w"), indent=1)
