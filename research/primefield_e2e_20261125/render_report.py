#!/usr/bin/env python3
"""Lay out native analysis outputs; this file performs no research computation.

The Rust CLI owns accounting, ratios, exponents, optimization and fits. This
renderer only formats its recorded values and transforms chart coordinates.
README.md, the SVGs and report.pdf share the content below. Requires ReportLab
and PyMuPDF; the final PDF uses lossless stream and object compression.
"""
from __future__ import annotations

import argparse
import html
import json
import re
import tempfile
from pathlib import Path

import fitz
from reportlab.graphics import renderPDF, renderSVG
from reportlab.graphics.shapes import Drawing, Line, PolyLine, Rect, String as _String
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    BaseDocTemplate, Flowable, Frame, PageBreak,
    PageTemplate, Paragraph, Spacer, Table, TableStyle,
)

ROOT = Path(__file__).resolve().parent
INK = colors.HexColor("#173046")
TEAL = colors.HexColor("#087f8c")
GOLD = colors.HexColor("#b06b25")
MUTED = colors.HexColor("#556773")
PALE = colors.HexColor("#edf4f6")
RULE = colors.HexColor("#cbd7dd")
WHITE = colors.white
WIDTH, HEIGHT = 595.28, 841.89
TEXT_WIDTH = WIDTH - 106

# Embed metrically compatible fonts: relying on a viewer's base14 substitutes
# can produce different glyph advances and unreadable letter spacing.
FONT_DIR = Path("/usr/share/fonts/truetype/liberation")
pdfmetrics.registerFont(TTFont("Helvetica", str(FONT_DIR/"LiberationSans-Regular.ttf")))
pdfmetrics.registerFont(TTFont("Helvetica-Bold", str(FONT_DIR/"LiberationSans-Bold.ttf")))
pdfmetrics.registerFontFamily("Helvetica", normal="Helvetica", bold="Helvetica-Bold",
                              italic="Helvetica", boldItalic="Helvetica-Bold")


def String(*args, **kwargs):
    """Use the same embedded font for every chart label and report paragraph."""
    kwargs.setdefault("fontName", "Helvetica")
    return _String(*args, **kwargs)

STYLES = {
    "body": ParagraphStyle("body", fontName="Helvetica", fontSize=9.6,
        leading=13.2, textColor=INK, spaceAfter=8),
    "small": ParagraphStyle("small", fontName="Helvetica", fontSize=8,
        leading=10.7, textColor=MUTED, spaceAfter=7),
    "title": ParagraphStyle("title", fontName="Helvetica-Bold", fontSize=26,
        leading=30, textColor=INK, spaceAfter=10),
    "subtitle": ParagraphStyle("subtitle", fontName="Helvetica", fontSize=14,
        leading=19, textColor=TEAL, spaceAfter=16),
    "h1": ParagraphStyle("h1", fontName="Helvetica-Bold", fontSize=18,
        leading=23, textColor=INK, spaceAfter=13),
    "h2": ParagraphStyle("h2", fontName="Helvetica-Bold", fontSize=11,
        leading=14, textColor=TEAL, spaceBefore=7, spaceAfter=6),
    "equation": ParagraphStyle("equation", fontName="Helvetica", fontSize=10,
        leading=15, textColor=INK, leftIndent=13, rightIndent=9,
        borderColor=RULE, borderWidth=.5, borderPadding=9,
        backColor=PALE, spaceBefore=3, spaceAfter=13),
    "cell": ParagraphStyle("cell", fontName="Helvetica", fontSize=8.2,
        leading=11, textColor=INK),
    "cell_head": ParagraphStyle("cell_head", fontName="Helvetica-Bold", fontSize=8.1,
        leading=10.7, textColor=WHITE),
    "mono": ParagraphStyle("mono", fontName="Helvetica", fontSize=8.1,
        leading=10.8, textColor=INK, backColor=PALE,
        borderPadding=8, spaceBefore=2, spaceAfter=12),
}


class Vector(Flowable):
    def __init__(self, drawing):
        super().__init__()
        self.drawing = drawing
        self.width, self.height = drawing.width, drawing.height

    def draw(self):
        renderPDF.draw(self.drawing, self.canv, 0, 0)


def p(text, style="body"):
    return Paragraph(text, STYLES[style])


def table(headers, rows, widths):
    cells = [[p(html.escape(str(x)), "cell_head") for x in headers]]
    cells.extend([[p(str(x), "cell") for x in row] for row in rows])
    result = Table(cells, colWidths=widths, repeatRows=1, hAlign="LEFT")
    result.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, 0), INK),
        ("BACKGROUND", (0, 1), (-1, -1), colors.HexColor("#f7f9fa")),
        ("ROWBACKGROUNDS", (0, 1), (-1, -1), [WHITE, colors.HexColor("#f3f7f8")]),
        ("VALIGN", (0, 0), (-1, -1), "TOP"),
        ("LEFTPADDING", (0, 0), (-1, -1), 8),
        ("RIGHTPADDING", (0, 0), (-1, -1), 8),
        ("TOPPADDING", (0, 0), (-1, -1), 7),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 7),
        ("LINEBELOW", (0, 0), (-1, 0), .7, INK),
        ("LINEBELOW", (0, -1), (-1, -1), .4, RULE),
    ]))
    return result


