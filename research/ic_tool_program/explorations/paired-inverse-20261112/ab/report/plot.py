"""Plots recorded values only: summary.json (summarise.sh) and ratios.tsv (ratios.sh)."""
import csv, json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = sys.argv[1]
s = json.load(open(f"{here}/summary.json"))
rows = list(csv.DictReader(open(f"{here}/report/ratios.tsv"), delimiter="\t"))
fig, axes = plt.subplots(1, 2, figsize=(8.4, 3.8), sharey=True)
for ax, metric in zip(axes, ["collect", "setup"]):
    for i, row in enumerate(["k0n53", "k0n61"]):
        for j, (kind, col) in enumerate([("ab", "#e45756"), ("aa", "#4c78a8")]):
            x = i + (j - 0.5) * 0.35
            pts = [float(r[f"{kind}_{metric}"]) for r in rows if r["row"] == row]
            ax.scatter([x] * len(pts), pts, s=6, color=col, alpha=0.35)
            st = s[row][f"{kind}_{metric}"]
            ax.errorbar([x + 0.08], [st["median"]], yerr=[[st["median"] - st["ci95"][0]], [st["ci95"][1] - st["median"]]],
                        fmt="o", color=col, capsize=3, label=("A/B unpaired/paired" if kind == "ab" else "A/A unpaired/unpaired") if i == 0 else None)
    ax.axhline(1, color="k", lw=0.6)
    ax.set_xticks([0, 1]); ax.set_xticklabels(["2^44.3 (k0n53)", "2^47.2 (k0n61)"], fontsize=8)
    ax.set_title(f"{metric}: per-round ratio, median and 95% bootstrap", fontsize=8)
    ax.set_ylim(0.85, 1.2); ax.grid(alpha=0.25)
axes[0].set_ylabel("time ratio (above 1: paired faster)"); axes[0].legend(fontsize=7, loc="upper left")
fig.text(0.01, 0.01, "42 rounds a row, one binary, KIC_SCAN_PAIRED=0 vs default; Intel Xeon container; dots include dropped rounds", fontsize=6)
fig.tight_layout(rect=(0, 0.03, 1, 1))
fig.savefig(f"{here}/report/ab.svg"); fig.savefig(f"{here}/report/ab.png", dpi=150)
