#!/usr/bin/env python3
"""Panel summary: cold time to verified recovery and useful rank/s per arm, ratio intervals."""
import json, glob, random, statistics as st, sys, math
D = sys.argv[1]
arms = {}
for p in sorted(glob.glob(f"{D}/r*-*.json")):
    if p.endswith(".meta.json"): continue
    arm = p.split("/")[-1].split("-", 1)[1][:-5]
    d = json.load(open(p))
    if arm == "ref":
        rec = dict(cold_ms=d["cold_wall_ms"], rank=d["lp_rank"] + d["factor_base_target_rank"], queries=d["queries_used"],
                   recovered=d["recovered_scalar"], build_ms=d["table_ms"], source_ms=d["source_ms"], consumer_ms=d["graph_and_replay_ms"])
    elif arm == "rho":
        rec = dict(cold_ms=d["cold_wall_ms"], rank=0, queries=0, recovered=d["recovered_scalar"], build_ms=0, source_ms=d["cold_wall_ms"], consumer_ms=0,
                   points=d["charged"]["total_points_computed"])
    else:
        rec = dict(cold_ms=d["cold_ms"], rank=d["rank"], queries=d["queries"], recovered=d["recovered"], build_ms=d["build_ms"],
                   source_ms=d["probe_ms"], consumer_ms=d["elim_ms"], probes=d["probes"], digest=d["stream_digest"])
    rec["rank_per_s"] = rec["rank"] / (rec["cold_ms"] / 1e3)
    arms.setdefault(arm, []).append(rec)
out = {"arms": {}, "ratios": {}}
for a, xs in arms.items():
    out["arms"][a] = {"n": len(xs), "all_recovered_987654321": all(x["recovered"] == 987654321 for x in xs),
                      **{k: st.median(x[k] for x in xs) for k in xs[0] if k not in ("recovered", "digest")}}
    if "digest" in xs[0]:
        out["arms"][a]["distinct_digests"] = len({x["digest"] for x in xs})
def boot(num, den, B=20000, seed=5, paired=True):
    rng = random.Random(seed); gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
    if paired:
        rs = [x["cold_ms"] / y["cold_ms"] for x, y in zip(num, den)]
        pt = gm(rs); bs = sorted(gm([rng.choice(rs) for _ in rs]) for _ in range(B))
    else:
        pt = gm([x["cold_ms"] for x in num]) / gm([x["cold_ms"] for x in den])
        bs = sorted(gm([rng.choice(num)["cold_ms"] for _ in num]) / gm([rng.choice(den)["cold_ms"] for _ in den]) for _ in range(B))
    return [pt, bs[int(.025 * B)], bs[int(.975 * B)]]
for a, b in [("ref", "pt1"), ("default", "pt1"), ("pt1", "pt4"), ("ref", "pt4"), ("default", "pt4")]:
    out["ratios"][f"{a}/{b}:cold_ms"] = boot(arms[a], arms[b])
for a in ("pt1", "pt4", "ref", "default"):
    out["ratios"][f"{a}/rho:cold_ms (unpaired, rho random seeds)"] = boot(arms[a], arms["rho"], paired=False)
# pt1 and pt4 with the same salt must give the same stream
out["pt1_pt4_same_stream_per_salt"] = all(x["digest"] == y["digest"] for x, y in zip(arms["pt1"], arms["pt4"]))
print(json.dumps(out, indent=1))
