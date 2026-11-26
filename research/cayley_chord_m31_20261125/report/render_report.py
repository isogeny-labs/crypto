#!/usr/bin/env python3
"""Lay out frozen evidence; this file performs no research computation or replay."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
from xml.sax.saxutils import escape

from reportlab import rl_config
from reportlab.graphics import renderSVG
from reportlab.graphics.shapes import Drawing, Line, Polygon, Rect, String
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    BaseDocTemplate, Frame, PageBreak, PageTemplate,
    Paragraph, Spacer, Table, TableStyle,
)

rl_config.invariant = 1
# Reproducible publication-date metadata; caller may supply another build epoch.
os.environ.setdefault("SOURCE_DATE_EPOCH", "1795651200")  # 2026-11-26 UTC
ROOT = Path(__file__).resolve().parent
EXPECTED_SUMMARY_SHA = "a4f1782166fd21ae3ef9211fd4158601940d4884a2450495c81e8dd95eb23ae9"
SOURCE_COMMIT = "eb55e93110676c91213d91d6c8d00489311a4bd8"
INK = colors.HexColor("#172B3A")
MUTED = colors.HexColor("#526675")
TEAL = colors.HexColor("#087F8C")
PALE = colors.HexColor("#ECF6F6")
BLUE = colors.HexColor("#3569A4")
LINE = colors.HexColor("#D6E1E7")
LIGHT = colors.HexColor("#F5F8FA")
WIDTH = 504
STAMP = "[pcc:unverified]"


def fonts() -> None:
    directory = Path("/usr/share/fonts/truetype/dejavu")
    for name, file in [
        ("ReportSans", "DejaVuSans.ttf"),
        ("ReportSans-Bold", "DejaVuSans-Bold.ttf"),
    ]:
        pdfmetrics.registerFont(TTFont(name, str(directory / file)))
    pdfmetrics.registerFontFamily(
        "ReportSans", normal="ReportSans", bold="ReportSans-Bold",
        italic="ReportSans", boldItalic="ReportSans-Bold",
    )


def label(drawing: Drawing, x: float, y: float, text: str,
          size: float = 9, color=INK, bold: bool = False,
          anchor: str = "start") -> None:
    drawing.add(String(x, y, text, fontName="ReportSans-Bold" if bold else "ReportSans",
                       fontSize=size, fillColor=color, textAnchor=anchor))


def extraction_diagram() -> Drawing:
    diagram = Drawing(WIDTH, 166)
    label(diagram, 0, 152, "SIGNED EXTRACTION FROM PUBLIC INPUTS", 9, TEAL, True)
    labels = [
        ("Input", ["Affine R = (w, yR)", "Cayley label c or λ"]),
        ("Slope", ["Quadratic in m", "At most two roots"]),
        ("Coordinates", ["X² - sX + q = 0", "For each slope"]),
        ("Signed output", ["yj = m(xj - w) - yR", "Affine pairs P + Q = R"]),
    ]
    box_width, gap = 114, 16
    for idx, (title, lines) in enumerate(labels):
        x = idx * (box_width + gap)
        diagram.add(Rect(x, 57, box_width, 74, rx=7, ry=7,
                         fillColor=PALE if idx in (1, 2) else LIGHT,
                         strokeColor=LINE, strokeWidth=0.8))
        label(diagram, x + 10, 111, title, 9.5, INK, True)
        for j, line in enumerate(lines):
            label(diagram, x + 10, 91 - j * 15, line, 7.25, MUTED)
        if idx < 3:
            start = x + box_width + 2
            end = x + box_width + gap - 2
            diagram.add(Line(start, 94, end, 94, strokeColor=TEAL, strokeWidth=1.2))
            diagram.add(Polygon([end, 94, end - 4, 97, end - 4, 91],
                                fillColor=TEAL, strokeColor=TEAL))
    label(diagram, 0, 34, "At most four ordered affine pairs; no separate ordinate square roots.", 9, INK)
    label(diagram, 0, 17, "Two quadratic stages can require up to three square-root calls. " + STAMP,
          8.1, MUTED)
    return diagram


def counts_chart(result: dict) -> Drawing:
    diagram = Drawing(WIDTH, 124)
    label(diagram, 0, 111, "RECORDED CORRECTNESS PANEL", 9, TEAL, True)
    categories = [
        ("Unplanted selectors", result["unplanted_selector_cases"], TEAL),
        ("Controlled / exceptional", result["control_cases"], BLUE),
    ]
    origin, span, scale = 158, 275, 2500
    for idx, (name, value, color) in enumerate(categories):
        y = 75 - idx * 34
        label(diagram, 0, y + 4, name, 8.8, INK)
        diagram.add(Rect(origin, y, span, 18, fillColor=LIGHT, strokeColor=None))
        diagram.add(Rect(origin, y, span * value / scale, 18,
                         fillColor=color, strokeColor=None))
        label(diagram, 451, y + 4, f"{value:,}", 10, INK, True, "end")
    label(diagram, origin, 15, "0", 7.5, MUTED)
    label(diagram, origin + span, 15, "2,500 cases", 7.5, MUTED, anchor="end")
    label(diagram, 0, 0, "Exact recorded counts; no timing axis or statistical interval. [1] " + STAMP,
          7.8, MUTED)
    return diagram


def build(result: dict) -> None:
    fonts()
    style = getSampleStyleSheet()
    style.add(ParagraphStyle("TitleX", fontName="ReportSans-Bold", fontSize=23,
                             leading=28, textColor=INK, spaceAfter=9))
    style.add(ParagraphStyle("SubtitleX", fontName="ReportSans", fontSize=9.3,
                             leading=13, textColor=MUTED, spaceAfter=12))
    style.add(ParagraphStyle("H2X", fontName="ReportSans-Bold", fontSize=12.4,
                             leading=16, textColor=INK, spaceBefore=12, spaceAfter=7))
    style.add(ParagraphStyle("BodyX", fontName="ReportSans", fontSize=9.1,
                             leading=13.1, textColor=INK, spaceAfter=8))
    style.add(ParagraphStyle("SmallX", fontName="ReportSans", fontSize=7.9,
                             leading=11.2, textColor=MUTED, spaceAfter=6))
    style.add(ParagraphStyle("FormulaX", fontName="ReportSans", fontSize=8.1,
                             leading=12.2, textColor=INK, spaceAfter=4))
    style.add(ParagraphStyle("TableX", fontName="ReportSans", fontSize=8.1,
                             leading=11.5, textColor=INK))
    style.add(ParagraphStyle("BannerX", fontName="ReportSans-Bold", fontSize=15,
                             leading=20, textColor=TEAL, spaceAfter=3))

    def p(text: str, kind: str = "BodyX") -> Paragraph:
        return Paragraph(text, style[kind])

    def heading(text: str) -> Paragraph:
        return p(text, "H2X")

    def formula(lines: list[str]) -> Table:
        inner = [[p(escape(line), "FormulaX")] for line in lines]
        table = Table(inner, colWidths=[WIDTH - 20], hAlign="LEFT")
        table.setStyle(TableStyle([
            ("BACKGROUND", (0, 0), (-1, -1), LIGHT),
            ("LEFTPADDING", (0, 0), (-1, -1), 10),
            ("RIGHTPADDING", (0, 0), (-1, -1), 10),
            ("TOPPADDING", (0, 0), (-1, 0), 9),
            ("BOTTOMPADDING", (0, -1), (-1, -1), 7),
        ]))
        return table

    def table(rows: list[list[str]], widths: list[float]) -> Table:
        data = [[p(escape(cell), "TableX") for cell in row] for row in rows]
        element = Table(data, colWidths=widths, hAlign="LEFT", repeatRows=1)
        element.setStyle(TableStyle([
            ("BACKGROUND", (0, 0), (-1, 0), PALE),
            ("ROWBACKGROUNDS", (0, 1), (-1, -1), [colors.white, LIGHT]),
            ("LINEBELOW", (0, 0), (-1, 0), 0.7, LINE),
            ("VALIGN", (0, 0), (-1, -1), "TOP"),
            ("LEFTPADDING", (0, 0), (-1, -1), 8),
            ("RIGHTPADDING", (0, 0), (-1, -1), 8),
            ("TOPPADDING", (0, 0), (-1, -1), 6),
            ("BOTTOMPADDING", (0, 0), (-1, -1), 6),
        ]))
        return element

    diagram, chart = extraction_diagram(), counts_chart(result)
    for drawing, name, title in [
        (diagram, "extraction-flow.svg", "Signed chord extraction from public inputs"),
        (chart, "recorded-case-counts.svg", "Recorded M31 correctness-panel composition"),
    ]:
        svg = renderSVG.drawToString(drawing)
        svg = svg.replace("font-family: ReportSans-Bold;",
                          "font-family: DejaVu Sans, sans-serif; font-weight: 700;")
        svg = svg.replace("font-family: ReportSans;", "font-family: DejaVu Sans, sans-serif;")
        svg = svg.replace("<title>...</title>", "<title>" + escape(title) + "</title>")
        svg = svg.replace("<desc>...</desc>", "<desc>" + STAMP + "; see REPORT.pdf for scope and sources.</desc>")
        (ROOT / name).write_text(svg, encoding="utf-8")

    story = [
        p("Cayley-constrained<br/>signed chord extraction", "TitleX"),
        p("M31 component qualification and product-label sampling bound<br/>"
          "Original evidence: 25 November 2026 | Publication: crypto PR #5232", "SubtitleX"),
        p("2,818 / 2,818 exact output-set matches", "BannerX"),
        p("Recorded local development checks on two established ordinary M31 fixtures. "
          "Producer evidence; no signed verifier verdict. [1] " + STAMP, "SmallX"),
        heading("1. Construction and checked domain"),
        p("The standalone Rust component takes a public affine target R=(w,yR), "
          "with yR nonzero, and a Cayley-product selector. It returns the complete set "
          "of affine ordered pairs P+Q=R satisfying that selector. A separately authored "
          "symmetric-Semaev implementation supplies the comparison. [1,2,4] " + STAMP),
        diagram,
        formula([
            "C(x) = (x+i)/(x-i),  i²=-1,  c=C(x1)C(x2)",
            "λ = i(1+c)/(1-c),  c≠1;  s=x1+x2,  q=x1*x2=1-λs",
            "(w+λ)m² + 2yR*m + (w²+a-1-λw) = 0",
            "s=m²-w;  q=w*m²+w²+a+2m*yR;  X²-sX+q=0",
            "yj=m(xj-w)-yR;  Δm/4=wλ²+(1-a)λ+w+b",
        ]),
        Spacer(1, 8),
        p("The line y=m(x-w)-yR passes through -R. Dividing its cubic intersection "
          "polynomial by x-w gives the coordinate quadratic. This fixes the signs "
          "without extra ordinate square roots. The formulas are an algebraic "
          "specialization of the chord/Semaev relation. [2,4] " + STAMP),
        p("<b>Exceptions:</b> λ=-w makes the first equation linear, with coefficient "
          "2yR. For c=1, use m²=w. Repeated summands are retained; identity summands "
          "are outside this affine interface. Nonsingularity is a caller precondition "
          "checked for both fixtures. Identity/zero-ordinate/off-curve targets and "
          "noncanonical inputs are rejected. [1,2] " + STAMP, "SmallX"),
        PageBreak(),
        p("Recorded qualification", "TitleX"),
        p("Study ID: cayley-chord-m31-correctness-v1 | Exact finite checks", "SubtitleX"),
        chart,
        Spacer(1, 12),
        table([
            ["Check", "Recorded result"],
            ["Complete signed candidate/reference sets", "2,818 / 2,818 matches"],
            ["Withheld positive pairs", "384 / 384 recovered"],
            ["Matched empty sets", "1,513 cases"],
            ["Returned ordered pair occurrences", "3,408; at most 4 per case"],
            ["Special branches", "128 linear; 128 identity-selector cases"],
            ["Repeated discriminants", "2 first-stage; 128 coordinate-stage cases"],
            ["Malformed inputs", "8 rejected"],
            ["Seed and replay controls", "24 distinct streams; byte-identical second run"],
        ], [265, 239]),
        p("Counts are copied from the frozen native summary [1]. Pair occurrences "
          "include positive controls and repeated results across queries; they are "
          "not independent factor-base relations. No statistical uncertainty or "
          "elapsed-time comparison was recorded. " + STAMP, "SmallX"),
        heading("Two distinct ordinary M31 fixtures"),
        table([
            ["Parameter", "Primary p31-subexp", "Secondary native algebraic"],
            ["p", "2147483647", "2147483647"],
            ["a; b", "2147483644; 21", "847514846; 2087444724"],
            ["Group/subgroup order r", "2147487637", "2147446831"],
            ["Generator (x,y)", "(1391443431,177979044)", "(625037715,2100627251)"],
            ["Cofactor; main targets", "1; 64", "1; 64"],
        ], [125, 184, 195]),
        p("Source identities are pinned at commit eb55e93110676c91213d91d6c8d00489311a4bd8. "
          "The native harness checked field/order primality, nonsingularity, generator "
          "equations and unreduced [r]G=O; prime r and the Hasse bound certify the "
          "group order for these fixtures. The cases are not pooled as one curve. "
          "[1,3] " + STAMP, "SmallX"),
        PageBreak(),
        p("Sampling bound and next step", "TitleX"),
        p("Derived identity, finite consistency checks, and explicit scope", "SubtitleX"),
        p("Let B be a fixed nonempty negation-closed signed base in G\\{O}, where G has odd "
          "order r. Write b=|B|=2m and let U be its m distinct Cayley x-labels. "
          "For product label c, let m<sub>c</sub>=|U ∩ cU<super>-1</super>| and let "
          "A<sub>R</sub>(c) count unordered signed pairs in B+B summing to R, with "
          "repeated summands allowed. For the expectations below, R is uniform "
          "over G\\{O}, and P and c are sampled independently of R. [2] " + STAMP),
        formula([
            "Σ[R≠O] A_R(c) = 2m_c ≤ b",
            "E[A_R(c)] ≤ b/(r-1)   if c is sampled independently of R",
            "Pr[R-P ∈ B] = (b-1)/(r-1)   for P uniform in B",
        ]),
        Spacer(1, 8),
        p("<b>Counting argument.</b> Each ordered x-label pair has four signed lifts. "
          "If d<sub>c</sub> counts u²=c, remove 2d<sub>c</sub> opposite pairs with zero "
          "sum. There are 2d<sub>c</sub> remaining diagonal pairs. Unordering gives "
          "((4m<sub>c</sub>-2d<sub>c</sub>)+2d<sub>c</sub>)/2=2m<sub>c</sub>. "
          "Uniformly averaging nonzero targets gives the bound. [2] " + STAMP),
        p("The bounded native check used B={±[j]G:1≤j≤32}. For each fixture, "
          "all 528 product labels satisfied the identity; the recorded totals were "
          "4,032 ordered and 2,048 unordered nonzero-sum pairs. The whole curve "
          "was not enumerated. [1] " + STAMP),
        p("<b>Implication.</b> Static Gaussian-smooth label selection cannot by itself "
          "give an asymptotic gain in this average candidate-yield metric over direct "
          "P-in-B / R-P membership sampling. This is not a runtime lower bound; it "
          "does not cover target-dependent selection, other relation forms, more "
          "summands, nonuniform target populations or independent-rank effects. "
          "The next candidate must identify target-dependent enrichment or a "
          "specific arithmetic/membership saving. [2] " + STAMP),
        heading("Provenance and remaining qualification"),
        p("Rust 1.100.0 on Linux x86_64; standalone build without Cargo dependencies. "
          "The first parallel-linker attempt failed with a resource error; final "
          "builds used one code-generation unit and one linker thread. Frozen "
          "protocol, source hashes, failure evidence and replay receipts accompany "
          "the component. [1,2] " + STAMP, "SmallX"),
        p("No new timing benchmark, complete DLP recovery, formal Lean certificate "
          "or signed PCC verdict is claimed. An identity.certificate/v1 record is "
          "not supplied by this report; the algebraic statements remain written "
          "derivations with finite checks. No new scientific execution was performed "
          "to render these pages. " + STAMP, "SmallX"),
        heading("Sources and evidence"),
        p('[1] <link href="https://github.com/isogeny-labs/crypto/pull/5232" color="#087F8C">'
          'crypto PR #5232 publication package</link>: evidence/result-run1.json, '
          'evidence/receipt.json and the frozen case records. Summary SHA-256: '
          '<font name="ReportSans" size="6.8">' + EXPECTED_SUMMARY_SHA + '</font>.', "SmallX"),
        p('[2] Study README, PROTOCOL.md, SOURCE.json, REVIEW.md, extractor.rs and '
          'reference.rs, in research/cayley_chord_m31_20261125/.', "SmallX"),
        p('[3] <link href="https://github.com/isogeny-labs/crypto/blob/' + SOURCE_COMMIT +
          '/src/cryptanalysis/ic_boundary.rs" color="#087F8C">Pinned primary fixture</link>; '
          '<link href="https://github.com/isogeny-labs/crypto/blob/' + SOURCE_COMMIT +
          '/research/prime31_ic_20261115/native/lib.rs" color="#087F8C">'
          'pinned native arithmetic and secondary fixture</link>.', "SmallX"),
        p('[4] I. Semaev, <link href="https://eprint.iacr.org/2004/031" '
          'color="#087F8C">Summation polynomials and the discrete logarithm problem '
          'on elliptic curves</link> (2004), summation-polynomial framework.', "SmallX"),
    ]

    def footer(canvas, document):
        canvas.saveState()
        canvas.setStrokeColor(LINE)
        canvas.line(54, 42, 558, 42)
        canvas.setFont("ReportSans", 7.4)
        canvas.setFillColor(MUTED)
        canvas.drawString(54, 28, "Isogeny Labs | M31 signed chord component | " + STAMP)
        canvas.drawRightString(558, 28, str(document.page))
        canvas.restoreState()

    document = BaseDocTemplate(
        str(ROOT / "REPORT.pdf"), pagesize=(612, 792),
        leftMargin=54, rightMargin=54, topMargin=44, bottomMargin=55,
        title="M31 Cayley-constrained signed chord extraction",
        author="Isogeny Labs", subject="Source-linked component qualification; PCC-unverified",
    )
    document.addPageTemplates(PageTemplate(
        id="report", frames=[Frame(54, 55, WIDTH, 693, leftPadding=0,
                                    rightPadding=0, topPadding=0, bottomPadding=0)],
        onPage=footer,
    ))
    document.build(story)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--summary", type=Path,
                        default=ROOT.parent / "evidence" / "result-run1.json")
    args = parser.parse_args()
    raw = args.summary.read_bytes()
    if hashlib.sha256(raw).hexdigest() != EXPECTED_SUMMARY_SHA:
        raise SystemExit("Frozen summary hash mismatch; do not relabel or replace evidence")
    build(json.loads(raw))


if __name__ == "__main__":
    main()
