import json, os
ROOT = "/home/user/crypto/research/pdp_shape_replay_20261015"
def load(p):
    with open(os.path.join(ROOT, p)) as f:
        return json.load(f)
pairs = {
    "chain": ("raw/chain-default/stage.json", "raw/chain-replay/stage.json"),
    "chain-holdout": ("raw/chain-holdout-default/stage.json", "raw/chain-holdout-replay/stage.json"),
    "holdout": ("raw/holdout-default/stage.json", "raw/holdout-replay/stage.json"),
    "frozen-pilot": ("pilot/frozen-default/stage.json", "pilot/frozen-replay/stage.json"),
}
out = {}
cell_ratios = {}
n_eq = n_diff = n_cmp = 0
for lad, (pd, pr) in pairs.items():
    d, r = load(pd), load(pr)
    key = lambda row: (row["curve"], row["m"], row["first_target"], row["factor_index"])
    dr = {key(x): x for x in d["rows"]}
    rr = {key(x): x for x in r["rows"]}
    tot_d = sum(x["word_ops"] for x in d["rows"])
    tot_r = sum(x["word_ops"] for x in r["rows"])
    out[lad] = {"pooled_default_over_replay": tot_d / tot_r, "total_default": tot_d, "total_replay": tot_r, "cells": len(d["rows"])}
    cell_ratios[lad] = []
    for k in dr:
        if k not in rr:
            print("unmatched cell", lad, k); continue
        cell_ratios[lad].append(dr[k]["word_ops"] / rr[k]["word_ops"])
        n_cmp += 1
        if dr[k]["verdict_digest"] == rr[k]["verdict_digest"]:
            n_eq += 1
        else:
            n_diff += 1
    print(lad, "cell ratios", [round(x, 4) for x in cell_ratios[lad]])
print("pooled", {k: round(v["pooled_default_over_replay"], 6) for k, v in out.items()})
allc = [x for v in cell_ratios.values() for x in v if v is not None]
print("headline-set cells (chain, chain-holdout, holdout) n =", sum(len(cell_ratios[l]) for l in ["chain", "chain-holdout", "holdout"]))
hs = [x for l in ["chain", "chain-holdout", "holdout"] for x in cell_ratios[l]]
print("cell ratio min/max over 9 headline cells", round(min(hs), 4), round(max(hs), 4))
print("verdict digests compared", n_cmp, "equal", n_eq, "differ", n_diff)
# e3 vs table
e3 = [json.loads(l) for l in open(os.path.join(ROOT, "raw/e3_vs_table.jsonl")) if l.strip()]
ratios = [x["e3_over_table"] for x in e3]
print("e3 rows", len(e3), "e3_over_table min/max", round(min(ratios), 4), round(max(ratios), 4))
print("e3 cell keys", sorted(e3[0].keys()))
print("e3 targets per row", sorted(set(x["targets"] for x in e3)), "ns/ells", sorted(set((x["n"], x["l"]) for x in e3)))
