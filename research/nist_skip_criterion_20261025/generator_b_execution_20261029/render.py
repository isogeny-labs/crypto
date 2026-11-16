"""Document layout and display of recorded values only; no curve computation."""
import html
import json
import os
import re
import shutil
from pathlib import Path

os.environ.setdefault("MPLCONFIGDIR", "/tmp/nist-generator-matplotlib")
os.environ.setdefault("XDG_CACHE_HOME", "/tmp/nist-generator-render-cache")
import fitz
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams.update({"svg.fonttype": "none", "svg.hashsalt": "nist-gb-20261029"})
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
from reportlab import rl_config
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import Image, PageBreak, Paragraph, SimpleDocTemplate, Spacer, Table, TableStyle

rl_config.invariant = 1
BASE = Path(__file__).resolve().parent
REPO = BASE.parents[2]
INSPECTION = Path("/workspace/nist-generator-execution-document-inspection")
DATA = json.loads((BASE / "results/figure-data.json").read_text())
RUNTIME = json.loads((BASE / "evidence/preflight-runtime.json").read_text())
DIAGNOSTICS = json.loads((BASE / "evidence/diagnostics/observation.json").read_text())


def save(fig, target, display):
    target.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(target.with_suffix(".svg"), metadata={"Date": "2026-10-29"})
    svg = target.with_suffix(".svg")
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(target.with_suffix(".png"), dpi=150)
    target.with_suffix(".json").write_text(json.dumps(display, indent=2) + "\n")
    plt.close(fig)


def diagram(target, boxes, caption):
    fig, ax = plt.subplots(figsize=(7.5, 1.7))
    ax.set(xlim=(0, 720), ylim=(164, 0))
    ax.axis("off")
    for i, (title, lines, status) in enumerate(boxes):
        x = 8 + i * 238
        edge, fill = ("#a56b2e", "#fff3e5") if status != "verified" else ("#386d87", "#eef5fa")
        ax.add_patch(FancyBboxPatch((x, 6), 226, 119, boxstyle="round,pad=0.1,rounding_size=6",
                                   facecolor=fill, edgecolor=edge, linewidth=.8))
        ax.text(x + 10, 30, title, fontsize=9.1, weight="bold", color="#18394d")
        for j, line in enumerate(lines):
            ax.text(x + 10, 52 + j * 19, line, fontsize=7.8, color="#263e4d")
    ax.text(9, 151, caption, fontsize=7.1, color="#4c6070")
    fig.subplots_adjust(left=0, right=1, top=1, bottom=0)
    save(fig, target, {"kind": "recorded-evidence-diagram/v1", "boxes": boxes, "caption": caption})


# Select the one displayed view; the saved native data also retain the matching second view.
p224 = [row for row in DATA["p224"] if row["view"] == "historical"]
fig, ax = plt.subplots(figsize=(7.4, 2.35))
labels = [row["label"].replace("p224-", "Index ") + " · " + row["role"] for row in p224]
values = [int(row["canonical_x"]) for row in p224]
palette = ["#256b91" if row["role"] == "published" else "#798e52" for row in p224]
ax.barh(labels, values, color=palette, height=.52)
ax.scatter(values, range(len(values)), color=palette, s=35, zorder=3)
for i, value in enumerate(values):
    ax.text(value + .08, i, str(value), va="center", fontsize=10, weight="bold")
ax.set(xlim=(-.1, 3.65), xticks=[0, 1, 2, 3], xlabel="Canonical Q x-coordinate · integer representative")
ax.invert_yaxis()
ax.spines[["top", "right"]].set_visible(False)
fig.text(.02, .02, "Attempt 002 · 4 P-224 choices · both coefficient views agree · exact values, no sampling interval", fontsize=8)
fig.subplots_adjust(left=.29, right=.97, top=.96, bottom=.30)
save(fig, BASE / "p224-canonical-x", {"source": "results/figure-data.json", "rows": p224,
                                     "second_view_agrees": True, "independent_curve_count": 4})

diagram(BASE / "evidence-boundary", [
    ("Verified finite inventory", ["21 original choices retained", "100,548 point-trait cells", "All declared cells checked"], "verified"),
    ("Fixed tests", ["0 / 19,152 single separators", "0 / 880 scalar recipe matches", "11 historical generators NA"], "verified"),
    ("Historical / attack claim", ["Original decision trace absent", "No supported historical rule", "Neither weakness established"], "unresolved")],
    "Actual attempt 002 · Controls are repeated representations, not new historical samples")

diagram(BASE / "reports/attempt-001/visual", [
    ("Positive control", ["Attempted; status 0", "Native tiny known-answer test", "Completed"], "verified"),
    ("Full production", ["Attempted; status 124", "300-second watchdog", "0 point-trait rows written"], "timeout"),
    ("Independent verification", ["Not attempted after timeout", "No execution status invented", "Partial output unadmitted"], "not_attempted")],
    "Attempt 001 · Frozen protocol 1 · 768 MiB address cap · 3 phases planned, 2 attempted, 1 completed")

