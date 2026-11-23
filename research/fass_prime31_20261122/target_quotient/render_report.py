#!/usr/bin/env python3
"""Render retained target-quotient records and the final report.

This is presentation code only. It reads REPORT.md and the retained native
JSONL records; it does not compute ranks, identities, timings, or cryptography.
Run only after REPORT.md and the chosen evidence are final:
python3 render_report.py --records evidence/<retained-call>/qualifier.stdout.jsonl
"""

from __future__ import annotations

import argparse
import hashlib
import html
import json
import re
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.font_manager import FontProperties, findfont
from matplotlib.mathtext import MathTextParser
from matplotlib.patches import FancyArrowPatch, FancyBboxPatch
from PIL import Image as PILImage
from pypdf import PdfReader
from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER, TA_LEFT
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    Flowable,
    Image,
    KeepTogether,
    Paragraph,
    SimpleDocTemplate,
    Spacer,
    Table,
    TableStyle,
)


ROOT = Path(__file__).resolve().parent
REPORT = ROOT / "REPORT.md"
RECORDS = None
PLOT_CATEGORY = "public_scan_0"
FIGURES = ROOT / "figures"
PDF = ROOT / "REPORT.pdf"
RECEIPT = ROOT / "renderer-qa.json"
INK = "#183044"
TEAL = "#176D75"
MUTED = "#586774"
PALE = "#EDF4F5"
RULE = "#C4D1D6"
PAGE_W, PAGE_H = letter
MARGIN = 46
CONTENT_W = PAGE_W - 2 * MARGIN


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save_figure(fig, name: str) -> None:
    svg = FIGURES / f"{name}.svg"
    fig.savefig(svg, bbox_inches="tight", pad_inches=0.08)
    # Matplotlib can emit trailing whitespace in SVG path-data lines.
    svg.write_text("\n".join(line.rstrip(" \t") for line in svg.read_text().splitlines()) + "\n")
    png = FIGURES / f"{name}.png"
    fig.savefig(png, dpi=150, bbox_inches="tight", pad_inches=0.08)
    # Palette reduction is image layout/compression, not data transformation.
    with PILImage.open(png) as src:
        src.convert("RGB").quantize(colors=192).save(png, optimize=True)
    plt.close(fig)


def draw_structure() -> None:
    fig, ax = plt.subplots(figsize=(7.2, 3.9))
    fig.patch.set_facecolor("white")
    ax.set(xlim=(0, 1), ylim=(0, 1))
    ax.axis("off")

    def node(cx, cy, w, h, label, detail, proposed=False):
        ax.add_patch(FancyBboxPatch(
            (cx - w / 2, cy - h / 2), w, h,
            boxstyle="round,pad=0.008,rounding_size=0.014",
            edgecolor=RULE if not proposed else TEAL,
            facecolor=PALE if not proposed else "white",
            linewidth=1.0,
            linestyle="--" if proposed else "-",
        ))
        ax.text(cx, cy + 0.037, label, ha="center", va="center",
                fontsize=10.2, color=INK, fontweight="medium")
        ax.text(cx, cy - 0.037, detail, ha="center", va="center",
                fontsize=8.6, color=MUTED)

    def edge(start, stop, dashed=False):
        ax.add_patch(FancyArrowPatch(start, stop, arrowstyle="-|>",
                     mutation_scale=11, linewidth=1.05, color=MUTED,
                     linestyle="--" if dashed else "-"))

    ax.text(0.5, 0.965, "Three right-message choices for the same first norm H",
            ha="center", va="center", fontsize=11.2, color=INK)
    node(0.17, 0.77, 0.30, 0.19, r"Full message: $R_V$",
         "Degree 2d; H unchanged* ")
    node(0.50, 0.77, 0.30, 0.19, r"Fixed subset: $R_W$",
         "Degree 2k\nGeneric rank min(d, 2k)")
    node(0.83, 0.77, 0.30, 0.19, r"Rational part: $G_X$",
         "$\\gcd(P_V,t^p-t)$\nActual degree e")
    node(0.50, 0.45, 0.72, 0.16,
         "Canonical remainder K(x,t) in the chosen quotient",
         "The fixed-W statement does not certify each rational-part fixture")
    node(0.25, 0.18, 0.45, 0.16,
         "Coefficient rank = value rank",
         "Invertible evaluation bases\non a split, squarefree grid")
    node(0.75, 0.18, 0.45, 0.16,
         "Zero-support indicator rank",
         "Nonlinear test: 1 where K = 0,\notherwise 0")
    edge((0.17, 0.67), (0.33, 0.54))
    edge((0.50, 0.67), (0.50, 0.54))
    edge((0.83, 0.67), (0.67, 0.54))
    edge((0.38, 0.36), (0.25, 0.27))
    edge((0.62, 0.36), (0.75, 0.27), True)
    ax.text(0.02, 0.025, r"*Full message: $X^d\ne1$. Rank statements concern the specified representative.",
            ha="left", va="center", fontsize=8.4, color=MUTED)
    fig.subplots_adjust(left=0, right=1, top=1, bottom=0)
    save_figure(fig, "structure")


