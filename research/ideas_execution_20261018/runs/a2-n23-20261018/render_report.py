"""Render existing native receipts only; no solver or result computation."""
import html
import json
import os
from pathlib import Path
os.environ.setdefault("MPLCONFIGDIR", "/tmp/ideas-a2-matplotlib")
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib import font_manager
from reportlab.lib import colors
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image, Table, TableStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

here = Path(__file__).resolve().parent
raw = json.loads((here / "a2_results_execution-20261018.json").read_text())
manifest = json.loads((here / "manifest.json").read_text())
cells = raw["solver_cells"]
fig, axes = plt.subplots(1, 2, figsize=(11, 4.4), constrained_layout=True,
                         gridspec_kw={"width_ratios": [1.5, 1]})
labels = []
for i, cell in enumerate(cells):
    spec, summary = cell["spec"], cell["summary"]
    label = f"K{spec['a']} / w{spec['w']} / {spec['arm']}"
    labels.append(label)
    if cell["timed_out"]:
        color, detail = "#ba593f", "cell timeout; 0 target records"
    elif cell["unsupported"]:
        color, detail = "#8c79a8", f"{cell['unsupported']} unsupported targets"
    else:
        color, detail = "#3d7793", f"{summary['targets']} records; D unresolved"
    axes[0].barh(i, spec["targets"], color=color, alpha=.8)
    axes[0].text(.4, i, detail, va="center", fontsize=8, color="white")
axes[0].set_yticks(range(len(cells)), labels, fontsize=8)
axes[0].invert_yaxis()
axes[0].set_xlim(0, 17)
axes[0].set_xlabel("Requested target slots per cell (not solved targets)")
axes[0].set_title("Five cells: 2 timeout / 2 completed / 1 unsupported")
completed = [cell for cell in cells if not cell["timed_out"] and not cell["unsupported"]]
for i, cell in enumerate(completed):
    summary = cell["summary"]
    axes[1].bar(i, summary["f4_mean_ms"], color="#3d7793", alpha=.8)
    axes[1].plot(i, summary["f4_median_ms"], "_", color="black", markersize=18)
    axes[1].text(i, summary["f4_mean_ms"], f"{summary['f4_mean_ms']:.1f}", ha="center", va="bottom", fontsize=9)
axes[1].set_xticks(range(len(completed)), [f"K{cell['spec']['a']} ONB\n16 targets; 46 vars" for cell in completed])
axes[1].set_ylabel("Recorded per-target F4 time (ms)")
axes[1].set_title("Completed cells only\nmean bars / median ticks", fontsize=10)
axes[1].grid(axis="y", alpha=.2)
fig.suptitle("n=23 native solver attempt — a2-n23-20261018; shared-host diagnostics", fontsize=12)
fig.savefig(here / "a2-solver-status.svg")
fig.savefig(here / "a2-solver-status.png", dpi=150)

styles = getSampleStyleSheet()
pdfmetrics.registerFont(TTFont("DejaVuSans", font_manager.findfont("DejaVu Sans")))
pdfmetrics.registerFont(TTFont("DejaVuSans-Bold", font_manager.findfont(font_manager.FontProperties(family="DejaVu Sans", weight="bold"))))
styles["BodyText"].fontName = "DejaVuSans"
styles["Title"].fontName = "DejaVuSans-Bold"
styles["BodyText"].fontSize = 9
styles["BodyText"].leading = 11
story = []
for block in (here / "REPORT.md").read_text().split("\n\n"):
    if block.startswith("| Cell"):
        rows = [["Cell", "Records", "Outcome / degree"]]
        for cell in cells:
            spec, summary = cell["spec"], cell["summary"]
            label = f"K{spec['a']}/w{spec['w']}/{spec['arm']}"
            outcome = "150 s cell timeout; unresolved" if cell["timed_out"] else f"{cell['unsupported']} unsupported; unmeasured" if cell["unsupported"] else "Completed; D unresolved, reached 3"
            rows.append([label, str(summary["targets"]), outcome])
        table = Table(rows, colWidths=[185, 45, 260], repeatRows=1)
        table.setStyle(TableStyle([("BACKGROUND", (0,0),(-1,0), colors.HexColor("#e7edf2")),("FONTNAME",(0,0),(-1,-1),"DejaVuSans"),("FONTNAME",(0,0),(-1,0),"DejaVuSans-Bold"),("FONTSIZE",(0,0),(-1,-1),8),("BOTTOMPADDING",(0,0),(-1,-1),7),("TOPPADDING",(0,0),(-1,-1),7),("GRID",(0,0),(-1,-1),.3,colors.lightgrey)]))
        story += [table, Spacer(1, 10), Image(str(here/"a2-solver-status.png"), width=490, height=196)]
        story += [Paragraph("Figure: requested slots by cell status; the 32 completed records retain unresolved solving degree. Timed-out/unsupported cells have no per-target timing plotted. Only recorded means/medians appear; no uncertainty interval or speed ratio is inferred. Both completed inputs have 46 variables, 3,567 equations and input degree 3.",styles["BodyText"]),Spacer(1,10)]
    elif block.startswith("# "):
        story += [Paragraph(html.escape(block[2:]), styles["Title"]), Spacer(1,10)]
    else:
        story += [Paragraph(html.escape(block).replace("\n"," "), styles["BodyText"]), Spacer(1,9)]
story += [Paragraph("Provenance: manifest.json, a2_results_execution-20261018.json, input-sha256.txt, build-repairs.patch and original stdout/stderr remain unchanged. Exact argv, source base and working revision are in manifest.json. Seed 20261012; 16 requested targets/cell; 150-second cell timeout; node budget 1024. Initial wrapper failure exited 127 before scientific execution. No GPU, WDSat or solving-degree resolution is credited.",styles["BodyText"])]
SimpleDocTemplate(str(here/"REPORT.pdf"),pagesize=A4,rightMargin=45,leftMargin=45,topMargin=42,bottomMargin=42).build(story)
