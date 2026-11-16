"""R7.5: corrected (c)/(b) containment figure for R6.7.  Plots recorded point and ci95 fields of
m2_torsion.json only (no new computation beyond comparing the interval with 1)."""
import json
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
matplotlib.rcParams["svg.fonttype"] = "none"
rows = json.load(open("m2_torsion.json"))["rows"]
PR = [("unpriced", "#2f6f9f", -0.22), ("priced_as_run", "#d08a1f", 0.0), ("priced_essential", "#3d8a4f", 0.22)]
fig, ax = plt.subplots(figsize=(11, 4.8))
counts = {}
for name, col, off in PR:
    n_in = 0
    for i, r in enumerate(rows):
        b = r["ratio_c_over_b_size_matched"][name]; lo, hi = b["ci95"]; inside = lo <= 1.0 <= hi; n_in += inside
        ax.errorbar(i + off, b["point"], yerr=[[b["point"] - lo], [hi - b["point"]]], fmt="o", ms=5, lw=1.2, color=col,
                    mfc=col if inside else "white", mew=1.4)
    counts[name] = n_in
    ax.plot([], [], "o", color=col, label=f"{name.replace('_', ' ')}: {n_in} of {len(rows)} intervals contain 1")
ax.axhline(1.0, color="#c0392b", lw=1.8, label="law value 1")
ax.set_xticks(range(len(rows)))
ax.set_xticklabels([f"{r['bits']}b s{r['seed']}\np={r['instance']['p']}" for r in rows], fontsize=6.5)
ax.set_ylabel("(c)/(b) size-matched ratio (point, 95% CI)")
ax.set_title("R6.7 erratum v2: m = 2, 18 torsion curves, per-curve (c)/(b) intervals against the law value 1\n"
             "filled = interval contains 1, hollow = excludes 1.  Original text said 12 of 18.", fontsize=9)
ax.grid(axis="y", alpha=0.3); ax.legend(fontsize=8, loc="upper right", framealpha=0.95)
ax.set_ylim(0.4, 1.55)
fig.tight_layout(); fig.savefig("erratum_v2_cb_containment.svg")
print(counts)
