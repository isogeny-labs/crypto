"""Figures for the Eigen-Seta IFMA study, drawn from committed records
only (plotting recorded values; the only arithmetic is median and the
run - prep difference already recorded in summary.txt).

Usage: python figures/plot.py <study_dir>
Writes decaps.{pdf,svg,png} and dudect.{pdf,svg,png} into <study_dir>/figures.
"""
import glob, os, re, statistics, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = sys.argv[1]
REC = os.path.join(D, "records")
RID = open(os.path.join(REC, "RUN_ID")).read().strip()
OUT = os.path.join(D, "figures")
SETS, ARMS = ["L1", "L3", "L5"], ["base", "ifma", "native"]
LABEL = {"base": "main 580b633", "ifma": "IFMA (#6, default build)", "native": "IFMA + target-cpu=native"}
COL = {"base": "#9aa5b1", "ifma": "#2f6db5", "native": "#3a9a5b"}


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

# dudect on decapsulation (L1), IFMA vs portable: t at each crop, recorded
rows = {}
for name, f in [("IFMA", "decaps_ifma_1000.txt"), ("portable", "decaps_portable_1000.txt")]:
    vals = []
    for line in open(os.path.join(REC, "dudect-20261018T0300Z", f)):
        m = re.match(r"t\((raw|crop <= p(\d+))\) = ([+-][0-9.]+)", line.strip())
        if m:
            vals.append(("raw" if m.group(1) == "raw" else "p" + m.group(2), float(m.group(3))))
    rows[name] = vals
fig, ax = plt.subplots(figsize=(5.6, 3.0))
for j, (name, col) in enumerate([("IFMA", COL["ifma"]), ("portable", COL["base"])]):
    labels = [k for k, _ in rows[name]]
    ax.bar([i + (j - 0.5) * 0.38 for i in range(len(labels))], [v for _, v in rows[name]], 0.38, color=col, label=name)
ax.axhline(4.5, color="r", lw=0.8, ls="--")
ax.axhline(-4.5, color="r", lw=0.8, ls="--", label="|t| = 4.5")
ax.set_xticks(range(len(labels)))
ax.set_xticklabels(labels)
ax.set_ylabel("Welch t")
ax.legend(fontsize=7)
fig.suptitle("dudect_decaps L1, 1,000 samples/class (fixed vs random ciphertexts); measured", fontsize=8)
fig.tight_layout()
save(fig, "dudect")
print({k: statistics.median(v) for k, v in dec.items()})
