"""IC leaderboard (docs/ic/leaderboard.json at origin/main): every board row's ratio to its
matched reference, with intervals where the board carries them, and Table B's ratio across
the pricing band. Reads data-fr-leaderboard.json only.

    python3 make_fig_fr_leaderboard.py   # writes fig-fr-leaderboard.{pdf,svg}
"""
import json
import os

from make_fig_fr_common import C, plt, save

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-fr-leaderboard.json")))
rows = D["rows"]
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 3.1), gridspec_kw={"width_ratios": [1.25, 1]})
groups = [
    ("A, Koblitz one target (validated)", lambda r: r["family"] == "A", C["blue"], "^", True),
    ("B, prime (validated)", lambda r: r["family"] == "B" and r["regime"] == "prime", C["orange"], "o", True),
    ("B, random binary (validated)", lambda r: r["family"] == "B" and r["regime"] == "char2", C["green"], "s", True),
    ("C, Koblitz 32-target batch (pre-gate)", lambda r: r["family"] == "C", C["pink"], "D", False),
]
for lab, f, col, m, filled in groups:
    g = sorted([r for r in rows if f(r)], key=lambda r: r["log2_r"])
    x = [r["log2_r"] for r in g]
    y = [r["ratio_rho"] for r in g]
    for r in g:
        if r["ratio_rho_ci"]:
            lo, hi = r["ratio_rho_ci"]
            a.plot([r["log2_r"]] * 2, [lo, hi], color=col, lw=1.0)
    a.plot(x, y, m, color=col, mfc=col if filled else "white", ms=4.5, label=lab, ls="none")
a.axhline(1, color=C["black"], ls="--", lw=0.9)
a.set_yscale("log")
a.set_ylim(0.8, 60)
a.set_xlabel(r"$\log_2 r$")
a.set_ylabel("IC / matched reference (each family's unit)")
a.legend(fontsize=6.3, loc="upper left")
a.text(0.98, 0.95, "(a)", transform=a.transAxes, ha="right", va="top")
B = sorted([r for r in rows if r["family"] == "B"], key=lambda r: r["log2_r"])
wcol = [C["blue"], C["orange"], C["red"]]
for i, w in enumerate(["0", "0.1", "1"]):
    for reg, m in (("prime", "o"), ("char2", "s")):
        g = [r for r in B if r["regime"] == reg]
        b.plot([r["log2_r"] + (i - 1) * 0.18 for r in g], [r["band_ratio_w"][i] for r in g], m,
               color=wcol[i], ms=3.8, ls="none",
               label=(f"$w={w}$" if reg == "prime" else None))
g = B
b.plot([r["log2_r"] + 0.36 for r in g], [r["band_ratio_extended_w"][2] for r in g], "x", color=C["grey"],
       ms=4, ls="none", label="$w=1$, extended charge")
b.axhline(1, color=C["black"], ls="--", lw=0.9)
b.set_yscale("log")
b.set_ylim(0.8, 60)
b.set_xlabel(r"$\log_2 r$  (circles prime, squares binary)")
b.set_ylabel("Table B: IC / negation rho at weight $w$")
b.legend(fontsize=6.3, loc="upper left")
b.text(0.98, 0.95, "(b)", transform=b.transAxes, ha="right", va="top")
save(fig, HERE, "fr-leaderboard")
