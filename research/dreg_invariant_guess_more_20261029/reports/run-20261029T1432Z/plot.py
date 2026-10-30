"""Layout only.  Draws plot-data.json, which the native scorer produces from
the committed records:

    target/release/examples/dreg_linear_guess_more_score --plot-data \
        research/dreg_invariant_guess_more_20261029 research/dreg_invariant_guess_20261029 \
        > plot-data.json

Nothing is classified or counted here: each bar's segments are the
`refuted`, `failed` and `unrun` branch counts computed in Rust, and the
title's figures are fields of the same file."""
import json
import os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
data = json.load(open(os.path.join(here, "plot-data.json")))
cells = data["cells"]
arms = ["S3", "X3"]
colour_seg = [("refuted", "#2e86ab", "refutes at degree 4"),
              ("failed", "#d1495b", "saturates degree 4, no refutation"),
              ("unrun", "#dddddd", "not run (early stop)")]

fig, axes = plt.subplots(2, 1, figsize=(11, 6.4), gridspec_kw={"height_ratios": [2, 1]})
for ax, study, title in [(axes[0], "new", "registered draws (primary)"),
                         (axes[1], "first", "the first study's draws (secondary)")]:
    draws = [c["draw"] for c in cells if c["study"] == study and c["arm"] == "S3"]
    ticks, names = [], []
    for di, d in enumerate(draws):
        for ai, arm in enumerate(arms):
            c = next(c for c in cells if c["study"] == study and c["draw"] == d and c["arm"] == arm)
            x = di * 3 + ai
            bottom = 0
            for key, colour, text in colour_seg:
                ax.bar(x, c[key], bottom=bottom, color=colour, edgecolor="black", linewidth=0.5,
                       label=text if (study, di, ai) == ("new", 0, 0) else None)
                bottom += c[key]
            ax.text(x, 8.25, "✓" if c["suffices"] else "✗", ha="center", fontsize=9)
            ticks.append(x)
            names.append(f"{arm}\n{d}")
    ax.set_xticks(ticks)
    ax.set_xticklabels(names, fontsize=7)
    ax.set_ylim(0, 9.3)
    ax.set_ylabel("branches", fontsize=8)
    ax.set_title(title, fontsize=9, loc="left")
axes[0].legend(fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.2), ncol=3)
fig.suptitle(
    "Invariant guess a7 + b7 (S3) against raw guess a7 (X3) at (n, ℓ) = (22, 8), three guesses each; ✓ = all 8 branches refute\n"
    f"primary: S3-only {data['s_only']}, X3-only {data['x_only']} discordant draws, one-sided sign test p = {data['p_s3']} → {data['verdict']}"
    f"; with the first 4 draws: {data['combined_s_only']} of {data['combined_discordant']}, p = {data['combined_p_s3']}",
    fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "branches.svg"))
fig.savefig(os.path.join(here, "branches.png"), dpi=150)
