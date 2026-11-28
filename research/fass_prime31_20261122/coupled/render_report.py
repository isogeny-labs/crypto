#!/usr/bin/env python3
"""Render recorded coupled-norm research into figures and a PDF.

This is presentation code only. It reads REPORT.md and the retained native
JSONL records; it does not compute ranks, identities, timings, or cryptography.
Run from any directory: python3 path/to/coupled/render_report.py
"""

from __future__ import annotations

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
RECORDS = ROOT / "evidence/attempt2/qualifier.stdout.jsonl"
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
    fig.savefig(FIGURES / f"{name}.svg", bbox_inches="tight", pad_inches=0.08)
    png = FIGURES / f"{name}.png"
    fig.savefig(png, dpi=150, bbox_inches="tight", pad_inches=0.08)
    # Palette reduction is image layout/compression, not data transformation.
    with PILImage.open(png) as src:
        src.convert("RGB").quantize(colors=192).save(png, optimize=True)
    plt.close(fig)


def draw_structure() -> None:
    fig, ax = plt.subplots(figsize=(7.2, 3.65))
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
        ax.text(cx, cy + 0.022, label, ha="center", va="center",
                fontsize=10.7, color=INK, fontweight="medium")
        ax.text(cx, cy - 0.037, detail, ha="center", va="center",
                fontsize=9.3, color=MUTED)

    def edge(start, stop, dashed=False):
        ax.add_patch(FancyArrowPatch(start, stop, arrowstyle="-|>",
                     mutation_scale=11, linewidth=1.05, color=MUTED,
                     linestyle="--" if dashed else "-"))

    node(0.50, 0.88, 0.62, 0.13,
         "Coupled summation equation + selector", r"$S_3(x,z,t),\quad L(z)=z^d-1$")
    node(0.50, 0.65, 0.54, 0.13,
         "Eliminate the selector coordinate z", r"Exact partial norm $N(x,t)$")
    node(0.245, 0.39, 0.445, 0.16,
         "Canonical coefficient form H", r"Generic separation rank: $d$")
    node(0.755, 0.39, 0.445, 0.16,
         "Unreduced arithmetic circuit", r"Division-free size: $O(\log d)$")
    node(0.50, 0.12, 0.74, 0.15,
         "Remaining common-root query with the right message",
         "Proposed: retain the circuit and recover exact witnesses", proposed=True)
    edge((0.50, 0.81), (0.50, 0.72))
    edge((0.40, 0.58), (0.245, 0.48))
    edge((0.60, 0.58), (0.755, 0.48))
    edge((0.245, 0.30), (0.37, 0.20), True)
    edge((0.755, 0.30), (0.63, 0.20), True)
    fig.subplots_adjust(left=0, right=1, top=1, bottom=0)
    save_figure(fig, "structure")


def draw_rank(records: list[dict]) -> list[dict]:
    selected = [r for r in records if r.get("kind") == "coefficient_rank"
                and r.get("fixture") == "prime31_project"]
    if not selected:
        raise ValueError("No retained prime31_project rank records found")
    # Ordering and plotting retain the recorded values, without deriving ranks.
    selected.sort(key=lambda r: r["d"])
    positions = list(range(len(selected)))
    degrees = [r["d"] for r in selected]
    ranks = [r["rank"] for r in selected]
    fig, ax = plt.subplots(figsize=(7.2, 3.65))
    fig.patch.set_facecolor("white")
    ax.bar(positions, ranks, width=0.60, color=TEAL, zorder=3)
    ax.set_xticks(positions, [str(d) for d in degrees])
    ax.set_xlabel("Selector degree d (selected values)", labelpad=9, color=INK)
    ax.set_ylabel("Exact coefficient rank (dimension)", labelpad=10, color=INK)
    ax.set_ylim(0, max(ranks) * 1.18)
    ax.yaxis.grid(True, color=RULE, linewidth=0.6, zorder=0)
    for spine in ["top", "right"]:
        ax.spines[spine].set_visible(False)
    for spine in ["left", "bottom"]:
        ax.spines[spine].set_color(RULE)
    ax.tick_params(colors=INK, labelsize=9)
    for position, rank in zip(positions, ranks):
        ax.annotate(str(rank), (position, rank), xytext=(0, 4),
                    textcoords="offset points", ha="center", va="bottom",
                    fontsize=9, color=INK)
    ax.set_title("Selected Prime31 coefficient ranks", loc="left",
                 fontsize=13, color=INK, pad=28, fontweight="medium")
    ax.text(0, 1.055, f"{len(selected)} exact recorded cells; no timing or uncertainty estimate",
            transform=ax.transAxes, ha="left", fontsize=9.4, color=MUTED)
    first = selected[0]
    fig.text(0.125, 0.013,
             f"p = {first['p']}     A = {first['a']}     B = {first['b']}",
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
    canvas.drawString(MARGIN, PAGE_H - 29, "ISOGENY LABS  /  COUPLED S3 ELIMINATION")
    canvas.setFillColor(colors.HexColor(MUTED))
    canvas.setFont("Body", 7.4)
    canvas.drawRightString(PAGE_W - MARGIN, PAGE_H - 29, "Research component  |  PR #4921")
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
                caption = ("Figure 1. The two exact representations after the first elimination. "
                    "Solid branches summarize written identities; the dashed common-root step is proposed work.")
            else:
                caption = ("Figure 2. Recorded coefficient ranks for the twelve selected Prime31 project-fixture "
                    "degrees in evidence/attempt2/qualifier.stdout.jsonl. Each bar is one exact cell. "
                    "No untested degree, sampling interval, or timing is plotted.")
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
            table = Table(data, colWidths=[CONTENT_W * 0.56, CONTENT_W * 0.44],
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
        title="What survives the first coupled S3 elimination?",
        author="Isogeny Labs", subject="Partial-norm coefficient rank and exact native checks",
        pageCompression=1)
    doc.build(story, onFirstPage=page_chrome, onLaterPages=page_chrome)
    reader = PdfReader(PDF)
    artifacts = [PDF, *sorted(FIGURES.glob("*.svg")), *sorted(FIGURES.glob("*.png"))]
    receipt = {
        "schema": "coupled-renderer-qa/v1",
        "scope": "document_layout_and_plotting_of_retained_records_only",
        "research_computation": False,
        "performance_measurement": False,
        "source_report": {"path": "REPORT.md", "sha256": sha(REPORT)},
        "graph_input": {"path": str(RECORDS.relative_to(ROOT)), "sha256": sha(RECORDS),
                        "filter": {"kind": "coefficient_rank", "fixture": "prime31_project"}},
        "plotted_values": [{k: r[k] for k in ("p", "a", "b", "d", "rank")} for r in selected],
        "renderer_source_sha256": sha(Path(__file__)),
        "page_count": len(reader.pages),
        "display_equations": equations,
        "all_report_markdown_blocks_consumed": True,
        "artifacts": [{"path": str(p.relative_to(ROOT)), "sha256": sha(p),
                       "size_bytes": p.stat().st_size} for p in artifacts],
        "visual_qa": {"status": "pending_render_and_inspection"},
    }
    if any(p.stat().st_size >= 1048576 for p in artifacts):
        raise ValueError("An artifact exceeds the 1 MiB ceiling")
    RECEIPT.write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({"pdf_pages": len(reader.pages), "equations": equations,
                      "pdf_bytes": PDF.stat().st_size, "rank_cells_plotted": len(selected)}))


if __name__ == "__main__":
    main()
