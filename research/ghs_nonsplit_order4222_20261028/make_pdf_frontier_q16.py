#!/usr/bin/env python3
"""Render the exact finite q16 direct-screen boundary as a compact PDF."""

import json
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import (
    HRFlowable, KeepTogether, Paragraph, SimpleDocTemplate, Spacer,
    Table, TableStyle,
)

HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "frontier_q16.json").read_text())
prior = json.loads((HERE / "prior_work_frontier_q16.json").read_text())
assert data["outside_orders"] == [3974, 3978, 4178]
assert [x["source_order"] for x in data["candidates"]] == data["outside_orders"]
assert [x["lmfdb_label"] for x in prior["candidates"]] == [
    "3.16.a_a_aet", "3.16.a_a_aep", "3.16.a_a_dd"]

output = HERE / "output/pdf/GHS_Q16_DIRECT_SCREEN_FRONTIER.pdf"
output.parent.mkdir(parents=True, exist_ok=True)
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name="ReportTitle", parent=styles["Title"],
                          fontName="Helvetica-Bold", fontSize=18, leading=22,
                          textColor=colors.HexColor("#17324f"), spaceAfter=13))
styles.add(ParagraphStyle(name="ReportSection", parent=styles["Heading2"],
                          fontName="Helvetica-Bold", fontSize=11.5, leading=14,
                          textColor=colors.HexColor("#174f77"),
                          spaceBefore=11, spaceAfter=6, keepWithNext=True))
styles.add(ParagraphStyle(name="ReportBody", parent=styles["BodyText"],
                          fontName="Helvetica", fontSize=9, leading=12.4,
                          spaceAfter=6))
styles.add(ParagraphStyle(name="TableHead", parent=styles["ReportBody"],
                          fontName="Helvetica-Bold", fontSize=8, leading=10,
                          alignment=TA_CENTER))
styles.add(ParagraphStyle(name="TableCell", parent=styles["ReportBody"],
                          fontSize=8, leading=10, alignment=TA_CENTER))
styles.add(ParagraphStyle(name="Small", parent=styles["ReportBody"],
                          fontSize=8.4, leading=11.5))


def p(text, style="ReportBody"):
    return Paragraph(text, styles[style])


story = [
    p("Q16 GHS direct-screen isogeny frontier", "ReportTitle"),
    HRFlowable(width="100%", thickness=1, color=colors.HexColor("#b8cbd8")),
    Spacer(1, 9),
    p("Exact structural bound", "ReportSection"),
    p("Across all <b>8,190</b> normalized ordinary elliptic models over F<sub>4096</sub>, "
      "the triangle, Hess, and nonsplit direct genus-three V4 screens cover "
      "<b>6,158</b> models. Their entire F<sub>4096</sub> isogeny closure "
      "covers <b>8,124</b>. Exactly <b>66 models in three source-order "
      "classes</b> lie outside that closure. This excludes those models from "
      "the three specified direct-cover routes, including all source-field "
      "isogeny walks into those routes."),
    p("Exact candidate representatives", "ReportSection"),
    p("K = F<sub>2</sub>[w]/(w<super>12</super>+w<super>7</super>+w<super>6</super>+"
      "w<super>5</super>+w<super>3</super>+w+1). Each source has "
      "Y<super>2</super>+XY = X<super>3</super>+32X<super>2</super>+b; "
      "all coefficients and coordinates below are polynomial-basis masks."),
]

headers = ["#E", "b", "Models", "r", "G", "Q = 7G", "LMFDB class"]
rows = [[p(x, "TableHead") for x in headers]]
for candidate, catalog in zip(data["candidates"], prior["candidates"]):
    rows.append([p(str(candidate["source_order"]), "TableCell"),
                 p(str(candidate["source_b"]), "TableCell"),
                 p(str(candidate["model_count"]), "TableCell"),
                 p(str(candidate["subgroup_prime"]), "TableCell"),
                 p(str(tuple(candidate["G"])), "TableCell"),
                 p(str(tuple(candidate["Q_equals_7G"])), "TableCell"),
                 p(catalog["lmfdb_label"], "TableCell")])
