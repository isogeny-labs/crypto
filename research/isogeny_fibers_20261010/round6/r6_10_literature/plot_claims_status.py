"""Render claims_status.json as a table SVG (layout only; no computation)."""
import json
import sys
from collections import Counter
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "claims_status.json").read_text())
rows = data["rows"]
COLORS = {
    "confirmed": "#2a9d8f",
    "confirmed-secondary": "#8ecae6",
    "bib-only": "#d9d9d9",
    "corrected": "#f4a261",
    "mismatch": "#e76f51",
    "unverified": "#b5838d",
    "withdrawn": "#6d597a",
}
ORDER = list(COLORS)


def short(s, n):
    return s if len(s) <= n else s[: n - 1] + "…"


n = len(rows)
fig_h = 1.6 + 0.24 * n
fig, ax = plt.subplots(figsize=(10.5, fig_h))
ax.set_xlim(0, 1)
ax.set_ylim(0, n + 3.2)
ax.axis("off")
cols = [(0.0, "id"), (0.045, "group"), (0.17, "citation / claim"), (0.555, "status"), (0.685, "what the fetched source says vs. our use")]
top = n + 1.6
for x, h in cols:
    ax.text(x, top, h, fontsize=8.5, fontweight="bold", va="center")
for i, r in enumerate(rows):
    y = n - i + 0.6
    if i % 2 == 0:
        ax.add_patch(plt.Rectangle((0, y - 0.5), 1, 1, color="#f6f6f6", zorder=0))
    ax.text(0.0, y, r["id"], fontsize=7.5, va="center", family="monospace")
    ax.text(0.045, y, short(r["group"].replace("image / symmetrised", "image / symm."), 17), fontsize=7.5, va="center")
    ax.text(0.17, y, short(r["citation"], 54), fontsize=7.5, va="center")
    ax.add_patch(plt.Rectangle((0.555, y - 0.38), 0.125, 0.76, color=COLORS[r["status"]], zorder=1))
    ax.text(0.6175, y, r["status"], fontsize=6.6, va="center", ha="center", zorder=2)
    ax.text(0.685, y, short(r["note"], 58), fontsize=7.0, va="center")
cnt = Counter(r["status"] for r in rows)
x = 0.0
for s in ORDER:
    ax.add_patch(plt.Rectangle((x, n + 2.45), 0.012, 0.5, color=COLORS[s]))
    label = f"{s}: {cnt.get(s, 0)}"
    ax.text(x + 0.016, n + 2.7, label, fontsize=8, va="center")
    x += 0.04 + 0.0085 * len(label)
ax.set_title(
    f"R6.10 literature closure: status of {n} citations and cited claims (rounds 4-5),\n"
    "checked 2026-10-18 against fetched sources (raw/fetch_log.json); full notes in RESULTS.md",
    fontsize=10, loc="left",
)
out = HERE / "claims_status.svg"
fig.savefig(out, bbox_inches="tight")
print(out, file=sys.stderr)