def draw_rank(records: list[dict]) -> list[dict]:
    selected = [r for r in records if r.get("kind") == "target_quotient"
                and r.get("fixture") == "prime31_project"
                and r.get("target_category") == PLOT_CATEGORY]
    if not selected:
        raise ValueError("No retained Prime31 target-quotient records for the chosen category")
    # Ordering and plotting retain the recorded values, without deriving ranks.
    selected.sort(key=lambda r: r["d"])
    positions = list(range(len(selected)))
    degrees = [r["d"] for r in selected]
    ranks = [r["rank_rational_remainder"] for r in selected]
    support_ranks = [r["boolean_support_rank_over_fp"] for r in selected]
    fig, ax = plt.subplots(figsize=(7.2, 3.95))
    fig.patch.set_facecolor("white")
    ax.bar([x - 0.18 for x in positions], ranks, width=0.34,
           color=TEAL, zorder=3, label="Canonical remainder / value rank")
    ax.bar([x + 0.18 for x in positions], support_ranks, width=0.34,
           color="#A65B2E", zorder=3, label="Boolean zero-support rank")
    ax.set_xticks(positions, [str(d) for d in degrees])
    ax.set_xlabel("Selector degree d (selected values)", labelpad=9, color=INK)
    ax.set_ylabel("Exact rank over F_p (dimension)", labelpad=10, color=INK)
    ax.set_ylim(0, max([1] + ranks + support_ranks) * 1.18)
    ax.yaxis.grid(True, color=RULE, linewidth=0.6, zorder=0)
    for spine in ["top", "right"]:
        ax.spines[spine].set_visible(False)
    for spine in ["left", "bottom"]:
        ax.spines[spine].set_color(RULE)
    ax.tick_params(colors=INK, labelsize=9)
    for position, rank in zip(positions, ranks):
        ax.annotate(str(rank), (position - 0.18, rank), xytext=(0, 4),
                    textcoords="offset points", ha="center", va="bottom",
                    fontsize=8.4, color=INK)
    for position, rank in zip(positions, support_ranks):
        ax.annotate(str(rank), (position + 0.18, rank), xytext=(0, 4),
                    textcoords="offset points", ha="center", va="bottom",
                    fontsize=8.4, color=INK)
    ax.set_title("Coefficient rank and zero-support rank", loc="left",
                 fontsize=13, color=INK, pad=28, fontweight="medium")
    ax.text(0, 1.055, f"Prime31 / {PLOT_CATEGORY} / {len(selected)} recorded targets; all selector rows",
            transform=ax.transAxes, ha="left", fontsize=9.4, color=MUTED)
    ax.legend(loc="upper left", frameon=False, fontsize=8.2)
    first = selected[0]
    fig.text(0.125, 0.013,
             f"p = {first['p']}     A = {first['a']}     B = {first['b']}     No timing plotted",
             fontsize=8.6, color=MUTED)
    fig.subplots_adjust(left=0.125, right=0.985, top=0.77, bottom=0.19)
    save_figure(fig, "rank")
    return selected


_font_names: dict[str, str] = {}


def register_font(path: str) -> str:
    if path not in _font_names:
        name = "MathFont" + str(len(_font_names))
        pdfmetrics.registerFont(TTFont(name, path))
        _font_names[path] = name
    return _font_names[path]


def normalize_math(source: str) -> tuple[str, str | None, bool]:
    tag_match = re.search(r"\\tag\{([^}]*)\}", source)
    tag = tag_match.group(1) if tag_match else None
    value = re.sub(r"\\tag\{[^}]*\}", "", source).strip()
    boxed = "\\boxed{" in value
    if boxed:
        start = value.index("\\boxed{")
        value = value[:start] + value[start + len("\\boxed{"):]
        # REPORT.md has one outer boxed expression; remove its matching tail.
        end = value.rfind("}")
        value = value[:end] + value[end + 1:]
    value = value.replace("\\mathbb F", "\\mathbb{F}")
    value = value.replace("\\mathcal A", "\\mathcal{A}")
    value = value.replace("\\mathsf T", "\\mathsf{T}")
    value = value.replace("\\widetilde R", "\\widetilde{R}")
    value = value.replace("\\mathbf1", "\\mathbf{1}")
    value = value.replace("\\bmod", "\\;\\mathrm{mod}\\;")
    value = value.replace("\\bigl", "").replace("\\bigr", "")
    value = re.sub(r"\\le(?![A-Za-z])", r"\\leq ", value)
    value = re.sub(r"\\text\{([^}]*)\}",
                   lambda m: "\\mathrm{" + m.group(1).replace(" ", "\\ ") + "}", value)
    return " ".join(value.split()), tag, boxed


