#!/usr/bin/env python3
"""Layout the iteration-six report and its vector topology figure.

This script renders supplied prose, equations, and recorded validation status.
It performs no finite-field, curve, resultant, solver, or benchmark calculation.
Run from any directory. The previous reports and mathematical source are read
only. Update report-validation.json and REPORT.md from an actual receipt before
rendering a final CI-result edition.
"""
from pathlib import Path
from xml.sax.saxutils import escape
import hashlib
import json
import platform
import re
import subprocess
import tempfile

import fitz
import pypdf
import reportlab
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, PageBreak
from reportlab.graphics.shapes import Drawing, Rect, String, Line
from reportlab.graphics import renderSVG, renderPDF

ROOT = Path(__file__).resolve().parent
FIG = ROOT / "figures"
FIG.mkdir(exist_ok=True)
VALIDATION = json.loads((ROOT / "report-validation.json").read_text())
WIDTH = 504
REPORT_PAGES = 7
NAVY = colors.HexColor("#153145")
TEAL = colors.HexColor("#087C80")
INK = colors.HexColor("#243746")
MUTED = colors.HexColor("#607482")
PALE = colors.HexColor("#EDF5F5")
GREY = colors.HexColor("#F2F5F7")
AMBER = colors.HexColor("#9B5B13")
WARM = colors.HexColor("#FAF1E5")
RULE = colors.HexColor("#CCD8DE")
WHITE = colors.white

FONT_ROOT = Path("/usr/share/fonts/truetype/liberation")
pdfmetrics.registerFont(TTFont("ReportSans", str(FONT_ROOT / "LiberationSans-Regular.ttf")))
pdfmetrics.registerFont(TTFont("ReportSansBold", str(FONT_ROOT / "LiberationSans-Bold.ttf")))
pdfmetrics.registerFontFamily("ReportSans", normal="ReportSans", bold="ReportSansBold",
                             italic="ReportSans", boldItalic="ReportSansBold")

styles = getSampleStyleSheet()
styles.add(ParagraphStyle("BodyR", fontName="ReportSans", fontSize=10.0,
                          leading=14.0, textColor=INK, spaceAfter=8))
styles.add(ParagraphStyle("SmallR", parent=styles["BodyR"], fontSize=8.5,
                          leading=11.4, spaceAfter=6))
styles.add(ParagraphStyle("CaptionR", parent=styles["BodyR"], fontSize=8.1,
                          leading=10.8, textColor=MUTED, spaceAfter=7))
styles.add(ParagraphStyle("TitleR", fontName="ReportSansBold", fontSize=29,
                          leading=33, textColor=NAVY, spaceAfter=12))
styles.add(ParagraphStyle("H1R", fontName="ReportSansBold", fontSize=21,
                          leading=25, textColor=NAVY, spaceAfter=12))
styles.add(ParagraphStyle("H2R", fontName="ReportSansBold", fontSize=12,
                          leading=15, textColor=TEAL, spaceBefore=8, spaceAfter=6))
styles.add(ParagraphStyle("EquationR", fontName="ReportSans", fontSize=12.4,
                          leading=20, textColor=NAVY, spaceAfter=7))
styles.add(ParagraphStyle("BoxEquationR", parent=styles["EquationR"],
                          fontSize=12.2, leading=24, spaceAfter=4))
styles.add(ParagraphStyle("LabelR", fontName="ReportSansBold", fontSize=8.1,
                          leading=11, textColor=TEAL, spaceAfter=6))
styles.add(ParagraphStyle("MonoR", parent=styles["SmallR"], fontSize=7.8,
                          leading=10.2, spaceAfter=5))


def p(text, style="BodyR"):
    text = text.replace("<=", "&le;").replace("t-1<n", "t-1&lt;n")
    parts = re.split(r"(<[^>]*>)", text)
    for i in range(0, len(parts), 2):
        parts[i] = parts[i].replace(">=", "&ge;")
    text = "".join(parts)
    return Paragraph(text, styles[style])


def h(text):
    return p(text, "H2R")


def eq(text):
    return p(text, "EquationR")


def caption(text):
    return p(text, "CaptionR")


