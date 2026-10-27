"""Plot recorded native values and typeset the exact-replay report only."""
from pathlib import Path
import csv
import html
import os
import re
os.environ.setdefault("MPLCONFIGDIR", "/tmp/nist-sea-matplotlib")
os.environ.setdefault("XDG_CACHE_HOME", "/tmp/nist-sea-document-cache")
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
rows = list(csv.DictReader((here / "results/coverage.tsv").open(), delimiter="\t"))
plt.rcParams.update({"font.size": 8, "svg.fonttype": "none"})
fig, ax = plt.subplots(figsize=(7.5, 4.8))
for index, row in enumerate(rows):
    value = int(row["replayed_lmax"])
    color = "#235b85" if row["role"] == "published" else "#be692e"
    ax.barh(index, value, color=color, height=0.68)
    ax.text(value + 5, index, row["replayed_lmax"], va="center", fontsize=7)
ax.set_yticks(range(len(rows)), [row["label"] for row in rows])
ax.invert_yaxis()
ax.set_xlim(0, 470)
ax.set_xlabel("Last odd prime in the recorded coverage proxy (decimal)")
ax.set_title("All 21 native coverage values replay exactly", loc="left", fontsize=12)
ax.legend(handles=[Patch(color="#235b85", label="Published"), Patch(color="#be692e", label="Skipped")], loc="upper right", frameon=False)
ax.grid(axis="x", color="#e5e9ed", linewidth=0.5)
ax.set_axisbelow(True)
ax.spines[["top", "right"]].set_visible(False)
fig.text(0.02, 0.01, "replay-001 | all 21 rows | ramified primes included; 2 and Atkin information omitted | exact values, no sampling interval", fontsize=6.5)
fig.tight_layout(rect=(0, 0.025, 1, 1))
fig.savefig(here / "reports/coverage.svg")
svg = here / "reports/coverage.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(here / "reports/coverage.png", dpi=150)
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
            target = "https://github.com/isogeny-labs/crypto/blob/codex/nist-skip-criterion-20261025/" + relative.as_posix()
        return '<a href="' + html.escape(target, quote=True) + '" color="#235b85">' + match[1] + '</a>'
    text = re.sub(r"\[([^]]+)\]\(([^)]+)\)", link, text)
    return text

story = []
for block in (here / "REPORT.md").read_text().strip().split("\n\n"):
    line = " ".join(block.splitlines())
    if line.startswith("# "):
        story.append(Paragraph(inline(line[2:]), styles["Title"]))
    elif line.startswith("!["):
        story.append(Image(str(here / "reports/coverage.png"), width=510, height=326.4))
        story.append(Spacer(1, 6))
    else:
        story.append(Paragraph(inline(line), styles["BodyText"]))

def footer(canvas, doc):
    canvas.setFont("Helvetica", 7)
    canvas.setFillColor(colors.HexColor("#55616b"))
    canvas.drawString(35, 21, "Exact source/data replay | replay-001 | protocol source e028a12d0b43 | 26 October 2026")
    canvas.drawRightString(A4[0] - 35, 21, str(doc.page))

raw_pdf = here / "REPORT.rendering.pdf"
SimpleDocTemplate(str(raw_pdf), pagesize=A4, rightMargin=35, leftMargin=35, topMargin=30, bottomMargin=35).build(story, onFirstPage=footer, onLaterPages=footer)
doc = fitz.open(raw_pdf)
for name in ["PROTOCOL.md", "replay.cpp", "REFERENCES.json", "results/coverage.tsv", "results/summary.json", "evidence/manifest.json", "evidence/replay-001.tar.xz"]:
    file = here / name
    doc.embfile_add(file.name, file.read_bytes(), filename=file.name, desc=name)
doc.set_metadata({"title": "Exact replay of the SEA coverage proxy", "subject": "All21 recorded values replay; no historical-selector, statistical-unusualness or performance claim"})
doc.save(here / "REPORT.pdf", garbage=4, deflate=True)
doc.close()
raw_pdf.unlink()
