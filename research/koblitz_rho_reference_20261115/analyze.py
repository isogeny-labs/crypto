#!/usr/bin/env python3
"""Reads sessions/*, writes results.json and tables (recorded values + `ecbench compare/band` output only)."""
import json, os, subprocess, math, statistics, glob, sys
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.abspath(os.path.join(HERE, "../.."))
EB = os.environ.get("ECBENCH", os.path.join(ROOT, "target/release/ecbench")); W = ["0", "0.1", "1"]
def run(*a):
    p = subprocess.run([EB, *a], capture_output=True, text=True)
    if p.returncode: raise SystemExit(f"{a}: {p.stderr[-400:]}")
    return p.stdout
def cmp(d, a, b, w):
    j = json.loads(run("compare", "--dir", d, "--a", a, "--b", b, "--weight", w, "--json"))
    o = j["ops"]; return dict(ratio=o["ratio_b_over_a"], lo=o["ci95"][0], hi=o["ci95"][1], pairs=o["pairs"], status=o["status"])
R = {"binary_sha256": None, "curves": {}}
for sd in sorted(glob.glob(os.path.join(HERE, "sessions/n*")), key=lambda s: int(os.path.basename(s)[1:])):
    n = os.path.basename(sd)[1:]
    if not os.path.exists(os.path.join(sd, "session.json")): continue
    ses = json.load(open(os.path.join(sd, "session.json"))); R["binary_sha256"] = ses.get("binary_sha256")
    bpath = os.path.join(HERE, "tables", f"band-n{n}.json"); os.makedirs(os.path.dirname(bpath), exist_ok=True)
    run("band", "--dir", sd, "--json", bpath)
    open(os.path.join(HERE, "tables", f"band-n{n}.txt"), "w").write(run("band", "--dir", sd, "--ranks"))
    band = json.load(open(bpath)); rows = {r["arm"]: r for r in band["rows"]}
    recs = [json.loads(l) for l in open(os.path.join(sd, "records.jsonl"))]; recs = [r for r in recs if not r["warmup"]]
    arms = {}
    for a in rows:
        rr = [r for r in recs if r["arm"] == a]
        tot = statistics.mean(r["cost"]["total_gae"] for r in rr)
        setup = statistics.mean(sum(p["gae"] for p in r["phases"] if p["name"] == "setup") for r in rr)
        nat = statistics.mean(sum(p["gae"] for p in r["phases"] if p["name"] == "natives") for r in rr)
        steps = [r["counters"].get("steps", r["counters"].get("walk_steps")) for r in rr]
        steps = [s for s in steps if s is not None]
        arms[a] = dict(runs=rows[a]["runs"], verified=rows[a]["verified"], mean_S=dict(zip(W, rows[a]["mean_s"])),
                       mean_total_gae_charged=tot, mean_setup_gae=setup, setup_share_of_charged=setup / tot if tot else None,
                       mean_natives_gae_at_recorded_price=nat, median_steps=statistics.median(steps) if steps else None,
                       mean_steps=statistics.mean(steps) if steps else None,
                       uncounted=rows[a].get("uncounted"), unpriced_native_mean_per_run=rows[a]["native_sum"] / rows[a]["runs"])
    slug = recs[0]["workload"]["curve"]["slug"]; r_ = recs[0]["workload"]["curve"]["r"]; A = recs[0]["workload"]["curve"]["automorphisms_available"]
    ideal = math.sqrt(math.pi * r_ / (2 * A)); sqrtr = math.sqrt(r_)
    for a in arms:
        arms[a]["steps_over_ideal_median"] = arms[a]["median_steps"] / ideal if arms[a]["median_steps"] is not None else None
    C = dict(slug=slug, r=r_, log2_r=math.log2(r_), A=A, ideal_steps=ideal, sqrt_r=sqrtr, arms=arms, ratios={})
    def pair(a, b):
        return {w: cmp(sd, a, b, w) for w in W}
    for a, b in [("rho-strong", "rho-v5"), ("rho-strong", "rho-strong-v2"), ("rho-strong", "rho-neg"), ("rho-strong", "rho-strong-aa"), ("rho-v5", "rho-v5-aa"), ("rho-v5", "rho-strong-v2")]:
        if a in rows and b in rows: C["ratios"][f"{b}/{a}"] = pair(a, b)
    for a in rows:
        if a.startswith("gen-") or a.startswith("k1-"):
            for ref in ("rho-strong", "rho-v5", "rho-strong-v2", "rho-neg"):
                C["ratios"][f"{a}/{ref}"] = pair(ref, a)
    R["curves"][n] = C
