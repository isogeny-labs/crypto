"""Lay out verified Rust summary values as the report graph and PDF source."""

import json
import csv
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import TwoSlopeNorm

plt.rcParams["svg.hashsalt"] = "projective-law-followup-20261022"


ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results"
ARMS = [
    ("projective", "Weighted-law wrapper", "#2b6cb0"),
    ("state", "Weighted walk state", "#b85c38"),
]
SUMMARIES = {
    arm: json.loads((RESULTS / f"summary-{arm}.json").read_text())
    for arm, _, _ in ARMS
}
SLICES = [16, 20, 24, 28, 32, 36, 40]


def row(arm, bits):
    return next(
        value for value in SUMMARIES[arm]["summaries"] if value["slice"] == str(bits)
    )


fig, (ax_rho, ax_external) = plt.subplots(2, 1, figsize=(9.0, 7.5), sharex=True)
for arm, label, color in ARMS:
    for ax, baseline in ((ax_rho, "rho"), (ax_external, "external")):
        center = [row(arm, bits)[f"candidate_over_{baseline}"] for bits in SLICES]
        band = [row(arm, bits)[f"{baseline}_ratio_range_36_cells"] for bits in SLICES]
        yerr = [
            [middle - interval[0] for middle, interval in zip(center, band)],
            [interval[1] - middle for middle, interval in zip(center, band)],
        ]
        ax.errorbar(
            SLICES,
            center,
            yerr=yerr,
            marker="o",
            capsize=3,
            linewidth=1.8,
            color=color,
            label=label,
        )
for ax, title in ((ax_rho, "Cost / matched source negation-rho"),
                  (ax_external, "Cost / wrapped external-law collector")):
    ax.axhline(1.0, color="#555555", linewidth=1, linestyle="--")
    ax.set_yscale("log")
    ax.set_ylabel(title)
    ax.grid(alpha=0.2, which="both")
ax_rho.legend(loc="upper left", frameon=False)
ax_external.set_xticks(SLICES)
ax_external.set_xlabel("Source E(Fp²) subgroup-size band (bits)")
fig.suptitle("Projective-law follow-up: paired full cold cost on 1,260 targets per arm")
fig.text(
    0.5, 0.01,
    "Markers: M+0.5S+10I, 0.1 GAE/logical op. Whiskers: min–max of 36 pricing cells; not sampling intervals.",
    ha="center", fontsize=8,
)
fig.tight_layout(rect=(0, 0.04, 1, 0.96))
fig.savefig(ROOT / "cost-ratios.svg", metadata={"Date": "2026-10-22"})
fig.savefig(ROOT / "cost-ratios.png", dpi=150)
plt.close(fig)

with (RESULTS / "price-grid-state-vs-wrapper.csv").open() as handle:
    direct_rows = [row for row in csv.DictReader(handle) if row["slice"] == "all"]
arithmetic_prices = [(s, i) for s in (0.0, 0.5, 1.0) for i in (1.0, 10.0, 100.0, 1000.0)]
logical_prices = [0.0, 0.1, 1.0]
direct_matrix = np.array([
    [next(float(item["state_over_wrapper"]) for item in direct_rows
          if float(item["S_per_M"]) == s and float(item["I_per_M"]) == i
          and float(item["native_GAE_per_logical"]) == native)
     for s, i in arithmetic_prices]
    for native in logical_prices
])
fig_direct, ax_direct = plt.subplots(figsize=(11.0, 3.6))
heat = ax_direct.imshow(direct_matrix, cmap="coolwarm", norm=TwoSlopeNorm(vcenter=1.0, vmin=0.5, vmax=1.35), aspect="auto")
ax_direct.set_xticks(range(12), [f"S={s:g}\nI={i:g}" for s, i in arithmetic_prices], fontsize=8)
ax_direct.set_yticks(range(3), [f"{price:g}" for price in logical_prices])
ax_direct.set_ylabel("GAE per logical operation")
ax_direct.set_title("Retained weighted state / weighted-law wrapper: pooled 1,260-target cold cost")
for row_index in range(3):
    for col_index in range(12):
        ax_direct.text(col_index, row_index, f"{direct_matrix[row_index, col_index]:.2f}",
                       ha="center", va="center", color="black", fontsize=8)
