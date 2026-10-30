#!/usr/bin/env python3
"""Two panels from results.json: S0*d/N' by recipe, and G across the section 5a band."""
import json, os
os.environ.setdefault("MPLCONFIGDIR", os.environ.get("TMPDIR", "/tmp"))
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
rows = json.load(open(os.path.join(HERE, "results.json")))
sizes = sorted({r["n"] for r in rows})
fig, (a1, a2) = plt.subplots(1, 2, figsize=(11, 4.2))
for i, n in enumerate(sizes):
    rs = [r for r in rows if r["n"] == n]
    for r in rs:
        sym = "sym" in r["recipe"] or n == 83 and "K3300" in r["recipe"]
        plain = "norm0" in r["recipe"] or "K600" in r["recipe"]
        x = i + (0.18 if sym else -0.18)
        y = r["bands"]["B0.0"]["S0d_over_Nprime"]
        a1.scatter(x, y, marker="s" if plain else "o", c="#c0392b" if sym else "#2c7fb8", s=28, zorder=3)
        a1.annotate(r["recipe"].replace(" norm", "/N").replace(" sym", "").replace(" (recorded, a=1)", ""),
                    (x, y), fontsize=6, xytext=(4, 0), textcoords="offset points", va="center")
a1.axhline(0.5, color="k", ls=":", lw=1)
a1.text(len(sizes) - 0.6, 0.53, "window threshold (sigma*delta = 0.318)", fontsize=7, ha="right")
a1.set_yscale("log")
a1.set_xticks(range(len(sizes)), [f"n={n}" for n in sizes])
a1.set_ylabel("S0 * d / N'   (GAE^2, w = 0, root call priced in field ops)")
a1.set_title("S0*d against the table bound (blue: plain build, red: pair symmetry)", fontsize=9)

best = {}
for r in rows:
    g = r["bands"]["B0.0"]["G"]
    if r["n"] not in best or g < best[r["n"]]["bands"]["B0.0"]["G"]:
        best[r["n"]] = r
for tag, lab, c in (("B0.0", "w = 0", "#2c7fb8"), ("B0.1", "B, w = 0.1", "#41ab5d"), ("B1.0", "B, w = 1", "#c0392b"), ("A1.0", "A (w on rho only), w = 1", "#999999")):
    xs = sorted(best)
    a2.plot(xs, [best[n]["bands"][tag]["G"] for n in xs], marker="o", label=lab, color=c)
xs = sorted(best)
a2.errorbar(xs, [best[n]["bands"]["B0.0"]["G"] for n in xs],
            yerr=[[best[n]["bands"]["A0.0"]["G"] - best[n]["bands"]["A0.0"]["G_lo"] for n in xs],
                  [best[n]["bands"]["A0.0"]["G_hi"] - best[n]["bands"]["A0.0"]["G"] for n in xs]],
            fmt="none", ecolor="#2c7fb8", capsize=3)
a2.axhline(1, color="k", lw=1)
a2.set_xlabel("field degree n")
a2.set_ylabel("G = min_k IC(k) / batch rho(k)  (log scale)")
a2.set_yscale("log")
a2.set_title("Best measured recipe per size, section 5a band", fontsize=9)
a2.legend(fontsize=7)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "s0d_and_g.svg"))
fig.savefig(os.path.join(HERE, "s0d_and_g.png"), dpi=150)
