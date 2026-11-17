"""Rendering only: draw recorded values, without recomputing research results.

This is the explicit report-layout exception to the repository's no-Python
research-execution rule. The arithmetic checker and new execution use Rust.
"""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).parent
data = json.loads((here.parent / "historical/cold_target_audit_q1061_v65.json").read_text())
s = data["summary"]
fig, axes = plt.subplots(1, 2, figsize=(10.2, 4.2))
values = [s["cover_mean_seconds"], s["direct_mean_seconds"]]
axes[0].bar(["Cover", "BSGS + negation"], values, color=["#286a8d", "#a5633b"])
axes[0].set_ylim(0, 0.1)
axes[0].set_ylabel("Recorded mean cold wall time (seconds)")
for i, value in enumerate(values):
    axes[0].text(i, value + .002, f"{value:.6f} s", ha="center", fontsize=9)
axes[1].plot([0, .16], [0, .16], color="gray", lw=1, label="Equal recorded time")
axes[1].scatter([r["cover_seconds"] for r in data["raw_trials"]],
                [r["direct_seconds"] for r in data["raw_trials"]], s=18, color="#286a8d", alpha=.7)
axes[1].set(xlim=(0, .16), ylim=(0, .16), xlabel="Cover (seconds)", ylabel="BSGS (seconds)")
axes[1].legend(fontsize=8)
fig.suptitle("Historical Python audit: 64 cold targets · PCC unverified", fontsize=12)
fig.text(.5, .015, "Frozen record only · isolation/A/A unknown · no counted-rho or native timing claim", ha="center", fontsize=8)
fig.tight_layout(rect=(0, .04, 1, .94))
fig.savefig(here / "recorded-wall.svg")
fig.savefig(here / "recorded-wall.png", dpi=150)
svg = here / "recorded-wall.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