def flow_diagram():
    d = Drawing(TEXT_WIDTH, 170)
    boxes = [
        (0, 105, 145, 52, "Work per attempt", "setup + solver + extraction"),
        (172, 105, 145, 52, "Useful yield", "solvability × retention"),
        (344, 105, 145, 52, "Rank goal", "independent increments"),
        (97, 24, 295, 54, "Relation collection", "rank goal × work / useful yield"),
    ]
    for x, y, w, h, title, text in boxes:
        d.add(Rect(x, y, w, h, rx=5, ry=5, fillColor=PALE,
                   strokeColor=RULE, strokeWidth=.6))
        d.add(String(x+w/2, y+h-19, title, textAnchor="middle",
                     fontName="Helvetica-Bold", fontSize=10, fillColor=INK))
        d.add(String(x+w/2, y+14, text, textAnchor="middle",
                     fontName="Helvetica", fontSize=8.4, fillColor=MUTED))
    for x in [72, 244, 416]:
        d.add(Line(x, 104, x, 92, strokeColor=TEAL, strokeWidth=1))
        d.add(Line(x, 92, 244, 92, strokeColor=TEAL, strokeWidth=1))
    d.add(Line(244, 92, 244, 79, strokeColor=TEAL, strokeWidth=1))
    d.add(Line(240, 84, 244, 79, strokeColor=TEAL, strokeWidth=1))
    d.add(Line(248, 84, 244, 79, strokeColor=TEAL, strokeWidth=1))
    return d


