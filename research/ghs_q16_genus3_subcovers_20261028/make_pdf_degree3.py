#!/usr/bin/env python3
"""Render the nonzero-trace degree-three cover finding as a compact PDF."""

import json
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import HRFlowable, Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle


HERE = Path(__file__).resolve().parent
OUT = HERE / "GHS_Q16_DEGREE3_NONZERO_TRACE.pdf"
scan = json.loads((HERE / "degree3_all_q16_scan_receipt.json").read_text())
assert scan["counts"] == {"models": 10664, "exact_hits": 60}
records = []
for index in (5, 12):
    suffix = "" if index == 5 else "_12"
    cover = json.loads((HERE / f"degree3_nonzero_trace_receipt{suffix}.json").read_text())
    attack = json.loads((HERE / f"attack_degree3_receipt{suffix}.json").read_text())["rows"][0]
    assert cover["catalog_model_index"] == index and attack["recovered_log"] == 7
    records.append((cover, attack))

styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name="TitleX", parent=styles["Title"], fontName="Helvetica-Bold",
    fontSize=16, leading=19, textColor=colors.HexColor("#13334c"), spaceAfter=5))
styles.add(ParagraphStyle(name="DeckX", parent=styles["Normal"], fontSize=9.2,
    leading=12, textColor=colors.HexColor("#38566b"), spaceAfter=9))
styles.add(ParagraphStyle(name="HeadX", parent=styles["Heading2"], fontName="Helvetica-Bold",
    fontSize=10.3, leading=12, textColor=colors.HexColor("#123d5a"), spaceBefore=8, spaceAfter=4))
styles.add(ParagraphStyle(name="BodyX", parent=styles["BodyText"], fontSize=8.2,
    leading=10.7, spaceAfter=5))
styles.add(ParagraphStyle(name="SmallX", parent=styles["BodyText"], fontSize=7.2,
    leading=9.5, spaceAfter=3))
styles.add(ParagraphStyle(name="CellX", parent=styles["BodyText"], fontSize=7.2,
    leading=9.0))


def P(text, style="BodyX"):
    return Paragraph(text, styles[style])


def table(rows, widths):
    tab = Table([[P(str(x), "CellX") for x in row] for row in rows],
                colWidths=widths, repeatRows=1, hAlign="LEFT")
    tab.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#e4eef5")),
        ("ROWBACKGROUNDS", (0, 1), (-1, -1), [colors.white, colors.HexColor("#f7fafc")]),
        ("LINEBELOW", (0, 0), (-1, 0), 0.6, colors.HexColor("#9bb5c6")),
        ("VALIGN", (0, 0), (-1, -1), "TOP"),
        ("LEFTPADDING", (0, 0), (-1, -1), 5),
        ("RIGHTPADDING", (0, 0), (-1, -1), 5),
        ("TOPPADDING", (0, 0), (-1, -1), 4),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 4),
    ]))
    return tab


story = [
    P("Q16 NONZERO-TRACE DEGREE-THREE COVERS", "TitleX"),
    P("Exact curve maps, subgroup transfer, bounded census, and same-target cost | 28 October 2026", "DeckX"),
    HRFlowable(width="100%", thickness=1.1, color=colors.HexColor("#2a7c99")),
    Spacer(1, 6),
    P("<b>Finding.</b> Two Frobenius orbits of elliptic curves over F_(16^3) have exact degree-three covers from cataloged ordinary genus-three curves over F_16, although the elliptic coefficient b has nonzero relative trace. Both representative 409-order subgroup maps are nonzero and both complete relation solves recover 7. Their measured full transfer is more expensive than rho. The exact cover instances are candidates for novelty; a new attack method or measured vulnerability is not established."),
    P("Bounded catalog screen", "HeadX"),
    P("The frozen LMFDB export contains 126 ordinary genus-three Weil-restriction-shaped classes and 10,664 equations. All 24 branch-supported degree-three exponent patterns were tested for each model: 255,936 model-pattern pairs. Every residue system has full column rank 24. Sixty exact identities survived, representing 30 distinct source equations in six classes. Of those descriptions, 24 have trace-zero b, 12 have b=1, and 24 have nonzero-trace b outside the trace-zero locus and b outside the base field. This is a complete screen for the displayed catalog and rational-function ansatz, not all GHS covers."),
    P("Exact cover criterion and bound", "HeadX"),
    P("For C: y^2+h(x)y=f(x), let r_i be the four roots of separable h in K=F_(16^3), and choose an exponent permutation e_i of (2,1,-2,-1). Set U=product (x-r_i)^e_i, X=lambda U, beta=lambda mu, b=beta^2. If f/h^2 + X + beta/X = H^2+H+u in K(x), then Y=X(y/h+H)+beta gives a degree-three map C_K to E: Y^2+XY=X^3+uX^2+b. Four nonzero reduced branch residues force every branch root to be a zero or pole of X. Other degree-three branch patterns have an unremovable order-three pole. With full-rank residue matrix, at most one (lambda,mu) pair can occur for each fixed model and pattern."),
    P("Two explicit 409-order subgroup representatives", "HeadX"),
    P("Here K=F_2[w]/(w^12+w^7+w^6+w^5+w^3+w+1), k=F_2[a]/(a^4+a+1), and the embedding sends a to K mask 1820. Integer masks give polynomial-basis coefficients. Both source curves are E: Y^2+XY=X^3+uX^2+b; both have order 4090=10*409. The exact H numerator and denominator are in the map receipts."),
]

