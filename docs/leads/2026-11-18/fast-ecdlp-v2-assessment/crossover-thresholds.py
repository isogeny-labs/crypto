#!/usr/bin/env python3
"""Plot the crossover thresholds recorded in Table 5 / Figure 1 of the
Kaarelson v2 preprint against a real 256-bit key. This lays out recorded
values only (no computation); see docs/leads/2026-11-18/fast-ecdlp-v2-assessment/REPORT.md.

Run: <scratch-venv>/bin/python crossover-thresholds.py
"""
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"  # keep SVG small (lesson L-5b42b657f746)
import matplotlib.pyplot as plt

# (label, threshold log2 L >=, status) — values transcribed from the paper's Table 5.
rows = [
    ("V8x, accel. base case, free sched.", 4.372e7, "needs (R1)"),
    ("V8x, Anthropic schedule", 6.292e7, "needs (R1)"),
    ("V8x, Lemma 8(ii) clustering (R1)", 1.693e8, "needs (R1)"),
    ("V8x, Lemma 8(ii), no (R1)", 2.52e8, "numeric/red-team"),
    ("V8x, c1(5) <= 91.8", 2.014e9, "numeric/red-team"),
    ("X network", 4.027e9, "paper + checked"),
    ("Compact network (S3.1)", 5.154e11, "paper + checked"),
    ("Edge network", 6.756e16, "Lean theorem"),
]
real_key = 8.0  # log2 L for a 256-bit key: p ~ 2^256, L = log2 p ~ 256, log2 L = 8

color = {
    "needs (R1)": "#d1495b",        # red: lowest thresholds, rest on assumption R1
    "numeric/red-team": "#edae49",  # amber: value numeric-only or schedule red-team checked
    "paper + checked": "#4b8fd1",   # blue: paper proof + numeric check
    "Lean theorem": "#2a9d55",      # green: end-to-end Lean
}

labels = [r[0] for r in rows]
vals = [r[1] for r in rows]
stats = [r[2] for r in rows]
y = range(len(rows))

fig, ax = plt.subplots(figsize=(8.6, 4.6))
ax.barh(list(y), vals, color=[color[s] for s in stats], height=0.62, zorder=3)
for yi, v in zip(y, vals):
    ax.text(v * 1.3, yi, f"{v:.3g}", va="center", ha="left", fontsize=8, zorder=4)

ax.axvline(real_key, color="black", ls="--", lw=1.4, zorder=5)
ax.text(real_key * 1.4, len(rows) - 0.4, "real 256-bit key\n(log2 L = 8)",
        fontsize=8.5, va="top", ha="left")

ax.set_xscale("log")
ax.set_xlim(3, 3e18)
ax.set_yticks(list(y))
ax.set_yticklabels(labels, fontsize=8.5)
ax.set_xlabel(r"crossover threshold on $\log_2 L \approx \log_2\log_2 p$ "
              r"(guarantee overtakes Pollard rho at $m=5$)", fontsize=9)
ax.set_title("How far the proved ECDLP crossover sits from any instantiable key\n"
             "(Kaarelson v2, Table 5 / Figure 1; values transcribed, not recomputed)",
             fontsize=10)
ax.grid(axis="x", ls=":", alpha=0.5, zorder=0)

handles = [plt.Rectangle((0, 0), 1, 1, color=c) for c in
           ["#2a9d55", "#4b8fd1", "#edae49", "#d1495b"]]
hl = ["Lean theorem (end-to-end)", "paper proof + numeric check",
      "value numeric-only / red-team schedule", "rests on assumption (R1)"]
ax.legend(handles, hl, fontsize=7.5, loc="lower right", framealpha=0.9)

fig.tight_layout()
fig.savefig("crossover-thresholds.svg")
print("wrote crossover-thresholds.svg")