def box(title, text, fill=PALE, body_style="BodyR"):
    t = Table([[p(title, "LabelR")], [p(text, body_style)]], colWidths=[WIDTH])
    t.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, -1), fill),
        ("BOX", (0, 0), (-1, -1), .5, RULE),
        ("LEFTPADDING", (0, 0), (-1, -1), 11),
        ("RIGHTPADDING", (0, 0), (-1, -1), 11),
        ("TOPPADDING", (0, 0), (-1, 0), 9),
        ("BOTTOMPADDING", (0, -1), (-1, -1), 9),
    ]))
    return t


def table(rows, widths):
    t = Table([[p(str(c), "SmallR") for c in row] for row in rows],
              colWidths=widths, repeatRows=1, hAlign="LEFT")
    t.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, 0), PALE),
        ("LINEBELOW", (0, 0), (-1, 0), .8, TEAL),
        ("VALIGN", (0, 0), (-1, -1), "TOP"),
        ("LEFTPADDING", (0, 0), (-1, -1), 8),
        ("RIGHTPADDING", (0, 0), (-1, -1), 8),
        ("TOPPADDING", (0, 0), (-1, -1), 6),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 6),
        ("LINEBELOW", (0, 1), (-1, -1), .3, RULE),
    ]))
    return t


def page(title, subtitle):
    return [p(title, "H1R"), caption(subtitle)]


def topology_diagram():
    """Draw supplied symbolic labels; no group or field operations are run."""
    d = Drawing(WIDTH, 242)
    for x in (0, 260):
        d.add(Rect(x, 0, 244, 242, rx=5, ry=5, fillColor=GREY,
                   strokeColor=RULE, strokeWidth=.6))
    d.add(String(12, 222, "Free-order left chain", fontName="ReportSansBold",
                 fontSize=11, fillColor=TEAL))
    d.add(String(272, 222, "Fixed balanced tree", fontName="ReportSansBold",
                 fontSize=11, fillColor=AMBER))
    for x in (12, 272):
        d.add(String(x, 206, "Same multiset: P, P, P, -P; prime n >= 5",
                     fontName="ReportSans", fontSize=7.5, fillColor=MUTED))

    def edge(a, b, colour=TEAL):
        d.add(Line(a[0], a[1], b[0], b[1], strokeColor=colour, strokeWidth=1.15))

    def node(cx, y, text, kind="internal"):
        w, hh = 42, 24
        fill = TEAL if kind == "root" else (WARM if kind == "zero" else WHITE)
        stroke = AMBER if kind == "zero" else (TEAL if kind == "root" else RULE)
        ink = WHITE if kind == "root" else (AMBER if kind == "zero" else INK)
        d.add(Rect(cx-w/2, y, w, hh, rx=3, ry=3, fillColor=fill,
                   strokeColor=stroke, strokeWidth=1 if kind == "zero" else .6))
        d.add(String(cx, y+8, text, textAnchor="middle", fontName="ReportSansBold",
                     fontSize=10.5, fillColor=ink))

    # Four-leaf comb. Every node label is part of the supplied proof example.
    edge((172, 172), (119, 145))
    edge((172, 172), (211, 145))
    edge((119, 121), (66, 94))
    edge((119, 121), (157, 94))
    edge((66, 70), (30, 43))
    edge((66, 70), (98, 43))
    node(172, 172, "2P", "root")
    node(119, 121, "3P")
    node(211, 121, "-P")
    node(66, 70, "2P")
    node(157, 70, "P")
    node(30, 19, "P")
    node(98, 19, "P")
    d.add(String(130, 37, "Every internal", fontName="ReportSans", fontSize=8,
                 fillColor=TEAL))
    d.add(String(130, 25, "sum is finite.", fontName="ReportSans", fontSize=8,
                 fillColor=TEAL))

    # Balanced pairing. The lone negative leaf must meet a positive leaf.
    edge((383, 172), (323, 139))
    edge((383, 172), (442, 139), AMBER)
    edge((323, 115), (293, 82))
    edge((323, 115), (347, 82))
    edge((442, 115), (416, 82), AMBER)
    edge((442, 115), (473, 82), AMBER)
    node(383, 172, "2P", "root")
    node(323, 115, "2P")
    node(442, 115, "O", "zero")
    for cx, label in ((293, "P"), (347, "P"), (416, "P"), (473, "-P")):
        node(cx, 58, label)
    d.add(String(272, 29, "Every pairing has a cancelling child.",
                 fontName="ReportSans", fontSize=8, fillColor=AMBER))
    d.add(String(272, 16, "O has no affine abscissa.", fontName="ReportSans",
                 fontSize=8, fillColor=AMBER))
    return d


