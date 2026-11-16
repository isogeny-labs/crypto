"""Layout recorded outputs only; allowed document rendering under research-visuals.

No cryptographic computation, experiment, recomputation of a scientific result,
performance timing, fitting or benchmark orchestration occurs in this file.
"""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
from matplotlib.patches import FancyArrowPatch, FancyBboxPatch

HERE = Path(__file__).resolve().parent
STUDY = HERE.parent
DATA = STUDY / "native" / "audit-data"
plt.rcParams.update({"font.family": "DejaVu Sans", "svg.fonttype": "none"})


def box(ax, xy, width, height, text, color="#edf4f5", size=11):
    ax.add_patch(FancyBboxPatch(xy, width, height, boxstyle="round,pad=.012",
                              facecolor=color, edgecolor="#3d6065", linewidth=1.2))
    ax.text(xy[0] + width / 2, xy[1] + height / 2, text,
            ha="center", va="center", fontsize=size, linespacing=1.35)


def arrow(ax, start, end, text=None):
    ax.add_patch(FancyArrowPatch(start, end, arrowstyle="-|>",
                                mutation_scale=14, linewidth=1.3, color="#3d6065"))
    if text:
        ax.text((start[0]+end[0])/2, (start[1]+end[1])/2, text,
                fontsize=9, ha="center", va="bottom", backgroundcolor="white")


