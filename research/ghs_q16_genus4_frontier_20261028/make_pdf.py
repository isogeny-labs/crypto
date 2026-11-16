#!/usr/bin/env python3
"""Render the q16 genus-four frontier result as a compact PDF report."""

import json
from collections import Counter
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER, TA_LEFT
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import (
    HRFlowable, KeepTogether, Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle,
)

HERE = Path(__file__).resolve().parent
summary = json.loads((HERE / "summary.json").read_text())
OUT = HERE / "GHS_Q16_GENUS4_FRONTIER.pdf"

styles = getSampleStyleSheet()
styles.add(ParagraphStyle(
    name="TitleX", parent=styles["Title"], fontName="Helvetica-Bold",
    fontSize=17, leading=19.5, textColor=colors.HexColor("#132c48"),
    alignment=TA_LEFT, spaceAfter=6,
))
styles.add(ParagraphStyle(
    name="Deck", parent=styles["Normal"], fontSize=10.2, leading=14.2,
    textColor=colors.HexColor("#38536a"), spaceAfter=10,
))
styles.add(ParagraphStyle(
    name="H1X", parent=styles["Heading2"], fontName="Helvetica-Bold",
    fontSize=10.8, leading=13, textColor=colors.HexColor("#123d5a"),
    spaceBefore=9, spaceAfter=4,
))
styles.add(ParagraphStyle(
    name="BodyX", parent=styles["BodyText"], fontSize=8.35, leading=11.1,
    spaceAfter=5,
))
styles.add(ParagraphStyle(
    name="SmallX", parent=styles["BodyText"], fontSize=7.25, leading=9.3,
    textColor=colors.HexColor("#43596b"), spaceAfter=4,
))
styles.add(ParagraphStyle(
    name="TinyX", parent=styles["BodyText"], fontSize=7.2, leading=9,
    alignment=TA_CENTER,
))


def P(text, style="BodyX"):
    return Paragraph(text, styles[style])


def heading(text):
    return P(text, "H1X")


def table(data, widths):
    t = Table(data, colWidths=widths, repeatRows=1, hAlign="LEFT")
    t.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#e3edf5")),
        ("TEXTCOLOR", (0, 0), (-1, 0), colors.HexColor("#123d5a")),
        ("ROWBACKGROUNDS", (0, 1), (-1, -1),
         [colors.white, colors.HexColor("#f7fafc")]),
        ("LINEBELOW", (0, 0), (-1, 0), 0.6, colors.HexColor("#8daabb")),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 4),
        ("TOPPADDING", (0, 0), (-1, -1), 4),
        ("LEFTPADDING", (0, 0), (-1, -1), 5),
        ("RIGHTPADDING", (0, 0), (-1, -1), 5),
        ("VALIGN", (0, 0), (-1, -1), "MIDDLE"),
    ]))
    return t


story = [
    P("Q16 GENUS-FOUR GHS FRONTIER", "TitleX"),
    P("Exact 66-curve census, evaluated subgroup maps, checked genus bound, "
      "and same-target attack costs | 28 October 2026", "Deck"),
    HRFlowable(width="100%", thickness=1.2, color=colors.HexColor("#2a7c99")),
    Spacer(1, 7),
    P("<b>Finding.</b> All 66 ordinary curves over F_(16^3) outside the "
      "K-isogeny closure of three earlier direct genus-three V4 screens "
      "have an evaluated, nonzero prime-subgroup transfer to a genus-four "
      "hyperelliptic Jacobian over F_16. A complete standard GHS parameter "
      "sweep returned no genus-three curve for any of the 66 equations. "
      "Full relation solves on three representatives were slower than rho."),
    heading("Exact finite cohort"),
    P("Let K = F_2[w]/(w^12+w^7+w^6+w^5+w^3+w+1), k = F_16, and "
      "E_b: y^2 + xy = x^3 + w^5 x^2 + b. Each b is a polynomial-basis bit "
      "mask. The source certificate in the repository lists every mask and "
      "proves the three-screen isogeny exclusion."),
]

rows = [[P("#E(K)", "TinyX"), P("Models", "TinyX"),
         P("Prime r", "TinyX"), P("b of measured source", "TinyX"),
         P("c=1 Jacobian order(s)", "TinyX")]]
for record in summary["classes"]:
    orders = Counter(m["jacobian_order"] for m in record["models"])
    row = [str(record["source_order"]), str(len(record["models"])),
           str(record["subgroup_prime"]), str(record["representative_b_mask"]),
           ", ".join(f"{n:,} ({count})" for n, count in sorted(orders.items()))]
    rows.append([P(s, "TinyX") for s in row])
