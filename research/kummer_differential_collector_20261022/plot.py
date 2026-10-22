"""Render full source-paired cold costs and all 36 pricing cells."""
import csv
import json
from pathlib import Path
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

B = Path(__file__).resolve().parent
R = B / "results"
s = json.loads((R / "summary.json").read_text())
grid = list(csv.DictReader((R / "price-grid.csv").open()))
arms = s["arms"]
def matching(**kwargs):
    return next(r for r in grid if all((float(r[k]) == float(v)) if k in
                ("S_per_M", "I_per_M", "native_GAE_per_logical") else r[k] == str(v)
                for k, v in kwargs.items()))

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5), layout="constrained")
bits = [16, 20, 24, 28, 32, 36, 40]
for arm in arms:
 marker = "x" if arm == "uncached" else "o"
 for inv, color in [(1, "#23738b"), (10, "#2d8d76"), (100, "#d18b32"), (1000, "#ae4c4c")]:
    y = [float(matching(arm=arm, slice=f"bits{b}", S_per_M=0.5, I_per_M=float(inv),
                        native_GAE_per_logical=0.1)["candidate_cold_over_rho"]) for b in bits]
    ax1.plot(bits, y, marker=marker, color=color, linestyle="--" if arm == "uncached" else "-", label=f"{arm}, I/M={inv}")
ax1.set_xlabel("Frozen target size (bits)")
ax1.set_ylabel("Complete cold candidate / same-source rho")
ax1.set_title("Cold ratio by bit size; S/M=0.5, native=0.1 GAE")
ax1.grid(alpha=0.2)
ax1.legend(fontsize=7)

heat_arm = "cached" if "cached" in arms else "uncached"
heat = [[float(matching(arm=heat_arm, slice="all", S_per_M=float(square), I_per_M=float(inv),
                        native_GAE_per_logical=0.1)["candidate_cold_over_rho"])
         for inv in (1, 10, 100, 1000)] for square in (0, .5, 1)]
img = ax2.imshow(heat, cmap="viridis", aspect="auto")
for i in range(3):
    for j in range(4):
        ax2.text(j, i, f"{heat[i][j]:.2f}", ha="center", va="center", color="white", fontsize=8)
ax2.set_xticks(range(4), ("1", "10", "100", "1000"))
ax2.set_yticks(range(3), ("0", "0.5", "1"))
ax2.set_xlabel("Inversion price I/M")
ax2.set_ylabel("Square price S/M")
ax2.set_title(f"{heat_arm.title()} arm; 1,260 targets; native=0.1 GAE")
fig.colorbar(img, ax=ax2, shrink=0.75, label="Cold candidate / rho")
svg = B / "collector-cost.svg"
fig.savefig(svg, bbox_inches="tight")
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(B / "collector-cost.png", dpi=160, bbox_inches="tight")

with PdfPages(B / "KUMMER_COLLECTOR.pdf") as pdf:
    page = plt.figure(figsize=(8.27, 11.69))
    page.text(0.07, 0.95, "Fixed-difference Kummer collector", fontsize=17, weight="bold")
    page.text(0.07, 0.92, "KUMMER-DIFFERENTIAL-COLLECTOR-20261022", fontsize=10)
    lines = [
        f"Completed targets per arm: {s['targets_per_arm']}.",
        f"Cold setup: {s['cold_setup_models']} unique model builds, charged in all {s['cold_model_charges']} targets.",
        f"Source transfers and source verifications: {s['cold_setup_targets']} each.",
        "Every candidate and rho answer was independently checked as k·P=Q",
        "on the original elliptic curve over Fp².",
        "Model setup includes root search, theta search and torsion matrices.",
        "Each cold target includes source transfer, initialization, the",
        "quotient walk, signed Recover, graph elimination and verification.",
        "Raw: results/<arm>/collector-*.jsonl, cold-*-setup.jsonl,",
        "results/target-price.csv and results/price-grid.csv.",
        "All 36 cells use S/M in {0,0.5,1}, I/M in {1,10,100,1000},",
        "and native cost in {0,0.1,1} GAE per declared logical operation.",
        "Ratios describe this measured implementation and frozen cohort.",
    ]
    for arm in arms:
        v = s["per_arm"][arm]
        lines.insert(1, f"{arm.title()}: {s['source_verified'][arm]} source-verified, {s['processed_divisors'][arm]:,} quotient steps.")
        lines.insert(2, f"{arm.title()}: {s['split_relations'][arm]:,} signed splits, {s['recover_fail'][arm]:,} Recover failures.")
        lines.insert(6, f"{arm.title()} cold ratio: {v['cold_ratio_min']:.4f}–{v['cold_ratio_max']:.4f}; wins/losses: {v['cold_win_cells']}/{v['cold_loss_cells']} cells.")
    y = 0.86
    for line in lines:
        page.text(0.07, y, line, fontsize=9)
        y -= 0.038
    pdf.savefig(page)
    plt.close(page)
    pdf.savefig(fig, bbox_inches="tight")
    rows = [r for r in grid if r["slice"] == "all"]
    for arm in arms:
     for native in (0., .1, 1.):
        page = plt.figure(figsize=(8.27, 11.69))
        page.text(0.07, 0.95, f"{arm}: native={native:g} GAE/logical", fontsize=14, weight="bold")
        page.text(0.07, 0.91, "S/M    I/M      candidate GAE          rho GAE        ratio     wins/losses", fontsize=9)
        y = 0.88
        for row in rows:
            if row["arm"] != arm or float(row["native_GAE_per_logical"]) != native:
                continue
            line = (f"{float(row['S_per_M']):>3.1f}  {float(row['I_per_M']):>5.0f}  "
                    f"{float(row['candidate_cold_GAE']):>17,.1f}  {float(row['rho_GAE']):>15,.1f}  "
                    f"{float(row['candidate_cold_over_rho']):>7.3f}  "
                    f"{row['candidate_wins']}/{row['candidate_losses']}")
            page.text(0.07, y, line, fontsize=8, family="monospace")
            y -= 0.045
        pdf.savefig(page)
        plt.close(page)
plt.close(fig)