# fit ln(v5/strong) = alpha + beta log2 r per weight
fit = {}
for w in W:
    xs = []; ys = []
    for n, C in R["curves"].items():
        q = C["ratios"].get("rho-v5/rho-strong"); 
        if q: xs.append(C["log2_r"]); ys.append(math.log(q[w]["ratio"]))
    if len(xs) >= 2:
        mx, my = statistics.mean(xs), statistics.mean(ys); sxx = sum((x - mx) ** 2 for x in xs)
        beta = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx; alpha = my - beta * mx
        res = [y - (alpha + beta * x) for x, y in zip(xs, ys)]
        fit[w] = dict(alpha=alpha, beta=beta, n_curves=len(xs), resid_sd=statistics.pstdev(res) if len(xs) > 2 else None)
R["fit_ln_ratio_v5_over_strong_vs_log2r"] = fit
# derived sensitivity (not a measurement): v5 leaves the per-step jump-index selection untallied, strong tallies it as partition_hashes_uncharged.
# Charging v5 one extra native operation per walk operation: S_v5(w) + w * walk_operations / sqrt(r).
for n, C in R["curves"].items():
    recs = [json.loads(l) for l in open(os.path.join(HERE, f"sessions/n{n}/records.jsonl"))]
    rr = [r for r in recs if r["arm"] == "rho-v5" and not r["warmup"]]
    wo = statistics.mean(r["counters"]["walk_operations"] for r in rr)
    C["v5_hash_counted_derived"] = {w: C["arms"]["rho-v5"]["mean_S"][w] + float(w) * wo / C["sqrt_r"] for w in W}
    C["v5_hash_counted_over_strong_derived"] = {w: C["v5_hash_counted_derived"][w] / C["arms"]["rho-strong"]["mean_S"][w] for w in W}
    C["v5_mean_walk_ops"] = wo
    rs = [r for r in recs if r["arm"] == "rho-strong" and not r["warmup"]]
    C["strong_mean_native_counters"] = {k: statistics.mean(r["counters"].get(k, 0) for r in rs) for k in ("canonicalisations_uncharged", "partition_hashes_uncharged", "table_queries_uncharged", "table_inserts_uncharged", "walk_steps", "scalar_multiplications")}
    C["v5_mean_native_counters"] = {k: statistics.mean(r["counters"].get(k, 0) for r in rr) for k in ("canonicalisations_uncharged", "distinguished_point_table_queries_uncharged", "distinguished_point_table_inserts_uncharged", "steps", "walk_operations", "setup_additions", "setup_doublings", "start_additions", "verification_operations", "jumps")}
    C["setup_gae_mean"] = {a: C["arms"][a]["mean_setup_gae"] for a in ("rho-strong", "rho-v5", "rho-strong-v2")}
# post hoc supplement
SUP = {}
for sd in sorted(glob.glob(os.path.join(HERE, "sessions_supp/n*")), key=lambda s: int(os.path.basename(s)[1:])):
    n = os.path.basename(sd)[1:]
    if not os.path.exists(os.path.join(sd, "session.json")): continue
    run("band", "--dir", sd, "--json", os.path.join(HERE, "tables", f"supp-band-n{n}.json"))
    b = json.load(open(os.path.join(HERE, "tables", f"supp-band-n{n}.json"))); rows = {r["arm"]: r for r in b["rows"]}
    SUP[n] = dict(arms={a: dict(mean_S=dict(zip(W, rows[a]["mean_s"])), verified=rows[a]["verified"], runs=rows[a]["runs"]) for a in rows},
                  ratios={f"{b_}/{a_}": {w: cmp(sd, a_, b_, w) for w in W} for a_, b_ in [("rho-strong", "rho-frob"), ("rho-v5", "rho-frob"), ("rho-strong", "rho-frob-plain"), ("rho-frob", "rho-frob-aa")]
                             if rows[a_]["verified"] == rows[a_]["runs"] and rows[b_]["verified"] == rows[b_]["runs"] and rows[b_]["runs"] > 0})
R["post_hoc_supplement"] = SUP
json.dump(R, open(os.path.join(HERE, "results.json"), "w"), indent=1)
print("wrote results.json", list(R["curves"]))
