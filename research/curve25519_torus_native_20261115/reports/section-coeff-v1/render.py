"""Report layout only: draw exact numeric fields already emitted by Rust.
No cryptographic arithmetic, fitting, testing or worker orchestration.
"""
import json
import pathlib
import re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = pathlib.Path(__file__).resolve().parent
data = json.loads((root.parent.parent / "section-coeff-v1/native-v1/SUMMARY.json").read_text())
rows = data["formal_degrees"]
fig, ax = plt.subplots(figsize=(8.7, 2.55), constrained_layout=True)
xs = [r["k"] for r in rows]
ys = [r["X5_T_degree"] for r in rows]
ax.plot(xs, ys, "o-", color="#245779", linewidth=1.5, markersize=5)
for x, y in zip(xs, ys):
    ax.annotate(str(y), (x, y), xytext=(0, 7), textcoords="offset points", ha="center", fontsize=9)
ax.set(xlabel="Toy window k (q = 3)", ylabel="Exact formal degree in T", xlim=(0.65, 8.35), ylim=(0, 98), xticks=xs)
ax.set_title("Recorded coefficient degrees in F101[T] — no fitted exponent", fontsize=11, loc="left")
ax.grid(axis="y", alpha=.18)
ax.spines[["top", "right"]].set_visible(False)
for suffix in ("pdf", "svg", "png"):
    fig.savefig(root / ("degree." + suffix), dpi=160)
plt.close(fig)
# Normalize only this new, not-yet-pinned Matplotlib SVG's path whitespace.
p = root / "degree.svg"
p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines()) + "\n")
