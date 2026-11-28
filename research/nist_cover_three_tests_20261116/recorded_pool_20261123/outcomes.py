"""Plot recorded audit values only; no research arithmetic or data generation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
initial = json.loads((root / "evidence/qualification-v2/result.json").read_text())
signed = json.loads((root / "evidence/signed-v2/result.json").read_text())
traces = json.loads((root / "evidence/traces-v1/result.json").read_text())
plt.rcParams.update({"svg.fonttype": "none", "font.size": 10})
fig, (ax, rank) = plt.subplots(1, 2, figsize=(9, 3), layout="constrained", gridspec_kw={"width_ratios": [1.15, 1]})
labels = ["Split into two\nrational supports", "Irreducible\nquadratic", "Repeated /\ndegree exception"]
values = [traces["split_distinct_rational"], traces["irreducible_quadratic"], traces["repeated_contact"] + traces["affine_degree_exceptions"]]
bars = ax.bar(labels, values, color=["#287e8e", "#52687d", "#9daab3"])
ax.bar_label(bars, padding=3)
ax.set_ylim(0, 145)
ax.set_ylabel("Recorded tangents")
ax.set_title("Residual splitting on 226 fixed tangents", fontsize=10)
names = ["Baseline", "All norm\nquartets", "Signed\nshort words", "Tangent\ntrace aliases"]
values = [initial["baseline_rank"], initial["all_norm_rank"], signed["union_rank"], traces["union_rank"]]
bars = rank.bar(names, values, color="#426e9e")
rank.bar_label(bars, padding=3)
rank.set_ylim(0, 8)
rank.set_ylabel("Coefficient rank on old base")
rank.set_title("No additional closed rows in tested domains", fontsize=10)
for a in (ax, rank):
    a.spines[["top", "right"]].set_visible(False)
    a.tick_params(axis="x", labelsize=8)
fig.savefig(root / "outcomes.svg")
fig.savefig(root / "outcomes.png", dpi=100)
# Normalize serializer whitespace only; plotted data and geometry are unchanged.
svg = root / "outcomes.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
