"""Plot E28's recorded values (experiments/33_tau_stage_a/summary.json); computes nothing new."""
import json, sys, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = pathlib.Path(__file__).parent
s = json.load(open(sys.argv[1])); rows = s["rows"]
lab = [f"{r['p']}/{r['seed']}" for r in rows]; x = range(len(rows))
fig, ax = plt.subplots(figsize=(8, 3.4))
ax.bar([i - 0.2 for i in x], [r["residuals_tau"] for r in rows], 0.4, label="τ arm (E19's fold, 4 points a column)", color="#8c8c8c")
ax.bar([i + 0.2 for i in x], [r["residuals_tau_psi"] for r in rows], 0.4, label="τψ arm (⟨τ_T, ψ⟩, 8 points a column)", color="#d9772b")
ax.set_xticks(list(x)); ax.set_xticklabels(lab); ax.set_yscale("log")
ax.set_xlabel("curve p / seed (GLS twists on E19's line, h = 4)"); ax.set_ylabel("residuals to the verified log")
ax.set_title("E28 Stage A: one exact-oracle stream, both folds (pooled ratio 1.75)", fontsize=10)
ax.legend(fontsize=8, frameon=False); fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(here / f"stage_a.{ext}", **kw)
fig, ax = plt.subplots(figsize=(8, 3.4))
ax.scatter(x, [r["dist_tau"] for r in rows], s=18, color="#8c8c8c", label="τ arm vs negation walk (model)")
ax.scatter(x, [r["dist_tau_psi"][0] for r in rows], s=18, color="#d9772b", label="τψ arm vs ⟨±ψ⟩ walk (model)")
ax.scatter(x, [r["e19_ratio_same_p"] for r in rows], s=18, marker="x", color="#4a7fb5", label="E19 measured, same p (mean of its seeds)")
ax.axhspan(4.42e3, 2.79e5, color="#2ca02c", alpha=0.15, label="E31 A on E25's curves (measured, §9.24)")
ax.set_yscale("log"); ax.set_xticks(list(x)); ax.set_xticklabels(lab)
ax.set_ylabel("S / matched rho, 0 GAE"); ax.set_xlabel("curve p / seed")
ax.set_title("E28 Stage A2 (model from measured constants): distance to the matched rho", fontsize=10)
ax.legend(fontsize=7, frameon=False, loc="lower left"); fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(here / f"distance_model.{ext}", **kw)