rows = [["LMFDB model", "Base curve C: y^2+h y=f", "(u,b), trace(b)", "(lambda,mu)"]]
for cover, _ in records:
    rows.append([f"3.16.a_a_ah / {cover['catalog_model_index']}", cover["catalog_equation"],
                 f"({cover['u_mask']},{cover['b_mask']}), {cover['relative_trace_b_mask']}",
                 f"({cover['lambda_mask']},{cover['mu_mask']})"])
story.extend([table(rows, [0.75*inch, 3.86*inch, 1.23*inch, 1.13*inch]),
              Spacer(1, 5)])

rows = [["Model", "G and Q=7G", "Image of G (Mumford u;v)", "Relations / steps", "Full / rho mean", "Ratio"]]
for cover, attack in records:
    rows.append([str(cover["catalog_model_index"]),
                 f"{cover['G_xy_masks']}; {cover['Q_equals_7G_xy_masks']}",
                 f"{cover['image_G_u_masks']}; {cover['image_G_v_masks']}",
                 f"{attack['relation_rows']} / {attack['walk_steps']}",
                 f"{attack['full_transfer_wall_seconds']:.3f} s / {1000*attack['rho_mean_one_solve_wall_seconds']:.3f} ms",
                 f"{attack['full_transfer_over_mean_rho_wall_ratio']:.0f}x"])
story.extend([table(rows, [0.38*inch, 1.47*inch, 1.78*inch, 0.73*inch, 1.76*inch, 0.85*inch]),
    Spacer(1, 5),
    P("The map pulls back P-2P and adds the three Frobenius conjugates. Both images are nonzero, have order 409, are defined over k, and map Q to seven times the image of G. Each full cost includes construction, transfer, factor-base setup, relation collection, and linear solve. Rho is the mean of 1,000 independent same-target solves in Sage. The relation rows and rho step samples are retained in the receipts.", "SmallX"),
    P("Prior work and limits", "HeadX"),
    P("Joux-Vitse (EUROCRYPT 2012, Appendix A.2) publishes characteristic-two trace-zero genus-three covers. General GHS conorm maps and cover/decomposition attacks are earlier work. The displayed b values have nonzero relative trace, so that specific trace-zero construction does not explain these maps. Search did not establish that the exact equations are absent from all publications. The Lean checks cover finite accounting and group-order arithmetic; Sage checks the exact identities, finite-field map, and attack. No global classification of vulnerable curves follows from this census."),
    P("Sources: Joux-Vitse, eprint.iacr.org/2011/020.pdf; Magma handbook, math.uzh.ch/sepp/magma-2.25.2-ds/HandbookVolume10.pdf; Iijima et al. (2005), doi:10.1093/ietfec/e88-a.1.97; LMFDB classes 3.16.a_a_ah and 3.16.a_a_h. Companion REPORT.md and JSON receipts retain the exact input, provenance, maps, and measured costs.", "SmallX"),
])


def footer(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#b8ccd7"))
    canvas.line(0.66*inch, 0.52*inch, 7.84*inch, 0.52*inch)
    canvas.setFont("Helvetica", 7.3)
    canvas.setFillColor(colors.HexColor("#557085"))
    canvas.drawString(0.66*inch, 0.35*inch, "GHS Q16 DEGREE-THREE COVERS | finite evidence and claim boundary")
    canvas.drawRightString(7.84*inch, 0.35*inch, f"Page {doc.page}")
    canvas.restoreState()


doc = SimpleDocTemplate(str(OUT), pagesize=(8.5*inch, 11*inch),
    leftMargin=0.66*inch, rightMargin=0.66*inch,
    topMargin=0.62*inch, bottomMargin=0.72*inch,
    title="Q16 nonzero-trace degree-three GHS cover probe", author="GHS research audit")
doc.build(story, onFirstPage=footer, onLaterPages=footer)
print(OUT)
