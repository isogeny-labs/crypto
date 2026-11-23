"""Chained-S3 SAT at n=83: median seconds per decided query by summand count m.
Reads only data-bin-n83chain.json; writes fig-bin-n83chain.{pdf,svg}."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-bin-n83chain.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25, "svg.fonttype": "none"})
ARMS = [("order", "unordered", "unordered (A0)", "#999999", "o"),
        ("order", "all-summands", "all-summand order (A1)", "#0072B2", "s"),
        ("order", "all-summands+t2.2", "order + root tables (A2)", "#E69F00", "^"),
        ("order", "all-summands+affine+t2.2", "A2, affine only (A3)", "#CC79A7", "v"),
        ("meet", "MEET", "A2 + meet lookup (1 Nov)", "#009E73", "D")]
fig, ax = plt.subplots(figsize=(7.0, 3.0))
for study, arm, lab, c, mk in ARMS:
    for kind, fill in (("public", True), ("planted", False)):
        xs, ys = [], []
        for r in D[study]:
            ok = (r["arm"] == arm) if arm != "MEET" else ("meet" in r["arm"])
            if ok and r["kind"] == kind and r["median_seconds_decided"]:
                xs.append(int(r["m"]) + (0.08 if kind == "planted" else 0))
                ys.append(float(r["median_seconds_decided"]))
        if xs:
            ax.plot(xs, ys, marker=mk, color=c, ls="-" if fill else ":", ms=4.5, mfc=c if fill else "white",
                    label=lab if fill else None)
ax.plot([], [], marker="o", color="#333333", ls="none", label="public targets (refuted)")
ax.plot([], [], marker="o", color="#333333", mfc="white", ls=":", label="planted targets (witness)")
ax.set_yscale("log")
ax.set_xticks(range(2, 10))
ax.set_xlim(1.6, 9.4)
ax.set_xlabel("summands $m$ (cells with no point: no query decided within the cap)")
ax.set_ylabel("median wall time per decided query (s)")
ax.legend(fontsize=6.6, frameon=False, loc="upper left", bbox_to_anchor=(1.01, 1.0))
fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(H, f"fig-bin-n83chain.{ext}"), bbox_inches="tight")
