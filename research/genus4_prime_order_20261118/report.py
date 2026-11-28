#!/usr/bin/env python3
"""Presentation only: REPORT.pdf (vector figure from analysis.json + RESULTS/tables text), under 64 KB."""
import json, os
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["pdf.compression"] = 9
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
HERE = os.path.dirname(os.path.abspath(__file__))
a = json.load(open(os.path.join(HERE, "analysis.json")))
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    f, ax = plt.subplots(figsize=(8, 4.4))
    ps = [16411, 32771]
    for arm, mk in (("x", "o-"), ("x*", "s-")):
        for n, c in (("0.1", "#ff7f0e"), ("1", "#d62728")):
            v = [a["primes"][str(p)]["arms"][arm][f"neg_w{n}"] for p in ps]
            ax.errorbar(ps, [x["ratio"] for x in v], yerr=[[x["ratio"] - x["lo"] for x in v], [x["hi"] - x["ratio"] for x in v]],
                        fmt=mk, color=c, capsize=3, label=f"{arm}, w = {n}")
    ax.axhline(1, color="k", lw=0.8, ls="--")
    ax.set(xscale="log", yscale="log", xlabel="p (prime-order genus-4 Jacobians, #J ~ p^4)",
           ylabel="IC / ideal negation rho sqrt(pi r/4)", title="Genus 4 IC vs strongest rho  [pcc:unverified]")
    ax.legend(fontsize=8)
    f.tight_layout(); pdf.savefig(f); plt.close(f)
    for src in ["RESULTS.md", "tables.md", "DEVIATIONS.md"]:
        lines = open(os.path.join(HERE, src)).read().splitlines()
        for i in range(0, len(lines), 95):
            t = plt.figure(figsize=(11, 8.5))
            t.text(0.03, 0.98, "\n".join(lines[i:i + 95]), va="top", family="monospace", fontsize=5.6)
            pdf.savefig(t); plt.close(t)
print(os.path.getsize(os.path.join(HERE, "REPORT.pdf")))
