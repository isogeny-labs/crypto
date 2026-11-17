"""Render only recorded values; arrows mark censoring, not exact degree 5."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
matplotlib.rcParams["svg.hashsalt"] = "e12b-completed-20261028"
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
rows = [json.loads(line) for line in (root / "E_3-24-7.jsonl").read_text().splitlines()]
fig, ax = plt.subplots(figsize=(7.4, 2.9), layout="constrained")
for row in rows:
    x = row["draw"]
    censored = row["d_f"] is None
    y = row["d_f_at_least"] if censored else row["d_f"]
    ax.scatter(x, y, s=80, facecolors="white" if censored else "#176b91",
               edgecolors="#176b91", linewidths=2, zorder=3)
    if censored:
        ax.annotate("", xy=(x, 5.45), xytext=(x, 5.06),
                    arrowprops={"arrowstyle": "->", "color": "#176b91", "lw": 1.8})
    ax.text(x + .09, y, "≥5 (censored)" if censored else "4 (exact label)",
            fontsize=9, va="center")
ax.axhline(4, color="#7a8088", linestyle="--", linewidth=.8)
ax.set(xlim=(-.3, 3.8), ylim=(3.65, 5.6), xticks=[0, 1, 2, 3], yticks=[4, 5],
       xlabel="Draw index (four frozen unsatisfiable draws)",
       ylabel="Reported last fall degree\n(dimensionless)",
       title="E12b · (m, n, ℓ) = (3, 24, 7) · N = 45 · degree cap 4")
ax.grid(axis="y", alpha=.2)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "draws.svg", metadata={"Date": None})
svg = root / "draws.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(root / "draws.png", dpi=160)
