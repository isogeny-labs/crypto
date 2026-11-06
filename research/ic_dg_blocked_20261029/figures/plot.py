# Layout only: plots values recorded by `ecbench compare` (P<b>/wall/analysis/compare-*.json) and the
# phase sums written by cards/tools/phase_sums.py (P<b>/wall/analysis/phase_sums.json). Nothing is computed here.
# Run from the study directory:
#   python3 figures/plot.py
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

SIZES = (24, 28, 32)
DECIDES = {24: False, 28: True, 32: False}      # VERDICT.md: P24 and P32 undecidable
PREDICTION = {24: (0.81, 0.87), 28: (0.77, 0.84), 32: (0.78, 0.85)}  # PROTOCOL.md §7
PHASES = (("phases.oracle_setup.wall_ns", "oracle set-up", "#f59e0b"),
          ("online.phases_ns.target_PDP", "target PDP", "#7c3aed"),
          ("time.solve_wall_ns", "solve wall", "#6b7280"))

def load(path):
    return json.load(open(path))

fig, axs = plt.subplots(1, 3, figsize=(13, 3.9))

ax = axs[0]
for i, b in enumerate(SIZES):
    lo_p, hi_p = PREDICTION[b]
    ax.fill_between([i - 0.3, i + 0.3], lo_p, hi_p, color="#fde68a", alpha=0.6,
                    label="§7 prediction" if i == 0 else None)
    w = load(f"P{b}/wall/analysis/compare-dg__dgb-w0.json")["wall"]
    r, (lo, hi) = w["corrected_ratio"], w["corrected_ci95"]
    filled = DECIDES[b]
    ax.errorbar(i - 0.08, r, yerr=[[r - lo], [hi - r]], fmt="o", color="#dc2626", capsize=3,
                mfc="#dc2626" if filled else "none",
                label=("decides (P28)" if filled else "undecidable, descriptive") if i in (0, 1) else None)
    e = load(f"P{b}/wall/analysis/compare-dg__dgb-edge-w0.json")["wall"]
    r, (lo, hi) = e["corrected_ratio"], e["corrected_ci95"]
    ax.errorbar(i + 0.08, r, yerr=[[r - lo], [hi - r]], fmt="s", color="#2563eb", capsize=3, mfc="none",
                label="dgb-edge position control" if i == 0 else None)
ax.axhline(1.0, color="#6b7280", lw=0.8, ls="--")
ax.set_ylim(0.74, 1.02)
ax.set_title("wall: corrected dgb / dg ns per GAE (A/A-corrected, 95%)", fontsize=8)
ax.set_xticks(range(3)); ax.set_xticklabels([f"{b} bits" for b in SIZES])

ax = axs[1]
for i, b in enumerate(SIZES):
    w = load(f"P{b}/wall/analysis/compare-dg__dgb-w0.json")["wall"]
    ax.bar(i - 0.18, w["ns_per_gae_a"], 0.35, color="#9ca3af", label="dg (FastMap)" if i == 0 else None)
    ax.bar(i + 0.18, w["ns_per_gae_b"], 0.35, color="#dc2626", label="dgb (blocked packed)" if i == 0 else None)
ax.set_ylabel("ns per GAE (upper bound per op)", fontsize=8)
ax.set_title("wall: Σ solve wall / Σ total GAE, admitted L2 pairs", fontsize=8)
ax.set_xticks(range(3)); ax.set_xticklabels([f"{b} bits" for b in SIZES])

ax = axs[2]
for i, b in enumerate(SIZES):
    p = load(f"P{b}/wall/analysis/phase_sums.json")["by_arm"]
    for j, (key, name, col) in enumerate(PHASES):
        x = i + (j - 1) * 0.27
        ax.bar(x - 0.065, p["dg"][key] / 1e6, 0.12, color=col, alpha=0.45,
               label=f"{name}: dg" if i == 0 else None)
        ax.bar(x + 0.065, p["dgb"][key] / 1e6, 0.12, color=col,
               label=f"{name}: dgb" if i == 0 else None)
ax.set_yscale("log")
ax.set_ylabel("Σ over 288 measured runs (ms, log)", fontsize=8)
ax.set_title("phase sums (descriptive): set-up falls, PDP probe rises", fontsize=8)
ax.set_xticks(range(3)); ax.set_xticklabels([f"{b} bits" for b in SIZES])

axs[0].legend(fontsize=6.5, loc="upper left")
axs[1].legend(fontsize=7, loc="upper left")
axs[2].legend(fontsize=6, loc="upper left", ncol=2)
fig.suptitle("XP-20261029-dg-blocked: cache-blocked packed pair table vs FastMap oracle, deferred guard at N_inc, "
             "prime field, 2 curves × 24 targets × 6 rounds per size, L2 on isolab-eks c8a.2xlarge", fontsize=9)
fig.tight_layout()
fig.savefig("figures/ratios.svg"); fig.savefig("figures/ratios.png", dpi=150)
