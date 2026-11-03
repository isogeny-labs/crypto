"""Layout only.  Draws plot-data.json, which the native scorer produces from
the committed records:

    target/release/examples/dreg_linear_guess_score --plot-data \
        research/dreg_invariant_guess_20261029 > plot-data.json

Nothing is classified or counted here: each bar's segments are the
`refuted`, `failed` and `unrun` branch counts computed in Rust."""
import json
import os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
data = json.load(open(os.path.join(here, "plot-data.json")))
cells = data["cells"]
draws = sorted({c["draw"] for c in cells})
arms = ["S3", "X3"]
label = {"S3": "S3: invariant guess a7 + b7", "X3": "X3: raw guess a7"}
seg = [("refuted", "#2e86ab", "refutes at degree 4"),
       ("failed", "#d1495b", "saturates degree 4, no refutation"),
       ("unrun", "#dddddd", "not run (early stop or killed)")]

fig, ax = plt.subplots(figsize=(9.5, 4.4))
ticks, names = [], []
for di, d in enumerate(draws):
    for ai, arm in enumerate(arms):
        c = next(c for c in cells if c["draw"] == d and c["arm"] == arm)
        x = di * 3 + ai
        bottom = 0
        for key, colour, text in seg:
            ax.bar(x, c[key], bottom=bottom, color=colour, edgecolor="black",
                   linewidth=0.6, label=text if (di, ai) == (0, 0) else None)
            bottom += c[key]
        ax.text(x, 8.2, c["verdict"].replace("does not suffice", "fails"),
                ha="center", fontsize=7.5, rotation=0)
        ticks.append(x)
        names.append(f"{arm}\ndraw {d}")
ax.set_xticks(ticks)
ax.set_xticklabels(names, fontsize=8)
ax.set_ylim(0, 9.2)
ax.set_ylabel("branches (of 8)", fontsize=9)
ax.legend(fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=3)
ax.set_title("Three guesses at (n, ℓ) = (22, 8): the invariant coordinate of s = x1 + x2 against a raw coordinate\n"
             f"registered verdict: {data['hypothesis']} — research/dreg_invariant_guess_20261029, runs 2026-10-29",
             fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "branches.svg"))
fig.savefig(os.path.join(here, "branches.png"), dpi=150)
