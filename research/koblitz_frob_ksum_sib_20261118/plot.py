#!/usr/bin/env python3
"""Plot recorded values from summary.json (layout/rendering only; no new computation)."""
import json
import pathlib
import shutil
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from PIL import Image

HERE = pathlib.Path(__file__).resolve().parent
SUMMARY_PATH = HERE / "summary.json"
RUN_REPORT_DIR = HERE / "reports" / "run-20261118T2145Z"
RUN_REPORT_DIR.mkdir(parents=True, exist_ok=True)

data = json.loads(SUMMARY_PATH.read_text())
# Keep summary.json compact (< 50 KB) for Git artifact gate compliance
SUMMARY_PATH.write_text(json.dumps(data, separators=(",", ":"), indent=1) + "\n")

plt.rcParams.update({
    "svg.fonttype": "none",
    "font.family": "DejaVu Sans",
    "font.size": 8.5,
    "axes.titlesize": 9.5,
    "axes.labelsize": 8.5,
    "legend.fontsize": 7.5,
})


def save_svg_and_png(fig, stem: str):
    svg_path = HERE / f"{stem}.svg"
    png_tmp = pathlib.Path(f"/tmp/{stem}-raw.png")
    png_path = HERE / f"{stem}.png"
    fig.savefig(svg_path, format="svg", bbox_inches="tight")
    fig.savefig(png_tmp, format="png", dpi=150, bbox_inches="tight")
    im = Image.open(png_tmp).convert("RGBA")
    bg = Image.new("RGBA", im.size, (255, 255, 255, 255))
    flat = Image.alpha_composite(bg, im).convert("RGB")
    q = flat.quantize(colors=96, method=Image.Quantize.MEDIANCUT)
    q.save(png_path, format="PNG", optimize=True)
    png_tmp.unlink(missing_ok=True)
    shutil.copy2(svg_path, RUN_REPORT_DIR / svg_path.name)
    shutil.copy2(png_path, RUN_REPORT_DIR / png_path.name)
    assert svg_path.stat().st_size < 64000, f"{svg_path} >= 64 KB"
    assert png_path.stat().st_size < 64000, f"{png_path} >= 64 KB"


# --- Figure 1: Koblitz Ladder Speedup across §5a Band + Cold S(w) Trajectory ---
k_order = ["K1-n29", "K0-n23", "K1-n23", "K0-n37", "K0-n41", "K0-n53"]
cmps = {(c["session"], c["candidate"], c["weight"]): c for c in data["comparisons"]}

fig1, (ax1, ax2) = plt.subplots(1, 2, figsize=(9.6, 3.8), dpi=150)

x_idx = list(range(len(k_order)))
labels = [
    f"{sid}\n({data['sessions'][sid]['log2_r']:.1f}b)"
    for sid in k_order
]

for w, col, marker, off, lbl in [
    (0.0, "#0b6e4f", "o", -0.14, "Counted S(w=0)"),
    (0.1, "#1d4ed8", "s", 0.0, "Band S(w=0.1)"),
    (1.0, "#7c3aed", "^", 0.14, "Band S(w=1.0)"),
]:
    xs = [i + off for i in x_idx]
    ys = [cmps[(sid, "cand-kf4-c2-free", w)]["speedup_inc_over_cand"] for sid in k_order]
    lo = [ys[i] - cmps[(k_order[i], "cand-kf4-c2-free", w)]["speedup_ci95_t"][0] for i in range(len(k_order))]
    hi = [cmps[(k_order[i], "cand-kf4-c2-free", w)]["speedup_ci95_t"][1] - ys[i] for i in range(len(k_order))]
    ax1.errorbar(xs, ys, yerr=[lo, hi], fmt=marker + "-", color=col, capsize=2.5, lw=1.4, ms=4.8, label=lbl)

wall_y = [cmps[(sid, "cand-kf4-c2-free", 0.0)]["wall_speedup_inc_over_cand"] for sid in k_order]
ax1.plot(x_idx, wall_y, "d--", color="#b45309", lw=1.2, ms=4.5, label="Solve wall ratio (L0)")
ax1.axhline(1.0, color="#6b7280", ls=":", lw=1.0)
ax1.set_xticks(x_idx)
ax1.set_xticklabels(labels, fontsize=7.5)
ax1.set_ylabel("Speedup: strictest IC incumbent / cand-kf4-c2-free")
ax1.set_title("A. Cohort K Speedup Across §5a Band (95% CI)")
ax1.grid(True, alpha=0.25)
ax1.legend(loc="upper left")

log2_rs = [data["sessions"][sid]["log2_r"] for sid in k_order]
inc_s0 = []
cand_s0 = []
rho_s0 = []
for sid in k_order:
    arms = data["sessions"][sid]["arms"]
    inc_best = min((v["S_w0"] for k, v in arms.items() if k.startswith("inc-")))
    inc_s0.append(inc_best)
    cand_s0.append(arms["cand-kf4-c2-free"]["S_w0"])
    rho_s0.append(arms["rho-strong"]["S_w0"])

