#!/usr/bin/env python3
"""Document layout only; plot recorded native values, never compute results."""
from pathlib import Path
import importlib.util
import json
import sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

ROOT = Path(__file__).resolve().parent
FIG = ROOT / "figures"
spec = importlib.util.spec_from_file_location(
    "prior_layout", ROOT.parent / "s4_twist_elimination/render_reports.py")
layout = importlib.util.module_from_spec(spec)
spec.loader.exec_module(layout)

def saved(fig, name):
    FIG.mkdir(exist_ok=True)
    fig.savefig(FIG / f"{name}.svg")
    svg = FIG / f"{name}.svg"
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(FIG / f"{name}.png", dpi=150)
    plt.close(fig)

def figures():
    v = json.loads((ROOT / "results/algebra-v1-validation.json").read_text())
    rows = v["arms"]
    labels = ["Phi", "Doubling", "Pure L0\npartial symmetry"]
    colors = ["#147d92", "#9b4380", "#b07c28"]
    fig, axes = plt.subplots(1, 2, figsize=(9.4, 3.9), layout="constrained")
    axes[0].bar(labels, [r["total_degree"]["max"] for r in rows], color=colors)
    axes[0].set_ylabel("Recorded total degree")
    axes[1].bar(labels, [r["extension_terms"]["max"] for r in rows], color=colors)
    axes[1].set_ylabel("Recorded extension monomials (max)")
    for ax in axes: ax.grid(axis="y", alpha=.2)
    fig.suptitle("algebra-v1: 297 constructions / 21 cases; polynomial size, not solver cost", fontsize=11)
    saved(fig, "construction-profile")
    v = json.loads((ROOT / "runs/verification-v8/variants/l0-peel/summary.json").read_text())
    fig, axes = plt.subplots(1, 2, figsize=(8.8, 3.6), layout="constrained")
    before, after = v["unreduced_relation_stats"], v["relation_stats"]
    axes[0].bar(["Partial", "Full four-factor"], [before["total_degree"], after["total_degree"]], color=["#68717a", "#b07c28"])
    axes[0].set_ylabel("Recorded total degree")
    axes[1].bar(["Partial", "Full four-factor"], [before["extension_terms"], after["extension_terms"]], color=["#68717a", "#b07c28"])
    axes[1].set_ylabel("Recorded extension monomials")
    for ax in axes: ax.grid(axis="y", alpha=.2)
    fig.suptitle("verification-v8: pure L0, p37 / seed0 / query2; construction only", fontsize=10)
    saved(fig, "pure-symmetry")
    records = [json.loads((ROOT / "results/source-block-regression" / f"{arm}-audit.json").read_text())
               for arm in ("l0-peel", "phi-l0-active", "double-l0-active")]
    fig, axes = plt.subplots(1, 2, figsize=(8.8, 3.7), layout="constrained")
    labels = ["Pure L0", "Phi", "Doubling"]
    axes[0].bar(labels, [r["actual_signed_relations"] for r in records], color=["#b07c28", "#147d92", "#9b4380"])
    axes[0].set_ylabel("Distinct signed physical relations")
    axes[1].bar(labels, [r["rank"] for r in records], color=["#b07c28", "#147d92", "#9b4380"])
    axes[1].axhline(records[0]["columns"], color="#68717a", linestyle="--", label="3 columns")
    axes[1].set_ylabel("Independent rank")
    axes[1].legend()
    for ax in axes:
        ax.set_ylim(0, 3.5)
        ax.set_yticks([0, 1, 2, 3])
        ax.grid(axis="y", alpha=.2)
    fig.suptitle("Complete source-block unions: p37 / seed0 / query2, one selected positive residual", fontsize=10)
    saved(fig, "source-block-rank")
    fig, ax = plt.subplots(figsize=(10.4, 3.9), layout="constrained")
    ax.axis("off")
    nodes = [(.13, "Public curve + G,Q\nHash-checked query [a,b]\nNo factor-log oracle"),
             (.5, "Joint S5 construction\nGlobal / source blocks / inverse\nFull symmetry for pure control"),
             (.87, "Complete physical sets\nIndependent rank and recovery\nThen full cold cost + rho")]
    for x, label in nodes:
        ax.text(x, .60, label, transform=ax.transAxes, ha="center", va="center", fontsize=9,
                bbox={"boxstyle":"round,pad=.65", "facecolor":"#eef3f7", "edgecolor":"#245e7d"})
    for x in (.31, .68):
        ax.annotate("", xy=(x+.04,.60), xytext=(x-.04,.60), xycoords="axes fraction",
                    arrowprops={"arrowstyle":"->", "color":"#245e7d"})
    ax.text(.5,.15,"A smaller polynomial or more presentations alone does not certify an index-calculus gain.",
            transform=ax.transAxes, ha="center", fontsize=10)
    fig.suptitle("Joint-image study: each arrow has a separate verification obligation", fontsize=11)
    saved(fig, "joint-mechanism")

def pdf(path):
    source = layout.typst(path.read_text(), path.parent)
    source = source.replace('.svg", width:', '.png", width:')
    path.with_suffix(".typ").write_text(source)
    typst.compile(str(path.with_suffix(".typ")), output=str(path.with_suffix(".pdf")), root=str(ROOT))

def zero_figure():
    fig, ax = plt.subplots(figsize=(10.6, 4.8), layout="constrained")
    ax.axis("off")
    boxes = [(0.17,.76,"Four distinct abscissae in Fp\nZero group sum\nS4 necessity: group bridge"),
             (.53,.76,"H4 = C0 + C1 B + C2 B² + C3 B³\nDegree of B is four\nHence every Ci = 0"),
             (.85,.76,"2048 D = sum Ai Ci\nExact integer certificate\nIDC / new Lean gates separate"),
             (.70,.28,"Odd characteristic implies D = 0\nField cancellation\nNew Lean statement pending"),
             (.23,.28,"Distinct roots imply D is nonzero\nProduct of squared differences\nQuartic identity: Lean gate passes")]
    for x,y,label in boxes:
        ax.text(x,y,label,transform=ax.transAxes,ha="center",va="center",fontsize=9,
                bbox={"boxstyle":"round,pad=.65","facecolor":"#eef3f7","edgecolor":"#245e7d"})
    for a,b in [((.34,.76),(.37,.76)),((.70,.76),(.72,.76)),((.85,.61),(.74,.43))]:
        ax.annotate("",xy=b,xytext=a,xycoords="axes fraction",arrowprops={"arrowstyle":"->","color":"#245e7d"})
    ax.annotate("contradiction",xy=(.42,.28),xytext=(.53,.28),xycoords="axes fraction",ha="center",fontsize=9,
                arrowprops={"arrowstyle":"->","color":"#9b4380"})
    fig.suptitle("Pure zero fiber: conditional algebraic obstruction; no nonzero-target or cost result",fontsize=11)
    saved(fig,"pure-zero-obstruction")

if __name__ == "__main__":
    if sys.argv[1:] == ["--zero-figure"]:
        zero_figure()
    elif len(sys.argv) > 1:
        for name in sys.argv[1:]:
            path = (ROOT / name).resolve()
            assert path.is_relative_to(ROOT) and path.suffix == ".md"
            pdf(path)
    else:
        figures()
        for path in [ROOT / "README.md", ROOT / "MECHANISMS_AND_CONTROLS.md",
                     *sorted((ROOT / "reports").glob("*/REPORT.md"))]: pdf(path)