v = DATA["verification"]
count_rows = [("Point-trait cells", v["trait_cells_checked"]),
              ("Point differences", v["point_differences_checked"]),
              ("Inherited differences", v["inherited_differences_checked"]),
              ("Fixed envelopes", v["cutoffs_checked"])]
fig, ax = plt.subplots(figsize=(7.4, 2.45))
ax.barh([x[0] for x in count_rows], [x[1] for x in count_rows], color="#2c7394", height=.58)
for i, (_, value) in enumerate(count_rows):
    ax.text(value + 1700, i, f"{value:,}", va="center", fontsize=9)
ax.set(xlim=(0, 145000), xlabel="Declared cells independently checked · count")
ax.invert_yaxis()
ax.ticklabel_format(style="plain", axis="x")
ax.spines[["top", "right"]].set_visible(False)
fig.text(.02, .02, "Attempt 002 · all 21 choices · deterministic coverage; categories are different record types", fontsize=8)
fig.subplots_adjust(left=.25, right=.97, top=.96, bottom=.30)
save(fig, BASE / "reports/attempt-002/visual", {"source": "results/verification.json", "rows": count_rows})

diagnostic_rows = [
    ("001 original · 768 MiB", "known_p192_default_768MiB"),
    ("002 APRCL · 768 MiB", "known_p192_aprcl_768MiB"),
    ("003 original + input print · 1536 MiB", "known_p192_default_1536MiB_with_repr"),
    ("004 exact 001 binary · 1536 MiB", "same_original_binary_1536MiB"),
    ("All 21 known p/n pairs · 1536 MiB", "all21_known_pn_pairs_1536MiB")]
fig, ax = plt.subplots(figsize=(7.4, 2.7))
ax.set(xlim=(0, 1), ylim=(4.6, -.6))
ax.axis("off")
for i, (label, key) in enumerate(diagnostic_rows):
    ok = DIAGNOSTICS[key] == 0
    ax.text(.02, i, label, va="center", fontsize=9)
    ax.text(.73, i, "0 · completed" if ok else "124 · timed out", va="center", fontsize=9,
            color="#26764b" if ok else "#aa682b", weight="bold")
fig.text(.02, .02, "Five recorded diagnostic invocations · no point-trait run, performance ratio or historical label", fontsize=8)
fig.subplots_adjust(left=0, right=1, top=.96, bottom=.12)
save(fig, BASE / "reports/diagnostics/visual", {"source": "evidence/diagnostics/observation.json",
                                             "rows": [{"label": a, "status": DIAGNOSTICS[b]} for a, b in diagnostic_rows]})

seconds = RUNTIME["observed_seconds"]
fig, ax = plt.subplots(figsize=(7.4, 2.45))
ax.barh(["Positive", "Synthetic producer", "Synthetic verifier"], list(seconds.values()), color="#687f93", height=.55)
for i, value in enumerate(seconds.values()):
    ax.text(value + .007, i, f"{value:.3f} s", va="center", fontsize=9)
ax.set(xlim=(0, .56), xlabel="Observed elapsed seconds · shared host, unadmitted planning data")
ax.invert_yaxis()
ax.spines[["top", "right"]].set_visible(False)
fig.text(.02, .02, "Synthetic preflight 001 · n = 1 / phase · resolution 0.001 s · no measured spread or NIST timing", fontsize=8)
fig.subplots_adjust(left=.26, right=.97, top=.96, bottom=.30)
save(fig, BASE / "reports/preflight/visual", {"source": "evidence/preflight-runtime.json", "seconds": seconds,
                                           "n_per_phase": 1, "resolution_seconds": .001, "admitted_performance": False})

for name, font in [("DejaVu", "DejaVuSans.ttf"), ("DejaVu-Bold", "DejaVuSans-Bold.ttf")]:
    pdfmetrics.registerFont(TTFont(name, "/usr/share/fonts/truetype/dejavu/" + font))
pdfmetrics.registerFontFamily("DejaVu", normal="DejaVu", bold="DejaVu-Bold", italic="DejaVu", boldItalic="DejaVu-Bold")
styles = getSampleStyleSheet()
for key in ["Normal", "Title", "Heading2"]:
    styles[key].fontName = "DejaVu"
    styles[key].textColor = colors.HexColor("#1c3547")
styles["Normal"].fontSize = 8.8
styles["Normal"].leading = 12.3
styles["Normal"].spaceAfter = 6.5
styles["Title"].fontSize = 17
styles["Title"].leading = 21
styles["Title"].spaceAfter = 13
styles["Heading2"].fontSize = 11.5
styles["Heading2"].leading = 15
styles["Heading2"].spaceBefore = 8
styles["Heading2"].spaceAfter = 6
cell_style = ParagraphStyle("Cell", parent=styles["Normal"], fontSize=8, leading=10.5, spaceAfter=1)


