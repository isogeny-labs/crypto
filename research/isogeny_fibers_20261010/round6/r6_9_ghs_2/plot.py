"""Plots recorded values only (walk.json, relrate.json) -> fig*.svg, figure_data.tsv, summary.json."""
import json, math, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = os.path.dirname(os.path.abspath(__file__))
walk = json.load(open(os.path.join(D, "walk.json")))
rel = json.load(open(os.path.join(D, "relrate.json")))
summary = {"walk": [], "relrate": []}
tsv = ["# walk: N\tclass_size\tisogeny_evals\texpanded\tpath"]
fig, ax = plt.subplots(figsize=(6.4, 4.4))
cols = {12: "#1f77b4", 14: "#ff7f0e", 15: "#2ca02c", 16: "#d62728", 18: "#9467bd"}
allx, ally = [], []
for x in walk:
    n = x["N"]
    recs = x["models"]["LG5"]["search_records"]
    ok = [r for r in recs if r["path"] is not None]
    if not ok:
        continue
    xs = [r["class_size"] for r in ok]
    ys = [r["isogeny_evals"] for r in ok]
    for r in ok:
        tsv.append(f"{n}\t{r['class_size']}\t{r['isogeny_evals']}\t{r['expanded']}\t{r['path']}")
    allx += xs; ally += ys
    ax.scatter(xs, ys, s=6, alpha=0.35, color=cols[n], label=f"N={n}")
    rat = [y / (2 * math.sqrt(s)) for s, y in zip(xs, ys)]
    rat_lin = [y / s for s, y in zip(xs, ys)]
    rs = sorted(rat)
    summary["walk"].append({
        "N": n, "starts": len(recs), "found": len(ok), "unreachable": len(recs) - len(ok),
        "mean_evals": sum(ys) / len(ys), "max_evals": max(ys),
        "mean_ratio_to_2sqrt": sum(rat) / len(rat), "median_ratio_to_2sqrt": rs[len(rs) // 2],
        "p90_ratio_to_2sqrt": rs[int(0.9 * len(rs))], "max_ratio_to_2sqrt": rs[-1],
        "frac_above_2sqrt": sum(r > 1 for r in rat) / len(rat),
        "mean_evals_over_class_size": sum(rat_lin) / len(rat_lin),
        "median_class_size": sorted(xs)[len(xs) // 2],
    })
# least-squares slope of log evals vs log class size (all found searches)
lx = [math.log(v) for v in allx]; ly = [math.log(v) for v in ally]
mx, my = sum(lx) / len(lx), sum(ly) / len(ly)
slope = sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)
summary["walk_loglog_slope_evals_vs_class_size"] = slope
grid = sorted(set(allx))
ax.plot(grid, [2 * math.sqrt(s) for s in grid], "k-", lw=1.2, label="2·(class size)^½")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("isogeny class size (curves)"); ax.set_ylabel("isogeny evaluations to reach LG5 target")
ax.set_title(f"R6.9 walk cost vs class size (measured; log-log slope {slope:.2f})")
ax.legend(fontsize=7, markerscale=2); fig.tight_layout()
fig.savefig(os.path.join(D, "fig1_walk_cost_vs_class_size.svg")); plt.close(fig)

tsv.append("# relrate: l\tgenus\tseed\tjac_order\tmeasured\tse\tmodel_exact\tmodel_inv_gfact\tvisited_frac")
fig, ax = plt.subplots(figsize=(6.4, 4.4))
mk = {2: "o", 3: "s", 4: "^", 5: "D"}
for r in rel:
    tsv.append(f"{r['l']}\t{r['genus']}\t{r['seed']}\t{r['jacobian_order_exact']}\t{r['measured_rate']}\t{r['measured_rate_se']}\t{r['model_exact_split_fraction']}\t{r['model_heuristic_inv_gfact']}\t{r['visited_fraction_of_jacobian']}")
    summary["relrate"].append({k: r[k] for k in ["l", "genus", "seed", "jacobian_order_exact", "measured_rate", "model_exact_split_fraction", "model_heuristic_inv_gfact", "ratio_measured_over_exact", "ratio_measured_over_heuristic", "visited_fraction_of_jacobian", "order_check_failures"]})
for l in mk:
    rr = [r for r in rel if r["l"] == l]
    ax.scatter([r["model_heuristic_inv_gfact"] for r in rr], [r["measured_rate"] for r in rr], marker=mk[l], facecolors="none", edgecolors="C3", label=f"vs 1/g!  q=2^{l}")
    ax.scatter([r["model_exact_split_fraction"] for r in rr], [r["measured_rate"] for r in rr], marker=mk[l], color="C0", s=14, label=f"vs exact split count  q=2^{l}")
g = [1e-5, 1]
ax.plot(g, g, "k-", lw=1); ax.plot(g, [4 * v for v in g], "k:", lw=0.8); ax.plot(g, [10 * v for v in g], "k--", lw=0.8)
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlim(1e-5, 1.2); ax.set_ylim(1e-3, 1.2)
ax.set_xlabel("modelled smooth (fully split) fraction"); ax.set_ylabel("measured rate on random walk")
ax.set_title("R6.9 relation rate: measured vs model (lines: 1×, 4×, 10×)")
ax.legend(fontsize=6, ncol=2); fig.tight_layout()
fig.savefig(os.path.join(D, "fig2_relation_rate_measured_vs_model.svg")); plt.close(fig)
open(os.path.join(D, "figure_data.tsv"), "w").write("\n".join(tsv) + "\n")
json.dump(summary, open(os.path.join(D, "summary.json"), "w"), indent=1)
print(json.dumps(summary["walk"], indent=1)); print("slope", slope)