class MathBlock(Flowable):
    """Vector math from matplotlib's mathtext path layout, with embedded fonts."""

    parser = MathTextParser("path")

    def __init__(self, source: str):
        super().__init__()
        self.expr, self.tag, self.boxed = normalize_math(source)
        self.layout = self.parser.parse("$" + self.expr + "$", dpi=72,
                                        prop=FontProperties(size=11))
        if self.layout.width > CONTENT_W - 45:
            size = 11 * (CONTENT_W - 45) / self.layout.width
            self.layout = self.parser.parse("$" + self.expr + "$", dpi=72,
                                            prop=FontProperties(size=size))
        for font, _, _, _, _ in self.layout.glyphs:
            register_font(font.fname)
        self.width = CONTENT_W
        self.height = float(self.layout.height) + 19

    def draw(self):
        canvas = self.canv
        ox = (CONTENT_W - self.layout.width) / 2
        baseline = 9 + self.layout.depth
        if self.boxed:
            canvas.setFillColor(colors.HexColor(PALE))
            canvas.roundRect(ox - 12, 2, self.layout.width + 24,
                             self.height - 3, 3, fill=1, stroke=0)
        canvas.setFillColor(colors.HexColor(INK))
        for font, size, codepoint, x, y in self.layout.glyphs:
            canvas.setFont(register_font(font.fname), size)
            canvas.drawString(ox + x, baseline + y, chr(codepoint))
        for x, y, width, height in self.layout.rects:
            canvas.rect(ox + x, baseline + y, width, height, fill=1, stroke=0)
        if self.tag:
            canvas.setFont("Body", 8.5)
            canvas.setFillColor(colors.HexColor(MUTED))
            canvas.drawRightString(CONTENT_W, baseline, f"({self.tag})")


def inline(source: str) -> str:
    value = html.escape(source, quote=False)
    value = re.sub(r"\[([^\]]+)\]\(([^)]+)\)",
                   lambda m: '<link href="' + html.escape(html.unescape(m.group(2)), quote=True)
                   + '" color="' + TEAL + '">' + m.group(1) + '</link>', value)
    value = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", value)
    value = re.sub(r"`([^`]+)`", r'<font name="Mono" size="8.7">\1</font>', value)
    return value


def setup_styles():
    for name, weight, style in [("Body", "normal", "normal"),
                                ("BodyBold", "bold", "normal"),
                                ("BodyItalic", "normal", "italic")]:
        pdfmetrics.registerFont(TTFont(name, findfont(FontProperties(
            family="DejaVu Sans", weight=weight, style=style))))
    pdfmetrics.registerFontFamily("Body", normal="Body", bold="BodyBold",
                                  italic="BodyItalic", boldItalic="BodyBold")
    pdfmetrics.registerFont(TTFont("Mono", findfont(FontProperties(family="DejaVu Sans Mono"))))
    styles = getSampleStyleSheet()
    styles.add(ParagraphStyle("ReportBody", fontName="Body", fontSize=9.3,
        leading=13.6, textColor=colors.HexColor(INK), spaceAfter=8,
        allowWidows=0, allowOrphans=0, splitLongWords=1))
    styles.add(ParagraphStyle("ReportTitle", parent=styles["ReportBody"],
        fontName="BodyBold", fontSize=22, leading=27, spaceAfter=12, keepWithNext=True))
    styles.add(ParagraphStyle("ReportH2", parent=styles["ReportBody"],
        fontName="BodyBold", fontSize=13.1, leading=17, spaceBefore=13,
        spaceAfter=8, keepWithNext=True))
    styles.add(ParagraphStyle("ReportH3", parent=styles["ReportBody"],
        fontName="BodyBold", fontSize=10.3, leading=14.6, spaceBefore=10,
        spaceAfter=6, keepWithNext=True))
    styles.add(ParagraphStyle("ReportCaption", parent=styles["ReportBody"],
        fontSize=8.1, leading=11.3, textColor=colors.HexColor(MUTED), spaceAfter=10))
    styles.add(ParagraphStyle("ReportTable", parent=styles["ReportBody"],
        fontSize=8.6, leading=12.2, spaceAfter=0))
    styles.add(ParagraphStyle("ReportBullet", parent=styles["ReportBody"],
        leftIndent=10, firstLineIndent=0, bulletIndent=0, spaceAfter=7))
    return styles