fig_direct.colorbar(heat, ax=ax_direct, label="Ratio; below 1 favors retained state")
fig_direct.tight_layout()
fig_direct.savefig(ROOT / "direct-price-grid.svg", metadata={"Date": "2026-10-22"})
fig_direct.savefig(ROOT / "direct-price-grid.png", dpi=160)
plt.close(fig_direct)

gate_rows = [json.loads(line) for line in (RESULTS / "arithmetic-branches.jsonl").read_text().splitlines()]
branch_names = ["c_zero_degree_one", "same_u_inverse", "same_u_mixed", "shared_cancellation", "shared_positive"]
branch_matrix = np.array([
    [item["weighted_targeted_add22_branches"].get(name, 0) for name in branch_names]
    for item in gate_rows
])
fig_branch, ax_branch = plt.subplots(figsize=(9.0, 6.0))
heatmap = ax_branch.imshow(branch_matrix, aspect="auto", cmap="Blues")
ax_branch.set_yticks(range(len(gate_rows)), [str(item["p"]) for item in gate_rows], fontsize=7)
ax_branch.set_xticks(range(len(branch_names)), [name.replace("_", " ") for name in branch_names], rotation=25, ha="right", fontsize=8)
ax_branch.set_ylabel("Destination field prime p (21 models)")
ax_branch.set_title("Targeted weighted 2+2 branches checked against independent Cantor")
fig_branch.colorbar(heatmap, ax=ax_branch, label="Checked cases per model")
fig_branch.tight_layout()
fig_branch.savefig(ROOT / "branch-coverage.svg", metadata={"Date": "2026-10-22"})
fig_branch.savefig(ROOT / "branch-coverage.png", dpi=150)
plt.close(fig_branch)

all_wrapper = next(v for v in SUMMARIES["projective"]["summaries"] if v["slice"] == "all")
all_state = next(v for v in SUMMARIES["state"]["summaries"] if v["slice"] == "all")
validation = json.loads((RESULTS / "independent-validation.json").read_text())
direct = validation["paired_state_over_wrapper"]
with (RESULTS / "cold-phase-breakdown.csv").open() as handle:
    phase_rows = list(csv.DictReader(handle))


def phase_gae(study_arm, method, names):
    return sum(float(item["GAE_at_M_plus_0.5S_plus_10I_and_0.1_native"])
               for item in phase_rows if item["study_arm"] == study_arm
               and item["slice"] == "40" and item["method"] == method
               and item["phase"] in names)
lines = [
    r"\documentclass[10pt]{article}",
    r"\usepackage[margin=0.7in]{geometry}",
    r"\usepackage{graphicx}",
    r"\usepackage{array}",
    r"\begin{document}",
    r"\begin{center}\Large Projective genus-2 law follow-up\\[3pt]\normalsize Frozen 21-model, 1,260-target paired cohort per arm\end{center}",
    r"\noindent\textbf{Scope.} Two complete cold graph-collector arms use the independently checked weighted genus-2 law. The wrapper normalizes every projective output. The state arm keeps weighted divisors across walk steps and normalizes split candidates at the relation boundary. Each arm has 1,260 source-scalar-verified candidate solves and 1,260 same-process, independently source-verified negation-rho solves. The archived wrapped external-law collector is the second comparator. All ratios below divide the summed candidate cost by the summed comparator cost for the same targets.",
    r"\medskip",
    r"\begin{center}\small\begin{tabular}{r|r r r|r r|r}",
    r"Bits & Wrapper/rho & State/rho & State/wrapper & Wrapper/external & State/external & State boundary I \\",
    r"\hline",
]
for bits in SLICES:
    w = row("projective", bits)
    s = row("state", bits)
    lines.append(
        f"{bits} & {w['candidate_over_rho']:.4f} & {s['candidate_over_rho']:.4f} & "
        f"{s['candidate_GAE']/w['candidate_GAE']:.4f} & "
        f"{w['candidate_over_external']:.4f} & {s['candidate_over_external']:.4f} & "
        f"{s['normalization_inversions']:,}" + r" \\"
    )
