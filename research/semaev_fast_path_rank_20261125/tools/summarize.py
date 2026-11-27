#!/usr/bin/env python3
"""Summarize the panel: useful rank per second to verified recovery, per arm; ratio intervals vs the reference."""
import json, glob, random, statistics as st, sys, math
D = sys.argv[1]
def load(p): return json.load(open(p))
arms = {}
for p in sorted(glob.glob(f"{D}/r*-*.json")):
    if p.endswith(".meta.json"): continue
    arm = p.split("/")[-1].split("-", 1)[1][:-5]
    d = load(p)
    if arm == "ref":
        rank = d["lp_rank"] + d["factor_base_target_rank"]
        rec = dict(cold_ms=d["cold_wall_ms"], rank=rank, queries=d["queries_used"], relations=d["relations_emitted"],
                   recovered=d["recovered_scalar"], build_ms=d["table_ms"], query_ms=d["source_ms"])
    else:
        rec = dict(cold_ms=d["cold_ms"], rank=d["total_rank"], queries=d["queries"], relations=d["distinct_relations"],
                   recovered=d["recovered_scalar"], build_ms=d["build_ms"], query_ms=d["query_ms_total"],
                   query_ms_mean=d["query_ms_mean"], dup=d["duplicate_relations"], unliftable=d["unliftable_witnesses"],
                   empty=d["empty_queries"], finite_support=d["finite_support"])
    rec["rank_per_s"] = rec["rank"] / (rec["cold_ms"] / 1e3)
    rec["rank_per_query"] = rec["rank"] / rec["queries"]
    arms.setdefault(arm, []).append(rec)
out = {"arms": {}, "ratios_vs_ref": {}}
for a, xs in arms.items():
    out["arms"][a] = {"n": len(xs), "all_recovered_987654321": all(x["recovered"] == 987654321 for x in xs),
                      **{k: st.median(x[k] for x in xs) for k in xs[0] if k != "recovered"}}
def boot(num, den, B=20000, seed=3):
    rng = random.Random(seed); gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
    pt = gm([x["cold_ms"] for x in num]) / gm([x["cold_ms"] for x in den])
    bs = sorted(gm([rng.choice(num)["cold_ms"] for _ in num]) / gm([rng.choice(den)["cold_ms"] for _ in den]) for _ in range(B))
    return [pt, bs[int(.025 * B)], bs[int(.975 * B)]]
for a in arms:
    if a != "ref" and all(x["recovered"] for x in arms[a]):
        out["ratios_vs_ref"][f"{a}/ref:cold_ms_to_recovery"] = boot(arms[a], arms["ref"])
print(json.dumps(out, indent=1))
