#!/usr/bin/env python3
"""Render the exact dense-index allocation formula; no timings are plotted."""
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
DEGREES = (13, 19, 23)
BYTES = tuple(8 * ((1 << n) + 1) for n in DEGREES)
MIB = tuple(b / (1 << 20) for b in BYTES)

fig, ax = plt.subplots(figsize=(6.4, 3.5))
fig.patch.set_facecolor("#f8fafc")
ax.set_facecolor("#f8fafc")
bars = ax.bar([str(n) for n in DEGREES], MIB, color=("#4f46e5", "#0d9488", "#ea580c"), width=0.58)
for bar, n, size in zip(bars, DEGREES, BYTES):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height()*1.08,
            f"{size:,} B", ha="center", va="bottom", fontsize=9)
ax.set_yscale("log", base=2)
ax.set_ylim(0.03, 256)
ax.set_ylabel("Direct-index allocation (MiB, log scale)")
ax.set_xlabel("Binary extension degree n")
ax.set_title("Derived storage: 8 × (2ⁿ + 1) bytes")
ax.grid(axis="y", alpha=0.24)
ax.set_axisbelow(True)
fig.text(0.08, 0.01, "Exact formula from the candidate layout; no runtime or RSS measurement.", fontsize=8, color="#475569")
fig.tight_layout(rect=(0, 0.045, 1, 1))
fig.savefig(HERE / "dense_storage.svg")