lines.extend([
    r"\hline",
    f"All & {all_wrapper['candidate_over_rho']:.4f} & {all_state['candidate_over_rho']:.4f} & "
    f"{direct['representative']:.4f} & "
    f"{all_wrapper['candidate_over_external']:.4f} & {all_state['candidate_over_external']:.4f} & "
    f"{all_state['normalization_inversions']:,}" + r" \\",
    r"\end{tabular}\end{center}",
    r"\noindent Unit: source-addition equivalent (GAE). Representative price: $M+0.5S+10I$ plus 0.1 GAE per native logical operation. Boundary inversion counts are field inversions charged in the state arm; the wrapper's normalization inversions are also charged. Every ratio uses full cold setup, transfer, initialization, relation collection, elimination, and source verification.",
    r"\medskip",
    f"\\noindent Pooled 36-cell wrapper/rho range: {all_wrapper['rho_ratio_range_36_cells'][0]:.4f}--{all_wrapper['rho_ratio_range_36_cells'][1]:.4f}; "
    f"state/rho range: {all_state['rho_ratio_range_36_cells'][0]:.4f}--{all_state['rho_ratio_range_36_cells'][1]:.4f}. "
    f"Cells below rho: wrapper {all_wrapper['cells_below_rho']}/36; state {all_state['cells_below_rho']}/36. "
    f"Pooled state/wrapper range: {direct['pooled_36_cell_range'][0]:.4f}--{direct['pooled_36_cell_range'][1]:.4f}; "
    f"{sum(float(item['state_over_wrapper']) < 1 for item in direct_rows)}/36 prices favor state and "
    f"{sum(float(item['state_over_wrapper']) > 1 for item in direct_rows)}/36 favor wrapper.",
    r"\noindent\textbf{Cold phases.} The standardized cold-phase-breakdown.csv reports arithmetic counts and source-addition-equivalent costs for construction, transfer, walk setup, relation collection and signed elimination, source verification, and a separate row for native logical work. Its phase rows sum to the total representative cost for each arm and comparator, for each bit size and all 1,260 targets.",
    r"\begin{center}\small\begin{tabular}{l|r r r r}",
    r"40-bit method & Setup & Search/elimination & Native logical & Verification \\",
    r"\hline",
])
for method_label, study_arm, method in [
    ("Wrapper", "projective", "candidate"),
    ("Weighted state", "state", "candidate"),
    ("External", "projective", "external"),
    ("Rho", "projective", "rho"),
]:
    setup_names = {"rho_initialization"} if method == "rho" else {
        "destination_construction", "transfer", "walk_initialization"
    }
    search_names = {"rho_search"} if method == "rho" else {"relations_and_signed_elimination"}
    lines.append(
        f"{method_label} & {phase_gae(study_arm, method, setup_names)/1e6:.3f} & "
        f"{phase_gae(study_arm, method, search_names)/1e6:.3f} & "
        f"{phase_gae(study_arm, method, {'native_logical_unallocated'})/1e6:.3f} & "
        f"{phase_gae(study_arm, method, {'source_verification'})/1e6:.3f}" + r" \\"
    )
lines.extend([
    r"\end{tabular}\end{center}",
    r"\noindent Values above are millions of source-addition equivalents for 180 paired 40-bit targets at the representative price. Setup is construction, transfer, and walk initialization, or rho initialization. Full per-size and pooled phase counts are in the CSV.",
    r"\begin{center}\includegraphics[width=0.95\textwidth]{cost-ratios.png}\end{center}",
    r"\begin{center}\includegraphics[width=0.95\textwidth]{direct-price-grid.png}\end{center}",
    r"\noindent\textbf{Arithmetic gate.} Raw branch records cover all 21 normalized models with non-unit $Z,W$ checks and targeted inverse, coincident-support, degree-drop, and doubling cases. The separate $p=739$ regression preserves the exact input and both $N_6$ variants. These are finite checks, not a proof of universal branch correctness.",
    r"\begin{center}\includegraphics[width=0.85\textwidth]{branch-coverage.png}\end{center}",
    r"\medskip",
    r"\noindent\textbf{Review files.} PROTOCOL.md defines the cohort and accounting. results/ contains raw per-target JSONL, source-replay and price summaries, both 36-cell grids, and the regression JSON. DATA\_DICTIONARY.md defines every standardized field. VALIDATION\_NOTES.md lists checks and remaining gaps. The SVG and PNG share the same recorded-values source.",
    r"\end{document}",
])
(ROOT / "REPORT.tex").write_text("\n".join(lines) + "\n")
for svg in ROOT.glob("*.svg"):
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