diagram = topology_diagram()
svg_path = FIG / "rational-chain-topology.svg"
png_path = FIG / "rational-chain-topology.png"
renderSVG.drawToFile(diagram, str(svg_path))
svg = svg_path.read_text()
svg = svg.replace("font-family: ReportSansBold;", "font-family: Liberation Sans, Arial, sans-serif; font-weight: bold;")
svg = svg.replace("font-family: ReportSans;", "font-family: Liberation Sans, Arial, sans-serif;")
svg = svg.replace("<title>...</title>", "<title>Affine chain completeness and a balanced-tree counterexample</title>")
svg_path.write_text(svg)
with tempfile.TemporaryDirectory(prefix="prime31-iteration6-figure-") as td:
    tmp = Path(td)
    renderPDF.drawToFile(diagram, str(tmp / "figure.pdf"))
    subprocess.run(["pdftoppm", "-singlefile", "-r", "105", "-png",
                    str(tmp / "figure.pdf"), str(tmp / "figure")], check=True, capture_output=True)
    png_path.write_bytes((tmp / "figure.png").read_bytes())

status = VALIDATION["status"]
ci_passed = status in ("passed", "success")
ci_blocked = status.startswith("blocked")
ci_summary = escape(VALIDATION["ci_summary"])
identity_summary = escape(VALIDATION["identity_summary"])
source_commit = escape(VALIDATION["source_commit"])
ci_line = ""
if VALIDATION.get("ci_url"):
    run_label = ("Source-bound failed CI run" if ci_blocked else
                 ("Source-bound CI receipt" if ci_passed else "Source-bound CI run (pending)"))
    ci_line = "<link href='" + escape(VALIDATION["ci_url"]) + "' color='#087C80'>" + run_label + "</link>. Its standing and scope are recorded above."
status_label = ("CI BLOCKED BEFORE NEW TESTS" if ci_blocked else
                ("SOURCE-BOUND CORRECTNESS RECEIPT" if ci_passed else "CI PENDING"))
next_action = VALIDATION.get("next_action", "Complete source-bound correctness replay, then specify an actual guarded-quadratic solver. Its exact empty-query and output contracts must charge setup, state dimension, misses, rational extraction, memory and signed rows against the maintained group-complement and meet-in-the-middle controls.")

story = []
# Page 1: the new representation and its domain.
story += [p("ISOGENY LABS  /  ITERATIONS 6-7", "LabelR"),
          p("An exact quadratic<br/>relation system", "TitleR"),
          p("Rational S3 chains, compact Dickson membership, and the next solving gate", "BodyR"),
          caption("22 November 2026, America/Los_Angeles  |  Existing research lane: PR #4928"),
          Spacer(1, 8),
          box("THE CONCRETE CANDIDATE", "Orient every S3 node from a known rational target, then attach a squaring ladder at each leaf. The resulting system has <b>mk+3m-5 variables</b> and <b>mk+3m-4 equations</b>, all of total degree at most two. On its finite Fp domain, full-fibre membership preserves exactly the rational signed witnesses. [M1, M2]"),
          Spacer(1, 12),
          table([
              ["Input", "Specified value or condition"],
              ["Prime field", "p = 2147483647 = 2<super>31</super>-1"],
              ["Source curve", "y<super>2</super> = x<super>3</super> + 847514846x + 2087444724"],
              ["Recorded group order", "n = 2147446831; prime and odd"],
              ["Target and chart", "R=(r,s) is rational and nonzero; every auxiliary x is finite and in Fp"],
              ["Membership", "F(X)=D<sub>N</sub>(X)-c is split and squarefree; N=2<super>k</super>"],
          ], [142, 362]),
          caption("Curve/order inputs are inherited from the lane; this iteration supplies no new point-count certificate. Counts assume m>=2 and k>=1. [M1]"),
          h("The guarantee and the next obligation"),
          p("A left chain with free leaf order represents every unordered signed multiset of m<=n nonzero points with a nonzero total in the prime-order group. Its compact equations still need an actual root-solving algorithm, including exact empty-target handling and all extraction costs. [M1, M2]"),
          box(status_label, ci_summary + " Mathematical derivations and independent written review retain <b>[pcc:unverified]</b> standing.", WARM, "SmallR"),
          PageBreak()]

