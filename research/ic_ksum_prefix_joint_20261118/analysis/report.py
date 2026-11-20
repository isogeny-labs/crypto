#!/usr/bin/env python3
"""README.md + summary.json -> REPORT.pdf (layout only: text pages and plots of recorded values). cwd = study dir."""
import json, textwrap
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
exec(open("analysis/plot.py").read().split("fig, axes")[0])  # imports and S, rows, W
txt = open("README.md").read().split("## Files")[0]
lines = []
for para in txt.split("\n"):
    lines += textwrap.wrap(para, 118) or [""]
with PdfPages("REPORT.pdf") as pdf:
    for start in range(0, len(lines), 62):
        fig = plt.figure(figsize=(8.27, 11.69)); fig.text(0.05, 0.97, "\n".join(lines[start:start + 62]), va="top", family="monospace", fontsize=6.6)
        pdf.savefig(fig); plt.close(fig)
    table = open("results.md").read().split("\n")
    fig = plt.figure(figsize=(11.69, 8.27)); fig.text(0.02, 0.98, "\n".join(textwrap.shorten(l, 260) for l in table), va="top", family="monospace", fontsize=4.6)
    pdf.savefig(fig); plt.close(fig)
    exec(open("analysis/plot.py").read().split('fig.tight_layout(); fig.savefig("ratios.svg")')[0].split("W = [\"0\", \"0.1\", \"1\"]\n", 1)[1])
    fig.tight_layout(); pdf.savefig(fig); plt.close(fig)
