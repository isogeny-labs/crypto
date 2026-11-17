"""Shared style for the fr figures (colour-blind-safe Okabe-Ito palette)."""
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none", "legend.frameon": False})
C = {"blue": "#0072B2", "orange": "#E69F00", "green": "#009E73", "pink": "#CC79A7",
     "grey": "#999999", "red": "#D55E00", "sky": "#56B4E9", "black": "#222222"}


def save(fig, here, slug):
    import os
    fig.tight_layout()
    for ext in ("pdf", "svg"):
        fig.savefig(os.path.join(here, f"fig-{slug}.{ext}"), bbox_inches="tight")
