#!/usr/bin/env python3
"""README.md + results.md + the figures' plotting functions -> REPORT.pdf (layout only). cwd = study dir."""
import textwrap
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["pdf.compression"] = 9
matplotlib.rcParams["pdf.fonttype"] = 42
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
src = open("analysis/plot.py").read().split("\nfor name, fn, size")[0]
exec(src)
txt = open("README.md").read().split("## Files")[0]
lines = []
for para in txt.split("\n"):
    lines += textwrap.wrap(para, 118) or [""]
with PdfPages("REPORT.pdf", metadata={"CreationDate": None, "ModDate": None}) as pdf:
    for start in range(0, len(lines), 62):
        fig = plt.figure(figsize=(8.27, 11.69)); fig.text(0.05, 0.97, "\n".join(lines[start:start + 62]), va="top", family="monospace", fontsize=6.6)
        pdf.savefig(fig); plt.close(fig)
    table = open("results.md").read().split("\n")
    fig = plt.figure(figsize=(11.69, 8.27)); fig.text(0.02, 0.98, "\n".join(textwrap.shorten(l, 230) for l in table), va="top", family="monospace", fontsize=4.8)
    pdf.savefig(fig); plt.close(fig)
    for fn, size in [(ratios, (10, 4.6)), (gap, (9, 4.4)), (model, (6, 5.5))]:
        fig = plt.figure(figsize=size); fn(fig); fig.tight_layout(); pdf.savefig(fig); plt.close(fig)
