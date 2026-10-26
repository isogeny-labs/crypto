"""Render the recorded L_E percentiles (out/curves.tsv, out/binary.tsv). Layout only: no computation."""
import csv
import pathlib

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = pathlib.Path(__file__).resolve().parent
STREAMS = {"863966734": "P-192 run", "891607927": "P-224 run", "863711343": "p239 run",
           "871696354": "P-256 run", "872952608": "P-384 run", "871002724": "P-521 run"}
ORDER = list(STREAMS)
PUB, SKIP, BIN = "#2a6fb0", "#d9822b", "#5b8c3a"


def rows(name):
    with open(HERE / "out" / name) as f:
        return list(csv.DictReader(f, delimiter="\t"))


curves, binary = rows("curves.tsv"), rows("binary.tsv")
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(11, 4.2), gridspec_kw={"width_ratios": [3, 1.3]})

for r in curves:
    x = ORDER.index(r["run"])
    pub = r["role"] == "published"
    ax1.scatter(x + (-0.12 if pub else 0.12), float(r["null_pct"]), s=46,
                color=PUB if pub else SKIP, marker="o" if pub else "s", zorder=3)
ax1.scatter([], [], color=PUB, marker="o", label="published (10)")
ax1.scatter([], [], color=SKIP, marker="s", label="skipped (11)")
ax1.axhline(50, color="0.6", lw=0.8, ls="--")
ax1.set_xticks(range(len(ORDER)), [STREAMS[k] for k in ORDER])
ax1.set_ylim(0, 100)
ax1.set_ylabel("L_E percentile under the Gekeler null (%)")
ax1.set_title("(a) Prime seed runs: L_E of each candidate", fontsize=10)
ax1.legend(loc="lower left", fontsize=8, frameon=False)

for i, r in enumerate(binary):
    primary = r["role"] == "primary"
    ax2.scatter(i, float(r["null_pct"]), s=46, color=BIN, marker="D" if primary else "^",
                facecolors=BIN if primary else "none", zorder=3)
ax2.axhline(50, color="0.6", lw=0.8, ls="--")
ax2.set_xticks(range(len(binary)), [r["label"] for r in binary], rotation=30)
ax2.set_ylim(0, 100)
ax2.set_title("(b) NSA binary curves (B1)", fontsize=10)
ax2.text(0.02, 0.03, "filled: primary (4)\nopen: B-163, secondary", transform=ax2.transAxes, fontsize=8)

fig.tight_layout()
fig.savefig(HERE / "mechanism.svg")
fig.savefig(HERE / "mechanism.png", dpi=150)
