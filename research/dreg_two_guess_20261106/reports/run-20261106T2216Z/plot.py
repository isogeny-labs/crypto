"""Layout only.  Draws plot-data.json, which the native scorer produces from
the committed records:

    target/release/examples/dreg_two_guess_score --plot-data research > plot-data.json

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
arms = [("K2L", 4), ("K2S", 4), ("L3", 8), ("S3", 8)]
segs = [("refuted", "#2e86ab", "refutes at degree 4"),
        ("failed", "#d1495b", "saturates degree 4, no refutation"),
        ("unrun", "#dddddd", "not run (early stop)")]
draws = [c["draw"] for c in cells if c["arm"] == "K2L"]

fig, ax = plt.subplots(figsize=(12, 4.6))
ticks, names = [], []
for di, d in enumerate(draws):
    for ai, (arm, total) in enumerate(arms):
        c = next(c for c in cells if c["draw"] == d and c["arm"] == arm)
        x = di * 5 + ai
        bottom = 0
        for key, colour, text in segs:
            ax.bar(x, c[key] / total, bottom=bottom, color=colour, edgecolor="black", linewidth=0.4,
                   hatch="//" if arm in ("L3", "S3") else None,
                   label=text if (di, ai) == (0, 0) else None)
            bottom += c[key] / total
        ax.text(x, 1.03, "✓" if c["suffices"] else "✗", ha="center", fontsize=7)
        ticks.append(x)
        names.append(f"{arm}\n{d}" if ai == 0 else arm)
ax.set_xticks(ticks)
ax.set_xticklabels(names, fontsize=5.5)
ax.set_ylim(0, 1.12)
ax.set_ylabel("share of the arm's branches", fontsize=9)
ax.legend(fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.17), ncol=3)
ax.set_title(
    "Two guesses (K2L, K2S: 4 branches) against three (L3, S3: 8 branches, committed, hatched) at (22, 8); ✓ = every branch refutes\n"
    f"K2L suffices on {data['k2l_suffices']} of 12, K2S on {data['k2s_suffices']}, L3 on {data['l3_suffices']}, S3 on {data['s3_suffices']}; "
    f"K2L-only {data['k2l_only']}, K2S-only {data['k2s_only']}; p = {data['p_two_sided']} → {data['verdict']}",
    fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "branches.svg"))
fig.savefig(os.path.join(here, "branches.png"), dpi=150)
