import json, math, pathlib
import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
D = pathlib.Path(__file__).resolve().parents[1]; s = json.load(open(D / "data/sim.json"))
F = list(range(0, 6)); model = [math.gamma(f + 1.5) / (math.gamma(f + 1) * math.gamma(1.5)) for f in F]
fig, ax = plt.subplots(figsize=(8, 4.6))
ax.plot(F, model, "-o", color="#2a6f97", label="model  Γ(F+3/2) / (Γ(F+1) Γ(3/2))")
ax.errorbar([int(k) - 1 for k in s["k"]], [v["ratio_to_k1"] for v in s["k"].values()],
            yerr=[2 * v["sem"] / s["k"]["1"]["mean_n"] for v in s["k"].values()], fmt="s", color="#e07a1f",
            label="Monte Carlo, 4,000 trials (±2 SEM)")
meas = [(2, 1.82, "autolab R4-I1 k = 32"), (2, 1.86, "R4-I1 k = 28"), (2, 1.94, "re-baseline k = 24"), (2, 1.81, "re-baseline k = 28")]
ax.scatter([m[0] + .08 for m in meas], [m[1] for m in meas], marker="x", color="#b23a48", s=50, label="measured wwalk ÷ rho-neg (ecbench, seed-averaged or seed = run)")
ax.axhline(1, color="black", lw=.8, ls="--"); ax.text(4.1, 1.03, "rho parity", fontsize=8)
ax.set_xlabel("F = factor-base elements with unknown logarithm (rows needed = F + 1)")
ax.set_ylabel("expected group operations ÷ rho")
ax.set_title("Generic index calculus: collision cost relative to rho (negation-folded walk)")
ax.legend(fontsize=7.5, loc="upper left"); ax.grid(alpha=.3)
fig.text(.01, -.03, "Sources: data/sim.json (scripts/sim.py); research/curve25519_autolab_20261104/{r4-i1,rebaseline-seedrun}/README.md [pcc:unverified].", fontsize=7, color="#444")
fig.tight_layout(); fig.savefig(D / "figures/parity_floor.svg", bbox_inches="tight"); fig.savefig(D / "figures/parity_floor.png", dpi=150, bbox_inches="tight")