story.extend([
    table(rows, [0.75*inch, 0.65*inch, 0.75*inch, 1.2*inch, 2.65*inch]),
    Spacer(1, 4),
    P("<b>Exhaustive parameter result:</b> for each equation, all 4,095 nonzero "
      "c in K were evaluated by Magma 2.30-1: 30 genus four, 450 genus six, "
      "3,585 genus seven, and 30 intrinsic rejections. Across 66 equations "
      "this accounts for 270,270 pairs, with zero genus-three outputs. "
      "The bound is exact for this documented intrinsic and parameterization; "
      "it is not a classification of all covers or isogenies."),
    P("<b>Subgroup map:</b> Magma's WeilDescent(E_b,k,1) returned C_b and h_b. "
      "For every b, the retained script checks genus(C_b)=4, G_b nonzero "
      "of order r, h_b(G_b) nonzero of order r, and h_b(7G_b)=7h_b(G_b). "
      "The exact G_b and reduced Mumford images are in the 66-model receipts."),
    heading("Necessary genus bound for any transfer"),
    P("For every one of the 66 source models, the Weil-restriction "
      "Frobenius polynomial is X^6 - tX^3 + 4096, with t in {123, 119, -81}. "
      "Sage Rabin tests prove these polynomials irreducible modulo 11, 11, "
      "and 5 respectively. By the finite-field isogeny theorem, each "
      "restriction is a simple threefold. Any nonzero algebraic transfer "
      "from it to a Jacobian requires target genus at least three."),
    P("Independently, if a prime-order subgroup r maps nontrivially into J(C)(F_q), then "
      "r divides #J(C)(F_q). The Weil bound implies "
      "r &lt;= (sqrt(q)+1)^(2g). At q=16, genus at most two gives "
      "#J &lt;= 5^4 = 625; hence the r=1,987 and r=2,089 subgroups require "
      "g &gt;= 3 for any subgroup-preserving transfer. GenusBound.lean "
      "checks the finite divisibility contradiction. The Weil bound, "
      "Lagrange's theorem, and the coordinate checks are explicit external inputs."),
    heading("End-to-end representative comparison"),
])

rows = [[P("#E (b)", "TinyX"), P("Relations", "TinyX"),
         P("GHS steps", "TinyX"), P("GHS CPU", "TinyX"),
         P("Rho mean steps", "TinyX"), P("Rho mean CPU", "TinyX"),
         P("GHS/rho", "TinyX")]]
for rec in summary["classes"]:
    a = rec["representative_attack"]
    vals = [f"{rec['source_order']} ({rec['representative_b_mask']})",
            str(a["relation_count"]), f"{a['relation_walk_steps']:,}",
            f"{a['full_ghs_cpu_seconds']:.2f} s",
            f"{a['rho_total_steps']/1000:.2f}",
            f"{a['rho_mean_cpu_seconds']:.5f} s",
            f"{a['ghs_over_mean_rho_cpu']:.0f}x"]
    rows.append([P(s, "TinyX") for s in vals])
story.extend([
    table(rows, [0.95*inch, 0.65*inch, 0.75*inch, 0.75*inch,
                 0.9*inch, 1.0*inch, 0.65*inch]),
    Spacer(1, 4),
    P("All three relation matrices recover 7 and verify it in the source "
      "and target groups. The source rho column uses 1,000 successful "
      "same-target trials with degenerate-collision restarts. Magma CPU "
      "has 0.01-second reporting granularity; ratios are finite-size "
      "same-backend observations. The remaining 63 full attack costs "
      "are unmeasured.", "SmallX"),
    heading("Prior work and claim boundary"),
    P("Binary cubic GHS genus-three/genus-four outcomes and Magma's transfer "
      "intrinsic are established prior art. All three Weil-restriction "
      "isogeny classes already have catalogued genus-three hyperelliptic "
      "Jacobians. The order-3,978 class also appears in earlier repository "
      "work. Thus this is a new exact finite census and map certificate, "
      "not an established new family, new method, or faster attack. "
      "An evaluated source-to-genus-three isogeny remains the main open route."),
    P("Sources: Gaudry-Hess-Smart, Weil Descent on Elliptic Curves "
      "(kantmath.de/publications/papers/weil_descent.pdf); Magma handbook "
      "(magma.maths.usyd.edu.au/magma/handbook/text/1561); Murai et al., "
      "SCIS 2022 abstract; LMFDB classes 3.16.a_a_aet, 3.16.a_a_aep, "
      "3.16.a_a_dd. Executable scripts, full receipts, Lean theorem, and "
      "66-row summary are in the companion research directory.", "SmallX"),
])


def page(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#b9cbd7"))
    canvas.line(0.66*inch, 0.53*inch, 7.84*inch, 0.53*inch)
    canvas.setFont("Helvetica", 7.5)
    canvas.setFillColor(colors.HexColor("#557085"))
    canvas.drawString(0.66*inch, 0.36*inch, "GHS Q16 GENUS-FOUR FRONTIER  |  exact finite evidence")
    canvas.drawRightString(7.84*inch, 0.36*inch, f"Page {doc.page}")
    canvas.restoreState()


doc = SimpleDocTemplate(
    str(OUT), pagesize=(8.5*inch, 11*inch),
    leftMargin=0.66*inch, rightMargin=0.66*inch,
    topMargin=0.63*inch, bottomMargin=0.72*inch,
    title="Q16 genus-four GHS frontier", author="GHS research audit",
)
doc.build(story, onFirstPage=page, onLaterPages=page)
print(OUT)
