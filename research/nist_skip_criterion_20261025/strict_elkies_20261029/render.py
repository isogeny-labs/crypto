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
matplotlib.rcParams.update({"svg.fonttype": "none", "svg.hashsalt": "nist-strict-elkies-20261029"})
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
INSPECTION = Path("/workspace/nist-strict-elkies-document-inspection")
DATA = json.loads((BASE / "results/figure-data.json").read_text())
RUNTIME = json.loads((BASE / "evidence/preflight-runtime.json").read_text())


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


# Display recorded native rows only. No character or label arithmetic.
rows = DATA['rows']
fig, ax = plt.subplots(figsize=(7.4, 4.05))
labels = [r['label'] + (' · no skip control' if int(r['bits']) in [384,521] else '') for r in rows]
values = [int(r['strict691']) for r in rows]
palette = ['#256b91' if r['role']=='published' else '#798e52' for r in rows]
ax.barh(labels, values, color=palette, height=.64)
for i,value in enumerate(values): ax.text(value+.6,i,str(value),va='center',fontsize=7.8)
ax.set(xlim=(0,80),xlabel='Strict splitting count · odd primes ≤ 691 · character +1 only')
ax.tick_params(axis='y',labelsize=7.4)
ax.invert_yaxis(); ax.spines[['top','right']].set_visible(False)
from matplotlib.patches import Patch
ax.legend(handles=[Patch(color='#256b91',label='Published'),Patch(color='#798e52',label='Omitted')],loc='lower right',bbox_to_anchor=(1,1.005),ncol=2,fontsize=7.5,frameon=False)
fig.text(.02,.015,'Attempt 001 · all 21 choices · deterministic exact counts; P-384/P-521 have no omitted controls',fontsize=7.2)
fig.subplots_adjust(left=.31,right=.96,top=.93,bottom=.15)
save(fig,BASE/'strict-counts',{'source':'results/figure-data.json','rows':rows,'uncertainty':'none: exact census'})
diagram(BASE/'evidence-boundary',[
 ('Verified empirical fit',['8 published vs 11 omitted','2 singleton controls retained','Strict +1 character only'],'verified'),
 ('Declared calibration',['23 / 4,480 labelings','Search over 303 cutoff positions','Already visible; no holdout'],'verified'),
 ('Historical / attack claim',['Original decisions unknown','No authenticated selector','Neither weakness established'],'unresolved')],
 'Externally reported pattern independently replayed · no new historical attribution or security claim')
seconds = RUNTIME['seconds']
fig,ax=plt.subplots(figsize=(7.4,2.4))
ax.barh(['Positive self-test','Producer self-test','Verifier self-test'],list(seconds.values()),color='#687f93',height=.55)
for i,value in enumerate(seconds.values()): ax.text(value+.0004,i,f'{value:.3f} s',va='center',fontsize=8.5)
ax.set(xlim=(0,.031),xlabel='Observed planning seconds · shared host, unadmitted timing');ax.invert_yaxis()
ax.spines[['top','right']].set_visible(False)
fig.text(.02,.02,'Preflight 002 · n=1 / phase · 0.001 s resolution · no measured spread or actual-cohort runtime',fontsize=7.4)
fig.subplots_adjust(left=.26,right=.97,top=.96,bottom=.3)
save(fig,BASE/'reports/preflight/visual',{'source':'evidence/preflight-runtime.json','seconds':seconds,'n_per_phase':1,'admitted_performance':False})

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


render(BASE/'REPORT.md',BASE/'REPORT.pdf','Strict splitting fit verified; historical use unresolved')
for name in ['attempt-001','preflight']:
    directory=BASE/'reports'/name
    render(directory/'REPORT.md',directory/'REPORT.pdf','Strict splitting study · '+name)
browse=REPO/'benchmarks/20261029-strict-elkies-preflight'
browse.mkdir(parents=True,exist_ok=True)
for name in ['REPORT.pdf','visual.svg','visual.png','visual.json']:
    shutil.copyfile(BASE/'reports/preflight'/name,browse/name)
