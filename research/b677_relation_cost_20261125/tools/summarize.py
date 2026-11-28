#!/usr/bin/env python3
"""Summarize the interleaved panel: counted cost, wall, paired bootstrap ratio intervals."""
import json, glob, random, statistics as st, sys, math
D = sys.argv[1]
reps = sorted({p.split('/')[-1].split('-')[0] for p in glob.glob(f"{D}/r*-baseline.json")})
def load(rep, arm): return json.load(open(f"{D}/{rep}-{arm}.json"))
def cost(d):
    if "charged_group_work" in d:  # baseline schema
        c = d["charged_group_work"]; fx = d["fixture_work"]
        return dict(M=c["fp_multiplications"] + fx["fp_multiplications"], I=c["fp_inversions"] + fx["fp_inversions"],
                    pts=d["total_group_additions"], probes=d["probe_additions"], wall=d["cold_wall_ms"],
                    queries=len(d["queries"]), rels=len(d["relation_rows"]), scalar=d["recovered_scalar"])
    if d.get("policy", "").startswith("equal-engineering"):
        c = d["charged"]
        return dict(M=c["total_fp_multiplications_incl_inversions"], I=c["batched_fp_inversions"] + c["setup_group_law"]["fp_inversions"],
                    pts=c["total_points_computed"], probes=0, wall=d["cold_wall_ms"], queries=0, rels=0, scalar=d["recovered_scalar"])
    if "attempts" in d:  # rho schema
        a = d["attempts"][0]["charged_ops"]
        return dict(M=a["fp_multiplications"], I=a["fp_inversions"], pts=a["group_additions"], probes=0,
                    wall=int(d["total_cold_ns"]) / 1e6, queries=0, rels=0, scalar=d["attempts"][0]["recovered_scalar"])
    c = d["charged"]
    return dict(M=c["total_fp_multiplications_incl_inversions"], I=c["total_fp_inversions"], pts=c["total_points_computed"],
                probes=d["probe_points"], wall=d["cold_wall_ms"], queries=d["queries_used"], rels=d["relations_emitted"],
                scalar=d["recovered_scalar"])
arms = ["baseline", "exact", "s26", "s13", "rho", "rhob"]
data = {a: [cost(load(r, a)) for r in reps] for a in arms}
data["rhob"] = [cost(json.load(open(p))) for p in sorted(glob.glob(f"{D}/*-rhob.json"))]
assert all(x["scalar"] == 987654321 for a in arms for x in data[a]), "every arm must recover the scalar"
def boot(num, den, key, B=20000, seed=1):
    rng = random.Random(seed); n = len(num)
    ratios = [num[i][key] / den[i][key] for i in range(n)]
    gm = lambda xs: math.exp(sum(map(math.log, xs)) / len(xs))
    bs = sorted(gm([ratios[rng.randrange(n)] for _ in range(n)]) for _ in range(B))
    return gm(ratios), bs[int(0.025 * B)], bs[int(0.975 * B)]
out = {"reps": len(reps), "arms": {}, "ratios": {}}
for a in arms:
    xs = data[a]
    out["arms"][a] = {k: {"median": st.median(x[k] for x in xs), "min": min(x[k] for x in xs), "max": max(x[k] for x in xs)}
                      for k in ["M", "I", "pts", "probes", "wall", "queries", "rels"]}
for a in ["exact", "s26", "s13"]:
    for key in ["M", "wall", "pts"]:
        out["ratios"][f"baseline/{a}:{key}"] = boot(data["baseline"], data[a], key)
def boot_unpaired(num, den, key, B=20000, seed=2):
    rng = random.Random(seed)
    mean = lambda xs: sum(xs) / len(xs)
    point = mean([x[key] for x in num]) / mean([x[key] for x in den])
    bs = sorted(mean([num[rng.randrange(len(num))][key] for _ in num]) /
                mean([den[rng.randrange(len(den))][key] for _ in den]) for _ in range(B))
    return point, bs[int(0.025 * B)], bs[int(0.975 * B)]
for a in ["baseline", "exact", "s26", "s13"]:
    for key in ["M", "wall", "pts"]:
        out["ratios"][f"{a}/rho_frozen:{key}"] = boot(data[a], data["rho"], key)
        out["ratios"][f"{a}/rho_equal_engineering_mean:{key}"] = boot_unpaired(data[a], data["rhob"], key)
out["rhob_seeds"] = len(data["rhob"])
out["arms"]["rhob"]["mean"] = {k: sum(x[k] for x in data["rhob"]) / len(data["rhob"]) for k in ["M", "pts", "wall"]}
print(json.dumps(out, indent=1))
