"""Figures for the Eigen-Seta speed-up study, drawn from committed records
only (plotting recorded values; the only arithmetic is median and the
run - prep difference already recorded in summary.txt).

Usage: python figures/plot.py <study_dir>
Writes decaps.{pdf,svg,png} and ct.{pdf,svg,png} into <study_dir>/figures.
"""
import glob, os, re, statistics, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = sys.argv[1]
REC = os.path.join(D, "records")
RID = open(os.path.join(REC, "RUN_ID")).read().strip()
OUT = os.path.join(D, "figures")
SETS, ARMS = ["L1", "L3", "L5"], ["base", "new", "native"]
LABEL = {"base": "main fc02c6b", "new": "PR #5", "native": "PR #5 + target-cpu=native"}
COL = {"base": "#9aa5b1", "new": "#2f6db5", "native": "#3a9a5b"}


def kv(line):
    return dict(f.split("=", 1) for f in line.rstrip("\n").split("\t") if "=" in f)


def save(fig, stem):
    for ext in ("pdf", "svg"):
        fig.savefig(os.path.join(OUT, f"{stem}.{ext}"))
    fig.savefig(os.path.join(OUT, f"{stem}.png"), dpi=150)
    plt.close(fig)


dec = {}
for s in SETS:
    for a in ARMS:
        dec[(s, a)] = [float(kv(l)["decaps_s"]) for l in open(os.path.join(REC, RID, f"{a}_{s}.tsv")) if l.startswith("op")]

fig, ax = plt.subplots(figsize=(6.6, 3.4))
w = 0.26
for j, a in enumerate(ARMS):
    xs = [i + (j - 1) * w for i in range(len(SETS))]
    med = [statistics.median(dec[(s, a)]) for s in SETS]
    bars = ax.bar(xs, med, w, color=COL[a], label=LABEL[a])
    ax.bar_label(bars, fmt="%.2f", fontsize=6, padding=1)
    for x, s in zip(xs, SETS):
        ys = dec[(s, a)]
        ax.scatter([x] * len(ys), ys, s=5, color="k", zorder=3)
ax.set_xticks(range(len(SETS)))
ax.set_xticklabels([f"{s}\n(n={len(dec[(s, 'base')])} per arm)" for s in SETS])
ax.set_yscale("log")
ax.set_ylabel("decapsulation wall time [s], log")
ax.legend(fontsize=7)
ax.grid(True, axis="y", which="both", alpha=0.3)
fig.suptitle(f"run {RID}; bars = median, dots = every sample; measured", fontsize=8)
fig.tight_layout()
save(fig, "decaps")

ct = {}
for line in open(os.path.join(REC, "ct-20261018T0005Z-callgrind", "summary.txt")):
    a, k, m = line.split()[:3]
    ct.setdefault(a, []).append((f"{k} {m}", int(re.search(r"secret=(\d+)", line).group(1))))
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(7.2, 3.0))
for j, a in enumerate(["base", "new"]):
    names = [n for n, _ in ct[a]]
    vals = [v / 1e9 for _, v in ct[a]]
    ax1.bar([i + (j - 0.5) * 0.4 for i in range(len(vals))], vals, 0.4, color=COL[a], label=LABEL[a])
    spread = max(v for _, v in ct[a]) - min(v for _, v in ct[a])
    ax2.bar([j], [spread], 0.6, color=COL[a])
    ax2.text(j, spread, f"{spread:,}", ha="center", va="bottom", fontsize=7)
ax1.set_xticks(range(len(names)))
ax1.set_xticklabels(names, fontsize=7)
ax1.set_ylabel("secret-phase instructions [10^9 Ir]")
ax1.legend(fontsize=7, loc="lower right")
ax2.set_xticks([0, 1])
ax2.set_xticklabels([LABEL["base"], LABEL["new"]], fontsize=7)
ax2.set_yscale("log")
ax2.set_ylabel("max - min over the 4 cases [Ir], log")
fig.suptitle("callgrind, all threads: Ir(run) - Ir(prep), identical inputs; measured", fontsize=8)
fig.tight_layout()
save(fig, "ct")
print({k: statistics.median(v) for k, v in dec.items()})
