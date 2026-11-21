#!/usr/bin/env python3
"""Render the local diagnostic result chart and illustrated PDF."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.backends.backend_pdf import PdfPages


CASES = ["k1_n17_m2", "k0_n15_f1_m3"]
CASE_LABELS = ["K1 / F2^17 / m=2", "K0 / F2^15 / m=3 interleaved"]
METRICS = ["solver_wall_ns", "word_xors", "word_xors_performed", "peak_matrix_bytes"]
METRIC_LABELS = ["Solver wall", "Logical XORs", "Performed XORs", "Peak matrix"]
COLORS = ["#176B87", "#D97706"]


def chart(summary: dict):
    fig, ax = plt.subplots(figsize=(11, 6.4))
    x = np.arange(len(METRICS))
    width = 0.35
    for index, (case, label, color) in enumerate(zip(CASES, CASE_LABELS, COLORS)):
        metrics = summary["cases"][case]["metrics"]
        speedups = [1 / metrics[metric]["candidate_over_full_width_geomean"] for metric in METRICS]
        positions = x + (index - 0.5) * width
        bars = ax.bar(positions, speedups, width, label=label, color=color, zorder=3)
        wall_ci = metrics["solver_wall_ns"]["paired_bootstrap_ci95"]
        wall_speed_ci = [1 / wall_ci[1], 1 / wall_ci[0]]
        ax.errorbar(
            positions[0],
            speedups[0],
            yerr=[[speedups[0] - wall_speed_ci[0]], [wall_speed_ci[1] - speedups[0]]],
            color="#111827",
            capsize=4,
            linewidth=1.2,
            zorder=5,
        )
        for bar, value in zip(bars, speedups):
            ax.text(
                bar.get_x() + bar.get_width() / 2,
                value * (1.12 if value >= 1 else 0.91),
                f"{value:.2f}x",
                ha="center",
                va="bottom" if value >= 1 else "top",
                fontsize=9,
                fontweight="semibold",
            )
    ax.axhline(1, color="#6B7280", linewidth=1.1, linestyle="--", zorder=2)
    ax.set_yscale("log")
    ax.set_ylim(0.75, 55)
    ax.set_xticks(x, METRIC_LABELS)
    ax.set_ylabel("Full-width / FL (higher favors FL; logarithmic)")
    fig.suptitle(
        "Boolean F4 Faugere-Lachartre split on fixed Semaev systems",
        x=0.065,
        y=0.985,
        ha="left",
        weight="bold",
        fontsize=16,
    )
    fig.text(
        0.065,
        0.945,
        "Local diagnostic, 8 paired rounds after 1 warmup; wall error bars are paired-bootstrap 95% intervals",
        fontsize=10,
        color="#4B5563",
    )
    ax.grid(axis="y", which="both", color="#E5E7EB", linewidth=0.8, zorder=0)
    ax.legend(frameon=False, loc="upper right")
    fig.text(
        0.01,
        0.01,
        "Correctness gate passed (identical input and reduced-basis SHA-256). Timing is not native-L2 admitted and has no A/A control.",
        fontsize=9,
        color="#4B5563",
    )
    fig.tight_layout(rect=(0, 0.045, 1, 0.91))
    return fig


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("summary", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    summary = json.loads(args.summary.read_text())
    if summary["correctness_gate"] != "pass":
        raise SystemExit("refusing to render a failed correctness gate")
    args.out.mkdir(parents=True, exist_ok=True)

    fig = chart(summary)
    svg_path = args.out / "speedups.svg"
    fig.savefig(svg_path)
    # Matplotlib's embedded font paths carry insignificant trailing spaces;
    # normalize them so `git diff --check` remains an evidence gate.
    svg_text = svg_path.read_text()
    svg_path.write_text("\n".join(line.rstrip() for line in svg_text.splitlines()) + "\n")
    fig.savefig(args.out / "speedups.png", dpi=180)
    with PdfPages(args.out / "REPORT.pdf") as pdf:
        pdf.savefig(fig)
        page = plt.figure(figsize=(11, 8.5))
        page.text(0.06, 0.93, "Result summary and claim boundary", fontsize=20, weight="bold")
        lines = [
            "Correctness: PASS — identical input and reduced-basis SHA-256 in every matched pair.",
            "K1/F2^17, m=2: 1.31x local solver-wall speedup; 49.7% fewer logical XORs; 19.2% fewer performed XORs.",
            "K0/F2^15, m=3 interleaved: 32.97x local solver-wall speedup; 95.2% fewer logical XORs; 93.6% fewer performed XORs.",
            "The m=2 peak matrix was 4.2% larger; the m=3 peak matrix was 81.1% smaller.",
            "",
            "Admission boundary",
            "These are local, single-thread, interleaved diagnostics. The frozen native-L2 EKS protocol includes A/A controls,",
            "but its trusted main-branch workflow is currently blocked before build by artifact hydration. No isolated timing",
            "claim, end-to-end ECDLP speedup, asymptotic claim, or sub-rho result is made.",
            "The source-checkpoint workflow separately reached AWS OIDC but the configured role refused the request.",
            "",
            "External reference",
            "No msolve or M4GB executable was present on the frozen host. Absence is reported; no native substitute is ranked",
            "as an external Groebner implementation.",
        ]
        y = 0.85
        for line in lines:
            page.text(0.07, y, line, fontsize=11, family="DejaVu Sans")
            y -= 0.055 if not line else 0.047
        page.text(
            0.07,
            0.08,
            "Source: f37434af0ddbf23d7a415108b29067cfc72abfe5  |  Protocol: SEMAEV-FL-20261118 v1",
            fontsize=9,
            color="#4B5563",
        )
        pdf.savefig(page)
        plt.close(page)
    plt.close(fig)


if __name__ == "__main__":
    main()
