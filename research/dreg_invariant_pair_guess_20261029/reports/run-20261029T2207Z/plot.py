"""Layout only.  Draws plot-data.json, which the native scorer produces from
the committed records:

    target/release/examples/dreg_linear_guess_pair_score --plot-data research > plot-data.json

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
arms = ["D3", "S3"]
segs = [("refuted", "#2e86ab", "refutes at degree 4"),
        ("failed", "#d1495b", "saturates degree 4, no refutation"),
        ("unrun", "#dddddd", "not run (early stop or killed)")]
draws = [c["draw"] for c in cells if c["arm"] == "D3"]

fig, ax = plt.subplots(figsize=(11, 4.4))
ticks, names = [], []
for di, d in enumerate(draws):
    for ai, arm in enumerate(arms):
        c = next(c for c in cells if c["draw"] == d and c["arm"] == arm)
        x = di * 3 + ai
        bottom = 0
        for key, colour, text in segs:
            ax.bar(x, c[key], bottom=bottom, color=colour, edgecolor="black", linewidth=0.5,
                   label=text if (di, ai) == (0, 0) else None)
            bottom += c[key]
        ax.text(x, 8.25, "✓" if c["suffices"] else "✗", ha="center", fontsize=9)
        ticks.append(x)
        names.append(f"{arm}\n{d}")
ax.set_xticks(ticks)
ax.set_xticklabels(names, fontsize=7)
ax.set_ylim(0, 9.3)
ax.set_ylabel("branches (of 8)", fontsize=9)
ax.legend(fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.17), ncol=3)
ax.set_title(
    "D3 = {a7+b7, a6+b6, e1[0]} against S3 = {a7+b7, c7, e1[0]} at (n, ℓ) = (22, 8); ✓ = all 8 branches refute\n"
    f"D3 suffices on {data['d3_suffices']} of 12, S3 on {data['s3_suffices']}; discordant D3-only {data['d3_only']}, "
    f"S3-only {data['s3_only']}; two-sided sign test p = {data['p_two_sided']} → {data['verdict']}",
    fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "branches.svg"))
fig.savefig(os.path.join(here, "branches.png"), dpi=150)
