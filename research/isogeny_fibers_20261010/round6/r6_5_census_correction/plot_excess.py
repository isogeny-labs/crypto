"""Plot recorded values only (no computation beyond reading them).

Reads census_p18_sizes.json (examples/r6_5_census_correction.rs `census`) and
reanalysis.json (`reanalyse`), copies the plotted rows to plot_data.json and
draws excess_vs_inv_size.svg: the excess of the zeta-closed base over the
plain base, ratio - 1, against 1/|F|, with the two candidate corrections.
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
cen = json.load(open(os.path.join(HERE, "census_p18_sizes.json")))
rea = json.load(open(os.path.join(HERE, "reanalysis.json")))

rows = []
for s in cen["summary"]:
    rows.append({k: s[k] for k in ("size", "n_reps", "inv_size", "predicted_a6", "predicted_a3",
                                    "derived_h2_ratio", "derived_h3_ratio", "derived_single_sd",
                                    "d2", "h2", "d3", "h3", "mean_hit_rate3")})
rec = [dict(file=os.path.basename(r["file"]), p=r["p"], ratio_c2=r["ratio_c2"], ratio_c3=r["ratio_c3"],
            ratio_h2_from_compositions=r["ratio_h2_from_compositions"],
            derived_single_sd=r["derived_single_sd"]) for r in rea["records"]]
json.dump(dict(census=rows, recorded=rec, p=cen["p"], b=cen["b"], r=cen["r"]),
          open(os.path.join(HERE, "plot_data.json"), "w"), indent=1)

C1, C2, C3, C4 = "#2a78d6", "#eb6834", "#1baf7a", "#6250d6"
fig, axes = plt.subplots(1, 2, figsize=(11, 4.6))
xs = [r["inv_size"] for r in rows]
grid = [0, max(xs) * 1.08]

ax = axes[0]
ax.plot(grid, [4 * g for g in grid], color=C4, lw=2, label="1 + (h−1)(|A|−2)/(2|F|), |A| = 6: slope 4 (derived)")
ax.plot(grid, [1 * g for g in grid], color=C4, lw=1.2, ls="--", label="same with |A| = 3: slope 1 (derived)")
for key, col, lab, dx in (("h2", C1, "H₂ ratio − 1 (pair hits, collision-free)", -0.0004),
                          ("d2", C2, "D₂ ratio − 1 (distinct targets, §15.2 definition)", 0.0004)):
    y = [r[key]["value"] - 1 for r in rows]
    lo = [r[key]["value"] - r[key]["ci95"][0] for r in rows]
    hi = [r[key]["ci95"][1] - r[key]["value"] for r in rows]
    ax.errorbar([x + dx for x in xs], y, yerr=[lo, hi], fmt="o", color=col, ms=6, capsize=3, lw=1.5, label=lab)
ax.scatter(xs, [r["d2"]["smallx"] - 1 for r in rows], marker="s", facecolors="none", edgecolors=C2, s=60,
           label="D₂ ratio − 1, the single small-x base (round-3 construction)")
ax.scatter([1 / 72] * len(rec), [r["ratio_c2"] - 1 for r in rec], marker="x", color="#52514e", s=50,
           label="round-3 recorded censuses, |F| = 72 (one base each)")
ax.axhline(0, color="#999", lw=0.8)
ax.set_xlabel("1 / |F|")
ax.set_ylabel("ζ-closed / plain − 1,  m = 2")
ax.set_title(f"m = 2, p = {cen['p']}, {rows[0]['n_reps']} random bases per size", fontsize=9)
ax.set_xlim(grid)
ax.legend(fontsize=7, loc="upper left")
ax.grid(alpha=0.25)

ax = axes[1]
y = [r["h3"]["value"] - 1 for r in rows]
lo = [r["h3"]["value"] - r["h3"]["ci95"][0] for r in rows]
hi = [r["h3"]["ci95"][1] - r["h3"]["value"] for r in rows]
ax.errorbar(xs, y, yerr=[lo, hi], fmt="o", color=C1, ms=6, capsize=3, lw=1.5, label="H₃ ratio − 1 (triple hits, collision-free)")
ax.plot(xs, [r["derived_h3_ratio"] - 1 for r in rows], color=C4, lw=2, marker="_", ms=14, ls=":",
        label="composition model E[g₃] ratio (derived)")
y = [r["d3"]["value"] - 1 for r in rows]
lo = [r["d3"]["value"] - r["d3"]["ci95"][0] for r in rows]
hi = [r["d3"]["ci95"][1] - r["d3"]["value"] for r in rows]
ax.errorbar([x + 0.0004 for x in xs], y, yerr=[lo, hi], fmt="o", color=C2, ms=6, capsize=3, lw=1.5,
            label="D₃ ratio − 1 (distinct targets; saturated at this r)")
for r in rows:
    ax.annotate(f"hit rate {r['mean_hit_rate3'][0]:.2f}", (r["inv_size"] + 0.0004, r["d3"]["value"] - 1),
                textcoords="offset points", xytext=(6, -12), fontsize=7, color="#52514e")
ax.scatter([1 / 72] * len(rec), [r["ratio_c3"] - 1 for r in rec], marker="x", color="#52514e", s=50,
           label="round-3 recorded censuses, m = 3")
ax.axhline(0, color="#999", lw=0.8)
ax.set_xlabel("1 / |F|")
ax.set_ylabel("ζ-closed / plain − 1,  m = 3")
ax.set_title("m = 3: raw hits follow the composition model", fontsize=9)
ax.set_xlim(grid)
ax.legend(fontsize=7, loc="upper left")
ax.grid(alpha=0.25)
fig.suptitle("R6.5: the A-pair correction against the §15.2 j = 0 census (bars: 95% bootstrap over bases)", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "excess_vs_inv_size.svg"))
fig.savefig(os.path.join(HERE, "excess_vs_inv_size.png"), dpi=150)
print("ok")
