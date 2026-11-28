"""Curve25519 miniature, isogeny-tower base: F4 yield and cost against tower depth t.
Reads only data-u16xfer-c25519-t.json; writes fig-u16xfer-c25519-t.{pdf,svg}."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-u16xfer-c25519-t.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25, "svg.fonttype": "none"})
C = {2: "#E69F00", 3: "#0072B2"}
M = {2: "s", 3: "o"}
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 2.8))
for G in (2, 3):
    rows = [r for r in D["rows"] if r["G"] == G]
    x = [r["t"] + (0.06 if G == 2 else 0) for r in rows]
    a.errorbar(x, [r["rate"] for r in rows],
               yerr=[[r["rate"] - r["lo"] for r in rows], [r["hi"] - r["rate"] for r in rows]],
               marker=M[G], color=C[G], ls="-" if G == 3 else "none", ms=4.5, capsize=2.5, lw=1,
               label=f"$G={G}$")
    b.plot(x, [r["muladds_per_target"] for r in rows], marker=M[G], color=C[G],
           ls="-" if G == 3 else "none", ms=4.5, lw=1, label=f"$G={G}$")
a.set_yscale("log")
a.set_xticks([3, 4, 5, 6]); a.set_xlabel("tower depth $t$")
a.set_ylabel("hits per $10^9$ F4 mul-adds (95% interval)")
a.legend(fontsize=7, frameon=False, loc="lower left")
b.set_yscale("log")
b.set_xticks([3, 4, 5, 6]); b.set_xlabel("tower depth $t$")
b.set_ylabel("F4 mul-adds per target")
b.legend(fontsize=7, frameon=False, loc="upper left")
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(H, f"fig-u16xfer-c25519-t.{ext}"), bbox_inches="tight")
