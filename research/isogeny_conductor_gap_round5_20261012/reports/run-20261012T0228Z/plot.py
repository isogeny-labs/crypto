"""Plot E6's yield ratios (tables.json from examples/isogeny_gap_round5_tables.rs); lays out recorded values."""
import json, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
t = json.load(open(os.path.join(here, "../../tables.json")))
fig, ax = plt.subplots(figsize=(7, 4.4))
rows = [r for r in t["rows"] if r["arm"] != "direct" and r["ratio"]]
for c, col in ((32, "#9db7d5"), (64, "#3b6ea8")):
    xs = [r["subgroup_order"] * (1.04 if c == 64 else 0.96) for r in rows if r["base_size"] == c]
    ys = [r["ratio"] for r in rows if r["base_size"] == c]
    ax.scatter(xs, ys, s=14, color=col, alpha=0.7, label=f"per curve, c = {c}")
for p in t["pooled"]:
    ax.errorbar([p["subgroup_order"]], [p["ratio"]], yerr=[[p["ratio"] - p["low"]], [p["high"] - p["ratio"]]],
                fmt="s", color="#c0632b", ms=6, capsize=3, zorder=4)
ax.scatter([], [], marker="s", color="#c0632b", label="pooled, 95 % Poisson interval")
xs = [1.5, 250]
ax.plot(xs, xs, color="black", lw=1, ls=":", label="ratio = |H| (prediction)")
f = t["exponent_fit"]
ax.plot(xs, [f["ratio_at_h1"] * x ** f["slope"] for x in xs], color="#c0632b", lw=1,
        label=f"fit: exponent {f['slope']:.3f} ± {f['slope_half_width_95']:.3f}")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("|H| (subgroup quotiented by)"); ax.set_ylabel("quotient ÷ direct decompositions")
ax.set_title("E6: yield of a pushed-forward base, 15 curves × 20,000 targets", fontsize=9)
ax.grid(True, which="both", alpha=0.3); ax.legend(fontsize=7, loc="upper left")
fig.tight_layout(); fig.savefig(os.path.join(here, "e6_yield_ratio.svg")); print("ok")