# Page 2: the full quadratic system and exact bookkeeping.
story += page("1  Equations and exact size", "Every vertex is oriented by its parent; the root point is a supplied constant.")
story += [p("At an internal vertex v, use child abscissae x<sub>l</sub>, x<sub>r</sub> and introduce a slope λ<sub>v</sub>. Introduce y<sub>v</sub> only for nonroot internal vertices; the root has (x<sub>v</sub>,y<sub>v</sub>)=(r,s). [M1]"),
          box("NODE EQUATIONS", "λ<sub>v</sub><super>2</super> = x<sub>l</sub> + x<sub>r</sub> + x<sub>v</sub> &nbsp; (Q1)<br/>2y<sub>v</sub>λ<sub>v</sub> = (x<sub>l</sub>-x<sub>v</sub>)(x<sub>r</sub>-x<sub>v</sub>) - (3x<sub>v</sub><super>2</super>+a) &nbsp; (Q2)<br/>y<sub>w</sub> = λ<sub>v</sub>(x<sub>w</sub>-x<sub>v</sub>) - y<sub>v</sub> &nbsp; (Q3)", body_style="BoxEquationR"),
          Spacer(1, 8),
          p("Q3 is imposed for each nonroot internal child w. Leaf ordinates are recovered by the same formula after solving. The cubic factorization on the next page makes separate curve equations unnecessary. [M1, M2] [pcc:unverified]"),
          h("Membership stays a squaring ladder"),
          eq("z<sub>i,0</sub>=x<sub>i</sub>; &nbsp; z<sub>i,j+1</sub>=z<sub>i,j</sub><super>2</super>-2; &nbsp; z<sub>i,k</sub>=c"),
          p("The last endpoint is substituted as a constant. Each leaf therefore contributes k equations and k-1 new variables, enforcing D<sub>2<super>k</super></sub>(x<sub>i</sub>)=c without expanding the degree-N membership polynomial. [M1]"),
          table([
              ["Component", "Variables", "Equations"],
              ["Nonroot abscissae", "2m-2", "0"],
              ["Nonroot internal ordinates", "m-2", "m-2"],
              ["Slopes and their constraints", "m-1", "2m-2"],
              ["Dickson intermediates", "m(k-1)", "mk"],
              ["<b>Total</b>", "<b>mk+3m-5</b>", "<b>mk+3m-4</b>"],
          ], [264, 120, 120]),
          caption("For m>=2 and k>=1. N=1 uses linear membership; m=1 tests the target directly. These are Fp-domain counts. Closure guards are additional. [M1, M2]"),
          p("The representation takes O(mk) field-sized data once valid curve and fibre inputs are supplied. This bounds construction, not elimination degree or solving work. Selecting and validating those inputs remains setup. [M1]"),
          PageBreak()]