ax2.plot(log2_rs, inc_s0, "s-", color="#dc2626", lw=1.5, ms=5.0, label="Strictest IC incumbent S(w=0)")
ax2.plot(log2_rs, cand_s0, "o-", color="#0b6e4f", lw=1.7, ms=5.2, label="cand-kf4-c2-free S(w=0)")
ax2.plot(log2_rs, rho_s0, "^--", color="#374151", lw=1.3, ms=4.8, label="Matched rho.signed_frobenius_strong")
ax2.set_yscale("log")
ax2.set_xlabel("Subgroup bit-length log2(r)")
ax2.set_ylabel("Cold single-target S(w=0) = GAE / sqrt(r) [log]")
ax2.set_title("B. Cold Single-Target Scaling (15.4 to 44.3 bits)")
ax2.grid(True, which="both", alpha=0.25)
ax2.legend(loc="upper right")

fig1.suptitle(
    f"Koblitz Cold Single-Target IC (commit {data['frozen_git_commit'][:10]}, sessions ECBS1h9130..ECBS1h0a25)",
    fontsize=9.5,
    y=1.02,
)
fig1.tight_layout()
save_svg_and_png(fig1, "fig-band-speedup")
plt.close(fig1)

# --- Figure 2: bitop/v1 Ratio-to-Floor + Cohort B Random-Binary Sibling Results ---
fig2, (ax3, ax4) = plt.subplots(1, 2, figsize=(9.6, 3.8), dpi=150)

inc_floor = []
cand_floor = []
rho_floor = []
for sid in k_order:
    arms = data["sessions"][sid]["arms"]
    inc_best = min((v["mean_ratio_to_floor_bitops"] for k, v in arms.items() if k.startswith("inc-")))
    inc_floor.append(inc_best)
    cand_floor.append(arms["cand-kf4-c2-free"]["mean_ratio_to_floor_bitops"])
    rho_floor.append(arms["rho-strong"]["mean_ratio_to_floor_bitops"])

ax3.plot(log2_rs, inc_floor, "s-", color="#dc2626", lw=1.5, ms=5.0, label="Strictest IC incumbent")
ax3.plot(log2_rs, cand_floor, "o-", color="#0b6e4f", lw=1.7, ms=5.2, label="cand-kf4-c2-free")
ax3.plot(log2_rs, rho_floor, "^--", color="#374151", lw=1.3, ms=4.8, label="rho.signed_frobenius_strong")
ax3.set_yscale("log")
ax3.set_xlabel("Subgroup bit-length log2(r)")
ax3.set_ylabel("bitop/v1 gate count / floor [log]")
ax3.set_title("A. Cohort K bitop/v1 Ratio to Floor (complete=true)")
ax3.grid(True, which="both", alpha=0.25)
ax3.legend(loc="upper right")

b_order = ["B21", "B24", "B27"]
bx = list(range(len(b_order)))
blabels = [f"{sid}\n({data['sessions'][sid]['curve'][:16]}..)" for sid in b_order]
for w, col, marker, off, lbl in [
    (0.0, "#0b6e4f", "o", -0.14, "R_0 (w=0)"),
    (0.1, "#1d4ed8", "s", 0.0, "R_0.1 (w=0.1)"),
    (1.0, "#7c3aed", "^", 0.14, "R_1.0 (w=1.0)"),
]:
    xs = [i + off for i in bx]
    ys = [cmps[(sid, "cand-ksum5-sib-free", w)]["R_w_cand_over_inc"] for sid in b_order]
    lo = [ys[i] - cmps[(b_order[i], "cand-ksum5-sib-free", w)]["R_w_ci95"][0] for i in range(len(b_order))]
    hi = [cmps[(b_order[i], "cand-ksum5-sib-free", w)]["R_w_ci95"][1] - ys[i] for i in range(len(b_order))]
    ax4.errorbar(xs, ys, yerr=[lo, hi], fmt=marker + "-", color=col, capsize=3.0, lw=1.4, ms=5.0, label=lbl)

ax4.axhline(1.0, color="#dc2626", ls="--", lw=1.1, label="Parity with strictest pruned ksum")
ax4.set_xticks(bx)
ax4.set_xticklabels(blabels, fontsize=7.5)
ax4.set_ylabel("Paired cost ratio R_w = cand / strictest incumbent")
ax4.set_title("B. Cohort B Random-Binary Sibling k-Sum (64 pairs/curve)")
ax4.grid(True, alpha=0.25)
ax4.legend(loc="upper left")

fig2.suptitle(
    "Complete bitop/v1 Gate Scaling & Generic Binary Table B Sibling Evaluation (sessions ECBS1h3ca1..ECBS1h022b)",
    fontsize=9.5,
    y=1.02,
)
fig2.tight_layout()
save_svg_and_png(fig2, "fig-bitop-and-binary")
plt.close(fig2)
print("Generated SVG and 150-dpi quantized PNG figures (< 64 KB each).")
