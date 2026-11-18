#!/usr/bin/env python3
"""Presentation only (post hoc): REPORT.pdf = figure + RESULTS.md + tables.md + sensitivity.json, paged,
under the artifact store's 64 KB data threshold. Replaces the PDF that analyze.py writes (analyze.py's
numbers and tables are unchanged). Run after analyze.py and sensitivity.py."""
import os
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["pdf.compression"] = 9
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
HERE = os.path.dirname(os.path.abspath(__file__))
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    import json
    a = json.load(open(os.path.join(HERE, "analysis.json"))); s = json.load(open(os.path.join(HERE, "sensitivity.json")))
    f, ax = plt.subplots(1, 2, figsize=(11, 4.4))
    g = a["select"]; base = g["2.0"]["mean_cost_q3_w1"]
    ax[0].plot([float(c) for c in g], [v["mean_cost_q3_w1"] / base for v in g.values()], "o-")
    ax[0].set(xlabel="cc (nb = cc p^(3/4))", ylabel="arm x cost / cc 2.0 (p = 8209)", title="Part A tuning [pcc:unverified]")
    rows = [("x Q3 measured", a["arms"]["x"]["Q3_w1"]), ("x vs sqrt(pi r/2)", s["x"]["vs_plain_theory_w1"]),
            ("x vs ideal neg.", s["x"]["vs_neg_ideal_w1"]), ("w Q3 measured", a["arms"]["w"]["Q3_w1"]),
            ("w vs ideal neg.", s["w"]["vs_neg_ideal_w1"])]
    for i, (n, v) in enumerate(rows):
        ax[1].errorbar([i], [v["ratio"]], yerr=[[v["ratio"] - v["lo"]], [v["hi"] - v["ratio"]]], fmt="o", capsize=3)
    ax[1].axhline(1, color="k", lw=0.8, ls="--"); ax[1].set_xticks(range(len(rows)), [r[0] for r in rows], rotation=20, fontsize=7)
    ax[1].set(ylabel="IC / rho, w = 1", title="p = 16411, 54 targets [pcc:unverified]")
    f.tight_layout(); pdf.savefig(f); plt.close(f)
    for src in ["RESULTS.md", "tables.md", "sensitivity.json"]:
        lines = open(os.path.join(HERE, src)).read().splitlines()
        for i in range(0, len(lines), 95):
            t = plt.figure(figsize=(11, 8.5))
            t.text(0.03, 0.98, "\n".join(lines[i:i + 95]), va="top", family="monospace", fontsize=5.6)
            pdf.savefig(t); plt.close(t)
print(os.path.getsize(os.path.join(HERE, "REPORT.pdf")))
