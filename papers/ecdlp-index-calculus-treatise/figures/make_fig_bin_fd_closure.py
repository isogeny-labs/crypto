"""Per-instance closure size and time of the binary Semaev m=2 descents.
Reads only data-bin-fd-instances.json; writes fig-bin-fd-closure.{pdf,svg}."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-bin-fd-instances.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25, "svg.fonttype": "none"})
C = {"P": "#0072B2", "R": "#E69F00", "U": "#009E73", "T": "#CC79A7", "S": "#D55E00"}
MK = {"P": "o", "R": "s", "U": "D", "T": "^", "S": "v"}
LAB = {"P": "P: polynomial V", "R": "R: random V", "U": "U: random V, uniform z",
       "T": "T: V in ker Tr", "S": "S: polynomial V, uniform z"}
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 3.0))
off = {"P": -0.3, "R": -0.1, "U": 0.1, "T": 0.3, "S": 0.0}
for r in D["m2"]:
    f = r["name"][0]
    x = r["n"] + off[f]
    filled = r["W_one_top"] == 1
    kw = dict(marker=MK[f], color=C[f], ms=4.5, mfc=C[f] if filled else "white", ls="none", zorder=3)
    if r["D_top"] == 4:
        a.plot(x, r["ncols_top"], **kw)
    b.plot(x, max(r["seconds_top"], 0.05), **kw)
for f in "PRUTS":
    a.plot([], [], marker=MK[f], color=C[f], ls="none", ms=4.5, label=LAB[f])
a.plot([], [], marker="o", color="#333333", mfc="white", ls="none", ms=4.5, label="1 not in $W_4$ (open)")
a.set_yscale("log"); b.set_yscale("log")
a.set_xlabel("$n$ (field $\\mathbb{F}_{2^n}$, $k=\\lceil n/2\\rceil$)")
a.set_ylabel("monomial columns at closure degree 4")
b.set_xlabel("$n$ (field $\\mathbb{F}_{2^n}$, $k=\\lceil n/2\\rceil$)")
b.set_ylabel("closure wall time at top degree (s)")
a.legend(fontsize=6.3, loc="upper left", frameon=False)
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(H, f"fig-bin-fd-closure.{ext}"), bbox_inches="tight")
