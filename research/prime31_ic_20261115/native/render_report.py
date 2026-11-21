"""Document layout only. Research results are recorded in README and test logs."""
from pathlib import Path
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, PageBreak
from svglib.svglib import svg2rlg
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

HERE = Path(__file__).resolve().parent
pdfmetrics.registerFont(TTFont("DejaVu", "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"))
pdfmetrics.registerFont(TTFont("DejaVuBold", "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"))
pdfmetrics.registerFontFamily("DejaVu", normal="DejaVu", bold="DejaVuBold", italic="DejaVu", boldItalic="DejaVuBold")
styles = getSampleStyleSheet()
for name in ["Title", "Heading2"]:
    styles[name].fontName = "DejaVuBold"
styles["Title"].fontSize = 23
styles["Title"].leading = 28
styles.add(ParagraphStyle(name="Body", fontName="DejaVu", fontSize=9,
    leading=14, spaceAfter=8, textColor=colors.HexColor("#24394e")))
styles.add(ParagraphStyle(name="SmallBody", parent=styles["Body"], fontSize=8,
    leading=11, spaceAfter=5))
styles["Title"].textColor = colors.HexColor("#152b45")
styles["Heading2"].textColor = colors.HexColor("#216985")
story = []
def p(text, style="Body"):
    story.append(Paragraph(text, styles[style]))
def table(rows, widths):
    grid = [[Paragraph(cell, styles["SmallBody"]) for cell in row] for row in rows]
    t = Table(grid, colWidths=widths, hAlign="LEFT")
    t.setStyle(TableStyle([
        ("BACKGROUND", (0,0), (-1,0), colors.HexColor("#e8f0f7")),
        ("VALIGN", (0,0), (-1,-1), "TOP"),
        ("GRID", (0,0), (-1,-1), .35, colors.HexColor("#bdcbd6")),
        ("LEFTPADDING", (0,0), (-1,-1), 7),
        ("RIGHTPADDING", (0,0), (-1,-1), 7),
        ("TOPPADDING", (0,0), (-1,-1), 6),
        ("BOTTOMPADDING", (0,0), (-1,-1), 6),
    ]))
    story.append(t); story.append(Spacer(1, 10))

p("Prime31 algebraic IC<br/>Native Rust continuation", "Title")
p("PR #4188 | November 14, 2026 (Pacific) | Producer development evidence", "SmallBody")
p("The existing Rust quotient-ring F4 and signature-engine modules are reused directly by source path. The new adapter implements a two-summand algebraic index-calculus pipeline with finite-field root extraction, signed EC relation replay, modular elimination and final scalar checking. [pcc:unverified]")
table([
    ["Requested fixture", "Exact value"],
    ["Prime field", "p = 2147483647 = 2^31 - 1"],
    ["Ordinary source curve", "y^2 = x^3 + 847514846 x + 2087444724"],
    ["Full prime group order", "2147446831; cofactor 1"],
    ["Generator", "(625037715, 2100627251)"],
], [155, 355])
p("Implementation and evidence", "Heading2")
p("The production oracle solves S3 together with the Dickson factor-base constraints. It uses no pair table or generic DLP fallback. Rational extraction follows linear basis consequences or branches at quadratic roots; every branch is exposed in counters. Resource limits return undetermined, never a false mathematical negative.")
drawing = svg2rlg(str(HERE / "pipeline.svg"))
drawing.scale(0.51, 0.51); drawing.width *= .51; drawing.height *= .51
story.append(drawing)
p("The complete prime31 cost comparison remains unperformed. Development checks on that field are relation-oracle checks, not complete prime31 ECDLP recoveries. No cheaper complete solver or new exponent has been established.")

story.append(PageBreak())
p("Checks, prior leads and next experiments", "Title")
table([
    ["Evidence", "Scope and limit"],
    ["40 focused release tests", "37 reused engine tests plus 3 adapter tests. Exact prime31 positive/negative relation controls and one complete p1031 toy public-target solve. Producer-local, unsigned."],
    ["Public-input interface", "The solver receives Q coordinates. Known scalars are used outside solve only in correctness assertions. The zero-call prime31 test correctly reports censorship."],
    ["Full cost", "Not established. F4 elimination counters exclude symbolic work, hashing, allocation and some normalisation/field work; no GAE, S or rho ranking."],
    ["Execution admission", "No supported authenticated EKS/checkpoint route in this runtime. No paid benchmark launched; no run is represented as queued."],
], [155, 355])
p("Previous leads checked", "Heading2")
p("#4188 already contains a C++ signed-pair-table complete prime31 pipeline; retain it as the generic-decomposition control. #4077 supplies the m=8 whole-call gate, not a degree-only target. The October Dickson-torus study already tested p+1 fibres; this construction is prior work. The heat map records bounded negative/cost results for several small-root, digit, residual-walk and transfer presentations. Norm-rank deficiency alone does not demonstrate target-positive cheap relations.")
p("A concrete implementation issue: v2(p-1)=1 prevents a binary Kummer base of size 8/16/32 on this field. The existing nonsplit Dickson route uses v2(p+1)=31. Separately, the current packed F4 engine has four free-variable slots; a chained m=8 presentation requires six. This limit concerns that representation, not mathematical impossibility.")
table([
    ["Next experiment", "Discriminating test"],
    ["EXP-R31-A", "Complete natural-positive baseline at Dickson depths 8/9/10; independently encoded targets, all cold phases, matched strongest rho/BSGS, useful rank and censoring. Execution/accounting admission first."],
    ["EXP-R31-B", "Audit a six-free-variable m=8 representation. Measure full invocation cost and useful independent output; apply the prior gamma/delta gate before claiming solver advantage."],
    ["EXP-R31-C", "Apply #4228's certified paired-S3 preprocessing to natural systems. Charge construction/extraction; then test algebraic large-prime combination with useful-rank accounting."],
], [155, 355])
p("Sources and reproduction", "Heading2")
p('Full details, commands and source links: native/README.md and EXPERIMENTS.md in <link href="https://github.com/isogeny-labs/crypto/pull/4188" color="#216985">github.com/isogeny-labs/crypto/pull/4188</link>. Exact source modules and dependency lockfile are checked in. Test output: development-tests.txt. PCC-20261115-d73efd standing: unverified.', "SmallBody")
p('Primary sources read: Petit-Kosters-Messeng, PKC 2016, Algorithm 1 and Sections 3.1-3.4 (<link href="https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf">author PDF</link>); Diem 2011, Sections 1-2. The KB wrapper is unavailable; review used bounded repository/PR searches. No novelty or Lean verification claim.', "SmallBody")

def footer(canvas, doc):
    canvas.setStrokeColor(colors.HexColor("#c3cfda")); canvas.line(42, 35, 553, 35)
    canvas.setFont("DejaVu", 8); canvas.setFillColor(colors.HexColor("#5b6e80"))
    canvas.drawString(42, 23, "Isogeny Labs | native prime31 continuation | [pcc:unverified]")
    canvas.drawRightString(553, 23, str(doc.page))

SimpleDocTemplate(str(HERE / "REPORT.pdf"), pagesize=A4, leftMargin=42,
    rightMargin=42, topMargin=36, bottomMargin=46,
    title="Prime31 algebraic IC - native Rust continuation").build(story,
        onFirstPage=footer, onLaterPages=footer)
