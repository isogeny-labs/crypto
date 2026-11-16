#!/usr/bin/env python3
"""Draw the PDP-formulation landscape as an SVG (layout only, no computation).

Boxes: the standard pipeline (Semaev summation polynomial -> Weil descent ->
Groebner), the points where each lead card (LEADS.md) intervenes, and the
bounds that fence the landscape. Edges are labelled proposed; nothing here is
measured. Source for figures/pdp_landscape.svg.
"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "pdp_landscape.svg")

W, H = 1400, 700
FONT = "font-family='DejaVu Sans, Arial, sans-serif'"


def box(x, y, w, h, title, lines, fill="#f4f6fa", stroke="#334", tsize=12, lsize=9.8):
    s = [f"<rect x='{x}' y='{y}' width='{w}' height='{h}' rx='8' ry='8' fill='{fill}' stroke='{stroke}' stroke-width='1.4'/>"]
    ty = y + 17
    s.append(f"<text x='{x + w / 2}' y='{ty}' text-anchor='middle' {FONT} font-size='{tsize}' font-weight='bold'>{title}</text>")
    for ln in lines:
        ty += 13.5
        s.append(f"<text x='{x + w / 2}' y='{ty}' text-anchor='middle' {FONT} font-size='{lsize}'>{ln}</text>")
    return "\n".join(s)


def arrow(x1, y1, x2, y2, dashed=False, color="#334"):
    dash = " stroke-dasharray='6,4'" if dashed else ""
    return f"<line x1='{x1}' y1='{y1}' x2='{x2}' y2='{y2}' stroke='{color}' stroke-width='1.4'{dash} marker-end='url(#arr)'/>"


def main():
    parts = [
        f"<svg xmlns='http://www.w3.org/2000/svg' width='{W}' height='{H}' viewBox='0 0 {W} {H}'>",
        "<defs><marker id='arr' markerWidth='10' markerHeight='10' refX='9' refY='3' orient='auto'>"
        "<path d='M0,0 L0,6 L9,3 z' fill='#334'/></marker></defs>",
        f"<rect width='{W}' height='{H}' fill='white'/>",
        f"<text x='{W / 2}' y='24' text-anchor='middle' {FONT} font-size='15' font-weight='bold'>"
        "Point decomposition: the standard pipeline and where the 14 lead cards intervene (every edge proposed, none measured)</text>",
    ]
    # Standard pipeline (top row): four boxes of width 290 with 40 px gaps.
    xs = [40, 370, 700, 1030]
    bw, by, bh = 290, 48, 80
    parts.append(box(xs[0], by, bw, bh, "1. Relation search", ["Q = P_1 + ... + P_m with P_i in a factor", "base F (subspace V, small x, digits)"]))
    parts.append(box(xs[1], by, bw, bh, "2. Summation polynomial", ["S_{m+1}(x_1, ..., x_m, x_Q) = 0: one equation", "over K, degree 2^{m-1} in each variable"]))
    parts.append(box(xs[2], by, bw, bh, "3. Weil descent / digit expansion", ["n equations over F_2 (or F_q) in m·ℓ unknowns;", "symmetrise under S_m and torsion"]))
    parts.append(box(xs[3], by, bw, bh, "4. Groebner / XL / SAT solve", ["cost set by the solving degree D; measured", "4 (msolve) to 7 (XL); PKM tower D ≥ 8 at N = 20"]))
    for i in range(3):
        parts.append(arrow(xs[i] + bw, by + bh / 2, xs[i + 1], by + bh / 2))

    # Lead boxes (second row).
    g1, g2, g3, g4 = "#eef7ee", "#eef1fb", "#fff4e6", "#f6eef7"
    ly, lh = 170, 96
    parts.append(box(xs[0], ly, bw, lh, "Replace stage 2 (the polynomial)", [
        "LEAD-01 MinRank / Kipnis–Shamir / support minors",
        "LEAD-10 chord (slope) small-root family over F_p",
        "LEAD-13 summation ideals and Nagao on covers",
        "LEAD-11 McGuire–Mueller evaluation (verify text)"], fill=g1))
    parts.append(box(xs[1], ly, bw, lh, "Replace stage 3 (descent, coordinates)", [
        "LEAD-03 ℓ-torsion (ℓ = 3, 4) symmetrisation,",
        "Kummer-model (Montgomery, theta) coordinates",
        "LEAD-06 trace-zero representation (n = 3, 5)",
        "LEAD-14 digit / tower systems as block systems"], fill=g1))
    parts.append(box(xs[2], ly, bw, lh, "Replace or amortise stage 4 (the solve)", [
        "LEAD-02 parametric eliminant: Groebner once per",
        "curve, per-target univariate root finding",
        "LEAD-07 Crossbred on the (ℓ, m) degree cells",
        "LEAD-09 Ore / linearized-polynomial elimination"], fill=g2))
    parts.append(box(xs[3], ly, bw, lh, "Cut wasted solves, steer the solver", [
        "LEAD-04 homomorphic class pre-partition",
        "(Tate pairing, ℓ-descent, binary trace parity)",
        "LEAD-08 learned orderings and branching for",
        "F4 and WDSat (home: ml-cryptanalysis)"], fill=g3))
    for i in range(4):
        parts.append(arrow(xs[i] + bw / 2, ly, xs[i] + bw / 2, by + bh, dashed=True))

    # Measurement / theory row.
    my, mh = 300, 78
    parts.append(box(40, my, 620, mh, "Measure the degree law (LEAD-05)", [
        "S_3 closure degree 3 | 4 | 5 for n ≤ 16 | 17–36 | 37–52, then slope ≈ 0.036 (Solovyev 2026, GitHub preprint);",
        "Kosters–Yeo 2015: 3, 4, 4, ≥ 5 at n = 16, 20, 30, 40; this repository: msolve 4 at cells (15, 5) and (11, 4),",
        "XL refutation 5–7 on the same cells. Which instrument is 'the' degree of regularity is the open question."], fill=g4, lsize=9.4))
    parts.append(box(700, my, 660, mh, "Theory gate (LEAD-12): which oracles can move the exponent at all", [
        "Corrigan-Gibbs–Kogan / Shoup: S·T² ≳ n for every oracle built from group operations and membership tests;",
        "structured GGM (ePrint 2026/384): structure must be exploited on ≥ q^{-1/2} of the points;",
        "non-adaptive bounds (ePrint 2025/783): table-lookup hybrids below √N advice need T = Ω(√N)."], fill=g4, lsize=9.4))

    # Closed ground (bottom).
    cy, ch = 410, 150
    parts.append(box(40, cy, 1320, ch, "Closed ground the cards must not re-open (theorem, exact computation or repository measurement)", [
        "BKK / mixed volume = Bézout · Semaev-cover monodromy (Z/2)^{m-2} · factor-base geometry conserves yield (KN-FIND-007) · isogeny transport ≤ a constant",
        "xedni and lifting (exact α = 3) · GGMP Frobenius-stable bases at n = 131 (none) · GHS and Joux–Vitse covers for ECC2K-130 (genus 2^130)",
        "single-polynomial Coppersmith for S_5 (reach 1/80) · shared-target Coppersmith 1.12× (2026-10-14) · Kummer-exponent Fourier spectrum flat (2026-10-14)",
        "quasi-subfield bases: no yield gain at n = 31 · algebraic-base solution counts 0.93–1.03× random · record-and-replay F4 2–7 % · PKM plateau broke (D ≥ 8)",
        "Baseline (docs/ic/LEADERBOARD.md): every row above matched rho, Koblitz 5.23×–21×, prime 5.47×–13.5×; one live exponent regime: cubic-GHS genus-3 at q^{1.30} vs rho q^{1.5}",
        "Success criterion (owner, 2026-10-15): every verified, measured improvement over the index-calculus baseline is a success; the gap to rho is reported as a distance, not a gate"],
        fill="#fbeeee", stroke="#a33", lsize=9.4))

    parts.append(f"<text x='{W / 2}' y='{H - 60}' text-anchor='middle' {FONT} font-size='10'>"
                 "Legend: green = a formulation replaces a pipeline stage; blue = solver; orange = filter or steering; purple = measurement or theory; red = closed ground.</text>")
    parts.append(f"<text x='{W / 2}' y='{H - 42}' text-anchor='middle' {FONT} font-size='10'>"
                 "Dashed arrows: proposed intervention points. Study research/pdp_literature_leads_20261022 (2026-10-22); cards in LEADS.md, assessments in leads/LEAD-NN.md, verifications in leads/LEAD-NN.verify.md.</text>")
    parts.append("</svg>")
    with open(OUT, "w") as f:
        f.write("\n".join(parts))
    print("wrote", OUT)


if __name__ == "__main__":
    main()