# Page 3: soundness, rationality, and the projection/domain distinction.
story += page("2  Why the leaves are rational", "A local line-intersection identity supplies a unique orientation at every node.")
story += [p("Let the parent be R=(r,s), put u=x-r and v=z-r, and suppose S3(x,z,r)=0 over Fp. With f(X)=X<super>3</super>+aX+b, define: [M1, E2]"),
          eq("λ = [uv-(3r<super>2</super>+a)]/(2s)"),
          eq("P=(x,λu-s), &nbsp; Q=(z,λv-s)"),
          box("THE CUBIC FACTORIZATION", "f(r+T) - (λT-s)<super>2</super> = T(T-u)(T-v)", body_style="BoxEquationR"),
          Spacer(1, 8),
          p("Centered S3 gives λ<super>2</super>=x+z+r. The line therefore meets E at -R, P and Q, including multiplicities, so P+Q=R. Division is only by 2s: repeated coordinates and tangent cases remain covered. [M1, M2]"),
          h("Root orientation propagates through the whole tree"),
          p("The root is rational. Its children become rational by the local formula; odd rational group order keeps their ordinates nonzero. Repeating the same step at every internal child makes all leaf points rational and all edge signs consistent. The lift is unique. [M1, M2]"),
          eq("V<sub>Fp</sub>(S3 tree, F(x<sub>i</sub>)) = V<sub>Fp</sub>(S3 tree, H(x<sub>i</sub>))"),
          p("Here H contains only the liftable fibre coordinates. This equality of Fp-valued assignments is what permits the full Dickson F at every leaf. Q1-Q3 give a bijective augmentation of those assignments. [M1, M2]"),
          box("A PROJECTED S4 CAN STILL HAVE A GHOST", "Choose nonliftable w and rational fibre point R with x-coordinate r. The geometric tuple P,-P,R gives S4(w,r,w,r)=0. Yet q(U)=S3(w,r,U) has nonsquare discriminant 16f(w)f(r), so its two roots lie in Fp<super>2</super> and neither lies in Fp. The rational affine chain rejects it. [M1]", WARM, "SmallR"),
          Spacer(1, 6),
          caption("The distinction is the domain: Fp coefficients alone do not restrict algebraic roots to Fp. A projected resultant, an extension-valued chain, and a finite-Fp chain are different relation objects. The new Rust fixture checks this boundary; its current status is recorded on page 6. [E1, R1]"),
          PageBreak()]

# Page 4: the topology counterexample carries the important scope boundary.
story += page("3  Completeness needs a free order", "The theorem covers the left chain; an arbitrary balanced shape can omit valid multisets.")
story += [p("For m>=3 the chain uses q<sub>j</sub> for its prefix abscissae: S3(x<sub>1</sub>,x<sub>2</sub>,q<sub>2</sub>), then S3(q<sub>j-1</sub>,x<sub>j</sub>,q<sub>j</sub>), ending with S3(q<sub>m-1</sub>,x<sub>m</sub>,r). For m=2 only S3(x<sub>1</sub>,x<sub>2</sub>,r) remains. [M1]"),
          h("Backward peeling proves the ordering lemma"),
          p("Take m<=n nonzero points with nonzero total in a prime-order group. At a stage with t>=2 remaining points and sum S, remove a point different from S. Failure would mean every remaining point equals S, forcing (t-1)S=O with 1<=t-1<n. Thus the remaining sum stays nonzero. Reverse the removals to obtain a chain with no zero prefix. [M1, M2]"),
          diagram,
          caption("Figure 1. P,P,P,-P has nonzero total 2P for prime n>=5. The left chain is finite at every internal node. Every balanced pairing contains a P,-P child and cannot represent this multiset in an affine-only chart. [M1, M2]"),
          h("What the completeness claim returns"),
          p("All unordered signed multisets of exactly m points are covered, including repeats and cancelling pairs, when leaf order is free. A fixed sorted order can lose witnesses. Recovering every ordered tuple needs permutation expansion or explicit identity branches; duplicate multisets and output work remain charged. R=O is a separate case. [M1]"),
          caption("The uniform m<=n bound is sharp: n+1 copies of a nonzero P have a zero prefix at length n in every ordering. [M2]"),
          PageBreak()]