def fmt(text, md):
    text = html.escape(text)
    def link(match):
        label, dest = match.group(1), html.unescape(match.group(2))
        if not re.match(r"[a-z]+:", dest):
            dest = "https://github.com/isogeny-labs/crypto/blob/codex/nist-period-ecpp-20261026/" + (md.parent / dest).resolve().relative_to(REPO).as_posix()
        return '<link href="' + html.escape(dest, quote=True) + '" color="#226e91">' + label + '</link>'
    text = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", link, text)
    text = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", text)
    return re.sub(r"`([^`]+)`", r"\1", text)


def render(md, output, label):
    flow, paragraphs, rows = [], [], []
    def flush():
        if paragraphs:
            flow.append(Paragraph(fmt(" ".join(paragraphs), md), styles["Normal"]))
            paragraphs.clear()
    def flush_table():
        if not rows:
            return
        cells = [[Paragraph(fmt(x.strip(), md), cell_style) for x in r.strip("|").split("|")]
                 for r in rows if not re.match(r"^\|[\s:|\-]+\|$", r)]
        table = Table(cells, colWidths=[146, 354], repeatRows=1, hAlign="LEFT")
        table.setStyle(TableStyle([("VALIGN", (0, 0), (-1, -1), "TOP"),
                                   ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#e9f1f5")),
                                   ("GRID", (0, 0), (-1, -1), .35, colors.HexColor("#bfcfda")),
                                   ("LEFTPADDING", (0, 0), (-1, -1), 6),
                                   ("RIGHTPADDING", (0, 0), (-1, -1), 6),
                                   ("TOPPADDING", (0, 0), (-1, -1), 4),
                                   ("BOTTOMPADDING", (0, 0), (-1, -1), 4)]))
        flow.extend([table, Spacer(1, 7)])
        rows.clear()
    for line in md.read_text().splitlines():
        if line.startswith("|"):
            flush()
            rows.append(line)
            continue
        flush_table()
        if not line.strip():
            flush()
        elif line == "<!-- pagebreak -->":
            flush()
            flow.append(PageBreak())
        elif line.startswith("!["):
            flush()
            match = re.fullmatch(r"!\[([^\]]+)\]\(([^)]+)\)", line)
            image = Image(str(md.parent / match.group(2)))
            image.drawHeight *= 500 / image.drawWidth
            image.drawWidth = 500
            flow.extend([Spacer(1, 5), image])
        elif line.startswith("# "):
            flush()
            flow.append(Paragraph(fmt(line[2:], md), styles["Title"]))
        elif line.startswith("## "):
            flush()
            flow.append(Paragraph(fmt(line[3:], md), styles["Heading2"]))
        else:
            paragraphs.append(line)
    flush()
    flush_table()
    def footer(canvas, doc):
        canvas.setFont("DejaVu", 7)
        canvas.setFillColor(colors.HexColor("#5b6c79"))
        canvas.drawString(45, 24, label + " · 2026-10-29")
        canvas.drawRightString(550, 24, str(doc.page))
    SimpleDocTemplate(str(output), pagesize=(595.28, 841.89), leftMargin=45, rightMargin=45,
                      topMargin=36, bottomMargin=42, title=md.read_text().splitlines()[0][2:],
                      author="Curve research").build(flow, onFirstPage=footer, onLaterPages=footer)
    name = output.relative_to(BASE).as_posix().replace("/", "--").replace(".pdf", "")
    folder = INSPECTION / name
    folder.mkdir(parents=True, exist_ok=True)
    doc = fitz.open(output)
    for i, page in enumerate(doc):
        page.get_pixmap(matrix=fitz.Matrix(1.45, 1.45)).save(folder / f"page-{i+1}.png")
    text = "\n".join(page.get_text() for page in doc)
    (folder / "readback.txt").write_text(text)
    print(json.dumps({"pdf": str(output), "pages": len(doc), "bytes": output.stat().st_size,
                      "text_chars": len(text), "inspection": str(folder)}))


render(BASE / "REPORT.md", BASE / "REPORT.pdf", "Verified finite comparison; historical selector unresolved")
for name in ["attempt-001", "attempt-002", "diagnostics", "preflight"]:
    directory = BASE / "reports" / name
    render(directory / "REPORT.md", directory / "REPORT.pdf", "NIST generator study · " + name)

# Browsing copies of the documentary planning observation, outside the admitted results table.
browse = REPO / "benchmarks/20261029-generator-preflight"
browse.mkdir(parents=True, exist_ok=True)
for name in ["REPORT.pdf", "visual.svg", "visual.png", "visual.json"]:
    shutil.copyfile(BASE / "reports/preflight" / name, browse / name)
