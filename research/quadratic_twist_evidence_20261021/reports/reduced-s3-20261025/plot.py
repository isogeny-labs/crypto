"""Presentation only: plot already-recorded native degree and rank values."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
data = json.loads((here / "coverage-data.json").read_text())
old, new = data["original"], data["reduced"]
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none", "pdf.use14corefonts": True})
fig, axes = plt.subplots(1, 3, figsize=(11, 3.2), constrained_layout=True)
labels = ["Main", "Twist"]
axes[0].bar([-.18, .82], [old[k]["targets"][0]["degree"]["closure"] for k in ("main", "twist")], width=.35, label="Original", color="#94a3b8")
axes[0].bar([.18, 1.18], [new[k]["targets"][0]["affine"]["closure"] for k in ("main", "twist")], width=.35, label="Reduced", color="#2563a6")
axes[0].set_xticks([0, 1], labels)
axes[0].set_title("First target: finite closure degree")
axes[0].set_yticks([0, 2, 4, 6])
axes[0].legend(fontsize=8)
axes[1].bar([-.18, .82], [old["homogeneous_regularity"], old["homogeneous_regularity"]], width=.35, color="#94a3b8")
axes[1].bar([.18, 1.18], [new[k]["targets"][0]["homogeneous"]["cutoff"] for k in ("main", "twist")], width=.35, color="#2563a6")
axes[1].set_xticks([0, 1], labels)
axes[1].set_title("Leading-form first-empty degree")
axes[1].set_yticks([0, 1, 2, 3])
for role, style, label in [("main", "o-", "Main reduced"), ("twist", "s--", "Twist reduced")]:
    rows = new[role]["targets"]
    axes[2].plot([r["n"] for r in rows], [r["affine"]["closure"] for r in rows], style, label=label)
axes[2].set_title("Every nonzero F43 target")
axes[2].set_xlabel("Public scalar n, target nP")
axes[2].set_xticks([1, 3, 5, 7, 9, 10])
axes[2].set_yticks([2, 3, 4])
axes[2].legend(fontsize=8)
for ax in axes:
    ax.spines[["top", "right"]].set_visible(False)
    ax.grid(axis="y", alpha=.18)
fig.suptitle("Exact S3 generator preprocessing · reduced-s3-20261025", fontsize=12)
fig.savefig(here / "degrees.svg")
fig.savefig(here / "degrees.pdf")
svg = here / "degrees.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