def page_chrome(canvas, doc):
    canvas.saveState()
    canvas.setStrokeColor(colors.HexColor(RULE))
    canvas.setLineWidth(0.6)
    canvas.line(MARGIN, PAGE_H - 38, PAGE_W - MARGIN, PAGE_H - 38)
    canvas.setFillColor(colors.HexColor(TEAL))
    canvas.setFont("BodyBold", 8)
    canvas.drawString(MARGIN, PAGE_H - 29, "ISOGENY LABS  /  TARGET-DEPENDENT QUOTIENT")
    canvas.setFillColor(colors.HexColor(MUTED))
    canvas.setFont("Body", 7.4)
    canvas.drawRightString(PAGE_W - MARGIN, PAGE_H - 29, "Research component  |  PR #4952")
    canvas.line(MARGIN, 35, PAGE_W - MARGIN, 35)
    canvas.drawString(MARGIN, 23, "Written derivation + native correctness evidence  |  [pcc:unverified]")
    canvas.drawRightString(PAGE_W - MARGIN, 23, str(doc.page))
    canvas.restoreState()


def markdown_story(source: str, styles):
    lines = source.splitlines()
    story = []
    i = 0
    math_count = 0
    sources_start = None
    while i < len(lines):
        line = lines[i].strip()
        if not line:
            i += 1
            continue
        if line == "$$":
            end = i + 1
            while end < len(lines) and lines[end].strip() != "$$":
                end += 1
            if end == len(lines):
                raise ValueError("Unclosed display math")
            story.append(MathBlock("\n".join(lines[i + 1:end])))
            math_count += 1
            i = end + 1
            continue
        image = re.fullmatch(r"!\[([^]]*)\]\(([^)]+)\)", line)
        if image:
            path = ROOT / image.group(2)
            png = path.with_suffix(".png")
            with PILImage.open(png) as img:
                ratio = img.height / img.width
            picture = Image(str(png), width=CONTENT_W, height=CONTENT_W * ratio)
            if path.stem == "structure":
                caption = ("Figure 1. Full, fixed-subset and rational squarefree messages are different moduli. "
                    "Coefficient rank and value-grid rank are linked by invertible evaluation maps; "
                    "the zero-support indicator is an entrywise nonlinear transformation.")
            else:
                caption = ("Figure 2. Exact retained target-quotient observations for the Prime31 "
                    f"{PLOT_CATEGORY} category, read from {RECORDS.relative_to(ROOT)}. "
                    "Both ranks use all selector rows; Boolean support is not signed relation output. "
                    "No untested degree, uncertainty interval or timing is plotted.")
            story.append(KeepTogether([picture, Spacer(1, 5),
                         Paragraph(inline(caption), styles["ReportCaption"])]))
            i += 1
            continue
        if line.startswith("|"):
            rows = []
            while i < len(lines) and lines[i].strip().startswith("|"):
                cells = [c.strip() for c in lines[i].strip().strip("|").split("|")]
                if not all(re.fullmatch(r":?-+:?", c) for c in cells):
                    rows.append(cells)
                i += 1
            data = [[Paragraph(inline(c if n else "**" + c + "**"), styles["ReportTable"])
                     for c in row] for n, row in enumerate(rows)]
            ncols = len(data[0])
            proportions = {2: [0.28, 0.72], 3: [0.44, 0.28, 0.28],
                           4: [0.34, 0.22, 0.22, 0.22],
                           5: [0.32, 0.17, 0.17, 0.17, 0.17],
                           6: [0.30, 0.065, 0.065, 0.22, 0.13, 0.22]}.get(ncols)
            if proportions is None:
                proportions = [1.0 / ncols] * ncols
            table = Table(data, colWidths=[CONTENT_W * v for v in proportions],
                          repeatRows=1, hAlign="LEFT")
            table.setStyle(TableStyle([
                ("VALIGN", (0, 0), (-1, -1), "TOP"),
                ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor(PALE)),
                ("LINEBELOW", (0, 0), (-1, 0), 0.7, colors.HexColor(RULE)),
                ("LINEBELOW", (0, 1), (-1, -1), 0.35, colors.HexColor(RULE)),
                ("LEFTPADDING", (0, 0), (-1, -1), 7),
                ("RIGHTPADDING", (0, 0), (-1, -1), 7),
                ("TOPPADDING", (0, 0), (-1, -1), 6),
                ("BOTTOMPADDING", (0, 0), (-1, -1), 6),
            ]))
            story.extend([table, Spacer(1, 9)])
            continue
        heading = re.match(r"^(#{1,3}) (.*)$", line)
        if heading:
            if heading.group(2) == "Sources and reproducibility":
                sources_start = len(story)
            style = {1: "ReportTitle", 2: "ReportH2", 3: "ReportH3"}[len(heading.group(1))]
            story.append(Paragraph(inline(heading.group(2)), styles[style]))
            i += 1
            continue
        if line.startswith("- "):
            story.append(Paragraph(inline(line[2:]), styles["ReportBullet"], bulletText="•"))
            i += 1
            continue
        paragraph = [line]
        i += 1
        while i < len(lines) and lines[i].strip() and not re.match(r"^(#|\$\$|!\[|\||- )", lines[i].strip()):
            paragraph.append(lines[i].strip())
            i += 1
        body = Paragraph(inline(" ".join(paragraph)), styles["ReportBody"])
        lookahead = i
        while lookahead < len(lines) and not lines[lookahead].strip():
            lookahead += 1
        if lookahead < len(lines) and lines[lookahead].strip() == "$$":
            body.keepWithNext = True
        story.append(body)
    if sources_start is not None:
        story = story[:sources_start] + [KeepTogether(story[sources_start:])]
    return story, math_count


