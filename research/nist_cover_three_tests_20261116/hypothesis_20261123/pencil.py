"""Lay out a hypothesis diagram; no curve computation or simulated data."""
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

root = Path(__file__).resolve().parent
plt.rcParams["svg.fonttype"] = "none"
fig, ax = plt.subplots(figsize=(9, 3.0), layout="constrained")
ax.set(xlim=(0, 10), ylim=(0, 4))
ax.axis("off")
for x, title, body, color in [
    (0.15, "Current: vertical lines", "Quartic residual\nConditional split rate: 1/24\nUp to 4 positive + 4 negative terms", "#e7edf3"),
    (5.15, "Proposed: lines through R", "Known root R + cubic residual\nConditional split rate: 1/6\nAfter R cancels: up to 3 + 3 terms", "#e1f0e8"),
]:
    ax.add_patch(FancyBboxPatch((x, 1.15), 4.7, 2.3, boxstyle="round,pad=0.03", facecolor=color, edgecolor="#36576b"))
    ax.text(x+2.35, 3.0, title, ha="center", weight="bold", fontsize=11)
    ax.text(x+2.35, 2.1, body, ha="center", va="center", fontsize=10, linespacing=1.65)
ax.text(5, 3.8, "Same P-192 d7 kernel3 plane quartic; same degree-6 elliptic map", ha="center", fontsize=11)
ax.text(5, 0.65, "Test: checked projected rank / complete collector work; expose all new columns", ha="center", fontsize=10)
ax.text(5, 0.15, "Forecast only · full symmetric monodromy + uniform sampling assumed · no measured speedup", ha="center", fontsize=9)
fig.savefig(root / "pencil.svg")
fig.savefig(root / "pencil.png", dpi=100)
