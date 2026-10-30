"""Presentation only: draw values already recorded by the independent native audit."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
DATA = json.loads((HERE / "coverage-data.json").read_text())
fixture = DATA["F43_prefix_size2"]
profile = fixture["profile"]
plt.rcParams.update({"font.size": 11, "svg.fonttype": "none"})
fig, axes = plt.subplots(1, 2, figsize=(11, 3.8), constrained_layout=True)
axes[0].bar(["Main prefix", "Twist prefix"],
            [fixture["main_first_target"]["after"], fixture["twist_first_target"]["after"]],
            color=["#2563a6", "#c15a18"])
axes[0].set_ylim(0, 2.5)
axes[0].set_yticks([0, 1, 2])
axes[0].set_ylabel("Independent coefficient rows mod 11")
axes[0].set_title("First public relation target, b=2")
axes[0].text(0.5, 2.25, "3 ordered twist roots give 2 independent rows",
             ha="center", fontsize=9)
axes[1].plot([r["n"] for r in profile], [r["main_closure"] for r in profile],
             "o-", color="#2563a6", label="Main prefix")
axes[1].plot([r["n"] for r in profile], [r["twist_closure"] for r in profile],
             "s--", color="#c15a18", label="Twist prefix")
axes[1].set_xticks([r["n"] for r in profile])
axes[1].set_yticks([4, 5, 6])
axes[1].set_ylim(3.7, 6.5)
axes[1].set_xlabel("Public target scalar n, R=nP")
axes[1].set_ylabel("Finite vanishing-space closure degree")
axes[1].set_title("All ten nonzero targets: equal means 52/10")
axes[1].legend(loc="upper right", fontsize=9)
for ax in axes:
    ax.spines[["top", "right"]].set_visible(False)
    ax.grid(axis="y", alpha=0.18)
fig.suptitle("F43 actual S3 systems · s3-yield-20261024", fontsize=13)
for ext in ("svg", "pdf", "png"):
    fig.savefig(HERE / f"profiles.{ext}", dpi=150)
    if ext == "svg":
        svg = HERE / "profiles.svg"
        svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
