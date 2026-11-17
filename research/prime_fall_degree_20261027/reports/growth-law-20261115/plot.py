"""Lay out the recorded d_F medians of data.json (no computation beyond the registered C-step formula).

Panel A: d_F against tower height t, one line per m, censored cells drawn as open up-triangles at their bound.
Panel B: the same values against the number of free unknowns N = m*t.
The registered C-step law for m = 2 (Amendment 6) is overlaid as a dashed step.
"""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(os.path.join(here, "data.json")))
colors = {"2": "#1f6feb", "3": "#d1242f", "4": "#1a7f37", "5": "#8250df", "6": "#bf8700"}

fig, axes = plt.subplots(1, 2, figsize=(11, 4.4))
for ax, xkey in zip(axes, ("t", "N")):
    for m, row in d["kummer_dF"].items():
        xs_e, ys_e, xs_c, ys_c = [], [], [], []
        for t, v in zip(row["t"], row["dF"]):
            x = t if xkey == "t" else int(m) * t
            if isinstance(v, str):
                xs_c.append(x); ys_c.append(int(v.lstrip(">=")))
            else:
                xs_e.append(x); ys_e.append(v)
        ax.plot(xs_e, ys_e, "-o", color=colors[m], label=f"m = {m}", ms=5)
        if xs_c:
            ax.plot(xs_c, ys_c, "^", mfc="none", color=colors[m], ms=8)
    ts = list(range(2, 18))
    cstep = [4 + (t - 2) // 4 for t in ts]
    xs = ts if xkey == "t" else [2 * t for t in ts]
    ax.step(xs, cstep, where="post", ls="--", color="#1f6feb", alpha=0.5, label="C-step (m = 2, registered)")
    ax.set_xlabel("tower height t" if xkey == "t" else "free unknowns N = m·t")
    ax.set_ylabel("last fall degree d_F (median of 2 draws)")
    ax.grid(alpha=0.3)
axes[0].set_title("A. d_F against t (Kummer, p = 2013265921)")
axes[1].set_title("B. d_F against N")
axes[0].legend(fontsize=8, loc="lower right")
fig.text(0.5, -0.02, "Filled: exact. Open triangle: one-sided bound (censored closure). ffd = 4 on every Kummer cell (t ≥ 2). "
         "Sources: data.json. [pcc:unverified]", ha="center", fontsize=8)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(here, f"fig_growth.{ext}"), dpi=150, bbox_inches="tight")
print("wrote fig_growth.svg, fig_growth.png")
