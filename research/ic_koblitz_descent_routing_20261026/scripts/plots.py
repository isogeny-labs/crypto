"""Lay out the band means and paired comparisons as SVG + 150 dpi PNG. Plotting
only: every number is read from analysis/band-*.json and
sessions/*/comparisons/*.json (ecbench output)."""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(__file__).resolve().parent.parent
out = here / "figures"
out.mkdir(exist_ok=True)
tags = ["k47", "k41", "k53"]
bands = {t: json.loads((here / "analysis" / f"band-{t}.json").read_text()) for t in tags}
x = [bands[t]["rows"][0]["log2_r"] for t in tags]
fig, axes = plt.subplots(1, 2, figsize=(10, 4.0))
for arm, c, lab in (("rho", "black", "rho.signed_frobenius_strong"), ("pipeline", "#4f81bd", "ic.pipeline (planner)"),
                    ("descent", "#c0504d", "ic.koblitz_descent (§23)")):
    axes[0].plot(x, [next(r["mean_s"][0] for r in bands[t]["rows"] if r["arm"] == arm) for t in tags], "o-", color=c, label=lab)
axes[0].set_yscale("log")
axes[0].set_xlabel("log2 r")
axes[0].set_ylabel("mean S (counted, w = 0)")
axes[0].set_title("Koblitz one-target cost in one unit", fontsize=9)
axes[0].legend(fontsize=7)
axes[0].grid(alpha=0.3, which="both")
for k, (a, b, c) in enumerate((("pipeline", "descent", "#c0504d"), ("rho", "pipeline", "#4f81bd"), ("rho", "descent", "#7f7f7f"))):
    cs = [json.loads((here / "sessions" / t / "comparisons" / f"{a}__{b}.json").read_text())["ops"] for t in tags]
    ys = [o["ratio_b_over_a"] for o in cs]
    axes[1].errorbar([v + (k - 1) * 0.25 for v in x], ys, yerr=[[y - o["ci95"][0] for y, o in zip(ys, cs)],
                     [o["ci95"][1] - y for y, o in zip(ys, cs)]], fmt="o-", color=c, capsize=3, label=f"{b} / {a}")
axes[1].axhline(1, color="black", lw=0.8)
axes[1].set_yscale("log")
axes[1].set_xlabel("log2 r")
axes[1].set_ylabel("paired ratio, 95% interval")
axes[1].set_title("The §23 descent costs 1.41–1.49× the planner's recipe", fontsize=9)
axes[1].legend(fontsize=7)
axes[1].grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, "ecbench GAE, S = GAE/sqrt(r); 24 verified runs per arm and curve; registered Koblitz curves (README).", fontsize=6)
fig.tight_layout()
fig.savefig(out / "routing.svg")
fig.savefig(out / "routing.png", dpi=150)
print("wrote", sorted(p.name for p in out.iterdir()))
