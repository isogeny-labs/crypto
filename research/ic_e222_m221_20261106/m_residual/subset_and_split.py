#!/usr/bin/env python3
"""Two diagnostics on the residual (unvalidated, pre-gate).  Reads results_residual_instances.json (per-instance
4-seed residuals of the 600 new Montgomery + 200 Edwards instances), results_old_M_all_instances.json (the 24 old
Montgomery instances) and the row files.
 (1) Subset test: draw 20000 random stratified subsets of 24 instances (8 each at rungs 12, 16, 20) from the new
     Montgomery sample, look at the distribution of the subset mean residual and where the old 24's +7.3% falls.
 (2) Split-half: per-instance residual from seeds (1,2) against seeds (3,4); correlation over instances, for the
     new 600 and for the old 24.  An instance-level effect would give a positive correlation.
Usage: subset_and_split.py OUT.json"""
import json, math, random, statistics, collections, sys
new = [r for r in json.load(open("results_residual_instances.json")) if r["family"] == "M"]
old = json.load(open("results_old_M_all_instances.json"))
old_mean = statistics.mean([r["resid_log"] for r in old])
byr = collections.defaultdict(list)
for r in new:
    byr[r["rung"]].append(r["resid_log"])
rng = random.Random("subset-v1")
means = []
for _ in range(20000):
    sub = []
    for rung in (12, 16, 20):
        sub += rng.sample(byr[rung], 8)
    means.append(statistics.mean(sub))
means.sort()
frac_ge = sum(1 for m in means if m >= old_mean) / len(means)
out = {"old24_mean_log_resid_4seeds": old_mean, "subset_mean_p2.5": means[500], "subset_mean_p50": means[10000], "subset_mean_p97.5": means[19500],
       "subset_sd": statistics.pstdev(means), "frac_subsets_ge_old": frac_ge}

def split(rows_file, model_file):
    rows = [json.loads(l) for l in open(rows_file)]
    model = {r["name"]: r for r in json.load(open(model_file))}
    obs = collections.defaultdict(lambda: collections.defaultdict(dict))
    for r in rows:
        obs[r["name"]][r["seed"]][r["arm"]] = r["trials"]
    xs, ys = [], []
    for name, o in obs.items():
        m = model[name]
        if m["family"] != "M":
            continue
        res = []
        for seeds in ((1, 2), (3, 4)):
            on = sum(o[s]["on"] for s in seeds); off = sum(o[s]["off"] for s in seeds)
            K = len(seeds) * 34
            res.append(math.log(on / off) - math.log(m["P_off"] / m["P_on"]) - (m["P_on"] - m["P_off"]) / (2 * K))
        xs.append(res[0]); ys.append(res[1])
    mx, my = statistics.mean(xs), statistics.mean(ys)
    cov = sum((a - mx) * (b - my) for a, b in zip(xs, ys)) / len(xs)
    corr = cov / (statistics.pstdev(xs) * statistics.pstdev(ys))
    # permutation null for the correlation
    prng = random.Random("perm"); ge = 0
    for _ in range(4000):
        zs = ys[:]; prng.shuffle(zs)
        mz = statistics.mean(zs)
        c = sum((a - mx) * (b - mz) for a, b in zip(xs, zs)) / len(xs) / (statistics.pstdev(xs) * statistics.pstdev(zs))
        ge += c >= corr
    return dict(n=len(xs), mean_s12=mx, mean_s34=my, corr=corr, perm_p_one_sided=ge / 4000)
out["split_half_new600"] = split("rows.jsonl", "model.json")
out["split_half_old24"] = split("rows_old_M.jsonl", "model_old_M.json")
json.dump(out, open(sys.argv[1], "w"), indent=1)
print(json.dumps(out, indent=1))
