"""Layout only: draws results/*.jsonl written by derive/ (native Rust). Computes nothing.

Usage: python3 plot.py   (writes figures/sizes.{svg,png}, figures/census.{svg,png})
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(__file__).parent
out = here / "figures"
out.mkdir(exist_ok=True)
rows = [json.loads(x) for x in (here / "results/counts.jsonl").read_text().splitlines()]
cen = [json.loads(x) for x in (here / "results/census.jsonl").read_text().splitlines()]
plt.rcParams.update({"font.size": 8.5, "svg.fonttype": "none"})
REPS = [("chain_s3", "chained S3 (cubic)", "#1f2328", "o"),
        ("quad_generic", "w = x_a x_b (control)", "#8c959f", "s"),
        ("e2_aux", "mu = e2 (norm form)", "#0969da", "^"),
        ("line", "mu = e2 + slope lambda", "#bf8700", "D")]

fig, axes = plt.subplots(1, 3, figsize=(10.5, 3.2))
sel = lambda r: r["n"] == 19 and r["l"] == 5 and r["a"] == 0
for key, lab, col, mk in REPS:
    pts = sorted((r["m"], r) for r in rows if sel(r) and r["rep"] == key)
    ms = [m for m, _ in pts]
    for ax, field in zip(axes, ["nvars", "distinct_monomials", "terms"]):
        ax.plot(ms, [r[field] for _, r in pts], marker=mk, color=col, label=lab, lw=1.2)
for ax, t in zip(axes, ["Boolean variables", "distinct monomials", "terms (written size)"]):
    ax.set_title(t)
    ax.set_xlabel("m (factor-base points)")
    ax.set_xticks([3, 4, 5])
    ax.grid(alpha=0.3, lw=0.4)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
axes[0].legend(frameon=False, fontsize=7.5)
fig.suptitle("Exact size of the Weil-descended decomposition system by representation "
             "(n = 19, l = 5, Koblitz a = 0, b = 1; derive/ counts)", fontsize=9)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(out / f"sizes.{ext}", bbox_inches="tight", **kw)

fig, ax = plt.subplots(figsize=(6.5, 2.8))
labels, on_e, tw = [], [], []
for c in cen:
    cells = {(x["on_curve"], x["lambda"]): x["count"] for x in c["cells"]}
    labels.append(f"n={c['n']} a={c['a']} b={'1' if c['b'] == 1 else 'rand'}")
    on_e.append(cells.get((3, True), 0) / c["triples"])
    tw.append(cells.get((0, False), 0) / c["triples"])
y = range(len(labels))
ax.barh(y, on_e, color="#0969da", label="all three on E, lambda exists")
ax.barh(y, tw, left=on_e, color="#bf8700", label="all three on the twist, no lambda")
ax.set_yticks(list(y), labels)
ax.set_xlim(0, 1)
ax.set_xlabel("fraction of all root triples of S3(x1, x2, X) = 0 (exhaustive)")
ax.legend(frameon=False, loc="upper center", bbox_to_anchor=(0.5, 1.25), ncol=2, fontsize=7.5)
for s in ("top", "right"):
    ax.spines[s].set_visible(False)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(out / f"census.{ext}", bbox_inches="tight", **kw)
print("wrote figures/sizes.* and figures/census.*")
