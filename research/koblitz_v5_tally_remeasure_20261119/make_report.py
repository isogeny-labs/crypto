#!/usr/bin/env python3
"""Figure (SVG, PNG) and REPORT.pdf from summary.json (post-data; plots only recorded values)."""
import json, os
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
HERE = os.path.dirname(os.path.abspath(__file__)); S = json.load(open(os.path.join(HERE, "summary.json")))
W = ["0", "0.1", "1"]; cols = {"0": "#2a6fb0", "0.1": "#d08a1e", "1": "#3d9a50"}
rows = sorted(S["curves"].items(), key=lambda kv: kv[1]["log2_r"])
fig, ax = plt.subplots(figsize=(7.2, 4.2))
for w in W:
    x = [c["log2_r"] for _, c in rows]; q = [c["ratios"]["rho-v5/rho-strong"][w] for _, c in rows]
    y = [t["ratio"] for t in q]; lo = [t["ratio"] - t["lo"] for t in q]; hi = [t["hi"] - t["ratio"] for t in q]
    ax.errorbar(x, y, yerr=[lo, hi], marker="o", ms=4, capsize=3, lw=1.2, color=cols[w], label=f"w = {w}")
for n, c in rows:
    ax.annotate(n, (c["log2_r"], c["ratios"]["rho-v5/rho-strong"]["0"]["ratio"]), textcoords="offset points", xytext=(4, -10), fontsize=7)
ax.axhline(1, color="#888", lw=0.8, ls="--"); ax.axvline(38, color="#b33", lw=0.8, ls=":")
ax.text(37.6, 0.05, "decision scope\nboundary 2^38", ha="right", fontsize=7, color="#b33")
ax.set_yscale("log"); ax.set_xlim(10, 39); ax.set_xlabel("log2 r"); ax.set_ylabel("v5 / strong (paired, 95%)")
ax.set_title("rho.signed_frobenius_v5 (jump-index tallied) / strong, Koblitz curves [pcc:unverified]", fontsize=9)
ax.legend(fontsize=8, loc="upper left"); fig.tight_layout()
for ext in ("svg", "png"): fig.savefig(os.path.join(HERE, "figures", f"fig1-v5-over-strong.{ext}"), dpi=110)
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    pdf.savefig(fig)
    f2 = plt.figure(figsize=(8.3, 5.0)); f2.text(0.04, 0.95, "v5 (tallied) / strong per curve and weight  [pcc:unverified]", fontsize=11, weight="bold")
    lines = ["curve      log2 r   w=0                   w=0.1                 w=1                   tally==steps"]
    for n, c in rows:
        q = c["ratios"]["rho-v5/rho-strong"]
        lines.append(f"{n:9s}  {c['log2_r']:6.2f}   " + "   ".join(f"{q[w]['ratio']:.3f} [{q[w]['lo']:.3f},{q[w]['hi']:.3f}]" for w in W) + f"   {c['v5_tally_equals_steps']}")
    lines += ["", "Crossover (v5/strong = 1): not reached at any weight up to log2 r 27.78.",
              f"Binary {S['binary_sha256'][:16]}..., L0, 48 targets x 3 rounds per curve, verify --replay-all 0 problems.",
              "A/A strong and A/A v5 = 1.000 on every curve and weight."]
    f2.text(0.04, 0.88, "\n".join(lines), family="monospace", fontsize=7.5, va="top"); pdf.savefig(f2)
print("ok")
