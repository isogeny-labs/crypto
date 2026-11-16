"""Render already recorded native results. No research computation or fitting."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
rows = json.loads((root / "cost_surface_amendment_01.json").read_text())["rows"]
fig, ax = plt.subplots(figsize=(8, 4.6))
for model, style in [("constant4", "-"), ("constant5", "--")]:
    for family, marker, color in [("k1", "o", "#146083"), ("r1", "s", "#a54829")]:
        selected = sorted((r for r in rows if r["m"] == 4 and r["w"] == 2.0
                           and r["assumed_conversion"] == 1.0 and r["model"] == model
                           and r["curve"].endswith(family)), key=lambda r: r["n"])
        ax.plot([r["n"] for r in selected], [r["conditional_gap_bits"] for r in selected],
                linestyle=style, marker=marker, color=color,
                label=f"sect...{family}, D={model[-1]}")
ax.axhline(0, color="#999999", linewidth=1)
ax.set(xlabel="Field degree n", ylabel="Modeled partial cost minus rho (bits)",
       title="Conditional binary descent surface: m=4, w=2, conversion=1")
ax.text(.01, .02, "Unvalidated degrees; unpriced phases; no security conclusion",
        transform=ax.transAxes, fontsize=8)
ax.grid(alpha=.15)
ax.legend(frameon=False, fontsize=9)
fig.tight_layout()
for suffix in ["svg", "png"]:
    fig.savefig(root / f"cost_surface.{suffix}", dpi=150)
plt.close(fig)

fourier = json.loads((root / "fourier_recovery.json").read_text())["rows"]
fig, ax = plt.subplots(figsize=(9, 4))
labels = [f"r={r['r']}\nd={r['synthetic_secret']}" for r in fourier]
bottom = [0.0] * len(fourier)
for key, name, color in [("correct_candidate_probability", "Correct", "#146083"),
                         ("wrong_candidate_probability", "Wrong", "#a54829"),
                         ("rejected_probability", "Rejected", "#a8b0b3")]:
    values = [r[key] for r in fourier]
    ax.bar(labels, values, bottom=bottom, label=name, color=color)
    bottom = [a+b for a,b in zip(bottom, values)]
ax.set(ylim=(0, 1), ylabel="Calculated Fourier outcome probability mass",
       title="Small cyclic groups: nearest-character decoding of power-of-two Fourier outputs")
ax.legend(frameon=False, ncol=3, loc="lower right", fontsize=8)
ax.tick_params(axis="x", labelsize=8)
fig.tight_layout()
for suffix in ["svg", "png"]:
    fig.savefig(root / f"fourier_recovery.{suffix}", dpi=150)
plt.close(fig)

q = json.loads((root / "quantum_boundary.json").read_text())
selected = [r for r in q["resources"] if r["shots"] == 1
            and r["assumed_independent_per_shot_success"] == .5]
fig, axes = plt.subplots(1, 2, figsize=(9, 4.0))
axes[0].bar([str(r["field_bits"]) for r in selected],
            [r["rnsl2017_logical_qubit_bound"] for r in selected], color="#146083")
axes[0].set(xlabel="Prime-field bits", ylabel="Logical qubit bound", title="RNSL 2017 formula")
ds = [r for r in q["diagnostics"] if r["synthetic_secret"] == 17]
axes[1].bar([str(r["b"]) for r in ds], [r["extended_euclid_divisions"] for r in ds],
            color=["#a54829" if r["recovered"] is None else "#146083" for r in ds])
axes[1].set(xlabel="Exact-congruence coefficient b", ylabel="Euclid divisions",
            title="Native algebraic diagnostic; d=17")
fig.suptitle("Separate resource units; no quantum execution or full postprocessor", fontsize=10)
fig.tight_layout()
for suffix in ["svg", "png"]:
    fig.savefig(root / f"quantum_boundary.{suffix}", dpi=150)
plt.close(fig)
