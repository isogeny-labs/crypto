"""Arms against ideal negation-map rho, with the frozen predictions and floors."""
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
d = json.load(open("scored.json")); P = json.load(open("PREDICTIONS.json"))["predictions"]
rows = [("rho.negation", d["rho"]["gae_over_ideal"], "F3 band 0.95-1.15"),
        ("res1eq (total)", d["res1eq"]["gae_over_ideal"], "F2 floor 1.45"),
        ("res1eq (walk)", d["res1eq"]["walk_ops_over_ideal"], "two-collision floor 1.5"),
        ("res1 (walk)", d["res1"]["walk_ops_over_ideal"], f"F1 {P[0]['prediction']:.3f}")]
fig, ax = plt.subplots(figsize=(6.4, 4.2))
for i, (name, (v, lo, hi), _) in enumerate(rows):
    ax.errorbar([v], [i], xerr=[[v - lo], [hi - v]], fmt="o", color="#2b6cb0", capsize=3)
ax.axvspan(0.95, 1.15, color="#c6f6d5", alpha=0.6, label="F3 band (rho)")
ax.axvline(1.5, color="#c05621", ls="--", lw=1, label="two-collision floor 1.5")
ax.axvline(P[0]["prediction"], color="#6b46c1", ls=":", lw=1, label=f"F1 prediction {P[0]['prediction']:.3f}")
ax.set_yticks(range(len(rows))); ax.set_yticklabels([r[0] for r in rows])
ax.set_xlabel("mean counted GAE per solve / ideal rho sqrt(pi r / 4), 95% interval")
ax.set_title("p36: collision floor and the matched rho reference")
ax.legend(fontsize=8, loc="lower right"); fig.tight_layout()
fig.savefig("ratios.svg"); fig.savefig("ratios.png", dpi=160)
with PdfPages("REPORT.pdf") as pdf:
    pdf.savefig(fig)
    t, a = plt.subplots(figsize=(8.5, 4)); a.axis("off")
    cells = [[n, f"{v:.3f}", f"[{lo:.3f}, {hi:.3f}]", p] for n, (v, lo, hi), p in rows]
    tab = a.table(cellText=cells, colLabels=["arm", "/ ideal rho", "95% interval", "frozen prediction"], loc="center")
    tab.auto_set_font_size(False); tab.set_fontsize(8); tab.scale(1, 1.4)
    a.set_title("Local L0 sandbox, 128 targets x 2 rounds, not validated", fontsize=9)
    pdf.savefig(t)