def exponent_chart(rows):
    """Plot already-computed native optima. Only coordinates are transformed."""
    d = Drawing(TEXT_WIDTH, 224)
    left, bottom, plot_w, plot_h = 46, 43, 425, 144
    for y, label in [(0, "0"), (.25, "0.25"), (.5, "0.50"), (.75, "0.75"), (1, "1.00")]:
        yy = bottom + y * plot_h
        d.add(Line(left, yy, left+plot_w, yy, strokeColor=RULE, strokeWidth=.4))
        d.add(String(left-8, yy-3, label, textAnchor="end", fontSize=8, fillColor=MUTED))
    d.add(Line(left, bottom+.5*plot_h, left+plot_w, bottom+.5*plot_h,
               strokeColor=GOLD, strokeWidth=1, strokeDashArray=[4, 3]))
    d.add(String(left+plot_w, bottom+.5*plot_h+5, "generic sqrt(n) exponent",
                 textAnchor="end", fontSize=8, fillColor=GOLD))
    palette = [TEAL, INK, colors.HexColor("#a15f91"), colors.HexColor("#518b52")]
    names = []
    for row in rows:
        if row["family"] not in names:
            names.append(row["family"])
    for i, name in enumerate(names):
        points = []
        for row in rows:
            if row["family"] == name:
                x = left + (row["arity"]-2) * plot_w / 6
                y = bottom + row["exponent"] * plot_h
                points.extend([x, y])
                d.add(Rect(x-2.5, y-2.5, 5, 5, fillColor=palette[i], strokeColor=None))
        d.add(PolyLine(points, strokeColor=palette[i], strokeWidth=1.5))
        lx = left + (i % 2) * 222
        ly = 15 - (i // 2) * 13
        d.add(Line(lx, ly+3, lx+13, ly+3, strokeColor=palette[i], strokeWidth=2))
        d.add(String(lx+18, ly, name, fontSize=8.1, fillColor=INK))
    for m in range(2, 9):
        x = left + (m-2)*plot_w/6
        d.add(String(x, bottom-13, str(m), textAnchor="middle", fontSize=8, fillColor=MUTED))
    d.add(String(left, 205, "Optimized conditional work exponent e", fontSize=10,
                 fontName="Helvetica-Bold", fillColor=INK))
    d.add(String(left+plot_w, bottom-26, "fixed arity m", textAnchor="end", fontSize=8, fillColor=MUTED))
    return d


def ratio_chart(rows):
    """Display native arithmetic ratios; keep the two cost units separate."""
    d = Drawing(TEXT_WIDTH, 184)
    left, plot_w, row_h, top = 145, 324, 37, 139
    for tick, label in [(0, "0"), (.1, "0.10"), (.2, "0.20"), (.3, "0.30")]:
        x = left + tick * plot_w / .35
        d.add(Line(x, 31, x, top+20, strokeColor=RULE, strokeWidth=.5))
        d.add(String(x, 18, label, textAnchor="middle", fontSize=8, fillColor=MUTED))
    for i, row in enumerate(rows):
        y = top-i*row_h
        d.add(String(left-10, y+2, row["label"], textAnchor="end", fontSize=8.4, fillColor=INK))
        for value, offset, color in [(row["raw_ops_ratio"], 2, TEAL), (row["recorded_gae_ratio"], -10, INK)]:
            width = value * plot_w / .35
            d.add(Rect(left, y+offset, width, 8, fillColor=color, strokeColor=None))
            d.add(String(left+width+4, y+offset, f"{value:.4f}", fontSize=7.8, fillColor=color))
    d.add(String(0, 171, "Candidate / interval baseline: archived ratios", fontSize=10,
                 fontName="Helvetica-Bold", fillColor=INK))
    d.add(Rect(145, 1, 8, 7, fillColor=TEAL, strokeColor=None))
    d.add(String(158, 0, "solver operations", fontSize=8, fillColor=INK))
    d.add(Rect(297, 1, 8, 7, fillColor=INK, strokeColor=None))
    d.add(String(310, 0, "recorded GAE subtotal", fontSize=8, fillColor=INK))
    return d


def markdown_text(text):
    text = re.sub(r'<a href="([^"]+)">([^<]+)</a>', r'[\2](\1)', text)
    text = text.replace("<b>", "**").replace("</b>", "**")
    text = text.replace("<i>", "*").replace("</i>", "*")
    text = text.replace("<br/>", "\n")
    # Resolve innermost markup first so nested exponents never leak HTML.
    while re.search(r"</?(?:super|sub)>", text):
        updated = re.sub(r"<super>([^<>]*)</super>", r"^(\1)", text)
        updated = re.sub(r"<sub>([^<>]*)</sub>", r"_(\1)", updated)
        if updated == text:
            raise ValueError("Unbalanced subscript/superscript report markup")
        text = updated
    return html.unescape(text)


def build_pages(model, results, pipeline):
    # Dynamic values below must be supplied by the native CLI; no calculations.
    choices = [
        ("unit trial, sparse LA", 3),
        ("unit trial, sparse LA", 4),
        ("linear B trial, sparse LA", 2),
        ("linear B trial, sparse LA", 3),
        ("linear B trial, sparse LA", 5),
        ("quadratic B trial, sparse LA", 7),
        ("linear B trial, dense LA", 6),
    ]
    scenario_rows = []
    for name, arity in choices:
        row = next(r for r in model["scenarios"] if r["name"] == name and r["arity"] == arity)
        scenario_rows.append({"name": name, "arity": arity,
            "beta_label": f'{row["beta_opt"]:z.6f}', "exponent_label": f'{row["exponent"]:z.6f}'})
    for row in model["stress_cases"]:
        scenario_rows.append({"name": row["name"], "arity": row["model"]["arity"],
            "beta_label": f'{row["optimum"]["beta"]:z.6f}',
            "exponent_label": f'{row["optimum"]["work_exponent"]:z.6f}'})
    plot_rows = [{"family": r["name"], "arity": r["arity"], "exponent": r["exponent"]}
                 for r in model["scenarios"] if r["arity"] <= 8]
    ratio_rows = []
    for source, label in [("degree32.jsonl", "original screen"),
                          ("degree32_replication_extend.jsonl", "replication extension"),
                          ("e2e_full_rank_13bit_20261185.jsonl", "full-rank 13-bit / 20261185")]:
        row = next(r for r in results["comparisons"] if r["source"] == source)
        ratio_rows.append({"label": label, "raw_ops_ratio": row["raw_ops_ratio"],
                           "recorded_gae_ratio": row["recorded_gae_ratio"]})
    totals = results["totals"]
    replication_preflight = next(r for r in results["sources"]
                                 if r["source"] == "degree32_replication.jsonl")
    evidence_rows = [
        ["Timed archive rows", f'{totals["timed_rows"]} rows; {totals["exhausted"]} exhausted', "Failures retained; preflights recorded separately"],
        ["Producer recovery flags", f'{totals["producer_verified"]} true', "Producer assertions; no independent recovery replay here"],
        ["Full-rank flags", f'{totals["full_rank_reported"]} reported', "Separate from scalar recovery; historical stopping rules differ"],
        ["Complete-cost qualification", f'{totals["complete_cost_qualified"]} qualified', "Complete cold totals remain unknown"],
    ]
    pinned = "aab505dd2d24cd5c98f9be1a2bd2243793a2001b"
    source_base = f"https://github.com/isogeny-labs/crypto/tree/{pinned}/research/pkm_adaptive_sweep_20261123"
    pages = []
    pages.append([
        ("title", "Prime-field index calculus"),
        ("subtitle", "End-to-end runtime accounting"),
        ("body", "<b>The integration connects useful relation yield to complete phase costs.</b> "
         "The native model turns explicit assumptions about solver work, relation coverage, rank retention, "
         "preprocessing and linear algebra into a conditional work exponent. A separate native adapter "
         "recomputes archived benchmark summaries while retaining incomplete and unpriced outcomes."),
        ("body", "This report answers the request to wire the existing work together and expose its "
         "end-to-end asymptotic complexity. It documents an executable accounting integration and a "
         "conditional mathematical model. The archived records do not yet supply a complete cold "
         "single-target runtime series from which an ECDLP exponent can be established. [pcc:unverified]"),
        ("figure", "pipeline.svg", flow_diagram(),
         "Figure 1. The collection budget combines work per attempt, useful yield and required rank. "
         "The cold total also includes factor-base construction, other setup, rank maintenance, final "
         "linear algebra, recovery and verification. These are accounting dependencies, not measured timings."),
        ("table", ["Requested connection", "Evidence delivered"], [
            ["Solver → useful relations", "Explicit probability and retention inputs; solver operations remain a separate unit from calibrated GAE."],
            ["Relations → independent rank", "Recorded ranks and full-rank flags; conditional rank-retention loss in the model."],
            ["Rank → total work", "Cold, reusable and target-dependent phase ledger; unknown required costs cannot become zero."],
            ["Total work → asymptotics", "Piecewise-affine exponent optimizer and finite arity scenarios; archived fits remain descriptive."],
        ], [154, TEXT_WIDTH-154]),
        ("small", "Status: branch-local producer analysis, not signed independent performance qualification. "
         "No new cryptographic benchmark or solver run is represented by this report. Historical inputs are "
         "immutable and are referenced by source commit, path and content hash."),
    ])
    pages.append([
        ("h1", "1. Count useful relations before pricing them"),
        ("body", "Let n be the subgroup order, B the actual number of admissible distinct factor-base "
         "points and m a fixed relation arity. The number of matrix columns is a separate quantity: sign "
         "folding can give two signed points per column, and a target column may be added. A map's "
         "polynomial degree, such as 2<super>t</super>, is not automatically B."),
        ("h2", "Finite coverage and collisions"),
        ("body", "For a uniform residual and a fixed admissible tuple convention, exact solvability is "
         "the size of the corresponding sumset divided by n. Tuple counting bounds that coverage; "
         "it does not establish uniformity, independence or a collision-free sum map."),
        ("equation", "p<sub>solvable</sub> = |S<sub>m</sub>(F)| / n ≤ min(1, N<sub>tuples</sub>/n)<br/>"
         "N<sub>tuples</sub> = C(B,m) for distinct unordered points;<br/>"
         "N<sub>tuples</sub> = C(B+m−1,m) when repetition is admissible."),
        ("body", "The finite native model keeps the factorial and binomial terms in logarithmic form. "
         "When exact or measured coverage is absent, its Poisson occupancy approximation "
         "1−exp(−N<sub>tuples</sub>/n) is labeled heuristic. Structured factor bases, signs, repeated "
         "elements and nonuniform residual walks can change the true coverage."),
        ("h2", "Solvability is only the first filter"),
        ("equation", "p<sub>increment</sub> = p<sub>solvable</sub> × "
         "P(valid relation found | solvable) × P(rank increment | valid relation)<br/>"
         "T<sub>collection</sub> ≈ R<sub>needed</sub> C<sub>trial</sub> / p<sub>increment</sub>."),
        ("body", "The last expression assumes a stationary per-attempt model and at most one independent "
         "rank increment per attempt. Rank-dependent collection uses a sum of conditional budgets over "
         "rank states. Multi-output, adaptive or graph collectors need a model that matches their actual "
         "output process. Repeated relations do not automatically increase rank."),
        ("body", "C<sub>trial</sub> is the expected work of all attempts, including unsuccessful searches, "
         "refutations, capped calls, point lifting, validation and bookkeeping. Successful-call timings "
         "alone underprice collection. When available, all attempted work divided by the actual useful "
         "rank gained is the direct finite diagnostic."),
        ("small", "Source basis: PKM 2016, §3.2 [1]; finite model implementation [7]. The familiar "
         "B<super>m</super>/(m!n) intensity is a sparse, fixed-arity approximation. It is not an "
         "unconditional theorem about every chosen factor base."),
    ])
    pages.append([
        ("h1", "2. Sum the cold costs in one work unit"),
        ("equation", "T<sub>cold</sub> = T<sub>base</sub> + T<sub>preprocess</sub> + "
         "T<sub>collection</sub> + T<sub>rank</sub> + T<sub>linear algebra</sub> + "
         "T<sub>recovery</sub> + T<sub>verification</sub>."),
        ("body", "Cold means one previously unseen target with all required setup charged to that target. "
         "A complete phase ledger states its work unit and required phases. An unknown phase leaves the "
         "complete total unknown; it is not assigned zero. Nested solver counters are not added again "
         "when the encompassing decomposition phase already includes them."),
        ("table", ["Quantity", "Meaning and handling"], [
            ["Native operations", "Keep operation kinds separate: field multiplications, group additions, row operations and instructions are not interchangeable."],
            ["Recorded GAE", "A calibrated group-addition-equivalent subtotal. A zero-priced positive-work phase or an uncharged counter prevents complete-cost interpretation."],
            ["Wall time", "Retain the exact timing interval, process/row distinction, setup boundary, host and isolation metadata. Do not add overlapping intervals."],
            ["Memory", "Separate live storage from peak solver allocation. Missing coverage is unknown; a reported zero is not evidence of zero memory use."],
            ["Failure/censoring", "Retain unsuccessful, exhausted, capped and malformed outcomes. A completed-only fit cannot establish complete expected runtime."],
        ], [116, TEXT_WIDTH-116]),
        ("h2", "Reusable work and online work"),
        ("body", "Only phases explicitly reusable across targets may be amortized. A relation phase tied "
         "to the target remains online. If reusable setup scales as n<super>s</super>, online work as "
         "n<super>o</super> and there are K=n<super>κ</super> targets, then:"),
        ("equation", "e<sub>cold</sub> = max(s,o)<br/>"
         "e<sub>K targets</sub> = max(s,κ+o)<br/>"
         "e<sub>average per target</sub> = max(s−κ,o)."),
        ("body", "These are different accounting questions. A warm online gain or a lower average over "
         "many targets does not replace the cold single-target result. The native finite aggregator "
         "uses stable log-sums rather than keeping only the dominant phase; the exponent model uses "
         "the maximum because it describes asymptotic powers."),
        ("small", "The historical field named total_gae is retained with its producer meaning. In this "
         "integration it is shown as recorded GAE whenever unpriced work or an incomplete qualification "
         "record prevents treating it as complete cold cost."),
    ])
    pages.append([
        ("h1", "3. Optimize the conditional exponent"),
        ("body", "Set B=n<super>β</super>, C<sub>trial</sub>=n<super>γ</super>B<super>δ</super> "
         "up to polylogarithmic factors, and let useful retention incur the loss "
         "n<super>−η_n</super>B<super>−η_B</super>. With Θ(B) required increments, write "
         "a=γ+η<sub>n</sub> and d=1+δ+η<sub>B</sub>. Fixed arity gives:"),
        ("equation", "e<sub>collection</sub>(β) = a+dβ+max(0,1−mβ)<br/>"
         "= max(a+dβ, a+1+(d−m)β).<br/>"
         "e(β) = max(e<sub>collection</sub>(β), ell·β, all other phase exponents)."),
        ("body", "ell=2 describes an assumed sparse solve with O(mB) nonzeros and O(B) iterations at "
         "fixed m. Dense elimination has a different exponent, and dense incremental rank auditing "
         "can dominate even when the final solver is sparse. The explicit base-materialization cost, "
         "preprocessing, recovery and affine memory constraints also belong in the envelope."),
        ("figure", "conditional-exponents.svg", exponent_chart(plot_rows),
         "Figure 2. Native optimizer outputs for declared fixed-arity scenarios. Lines connect "
         "separate conditional models; they are not measured scaling curves. The dashed line is "
         "the generic square-root exponent. No numerical uncertainty interval applies to these "
         "deterministic model evaluations. [pcc:unverified]"),
        ("body", "Each phase is affine in β after the collection term is expanded into two lines. "
         "The maximum is convex and piecewise affine. On the feasible closed interval, an optimum "
         "occurs at an endpoint or at a pairwise line intersection (flat minima include such points). "
         "The implementation enumerates these candidates, avoiding a grid-resolution assumption."),
        ("small", "This is a model optimization argument. It supplies no missing solver-cost bound, "
         "coverage law, rank-retention law or independent performance evidence. Its assumptions "
         "must be connected to a concrete algorithm before its exponent becomes an algorithmic bound."),
    ])
    pages.append([
        ("h1", "4. What the model says, conditionally"),
        ("table", ["Scenario", "m", "β at optimum", "Exponent e"], [
            [r["name"], str(r["arity"]), r["beta_label"], r["exponent_label"]]
            for r in scenario_rows
        ], [237, 30, 108, TEXT_WIDTH-375]),
        ("body", "The table contains native evaluations of explicit assumptions. It does not assert "
         "that an implemented decomposition oracle attains the listed trial costs. [pcc:unverified]"),
        ("h2", "The current two-summand architecture"),
        ("body", "For m=2 with work at least linear in B per attempt, no favorable rank loss and the "
         "standard full-base relation goal, collection costs at least order n in this model. In the "
         "sparse-coverage regime, Θ(B) increments require Θ(n/B) attempts, and multiplying by "
         "Ω(B) work per attempt gives Ω(n). Saturation does not lower the resulting exponent. "
         "This conclusion is scoped to that architecture and cost assumption."),
        ("h2", "A useful threshold test"),
        ("body", "With only the collection envelope and ell·β active, m&gt;d and nonnegative coefficients, "
         "the conditional optimum can be written as:"),
        ("equation", "β* = min(1/m, (a+1)/(m−d+ell))<br/>"
         "e* = max(a+d/m, ell(a+1)/(m−d+ell))."),
        ("body", "A strict exponent below 1/2 requires both terms to be below 1/2. At a=0 this "
         "requires m&gt;max(2d,d+ell). A fixed polynomial trial-cost model thus states what would have "
         "to be demonstrated about a higher-arity oracle; increasing m alone does not demonstrate it."),
        ("h2", "Growing arity is a separate asymptotic analysis"),
        ("body", "If m grows with n, retain log<sub>n</sub>(m!), all m-dependent solver costs, matrix "
         "density, memory and uniformity bounds. Substituting m→∞ into a fixed-m formula silently "
         "drops terms and cannot establish subexponential runtime. The finite model exposes these "
         "inputs rather than asserting that they vanish."),
    ])
    pages.append([
        ("h1", "5. Reconcile the archived measurements"),
        ("body", f"The archive is pinned to <a href=\"{source_base}\">source commit {pinned[:12]}</a> "
         "in PR #5067. The native adapter recomputes the selected records without changing those "
         "inputs. Original selected-screen results, later full-rank panels and alternative engines "
         "remain separate observation groups."),
        ("figure", "archived-ratios.svg", ratio_chart(ratio_rows),
         "Figure 3. Native arithmetic ratios from the archived records identified in the source "
         "manifest. Solver operations and recorded GAE have distinct denominators and calibration. "
         "These historical arithmetic ratios are not new benchmark measurements or complete-cost "
         "speedup claims. Sources: degree32.jsonl, degree32_replication_extend.jsonl and "
         "e2e_full_rank_13bit_20261185.jsonl. [pcc:unverified]"),
        ("table", ["Recorded series", "Rows / outcome", "Accounting status"], evidence_rows,
         [147, 144, TEXT_WIDTH-291]),
        ("body", "The uploaded summary combined a raw-operation comparison with a calibrated-cost "
         "ratio. Both may be correctly computed and still differ substantially: the operations have "
         "different measured costs, and the phase totals cover different work. Report the numerator, "
         "denominator, units, panel and inclusion rule with every ratio."),
        ("body", "Later full-rank rows improve the documented linear-algebra stopping condition. "
         "A recorded recovery value, a producer verified flag and a recorded full-rank flag are "
         "distinct evidence fields; none substitutes for signed independent replay. A source file "
         "hash binds the input bytes, not the executable that originally produced them. [pcc:unverified]"),
        ("body", f"The incomplete replication file contains {replication_preflight['preflight_rows']} "
         f"preflight rows, {replication_preflight['preflight_exhausted']} marked exhausted, and "
         f"{replication_preflight['failed_envelopes']} failed envelopes. It has no timed envelope. "
         "Those outcomes remain in the native source summary and are excluded from the timed ratios "
         "by their recorded stage, not because they failed. [pcc:unverified]"),
        ("small", "No confidence interval is invented for an arithmetic aggregate. Any descriptive "
         "fit supplied by the native analyzer retains its actual group and input range. The "
         "tiny, fixed-parameter panels do not meet the repository's prospective exponent-evidence criteria."),
    ])
    pages.append([
        ("h1", "6. Corrections to the uploaded summary"),
        ("table", ["Original implication", "Required correction"], [
            ["Graph cycles give a universal obstruction", "The cost inequality is conditional on the residual distribution, edge-extraction cost and useful-cycle stopping law. A graph cycle need not be a row dependency or have a nonzero target projection."],
            ["Unique factorization makes decomposition efficient", "Uniqueness does not imply an efficient factorization algorithm. State the actual smoothness/factoring procedure and its cost."],
            ["O(mb) encoding size destroys subexponentiality", "The cited 2026 Theorem 4 explicitly uses O(mb) edges. Its cost bound retains both b/m and mb/sqrt(log b); the admissible growing-arity choice keeps their sum sublinear in b."],
            ["The encoding forces Jacobian coordinates", "The cited construction uses affine coordinates, exceptional-case selectors and auxiliary inverse/slope witnesses. A separate width objection needs the actual encoding assumptions."],
            ["SWU totality implies uniform or injective indexing", "RFC 9380 distinguishes encoding properties. A total curve map does not preserve a chosen index population injectively or uniformly. Count distinct points and collisions."],
            ["Ordering all slots always supplies an m! quotient", "Permutation symmetry requires compatible slot domains and constraints. Distinct slot factor bases and repeated entries change the symmetry/stabilizer count."],
            ["A chain implies treewidth1 for the whole solve", "The graph definition matters. A chain component does not establish a whole-system elimination bound; cross-slot constraints and field arithmetic must be included."],
        ], [149, TEXT_WIDTH-149]),
        ("equation", "2026 conditional bound [2]: 2<super>O(b/m + mb/√(log b))</super><br/>"
         "m = ceil((log b)<super>1/4</super>) gives "
         "2<super>O(b/(log b)<super>1/4</super>)</super>."),
        ("body", "The pinned graph source contains finite signed-incidence examples and "
         "recovery conditional on a nonzero target coefficient [5]. It does not establish an "
         "unconditional natural-stream stopping distribution. The 2026 reduction is conditional "
         "on its tensor-contraction premise; its asymptotic expression is not a concrete runtime "
         "estimate for deployed curves."),
        ("body", "For example, a pair-sum directory model charges L²+nB/L², while an idealized "
         "oracle model with Θ(L) required accepted edges charges L+n/L. Their square-root bounds "
         "depend on those specified setup, occupancy and stopping assumptions; neither expression "
         "is a universal statement about every graph-based method."),
        ("small", "These corrections preserve the published experiments and narrow interpretations "
         "to their supported hypotheses. They do not reopen or relaunch unrelated stopped research tasks."),
    ])
    pages.append([
        ("h1", "7. Reproduce and extend the accounting"),
        ("body", "The native CLI reads the source manifest, checks the pinned input bytes, preserves "
         "outcome counts with links to the immutable row-level records, and emits the accounting and "
         "model outputs consumed by this renderer. "
         "Use the command documented with the CLI to regenerate those outputs, then render:"),
        ("code", "python3 research/primefield_e2e_20261125/render_report.py"),
        ("body", "The renderer only formats native values and plots coordinates; it does not compute "
         "ratios, regressions, relation budgets or exponents. Unit tests exercise the model's analytic "
         "cases, missing-phase rejection, numerical edge cases and malformed archived inputs. "
         "Consult the PR's current test receipt for commands and actual pass/fail scope; a small "
         "standalone build does not establish qualification of the whole repository."),
        ("h2", "What is needed for a measured exponent"),
        ("body", "A prospective series needs complete cold costs, comparable parameter policies "
         "B(n) and m(n), measured relation/rank yield, setup and memory coverage, retained failures, "
         "source-bound execution and independent replay. The repository's T2 criterion requires "
         "at least six sizes spanning at least 24 bits with a held-out size. A T3 subexponential "
         "claim needs a mathematical argument with explicit assumptions; measurements alone cannot "
         "supply it [6]."),
        ("body", "For a raw fit log<sub>2</sub>T against log<sub>2</sub>n, a square-root comparison "
         "uses slope 1/2. For log<sub>2</sub>(T/T<sub>rho</sub>) the corresponding threshold is zero "
         "when the reference scales as n<super>1/2</super>. The uploaded summary's normalized "
         "threshold of 0.50 therefore mixed the two axes."),
        ("h2", "Sources and evidence custody"),
        ("small", "[1] Petit, Kosters and Messeng (2016), <a href=\"https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf\">Algebraic approaches for the elliptic curve discrete logarithm problem over prime fields</a>, especially §3.2. It separates preprocessing, relation collection and linear algebra."),
        ("small", "[2] Akshima et al. (2026), <a href=\"https://github.com/bochum-crypto/subexponential-ecdl/blob/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f/subexponential-ecdl.pdf\">A Subexponential Time Algorithm for Elliptic Curve Discrete Logarithm</a>, v0.2, 29 October 2026, Theorem 4 and Lemmas 11–13. Conditional source; no independent proof claim is made here."),
        ("small", "[3] <a href=\"https://www.rfc-editor.org/rfc/rfc9380.html\">RFC 9380: Hashing to Elliptic Curves</a>, encoding/map properties and simplified SWU definitions."),
        ("small", f"[4] <a href=\"{source_base}\">crypto archived PKM study at {pinned[:12]}</a>; the manifest records exact paths/hashes. Original JSONL files and their producer reports remain canonical historical evidence."),
        ("small", "[5] <a href=\"https://github.com/isogeny-labs/crypto/blob/26233d8bd28031bf64e959c7234815bf1a38958b/research/ic_cycle_closure_20261125/README.md\">Cycle-closure report</a> and papers/ic-negative-results/lean/IcNegative/CycleClosure.lean at main 26233d8bd280. Source-scope review only in this integration; no new Lean build is represented."),
        ("small", "[6] <a href=\"https://github.com/isogeny-labs/crypto/blob/26233d8bd28031bf64e959c7234815bf1a38958b/docs/ic/SUCCESS_CRITERIA.md\">Repository success criteria</a> and its AGENTS.md. [7] This PR's complexity_model.rs, complexity_accounting.rs and tools/primefield-e2e; immutable input manifest and native outputs beside this report."),
        ("small", "Sources checked (KB): KB unavailable; the wrapper returned exit127 because no research-kb executable/catalog was installed. Parent-reviewed primary papers, the archived source records and current graph correction were the fallback. No search-absence or novelty conclusion follows."),
        ("small", "Canonical graphs and benchmark copies: no new eligible measured speedup or leaderboard entry is produced. Existing historical graphs in PR #5067 and canonical scoreboards remain unchanged. This report adds accounting/model visuals, not a new performance run. [pcc:unverified]"),
    ])
    fit_rows = []
    for name, label in [("prime-abscissa|f4|", "Interval / F4"),
                        ("prime-isogeny|f4|", "Vélu / F4")]:
        row = next(r for r in results["descriptive_solver_fits"] if r["method"].startswith(name))
        fit = row["fit"]
        fit_rows.append([label, f'{fit["raw_slope"]:.4f}',
            f'{fit["normalized_to_sqrt_n_slope"]:.4f}',
            f'{fit["distinct_sizes"]} orders / {fit["span_bits"]:.3f} bits'])
    pages.append([
        ("h1", "Appendix. Native outputs and run commands"),
        ("table", ["Archived solver series", "Raw slope", "Slope vs √n", "Input range"],
         fit_rows, [147, 84, 98, TEXT_WIDTH-329]),
        ("body", "These OLS slopes describe the raw solver-operation file means in the recorded "
         "full-rank F4 panels. They are not calibrated total-runtime slopes. The fits combine "
         "different curves and repeated seed files at nominally fixed t; their short input range "
         "and incomplete cost coverage prevent an asymptotic inference. [pcc:unverified]"),
        ("table", ["Cost/provenance gap", "Implication"], [
            ["Positive uncharged native counters", "Auxiliary work remains outside the recorded GAE subtotal."],
            ["Positive setup wall time with zero GAE", "The phase is present but not fully priced in that work unit."],
            ["Missing calibration for positive work", "A zero/null price cannot establish zero cost."],
            [f'{totals["missing_source_commit"]} timed rows missing producer git commit', "Input commit pinning does not restore the original executable's source binding."],
        ], [210, TEXT_WIDTH-210]),
        ("body", "Native analysis used Rust 1.100.0. From the repository root, fetch the pinned "
         "archive commit if it is absent, build "
         "the native tool and regenerate the outputs. The example pipeline input deliberately "
         "leaves recovery/verification unknown, so its complete budget must remain null even "
         "though its conditional exponent can be evaluated."),
        ("body", f"For that illustrative input, the native result gives e*="
         f"{pipeline['conditional_exponent']['work_exponent']:.6f} and log<sub>2</sub> relation work="
         f"{pipeline['conditional_finite_relations']['relation_work_log2']:.6f}, retaining "
         f"log<sub>2</sub>(m!)={pipeline['conditional_finite_relations']['factorial_log2']:.6f}. "
         "The finite probability model is the Poisson occupancy heuristic. The unit is explicitly "
         "hypothetical, and complete_budget remains null because recovery_verification is unknown. "
         "This demonstrates the connection without presenting it as a measurement. [pcc:unverified]"),
        ("code", "git fetch origin aab505dd2d24cd5c98f9be1a2bd2243793a2001b\n"
         "cargo test --locked --release --manifest-path tools/primefield-e2e/Cargo.toml\n"
         "cargo build --locked --release --manifest-path tools/primefield-e2e/Cargo.toml\n"
         "e2e_study=research/primefield_e2e_20261125\n"
         "e2e_bin=tools/primefield-e2e/target/release/primefield-e2e\n"
         '"$e2e_bin" analyze . "$e2e_study/sources.json" "$e2e_study"\n'
         '"$e2e_bin" models "$e2e_study"\n'
         '"$e2e_bin" evaluate "$e2e_study/pipeline-input.json" "$e2e_study/pipeline-result.json"\n'
         'python3 "$e2e_study/render_report.py"'),
        ("small", "Rendering dependencies: Python with ReportLab and PyMuPDF, Liberation Sans Regular/Bold fonts "
         "under /usr/share/fonts/truetype/liberation, and Poppler for page inspection. These "
         "dependencies lay out the report only; Rust performs the analysis."),
    ])
    return pages


def page_decoration(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(TEAL)
    canvas.setLineWidth(2)
    canvas.line(53, HEIGHT-34, WIDTH-53, HEIGHT-34)
    canvas.setFillColor(MUTED)
    canvas.setFont("Helvetica", 7.5)
    canvas.drawString(53, HEIGHT-25, "ISOGENY LABS  /  RUNTIME ACCOUNTING")
    canvas.drawRightString(WIDTH-53, HEIGHT-25, "25 NOV 2026")
    canvas.setStrokeColor(RULE)
    canvas.setLineWidth(.5)
    canvas.line(53, 42, WIDTH-53, 42)
    canvas.setFillColor(MUTED)
    canvas.setFont("Helvetica", 7)
    canvas.drawString(53, 29, "Conditional models and archived producer evidence")
    canvas.drawRightString(WIDTH-53, 29, str(doc.page))
    canvas.restoreState()


def render(pages):
    story, md = [], []
    for page_index, page in enumerate(pages):
        if page_index:
            story.append(PageBreak())
        for block in page:
            kind = block[0]
            if kind in ("title", "subtitle", "h1", "h2", "body", "small", "equation"):
                story.append(p(block[1], kind))
                prefix = "# " if kind == "title" else "## " if kind == "h1" else "### " if kind == "h2" else ""
                md.append(prefix + markdown_text(block[1]) + "\n")
            elif kind == "table":
                _, headers, rows, widths = block
                story.append(table(headers, rows, widths))
                story.append(Spacer(1, 9))
                md.append("| " + " | ".join(headers) + " |")
                md.append("| " + " | ".join(["---"]*len(headers)) + " |")
                md.extend("| " + " | ".join(markdown_text(str(x)).replace("\n", " ") for x in row) + " |" for row in rows)
                md.append("")
            elif kind == "figure":
                _, name, drawing, caption = block
                renderSVG.drawToFile(drawing, str(ROOT/name))
                story.append(Vector(drawing))
                story.append(Spacer(1, 5))
                story.append(p(caption, "small"))
                md.extend([f"![{markdown_text(caption)}]({name})", "", markdown_text(caption), ""])
            elif kind == "code":
                story.append(p(html.escape(block[1]).replace("\n", "<br/>"), "mono"))
                md.append("```sh\n"+block[1]+"\n```\n")
            else:
                raise ValueError(kind)
    (ROOT/"README.md").write_text("\n".join(md)+"\n")
    with tempfile.TemporaryDirectory(prefix="primefield-report-") as temporary:
        raw_pdf = Path(temporary)/"report.pdf"
        doc = BaseDocTemplate(str(raw_pdf), pagesize=(WIDTH, HEIGHT),
            title="Prime-field index calculus: end-to-end runtime accounting",
            author="Isogeny Labs", subject="Conditional native models and archived evidence",
            leftMargin=53, rightMargin=53, topMargin=55, bottomMargin=56,
            pageCompression=1, allowSplitting=1, invariant=1)
        frame = Frame(53, 56, TEXT_WIDTH, HEIGHT-111,
                      leftPadding=0, rightPadding=0, topPadding=0, bottomPadding=0)
        doc.addPageTemplates([PageTemplate(id="report", frames=[frame], onPage=page_decoration)])
        doc.build(story)
        # Keep embedded font programs and vector content. Cleaning removes
        # redundant drawing operators; deflate/object streams are lossless.
        # Fixed metadata and no_new_id make repeated renders byte-identical.
        with fitz.open(raw_pdf) as pdf:
            # Use the report's declared study date, not the render clock or
            # ReportLab's invariant-mode default date in January 2000.
            pdf.set_metadata({
                "title": "Prime-field index calculus: end-to-end runtime accounting",
                "author": "Isogeny Labs",
                "subject": "Conditional native models and archived evidence",
                "creator": "ReportLab and PyMuPDF report renderer",
                "producer": "ReportLab PDF Library and PyMuPDF",
                "creationDate": "D:20261125000000-08'00'",
                "modDate": "D:20261125000000-08'00'",
            })
            pdf.save(ROOT/"report.pdf", garbage=4, clean=True, deflate=True,
                     deflate_fonts=True, use_objstms=1, compression_effort=100,
                     no_new_id=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--model", type=Path, default=ROOT/"model-results.json")
    parser.add_argument("--results", type=Path, default=ROOT/"results.json")
    parser.add_argument("--pipeline", type=Path, default=ROOT/"pipeline-result.json")
    args = parser.parse_args()
    model = json.loads(args.model.read_text())
    results = json.loads(args.results.read_text())
    pipeline = json.loads(args.pipeline.read_text())
    render(build_pages(model, results, pipeline))


if __name__ == "__main__":
    main()