table = Table(rows, colWidths=[43, 34, 44, 39, 73, 73, 99], repeatRows=1)
table.setStyle(TableStyle([
    ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#e8f1f6")),
    ("GRID", (0, 0), (-1, -1), 0.35, colors.HexColor("#b8cbd8")),
    ("VALIGN", (0, 0), (-1, -1), "MIDDLE"),
    ("TOPPADDING", (0, 0), (-1, -1), 7),
    ("BOTTOMPADDING", (0, 0), (-1, -1), 6),
]))
story += [table, Spacer(1, 6)]
story += [
    p("Proof and source checks", "ReportSection"),
    p("The exact finite-field replay enumerates each screen and uses a "
      "trace-dual Walsh transform to compute every elliptic source order. "
      "The 6,158 screened models attain 125 of the 128 orders in the full "
      "cohort. The missing orders are 3974, 3978, and 4178, containing "
      "12, 30, and 24 models. The finite-field isogeny theorem makes "
      "equal source order equivalent to F<sub>4096</sub>-isogeny, proving "
      "the 8,124/66 split without a walk-radius limit. Sage's native "
      "elliptic routine independently recounts the three representatives "
      "and verifies their listed subgroup points."),
    p("What the exception means", "ReportSection"),
    p("LMFDB already records genus-three hyperelliptic Jacobians in all "
      "three classes. The simple-root gate gcd(r, 3(1-16<super>3</super>))=1 "
      "proves that some isogeny preserves each selected prime subgroup. "
      "That existence result supplies no evaluated map or attack timing. "
      "The order-3978 class was also present in the earlier repository "
      "screen; its selected prime has only 17 elements. The order-3974 "
      "and order-4178 classes have selected primes 1987 and 2089 and "
      "are the stronger map-construction targets."),
    p("Complete Magma parameter sweep on representatives", "ReportSection"),
    p("For each exact representative, Magma 2.30-1 exhausted all 4,095 "
      "nonzero parameters c in its documented WeilDescentGenus intrinsic: "
      "30 intrinsic rejections, 30 genus-four outputs, 450 genus-six "
      "outputs, and 3,585 genus-seven outputs. <b>Zero parameters gave "
      "genus three.</b> This is a complete parameter-specific result for "
      "those equations; it does not exclude other covers or the "
      "Jacobian-isogeny route."),
    p("Claim boundary and sources", "ReportSection"),
    p("This is a new <i>finite exclusion certificate for these three "
      "screens</i>, not a new geometric family, new transfer method, or "
      "measured GHS advantage. Other covers and correspondences remain "
      "possible. Exact equations, all 66 source masks, code, LMFDB labels, "
      "and per-candidate prior-work checks are in Q16_FRONTIER.md, "
      "frontier_q16.json, and prior_work_frontier_q16.json. Relevant prior "
      "methods: Murai et al. 2022 (minimal-genus classification) and "
      "Tian 2020/2023 (Weil-restriction isogeny transfer).", "Small"),
]


def footer(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor("#b8cbd8"))
    canvas.line(0.65 * inch, 0.55 * inch, 7.85 * inch, 0.55 * inch)
    canvas.setFont("Helvetica", 7.5)
    canvas.setFillColor(colors.HexColor("#687781"))
    canvas.drawString(0.67 * inch, 0.38 * inch,
                      "Q16 direct-screen frontier | Exact finite audit | 2026-10-28")
    canvas.drawRightString(7.83 * inch, 0.38 * inch, f"Page {doc.page}")
    canvas.restoreState()


doc = SimpleDocTemplate(str(output), pagesize=letter,
                        leftMargin=0.68 * inch, rightMargin=0.68 * inch,
                        topMargin=0.60 * inch, bottomMargin=0.65 * inch,
                        title="Q16 GHS direct-screen isogeny frontier",
                        author="GHS research audit")
doc.build(story, onFirstPage=footer, onLaterPages=footer)
print(output)