# Page 5: rationality guards and complete costs.
story += page("4  Field guards and complete cost", "The field domain must survive elimination, and solving work must remain visible.")
story += [p("A closure-based solver needs Fp guards on the m-2 internal abscissae. Full splitting already places leaf x values in Fp; root-oriented propagation then forces all slopes and ordinates into Fp. [M1, M2]"),
          eq("h<sub>1</sub>=x<super>2</super>; &nbsp; h<sub>j</sub>=h<sub>j-1</sub><super>2</super> (2<=j<=30); &nbsp; h<sub>30</sub><super>2</super>=h<sub>1</sub>"),
          p("This circuit imposes x<super>2<super>31</super></super>-x<super>2</super>=x(x<super>p</super>-x)=0. Its zero set is exactly Fp. It adds 30 variables and 31 quadratic equations per internal x. [M1, M2]"),
          table([
              ["Encoding", "Total variables", "Total equations"],
              ["Native Fp domain", "mk+3m-5", "mk+3m-4"],
              ["Mersenne zero-set guards", "mk+33m-65", "mk+34m-66"],
          ], [228, 138, 138]),
          caption("Zero has multiplicity two in the shorter guard. Raw root multiplicity is not a witness count. A squarefree eliminated x<super>p</super>-x guard costs 59 variables and 60 equations per internal x. Native Fp solving needs neither extra guard. [M1, M2]"),
          h("Witness extraction has an explicit local price"),
          eq("W<sub>lift</sub> <= (m-1)(I<sub>p</sub>+4M<sub>p</sub>+S<sub>p</sub>+6A<sub>p</sub>+2C<sub>p</sub>)"),
          p("I, M, S, A and C denote inversion, multiplication, squaring, addition/subtraction and constant scaling. Membership evaluation adds mk squarings and mk subtractions. Signed replay, normalization and deduplication remain additional; writing z rows costs at least Ω(mz) labels. [M1, M2]", "SmallR"),
          box("THE COMPLETE RELATION COST", "Parameter/fibre setup + system construction + solving + rational root isolation + oriented extraction + output. The unresolved term is the solver, especially an exact empty-target certificate. [M1]", body_style="SmallR"),
          Spacer(1, 6),
          h("Target-independent sharing still needs paid states"),
          eq("f<sub>j</sub>(S)=1<sub>S≠O</sub> sum<sub>P in A</sub> f<sub>j-1</sub>(S-P)"),
          p("Here A=-A and L=2B. A sparse explicit build extends each supported state by L choices. Complete prefix-witness lists still have Θ(L<super>j</super>) entries for fixed j. Aggregate state graphs can be smaller; their retained counts, edges and regenerated query work must be priced. [M1, M2]", "SmallR"),
          h("A concrete m=4 budget"),
          p("In the conventional uniform-target/full-rank model, charge setup B<super>σ</super>, empty-inclusive queries B<super>τ</super> and sparse linear algebra B<super>2</super>. With α=max(σ,2), improving the model's one-half exponent requires α+τ&lt;m-1. For m=4 and σ≤2 this means <b>τ&lt;1</b>. This is a necessary conditional budget, not an achieved solver bound. [R2]", "SmallR"),
          caption("A growing-field exponent claim must state its family. Infinitely many Mersenne primes are not assumed."),
          PageBreak()]

# Page 6: source-bound evidence and the successor obligation.
story += page("5  Evidence and the next gate", "Correctness receipts, mathematical review, and performance claims have distinct scopes.")
story += [table([
              ["Record", "Current standing"],
              ["Mathematical note", "Written derivation; independent written audit. [pcc:unverified]"],
              ["Original-root-lock correctness CI", ci_summary],
              ["Centered S3 / discriminant", "Existing native evaluated-tier records; proof:null. [E2]"],
              ["New cubic-residual identity", identity_summary],
              ["Performance", escape(VALIDATION["performance"])],
          ], [156, 348]),
          Spacer(1, 5),
          p(ci_line, "SmallR"),
          p("Frozen source: " + source_commit + ". [E1]", "MonoR"),
          p("The frozen panel covers the original N=64/B=31 fibre, an explicit Fp<super>2</super> ghost, repeated and damaged certificates, repaired identity prefixes, the balanced-tree counterexample, and all 64 triples plus 256 quadruples on a fixed four-point signed base. " + ("The retained receipt gives the actual executed results." if ci_passed else "These are specified controls, not yet reported observations.") + " [E1]", "SmallR"),
          box("NEXT DISCRIMINATING ACTION", escape(next_action) + " [M1, E1]", body_style="SmallR"),
          Spacer(1, 7),
          h("Prior art and source map"),
          p("<b>L1.</b> Semaev (2015), <link href='https://arxiv.org/pdf/1504.01175' color='#087C80'>arXiv:1504.01175</link>, Section 3 equation (4), Section 4.1 Lemma 2 and Section 4.2: ground-field S3 chains and a related minimality lemma already exist. <b>R1.</b> Native prime31_m8_oriented_f4.rs already has Dickson ladders and orientation of its final pair. This continuation makes no literature-wide novelty claim.", "SmallR"),
          p("<b>M1.</b> MATHEMATICAL_NOTE.md, unchanged proof note. <b>M2.</b> INDEPENDENT_REVIEW.md, written proof audit. <b>R2.</b> ROOT_REVIEW.md, integration and exponent-budget review. <b>E1.</b> SCOPE.md, SOURCE_FREEZE.json and report-validation.json, with the actual CI receipt when present. <b>E2.</b> ../identity-certificates/centered-s3.json and s3-discriminant.json: IDC1hf62e2402e4871a82 and IDC1h869923a46a34c70c. Evaluated identities do not certify the all-arity theorem.", "SmallR"),
          p("M1 SHA-256: 948d36ca3d72661aa4cf16786ebed30fcb2ba146afb0df4420767a91e8f35ddc", "MonoR"),
          p("PCC-20261123-ed96d7: representation; PCC-20261123-36ec87: scoped budget; PCC-20261123-184d4e: audits. All three records remain unverified.", "SmallR"),
          caption("Continuation records: <link href='https://github.com/isogeny-labs/crypto/pull/4928' color='#087C80'>isogeny-labs/crypto PR #4928</link> and the <link href='https://chatgpt.com/space/page_729b0b80bdb4819193f457c80ecabe84' color='#087C80'>research Space</link>. New mathematical standing remains [pcc:unverified]. Previous pinned reports and evidence are unchanged."),
          PageBreak()]

