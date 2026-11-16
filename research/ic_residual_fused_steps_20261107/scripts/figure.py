"""Measured against predicted walk-step ratios: simulation (exploratory and
confirmatory) and the real ecbench code.  Writes figures/ratios.svg, .png
and REPORT.pdf (the figure plus the result table)."""
import json, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

def h(arm, F, J):
    s = 0 if arm.endswith("_ns") else 1
    return ((J - 1) / J + (F - 1) / F + s) if arm.startswith("base") else ((J * F - 1) / (J * F) + s)

pts = []  # (label, predicted, value, lo, hi, kind)
for r in json.load(open("data/ratios.json")):
    num, den = r["ratio"].split("/")
    pred = 1.0 if num == "fusedind" else math.sqrt(h(den, r["F"], 16) / h(num, r["F"], 16))
    pts.append((f"F={r['F']} {r['ratio']}", pred, r["value"], *r["ci"], "simulation, exploratory (model fitted here)"))
for r in json.load(open("data/confirm-scored.json"))["rows"]:
    pts.append((f"F={r['F']} J={r['J']} {r['ratio']}", r["prediction"], r["value"], *r["ci"], "simulation, confirmatory (frozen)"))
for r in json.load(open("real/scored.json")):
    pts.append((f"ecbench p36 B={r['B']} res{r['B']}f/res{r['B']}", r["prediction"], r["steps_ratio"], *r["ci"], "real code, p36 (frozen)"))

colors = {"simulation, exploratory (model fitted here)": "#9aa5b1",
          "simulation, confirmatory (frozen)": "#2b6cb0",
          "real code, p36 (frozen)": "#c05621"}
fig, ax = plt.subplots(figsize=(6.4, 6.0))
for kind, c in colors.items():
    sel = [p for p in pts if p[5] == kind]
    ax.errorbar([p[1] for p in sel], [p[2] for p in sel],
                yerr=[[p[2] - p[3] for p in sel], [p[4] - p[2] for p in sel]],
                fmt="o", ms=4, color=c, ecolor=c, elinewidth=1, capsize=2, label=kind)
lo, hi = 0.95, 1.47
ax.plot([lo, hi], [lo, hi], color="#444", lw=0.8, ls="--", label="measured = predicted")
ax.set_xlim(lo, hi); ax.set_ylim(lo, hi)
ax.set_xlabel("predicted walk-step ratio (coincidence-hazard model)")
ax.set_ylabel("measured walk-step ratio, 95% interval")
ax.set_title("Why the fused residual walk takes more steps")
ax.legend(fontsize=8, loc="upper left")
fig.tight_layout()
fig.savefig("figures/ratios.svg"); fig.savefig("figures/ratios.png", dpi=160)

with PdfPages("REPORT.pdf") as pdf:
    pdf.savefig(fig)
    t, a = plt.subplots(figsize=(8.5, 11)); a.axis("off")
    rows = [[p[0], f"{p[1]:.4f}", f"{p[2]:.4f}", f"[{p[3]:.4f}, {p[4]:.4f}]", "yes" if p[3] <= p[1] <= p[4] else "no", p[5].split(",")[0] + (" expl." if "explor" in p[5] else "")] for p in pts]
    tab = a.table(cellText=rows, colLabels=["ratio", "predicted", "measured", "95% interval", "inside", "source"], loc="upper center", cellLoc="left")
    tab.auto_set_font_size(False); tab.set_fontsize(6.5); tab.scale(1, 1.25)
    a.set_title("Walk-step ratios: prediction against measurement (local L0 sandbox; not validated)", fontsize=9)
    pdf.savefig(t)
print(len(pts), "points")
