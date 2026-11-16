"""Layout only: draws numbers already computed by the Rust scorer (summary.txt).
No research computation happens here."""
import re
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

run = Path(sys.argv[1])
txt = (run / "summary.txt").read_text().splitlines()

# per-unit kappa on 2E, from the scorer's description block
kappa = {}  # (n, d, id) -> {arm: kappa_2E}
unit = None
for line in txt:
    m = re.match(r"  n=(\d) d=(\d) (\w+) nE=", line)
    if m:
        unit = (int(m[1]), int(m[2]), m[3])
        kappa[unit] = {}
        continue
    m = re.match(r"    (\w+)\s+m=.*kappa 2E=([\d.NaNinf-]+) odd=", line)
    if m and unit:
        kappa[unit][m[1]] = float(m[2])
    if line.startswith("P1 ("):
        break

# per-unit statistic rows: obs, CI, pred; assigned to the next "KEY: units" line
stats = {}
buf = []
for line in txt:
    m = re.match(r"\s+n=(\d) d=(\d) (\w+)\s+obs\s+([\d.eE+-]+) 99% CI \[\s*([\d.eE+inf-]+),\s*([\d.eE+inf-]+)\] pred\s+([\d.eE+-]+)", line)
    if m:
        buf.append((int(m[1]), int(m[2]), m[3], float(m[4]), float(m[5]), float(m[6]), float(m[7])))
        continue
    m = re.match(r"  (P\d_\w+?)(?:\.(odd|even))?: units", line)
    if m:
        stats.setdefault(m[1], []).extend(buf)
        buf = []

arms = ["std", "tz", "hz", "mix", "twh", "tw"]
fig, ax = plt.subplots(1, 2, figsize=(12, 4.6))

for par, col, off in [(1, "#3b6ea5", -0.15), (0, "#c9883a", 0.15)]:
    for i, a in enumerate(arms):
        ys = [v[a] for (n, d, _), v in kappa.items() if n % 2 == par and a in v]
        ax[0].scatter([i + off] * len(ys), ys, s=14, color=col, alpha=0.8,
                      label=("odd n (3, 5)" if par else "even n (4)") if i == 0 else None)
ax[0].axhline(2.0, color="black", lw=0.8, ls="--")
ax[0].axhline(1.0, color="grey", lw=0.6, ls=":")
ax[0].set_xticks(range(len(arms)))
ax[0].set_xticklabels(arms)
ax[0].set_ylabel("kappa on 2E = mean N(R) * #E / m^n")
ax[0].set_ylim(0, 2.6)
ax[0].set_title("(a) size-normalised yield on 2E targets, every Tr(a)=0 curve", fontsize=9)
ax[0].legend(fontsize=8, loc="lower right")

panels = [("P1_D_2E", "D = k_tw/k_std (2E)"), ("P6_H_2E", "H = k_hz/k_std (2E)"), ("P6_H_odd", "H (odd-order)"),
          ("P6_L_odd", "L = k_tw/k_hz (odd-order)"), ("P6_T_odd", "T = eta_tz/eta_std (odd-order)")]
for j, (key, lab) in enumerate(panels):
    rows = [r for r in stats.get(key, []) if r[0] % 2 == 1]
    for k, r in enumerate(rows):
        x = j + (k - len(rows) / 2) * 0.035
        hi = r[5] if r[5] != float("inf") else r[3] * 10
        ax[1].plot([x, x], [r[4], hi], color="#3b6ea5", lw=0.8)
        ax[1].scatter([x], [r[3]], s=10, color="#3b6ea5")
        ax[1].scatter([x], [r[6]], s=14, marker="_", color="#b03030")
ax[1].set_yscale("log", base=2)
ax[1].set_xticks(range(len(panels)))
ax[1].set_xticklabels([p[1] for p in panels], fontsize=7, rotation=12)
ax[1].axhline(1.0, color="grey", lw=0.6, ls=":")
ax[1].axhline(2.0, color="black", lw=0.8, ls="--")
ax[1].set_title("(b) odd n, per curve: observed with 99% bootstrap CI (blue), prediction (red)", fontsize=9)

fig.tight_layout()
out = run.parent.parent
fig.savefig(out / "fig_odd_n_trace.svg")
fig.savefig(out / "fig_odd_n_trace.png", dpi=150)