# Page 7: the reviewed growing-family and factor-base availability refinements.
story += page("6  Growing fields and available fibres", "Iteration seven supplies input availability; solving complexity remains a separate obligation.")
story += [h("An unconditional smooth p+1 family"),
          p("Set Q=2<super>k</super> and let p be the least prime congruent to -1 modulo 4Q. Linnik's theorem gives p≤C(4Q)<super>L</super> for absolute constants. Hence Q=Ω(p<super>1/L</super>) without assuming infinitely many Mersenne primes. Enlarge L to at least 2 and cap q at the largest power of two not exceeding min(Q,√p/16); q still has the same polynomial lower bound. [F1, F2]", "SmallR"),
          eq("4q | (p+1), &nbsp; q=Ω(p<super>1/L</super>), &nbsp; q≤√p/16"),
          p("For norm-one z in Fp<super>2</super> and ζ of order q, set c=z<super>q</super>+z<super>-q</super>. When c≠±2, the q values x<sub>j</sub>=zζ<super>j</super>+(zζ<super>j</super>)<super>-1</super> are exactly the distinct Fp roots of D<sub>q</sub>(X)-c. Enumerating them costs Θ(q) extension-field operations and output slots. A full torus generator is unnecessary. [F1, F2]", "SmallR"),
          h("Two ways to obtain a usable rational base"),
          table([
              ["Input held fixed", "Written availability result"],
              ["A symmetric full fibre F, |F|=N≥16; p≡3 mod4", "Vary (a,b). Pairwise-independent evaluations give a positive-density set of irreducible cubics with N/4≤B≤3N/4. These curves have odd order and are ordinary. A quadratic-twist choice removes anomalous order p. [F3]"],
              ["Any nonsingular odd-order E/Fp", "Vary the regular Dickson fibre. If p≥4096 and 4≤q≤√p/16, at least 1/6 of the full q-fibres satisfy B≥q/4. Uniform norm-one sampling gives a uniform regular fibre after excluding c=±2. [F2]"],
          ], [158, 346]),
          Spacer(1, 7),
          box("CONSTRUCTIVE SETUP, WITH ITS SCOPE", "For an already certified odd-order curve and field, fewer than seven seed trials suffice in expectation. Enumeration and liftability screening cost expected O(q log p) field operations, plus charged inversions. If E itself has prime order, these rational points belong to that group. This does not certify a growing prime-order curve family or membership in a prescribed proper subgroup. [F2]", body_style="SmallR"),
          Spacer(1, 7),
          p("The variable-curve route does not itself guarantee prime order. For fixed arity m, excluding leaf points of order at most m-1 preserves a Θ(N) base asymptotically and recovers the free-order peeling hypothesis. Its additional membership guards are outside the frozen counts. Subgroup size, relation rank and the exact target solver remain separately charged. [F3]", "SmallR"),
          h("Provenance and limits"),
          p("<b>F1.</b> ../iteration7/SMOOTH_TORUS_FAMILY.md. <b>F2.</b> ../iteration7/INDEPENDENT_REVIEW.md and FIXED_CURVE_FIBRE_DENSITY.md. <b>F3.</b> ../iteration7/SELECTABLE_CURVE_AVAILABILITY.md. Linnik: Koukoulopoulos, <i>The Distribution of Prime Numbers</i>, Chapter 27, Theorem 27.1. Ordinary traces: Waterhouse (1969), Theorem 4.1. Fixed-curve density uses the degree-five Weil character bound. [pcc:unverified]", "SmallR"),
          caption("These are written proofs and reviewed deductions, not numerical measurements or signed formal certificates. No asymptotic relation-solving speedup is asserted.")]


