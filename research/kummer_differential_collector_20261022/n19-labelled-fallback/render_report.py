#!/usr/bin/env python3
"""Render the validated N19 CSV as a neutral SVG and LaTeX report.

This is presentation only; analyze.py and validate.py own the arithmetic.
"""

import csv
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
ANALYSIS = HERE / "results/analysis"
BITS = (16, 20, 24, 28, 32, 36, 40)
INV = (1, 10, 100, 1000)
COLORS = ("#1d7289", "#27866e", "#c08329", "#a54c5e")


def number(value, digits=4):
    return "null" if value in (None, "") else f"{float(value):.{digits}f}"


def main():
    rows = list(csv.DictReader((ANALYSIS / "price-grid.csv").open()))
    summary = json.loads((ANALYSIS / "summary.json").read_text())
    assert len(rows) == 288 and summary["targets"] == 1260

    def row(slice_name, sq, inv, native):
        hits = [r for r in rows if r["slice"] == slice_name
                and float(r["S_per_M"]) == sq
                and float(r["I_per_M"]) == inv
                and float(r["native_GAE_per_logical"]) == native]
        assert len(hits) == 1
        return hits[0]

    # Logarithmic vertical scale shows all prices without clipping a loss.
    series = [[row(f"bits{bits}", .5, inv, .1)["candidate_cold_over_rho"]
               for bits in BITS] for inv in INV]
    baseline = [row(f"bits{bits}", .5, 10, .1)["baseline_cold_over_rho"]
                for bits in BITS]
    present = [float(x) for line in series for x in line if x]
    present += [float(x) for x in baseline if x]
    lo = min(present + [1.0])
    hi = max(present + [1.0])
    low = 10 ** math.floor(math.log10(lo))
    high = 10 ** math.ceil(math.log10(hi))
    if high == low:
        high *= 10
    x0, x1, y0, y1 = 70, 670, 70, 355

    def xpos(i):
        return x0 + i * (x1 - x0) / 6

    def ypos(v):
        return y1 - (math.log10(v) - math.log10(low)) / (math.log10(high)-math.log10(low)) * (y1-y0)

    svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="470" viewBox="0 0 1120 470">',
           '<rect width="1120" height="470" fill="white"/>',
           '<text x="60" y="30" font-family="sans-serif" font-size="17">N19 full-cold cost by bit slice; S/M=0.5, native=0.1</text>',
           '<text x="70" y="53" font-family="sans-serif" font-size="12">N19 / matched rho</text>',
           f'<line x1="{x0}" y1="{y0}" x2="{x0}" y2="{y1}" stroke="#333"/>',
           f'<line x1="{x0}" y1="{y1}" x2="{x1}" y2="{y1}" stroke="#333"/>']
    for bits_i, bits in enumerate(BITS):
        x = xpos(bits_i)
        svg.append(f'<text x="{x:.1f}" y="380" text-anchor="middle" font-family="sans-serif" font-size="12">{bits}</text>')
    svg.append('<text x="370" y="410" text-anchor="middle" font-family="sans-serif" font-size="12">target bit slice</text>')
    exponent = math.floor(math.log10(low))
    while 10 ** exponent <= high * (1 + 1e-12):
        value = 10 ** exponent
        y = ypos(value)
        svg.append(f'<line x1="{x0}" y1="{y:.1f}" x2="{x1}" y2="{y:.1f}" stroke="#ddd"/>')
        svg.append(f'<text x="60" y="{y+4:.1f}" text-anchor="end" font-family="sans-serif" font-size="11">{value:g}</text>')
        exponent += 1
    for inv_i, inv in enumerate(INV):
        color = COLORS[inv_i]
        points = [(xpos(i), ypos(float(v))) for i, v in enumerate(series[inv_i]) if v]
        if points:
            coords = " ".join(f"{x:.1f},{y:.1f}" for x, y in points)
            svg.append(f'<polyline points="{coords}" fill="none" stroke="{color}" stroke-width="2"/>')
            for x, y in points:
                svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="3" fill="{color}"/>')
        lx = 75 + 260 * inv_i
        svg.append(f'<line x1="{lx}" y1="435" x2="{lx+24}" y2="435" stroke="{color}" stroke-width="2"/>')
        svg.append(f'<text x="{lx+30}" y="440" font-family="sans-serif" font-size="12">N19 / rho, I/M={inv}</text>')
    # The aggregate baseline delta rounds to 1.0000 in ordinary ratio plots.
    # Plot its slice-level parts in parts per million on a separate log axis.
    ppm = [(float(x)-1) * 1e6 if x else None for x in
           (row(f"bits{bits}", .5, 10, .1)["candidate_cold_over_baseline"] for bits in BITS)]
    dx0, dx1, dy0, dy1 = 750, 1070, 70, 355
    dlo, dhi = .01, 1000
    def dxpos(i):
        return dx0 + i * (dx1-dx0) / 6
    def dypos(v):
        return dy1 - (math.log10(v)-math.log10(dlo))/(math.log10(dhi)-math.log10(dlo))*(dy1-dy0)
    svg.append('<text x="750" y="53" font-family="sans-serif" font-size="12">(N19 / baseline - 1) × 10^6; I/M=10</text>')
    svg.append(f'<line x1="{dx0}" y1="{dy0}" x2="{dx0}" y2="{dy1}" stroke="#333"/>')
    svg.append(f'<line x1="{dx0}" y1="{dy1}" x2="{dx1}" y2="{dy1}" stroke="#333"/>')
    for value in (.01,.1,1,10,100,1000):
        y = dypos(value)
        svg.append(f'<line x1="{dx0}" y1="{y:.1f}" x2="{dx1}" y2="{y:.1f}" stroke="#ddd"/>')
        svg.append(f'<text x="742" y="{y+4:.1f}" text-anchor="end" font-family="sans-serif" font-size="10">{value:g}</text>')
    delta_points = [(dxpos(i), dypos(v)) for i,v in enumerate(ppm) if v is not None and v > 0]
    coords = " ".join(f"{x:.1f},{y:.1f}" for x,y in delta_points)
    svg.append(f'<polyline points="{coords}" fill="none" stroke="#555" stroke-width="2"/>')
    for x,y in delta_points:
        svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="3" fill="#555"/>')
    for i,bits in enumerate(BITS):
        svg.append(f'<text x="{dxpos(i):.1f}" y="380" text-anchor="middle" font-family="sans-serif" font-size="11">{bits}</text>')
    svg.append('<text x="910" y="410" text-anchor="middle" font-family="sans-serif" font-size="12">target bit slice</text>')
    svg.append('<text x="70" y="460" font-family="sans-serif" font-size="10">Empty series points denote null ratios; exact ratios and all wins/losses are in the standardized CSV.</text>')
    svg.append('</svg>')
    (HERE / "cost-graph.svg").write_text("\n".join(svg) + "\n")

    all_rows = [row("all", sq, inv, native)
                for sq in (0, .5, 1) for inv in INV for native in (0, .1, 1)]
    reference = row("all", .5, 10, .1)
    lines = [r"\documentclass[10pt]{article}",
             r"\usepackage[margin=0.62in]{geometry}",
             r"\usepackage{booktabs,longtable,graphicx,url}",
             r"\title{N19: Signed-label Kummer fallback, paired cohort}",
             r"\author{Frozen 1,260-target evidence packet}",
             r"\date{25 October 2026}",
             r"\begin{document}\maketitle",
             "The frozen arm recomputes a public signed Jacobian successor when a selected theta xADD output is zero. "
             "Every executed fallback operation is charged. Source scalars and paired negation-rho answers are independently replayed. "
             "The 21-model forced-zero gate and one diagnostic target are separate from the timed cohort.",
             r"\begin{center}\begin{tabular}{lr}\toprule Measure & Observed \\ \midrule",
             f"Candidate verified targets & {summary['candidate_verified']} / 1260 " + r"\\",
             f"Rho verified targets & {summary['rho_verified']} / 1260 " + r"\\",
             f"Natural fallback steps & {summary['fallback_steps']} " + r"\\",
             f"Fallback failures & {summary['fallback_failures']} " + r"\\",
             f"Reference cell full-cold candidate/rho & {number(reference['candidate_cold_over_rho'],6)} " + r"\\",
             f"Reference cell full-cold candidate/baseline & {number(reference['candidate_cold_over_baseline'],9)} " + r"\\",
             r"\bottomrule\end{tabular}\end{center}",
             r"\begin{center}\includegraphics[width=0.97\linewidth]{research/kummer_differential_collector_20261022/n19-labelled-fallback/cost-graph.png}\end{center}",
             r"\noindent The graph shows every bit slice at four inversion prices for $S/M=0.5$ and native logical charge 0.1 GAE. "
             r"The complete 36-cell full-cohort grid follows; a null is retained if any candidate in that cell failed.",
             r"\small\begin{longtable}{rrrrrrrr}\toprule $S/M$ & $I/M$ & Native & Candidate/rho & Candidate/baseline & Wins & Losses & Nulls \\ \midrule\endhead"]
    for r in all_rows:
        lines.append(f"{r['S_per_M']} & {r['I_per_M']} & {r['native_GAE_per_logical']} & "
                     f"{number(r['candidate_cold_over_rho'],6)} & {number(r['candidate_cold_over_baseline'],9)} & "
                     f"{r['candidate_wins']} & {r['candidate_losses']} & {r['null_ratios']} " + r"\\")
    lines += [r"\bottomrule\end{longtable}\normalsize",
              r"\noindent The standardized target-level and slice-level CSVs, raw JSONL, setup receipts, "
              r"independent validator and source hashes accompany this PDF. No stage result is substituted for full-cold cost.",
              r"\end{document}"]
    (HERE / "REPORT.tex").write_text("\n".join(lines) + "\n")
    print(json.dumps({"reference_candidate_over_rho": number(reference["candidate_cold_over_rho"], 6),
                      "graph": str(HERE / "cost-graph.svg"), "latex": str(HERE / "REPORT.tex")}))


if __name__ == "__main__":
    main()