def main() -> None:
    global RECORDS, PLOT_CATEGORY
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--records", required=True,
                        help="Retained JSONL path relative to this report directory")
    parser.add_argument("--target-category", default="public_scan_0")
    args = parser.parse_args()
    RECORDS = ROOT / args.records
    PLOT_CATEGORY = args.target_category
    if not REPORT.is_file() or not RECORDS.is_file():
        raise FileNotFoundError("Final REPORT.md and retained JSONL are required; no provisional rendering")
    FIGURES.mkdir(exist_ok=True)
    matplotlib.rcParams.update({"svg.fonttype": "none", "font.family": "DejaVu Sans",
                                "font.size": 10, "savefig.facecolor": "white"})
    records = [json.loads(line) for line in RECORDS.read_text().splitlines() if line.strip()]
    draw_structure()
    selected = draw_rank(records)
    styles = setup_styles()
    source = REPORT.read_text()
    story, equations = markdown_story(source, styles)
    doc = SimpleDocTemplate(str(PDF), pagesize=letter, rightMargin=MARGIN,
        leftMargin=MARGIN, topMargin=53, bottomMargin=48,
        title="Target-dependent quotient rank and zero support",
        author="Isogeny Labs", subject="Exact target-quotient structure and retained native checks",
        pageCompression=1)
    doc.build(story, onFirstPage=page_chrome, onLaterPages=page_chrome)
    reader = PdfReader(PDF)
    artifacts = [PDF, *sorted(FIGURES.glob("*.svg")), *sorted(FIGURES.glob("*.png"))]
    receipt = {
        "schema": "target-quotient-renderer-qa/v1",
        "scope": "document_layout_and_plotting_of_retained_records_only",
        "research_computation": False,
        "performance_measurement": False,
        "source_report": {"path": "REPORT.md", "sha256": sha(REPORT)},
        "graph_input": {"path": str(RECORDS.relative_to(ROOT)), "sha256": sha(RECORDS),
                        "filter": {"kind": "target_quotient", "fixture": "prime31_project",
                                   "target_category": PLOT_CATEGORY}},
        "plotted_values": [{k: r[k] for k in ("p", "a", "b", "d", "target_x", "target_y",
                            "rank_rational_remainder", "boolean_support_rank_over_fp")}
                           for r in selected],
        "renderer_source_sha256": sha(Path(__file__)),
        "page_count": len(reader.pages),
        "display_equations": equations,
        "all_report_markdown_blocks_consumed": True,
        "artifacts": [{"path": str(p.relative_to(ROOT)), "sha256": sha(p),
                       "size_bytes": p.stat().st_size} for p in artifacts],
        "visual_qa": {"status": "pending_render_and_inspection"},
    }
    for artifact in artifacts:
        limit = {".png": 65536, ".svg": 1048576, ".pdf": 10485760}[artifact.suffix]
        if artifact.stat().st_size >= limit:
            raise ValueError(f"Artifact exceeds its size ceiling: {artifact.name}")
    RECEIPT.write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({"pdf_pages": len(reader.pages), "equations": equations,
                      "pdf_bytes": PDF.stat().st_size, "rank_cells_plotted": len(selected)}))


if __name__ == "__main__":
    main()
