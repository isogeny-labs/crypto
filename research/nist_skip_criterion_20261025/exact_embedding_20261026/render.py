"""Plot recorded native indices and typeset the exact-order report; no research computation."""
from pathlib import Path
import csv
import html
import os
import re
os.environ.setdefault("MPLCONFIGDIR", "/tmp/nist-order-matplotlib")
os.environ.setdefault("XDG_CACHE_HOME", "/tmp/nist-order-document-cache")
Path(os.environ["XDG_CACHE_HOME"]).mkdir(parents=True, exist_ok=True)
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib.pagesizes import A4
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image
import fitz

here = Path(__file__).resolve().parent
(here / "reports").mkdir(exist_ok=True)
rows = list(csv.DictReader((here / "results/degrees.tsv").open(), delimiter="\t"))
plt.rcParams.update({"font.size": 8, "svg.fonttype": "none"})
fig, ax = plt.subplots(figsize=(7.5, 5.2))
for index, row in enumerate(rows):
    value = int(row["index_hex"], 16)  # Parse a recorded integer for display only.
    color = "#235b85" if row["role"] == "published" else "#be692e"
    ax.scatter(value, index, color=color, s=24, zorder=3)
    ax.annotate(row["index_hex"], (value, index), xytext=(5, 0),
                textcoords="offset points", va="center", fontsize=7)
ax.set_xscale("log")
ax.set_xlim(0.75, 200)
ax.set_xticks([1, 3, 10, 42, 100], ["0x1", "0x3", "0xA", "0x2A", "0x64"])
ax.set_yticks(range(len(rows)), [row["label"] for row in rows])
ax.invert_yaxis()
ax.axvline(100, color="#8a3d3d", linewidth=0.8, linestyle="--")
ax.set_xlabel("Recorded index c = (n-1)/k, logarithmic axis; hexadecimal labels")
ax.set_title("Exact embedding indices: published and skipped values overlap", loc="left", fontsize=11)
ax.legend(handles=[Patch(color="#235b85", label="Published"), Patch(color="#be692e", label="Skipped")],
          loc="lower right", frameon=False)
ax.grid(axis="x", color="#e5e9ed", linewidth=0.5, which="major")
ax.spines[["top", "right"]].set_visible(False)
fig.text(0.02, 0.01, "exact-001 | all21 verified rows | dashed line: modern c<0x64 bound | exact values, no sampling interval", fontsize=6.5)
fig.tight_layout(rect=(0, 0.025, 1, 1))
fig.savefig(here / "reports/indices.svg")
svg = here / "reports/indices.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(here / "reports/indices.png", dpi=150)
plt.close(fig)

styles = getSampleStyleSheet()
styles["Title"].fontSize = 16
styles["Title"].leading = 20
styles["BodyText"].fontSize = 9
styles["BodyText"].leading = 12
styles["BodyText"].spaceAfter = 7

def inline(text):
    text = html.escape(text)
    text = re.sub(r"`([^`]+)`", r'<font name="Courier">\1</font>', text)
    def link(match):
        target = html.unescape(match[2])
        if not target.startswith(("https://", "http://")):
            relative = (here / target).resolve().relative_to(here.parents[2])
            target = "https://github.com/isogeny-labs/crypto/blob/codex/nist-exact-embedding-20261026/" + relative.as_posix()
        return '<a href="' + html.escape(target, quote=True) + '" color="#235b85">' + match[1] + '</a>'
    return re.sub(r"\[([^]]+)\]\(([^)]+)\)", link, text)

story = []
for block in (here / "REPORT.md").read_text().strip().split("\n\n"):
    line = " ".join(block.splitlines())
    if line.startswith("# "):
        story.append(Paragraph(inline(line[2:]), styles["Title"]))
    elif line.startswith("!["):
        story.append(Image(str(here / "reports/indices.png"), width=510, height=353.6))
        story.append(Spacer(1, 6))
    else:
        story.append(Paragraph(inline(line), styles["BodyText"]))

def footer(canvas, doc):
    canvas.setFont("Helvetica", 7)
    canvas.setFillColor(colors.HexColor("#55616b"))
    canvas.drawString(35, 21, "Exact embedding degrees | exact-001 | protocol source 56d74aa19bd1 | 26 October2026")
    canvas.drawRightString(A4[0] - 35, 21, str(doc.page))

raw_pdf = here / "REPORT.rendering.pdf"
SimpleDocTemplate(str(raw_pdf), pagesize=A4, rightMargin=35, leftMargin=35, topMargin=30, bottomMargin=35).build(story, onFirstPage=footer, onLaterPages=footer)
doc = fitz.open(raw_pdf)
for name in ["PROTOCOL.md", "compute.cpp", "verify.cpp", "order_common.hpp", "results/degrees.tsv", "results/summary.json", "results/verification.json", "evidence/manifest.json", "evidence/delivery.json", "evidence/exact-001.tar.xz"]:
    file = here / name
    doc.embfile_add(file.name, file.read_bytes(), filename=file.name, desc=name)
doc.set_metadata({"title": "Exact embedding degree: two proposed filters fail", "subject": "All21 degrees certified; two fixed filters fail; historical criterion unresolved"})
doc.save(here / "REPORT.pdf", garbage=4, deflate=True)
doc.close()
raw_pdf.unlink()
