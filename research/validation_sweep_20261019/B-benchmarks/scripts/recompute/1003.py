import json, math, statistics as st
R = "research/lit2026_w12_flagged_claims_20261011/reports/run-20261012T0009Z"
s = json.load(open(R + "/scaling.json"))
runs = s["runs"]
rho = [r for r in runs if r["method"] == "rho.negation"]
zm = [r for r in runs if r["method"].startswith("zero_minor")]
print("counts rho", len(rho), "zm", len(zm), "zm search_ops present", sum(1 for r in zm if r.get("search_ops")))
def ols(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    return b, n
# slope: search-phase zero-minor ops (field ops = muls+invs; search_ops as stored)
zs = [r for r in zm if r.get("search_ops")]
b, n = ols([r["log2_r"] for r in zs], [math.log2(r["search_ops"]) for r in zs])
print("OLS slope log2(search_ops) vs log2 r, n=%d: %.6f  (stored search-phase slope 0.904887, t-CI 0.745655-1.064119)" % (n, b))
zall = zm
b2, n2 = ols([r["log2_r"] for r in zall], [math.log2(r["ops"]) for r in zall])
print("OLS slope log2(ops) all zm runs n=%d: %.6f (stored 0.783045)" % (n2, b2))
zs3 = [r for r in zm if r["method"].endswith("d=3)")]
b3, n3 = ols([r["log2_r"] for r in zs3], [math.log2(r["ops"]) for r in zs3])
print("OLS slope d=3 all phases n=%d: %.6f (stored 0.631113)" % (n3, b3))
rb, nr = ols([r["log2_r"] for r in rho], [math.log2(r["ops"]) for r in rho])
print("rho ops slope n=%d: %.6f (stored 0.377192)" % (nr, rb))
# paired ratios: candidate pairing rules (study does not state the rho reduction per target)
for bits in sorted({r["bits"] for r in runs}):
    zb = [r for r in zm if r["bits"] == bits]
    cands = {"rho_median_over_seeds": [], "rho_mean_over_seeds": []}
    for z in zb:
        rs = [r["ops"] for r in rho if r["bits"] == bits and r["target"] == z["target"]]
        cands["rho_median_over_seeds"].append(z["ops"] / st.median(rs))
        cands["rho_mean_over_seeds"].append(z["ops"] / st.mean(rs))
    out = {k: round(math.exp(sum(math.log(x) for x in v) / len(v)), 3) for k, v in cands.items()}
    print("bits", bits, "targets", len(zb), "geomean candidates", out, "stored", [p["ratio_zero_minor_over_rho_geomean"] for p in s["paired_ratio"] if p["bits"] == bits])