def on_page(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(RULE)
    canvas.setLineWidth(.6)
    canvas.line(54, 750, 558, 750)
    canvas.setFont("ReportSansBold", 8)
    canvas.setFillColor(TEAL)
    canvas.drawString(54, 760, "ISOGENY LABS  /  PRIME-FIELD RELATION RESEARCH")
    canvas.setFont("ReportSans", 7.6)
    canvas.setFillColor(MUTED)
    canvas.drawRightString(558, 760, "ITERATIONS 6-7  |  2026-11-22 PT")
    canvas.line(54, 42, 558, 42)
    canvas.setFont("ReportSans", 7.4)
    canvas.drawString(54, 29, "Unsigned mathematical checkpoint  [pcc:unverified]")
    canvas.drawRightString(558, 29, f"{doc.page}  /  {REPORT_PAGES}")
    canvas.restoreState()


pdf_path = ROOT / "REPORT.pdf"
doc = SimpleDocTemplate(str(pdf_path), pagesize=(612, 792),
                        rightMargin=54, leftMargin=54, topMargin=58, bottomMargin=58,
                        title="Prime31 iteration 6 - exact quadratic relation system",
                        author="Isogeny Labs", subject="Rational S3 chain and quadratic Dickson membership",
                        pageCompression=1)
doc.build(story, onFirstPage=on_page, onLaterPages=on_page)
with fitz.open(pdf_path) as pdf:
    compact = pdf.tobytes(garbage=4, clean=True, deflate=True, deflate_fonts=True, use_objstms=1)
pdf_path.write_bytes(compact)
reader = pypdf.PdfReader(pdf_path)
if len(reader.pages) != REPORT_PAGES:
    raise RuntimeError(f"Expected {REPORT_PAGES} pages; got {len(reader.pages)}")
files = [pdf_path, ROOT / "REPORT.md", Path(__file__), ROOT / "report-validation.json", svg_path, png_path]
for path in files:
    if path.stat().st_size >= 65536:
        raise RuntimeError(f"Artifact exceeds 64 KiB: {path.name} ({path.stat().st_size} bytes)")
receipt = {
    "schema": "prime31-iteration6-render-receipt/v1",
    "purpose": "Document layout and artifact verification only; no cryptographic calculations",
    "pdf_pages": len(reader.pages),
    "pdf_links": sum(len(page.get("/Annots", [])) for page in reader.pages),
    "validation_status": VALIDATION["status"],
    "source_commit": VALIDATION["source_commit"],
    "tools": {"python": platform.python_version(), "reportlab": reportlab.Version,
              "pypdf": pypdf.__version__, "pymupdf": fitz.VersionBind},
    "files": [{"path": str(path.relative_to(ROOT)), "bytes": path.stat().st_size,
               "sha256": hashlib.sha256(path.read_bytes()).hexdigest()} for path in files],
    "render_command": "python3 research/prime31_summation_compression_20261122/iteration6/render_report.py",
    "qa_command": "pdftoppm -r 110 -png REPORT.pdf <external-qa-folder>/page",
    "visual_check": "Pending page-by-page rendered inspection for this generated PDF hash."
}
(ROOT / "render-receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps({"pages": len(reader.pages), "pdf_bytes": pdf_path.stat().st_size,
                  "validation_status": VALIDATION["status"],
                  "files": receipt["files"]}, indent=2))
