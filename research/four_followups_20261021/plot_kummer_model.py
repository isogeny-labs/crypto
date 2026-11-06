"""Layout only: plot native Kummer-model gate rows."""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
rows = [json.loads(line) for line in (ROOT / "results/kummer-model-gate.jsonl").read_text().splitlines()]
assert len(rows) == 21
fig, ax = plt.subplots(figsize=(9.3, 4.2), layout="constrained")
for i, row in enumerate(rows):
    assert row["factorization_verified"] and row["finite_root_count"] == 5
    ax.scatter([i] * 5, row["finite_root_fraction_of_p"],
               s=21, color="#176b8a", zorder=3)
ax.set_xticks(range(21), [str(row["p"]) for row in rows], rotation=60, ha="right", fontsize=7)
ax.set_ylim(-0.04, 1.04)
ax.set_ylabel(r"Rational root $x/p$")
ax.set_xlabel("Prime field of the transferred genus-2 model")
ax.set_title("All five finite Weierstrass roots lie in the base field on 21/21 models")
ax.grid(axis="y", alpha=0.22)
fig.savefig(ROOT / "kummer-model-roots.svg")
fig.savefig(ROOT / "kummer-model-roots.png", dpi=190)
