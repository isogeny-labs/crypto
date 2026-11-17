"""Layout only.  Draws plot-data.json, which the native scorer produces from
the committed records:

    target/release/examples/dreg_m4_link_score --plot-data research > plot-data.json

Nothing is classified or counted here: each bar is a `refuted` count computed
in Rust, and the title's figures are fields of the same file."""
import json
import os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
data = json.load(open(os.path.join(here, "plot-data.json")))
cells = data["cells"]
draws = [(c["draw"], c["n"]) for c in cells if c["arm"] == "I"]
arms = [("I", "#2e86ab", "I: one invariant guess per link"),
        ("R", "#d1495b", "R: raw guesses in the same places")]

fig, ax = plt.subplots(figsize=(10, 4.2))
for ai, (arm, colour, text) in enumerate(arms):
    xs, ys = [], []
    for di, (d, n) in enumerate(draws):
        c = next(c for c in cells if c["draw"] == d and c["arm"] == arm)
        xs.append(di * 3 + ai)
        ys.append(c["refuted"])
    ax.bar(xs, ys, color=colour, edgecolor="black", linewidth=0.5, label=text)
ax.set_xticks([di * 3 + 0.5 for di in range(len(draws))])
ax.set_xticklabels([f"draw {d}\n(n = {n})" for d, n in draws], fontsize=8)
ax.set_ylim(0, 1024)
ax.set_ylabel("branches refuting at degree 3 (of 1024)", fontsize=9)
ax.legend(fontsize=8, loc="upper right")
ax.set_title(
    "m = 4, ℓ = 5, k = 10 guesses, guess-then-degree-3\n"
    f"pooled I {data['pooled_i']}/8192, R {data['pooled_r']}/8192; draws I > R {data['i_more']}, "
    f"R > I {data['r_more']}; two-sided sign test p = {data['p_two_sided']} → {data['verdict']}",
    fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "branches.svg"))
fig.savefig(os.path.join(here, "branches.png"), dpi=150)