def main():
    root = json.loads((DATA / "002-result.json").read_text())
    integration = json.loads((DATA / "004-result.json").read_text())
    example = json.loads((DATA / "005-example.json").read_text())
    diagram = plt.figure(figsize=(12, 7.2))
    diagram.suptitle("Full auxiliary-curve root finding", fontsize=18, y=.98)
    diagram.text(.5, .925, "Paper-guided pipeline with source-coordinate Sylow recursion | unsigned development evidence",
                 ha="center", fontsize=10)
    ax = diagram.add_axes((.025, .10, .95, .78)); ax.set(xlim=(0, 1), ylim=(0, 1)); ax.axis("off")
    box(ax, (.02, .87), .24, .10, "Optional 2-isogeny\n+ charged basis completion", "#fff1dd", 10)
    box(ax, (.34, .87), .43, .10, "Valid datum: E, twist, orders, factors, bases\nConstruction + verification remain separate", "#fff1dd", 10)
    arrow(ax, (.27, .92), (.325, .92))
    box(ax, (.02, .62), .20, .14, "Split squarefree f\noriginal root variable t")
    box(ax, (.31, .62), .23, .14, "Algorithm 2 lift\nonto E and its twist")
    box(ax, (.64, .62), .32, .14, "Source ring Fp[t]/u\nstate: (u, v, offset C)")
    arrow(ax, (.23, .69), (.295, .69)); arrow(ax, (.555, .69), (.625, .69))
    arrow(ax, (.55, .855), (.44, .775))
    box(ax, (.64, .32), .32, .19, "For each representative P:\nT = C + P\ng = gcd(u, ψM(S(t) − T))\nchild: (g, v mod g, T)", size=10.5)
    arrow(ax, (.75, .605), (.50, .525), "extract first")
    arrow(ax, (.965, .42), (.965, .625), "recurse")
    box(ax, (.31, .32), .23, .19, "Before inversions:\nextract source ±T\nusing known points", "#fff1dd", 11)
    arrow(ax, (.555, .42), (.625, .42))
    box(ax, (.02, .32), .20, .19, "Linear factors\nread original roots\ntwist output: βt")
    arrow(ax, (.295, .42), (.235, .42))
    arrow(ax, (.80, .305), (.12, .28), "linear branch")
    arrow(ax, (.12, .28), (.12, .31))
    ax.text(.49, .235, "Multiplication isogenies are evaluated by normalized division polynomials.\nThe source parameter stays separating through translated-x collisions.",
            ha="center", fontsize=10)
    box(ax, (.02, .055), .94, .105, "Original target-curve labels  →  norm/GCD  →  root backend above  →  signed witness  →  kP = Q",
        "#e8eef8", 10.5)
    diagram.text(.5, .035, "No characteristic-polynomial / inverse-composition reconstruction in the pullback path.\nNo runtime ratio is inferred. Auxiliary orders and mixed target labels are distinct objects.",
                 ha="center", fontsize=9)
    diagram.savefig(HERE / "native-flow.svg", bbox_inches="tight")
    svg_path = HERE / "native-flow.svg"
    svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")
    diagram.savefig(HERE / "native-flow.png", dpi=150, bbox_inches="tight")

    report = plt.figure(figsize=(8.27, 11.69))
    report.text(.08, .95, "Native paper/pullback development report", fontsize=17, weight="bold")
    report.text(.08, .915, "Unsigned producer evidence · no timings or fitted exponent · PR #4402", fontsize=10)
    report.text(.08, .86, "Recorded correctness checks", fontsize=13, weight="bold")
    lines = [
        f"Independent root audit: {root['complete_root_finding_checks']} literal calls; "
        f"{root['complete_pullback_root_finding_checks']} pullback calls.",
        f"{root['complete_translation_checks']} translations; {root['division_polynomial_point_checks']} division-polynomial checks.",
        f"Integration: {integration['complete_incidence_calls']} calls across four backends; "
        f"{integration['natural_cases']} natural cases.",
        f"Natural cases: {integration['natural_hits']} hits and {integration['natural_misses']} misses; "
        f"{integration['nonlinear_root_backend_calls']} nonlinear backend calls.",
        f"Isogeny: {integration['isogeny_homomorphism_pairs']} homomorphism pairs; twist compatibility checked.",
        "Mapped generator order product 36; complete codomain basis order product 72.",
        "Completed auxiliary curve/twist bases: 72 × 1 and 96 × 1.",
        "Exhaustive toy construction is counted and refused for p > 257.",
    ]
    y = .825
    for line in lines:
        report.text(.085, y, line, fontsize=10); y -= .026
    report.text(.08, .61, "Incidence inputs: p83/p97, B2/3/4; a correctness panel, not a scaling result.", fontsize=9)
    report.text(.08, .58, "One fixed F83 example: structural counts", fontsize=13, weight="bold")
    table_ax = report.add_axes((.08, .44, .84, .115)); table_ax.axis("off")
    rows = []
    for arm in example["arms"]:
        counts = arm["structural_counts"]
        rows.append([arm["method"], counts.get("translation_calls", 0),
                     counts.get("primary.membership_tests", counts.get("pullback_membership_tests", 0)), "same 10 roots"])
    table = table_ax.table(cellText=rows, colLabels=["Path", "Translations", "Membership tests", "Output"], loc="center")
    table.auto_set_font_size(False); table.set_fontsize(9); table.scale(1, 1.6)
    report.text(.08, .405, "Linear stopping differs too. These counts do not isolate a timing speed factor.", fontsize=9)
    report.text(.08, .35, "Derived supplied-datum cost allowance", fontsize=13, weight="bold")
    report.text(.10, .31, r"$\widetilde{O}(\Xi^2 d\log^2 p + d\log^3 p)$", fontsize=17)
    report.text(.08, .27, "Fast polynomial arithmetic; monic split separable input; characteristic > 3.", fontsize=10)
    report.text(.08, .245, "Cold cost adds datum construction and validation. Formal/PCC standing is pending.", fontsize=10)
    report.text(.08, .22, "The Horner incidence norm is unchanged: O(B²) degree-B quotient products.", fontsize=10)
    report.text(.08, .16, "Sources and reproducibility", fontsize=12, weight="bold")
    report.text(.08, .13, "reports/PULLBACK.md · native/README.md · native/audit-data/002,004,005", fontsize=9)
    report.text(.08, .105, "Spaenlehauer Algorithms 1–6, Remark 5.4, §8.1; GvdHL15 Algorithm 2.", fontsize=9)
    report.text(.08, .08, "Classical Velu controls: src/isogeny/velu.rs. No priority or fastest-tool claim.", fontsize=9)
    with PdfPages(HERE / "NATIVE_REPORT.pdf") as pdf:
        pdf.savefig(diagram, bbox_inches="tight")
        pdf.savefig(report)
    plt.close(diagram); plt.close(report)


if __name__ == "__main__":
    main()
