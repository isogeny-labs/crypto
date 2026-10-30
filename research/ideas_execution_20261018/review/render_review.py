"""Render recorded review/replication values only; performs no research trial."""
from pathlib import Path
import html
import json
import subprocess

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.units import inch
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer,Image,Table,TableStyle,PageBreak
from PIL import Image as Raster

root=Path(__file__).resolve().parent
amendment=root.parent/"targets_prime/verification_v2/runs"
ordered=json.loads((amendment/"run-20261018-verifier-v2/ordered.stdout").read_text())
strict=json.loads((amendment/"run-20261018-verifier-v2/strict.stdout").read_text())
replica=json.loads((amendment/"run-20261018-fp2-replication-v2/complete-verification.stdout").read_text())
cohort=json.loads((amendment/"run-20261018-fp2-replication-v2/cohort-comparison.json").read_text())

fig,axes=plt.subplots(1,2,figsize=(10,3.5),layout="constrained")
axes[0].bar(["Original ordered","Original strict","Corrected Fp2"],
            [ordered["verified_certificates"],strict["verified_certificates"],replica["verified_certificates"]],
            color=["#456d90","#6e96b7","#5e9072"])
axes[0].set_ylabel("Certificates with exact workload binding")
axes[0].set_title("Additive verification: every source hit covered")
axes[1].bar(["Old table hits","New table hits","Old rho answers","New rho answers"],
            [cohort["old_two_sum_hits"],cohort["new_two_sum_hits"],cohort["old_rho_successes"],cohort["new_rho_successes"]],
            color=["#456d90","#5e9072","#456d90","#5e9072"])
axes[1].set_ylabel("Recorded outcomes in the fixed 24-fixture panel")
axes[1].set_title("Accepted identities and all records unchanged")
for ax in axes:ax.tick_params(axis="x",rotation=15);ax.grid(axis="y",alpha=.2);ax.set_axisbelow(True)
fig.suptitle("Internal review and corrected replication · correctness counts only",fontsize=12)
for ext in ["svg","png"]:fig.savefig(root/f"review-counts.{ext}",dpi=180)
plt.close(fig)
for ext in ["svg","png"]:subprocess.run(["dot","-T"+ext,str(root/"review-flow.dot"),"-o",str(root/f"review-flow.{ext}")],check=True)

font_dir=Path(matplotlib.get_data_path())/"fonts/ttf"
pdfmetrics.registerFont(TTFont("ReviewSans",str(font_dir/"DejaVuSans.ttf")))
pdfmetrics.registerFont(TTFont("ReviewSansBold",str(font_dir/"DejaVuSans-Bold.ttf")))
styles=getSampleStyleSheet()
for name in ["BodyText","Title","Heading1","Heading2"]:styles[name].fontName="ReviewSans" if name=="BodyText" else "ReviewSansBold"
styles["BodyText"].fontSize=9;styles["BodyText"].leading=12
cell_style=ParagraphStyle("Cell",parent=styles["BodyText"],fontSize=7.5,leading=10)
doc=SimpleDocTemplate(str(root/"REPORT.pdf"),pagesize=(8.27*inch,11.69*inch),leftMargin=45,rightMargin=45,topMargin=40,bottomMargin=40)
story=[];table=[]
def text(value):return html.escape(value.replace("`",""))
def flush():
    if not table:return
    rows=[[Paragraph(text(cell),cell_style) for cell in row] for row in table]
    t=Table(rows,colWidths=[22,55,125,170,133],repeatRows=1)
    t.setStyle(TableStyle([("BACKGROUND",(0,0),(-1,0),colors.HexColor("#e7f0f8")),("VALIGN",(0,0),(-1,-1),"TOP"),("GRID",(0,0),(-1,-1),.4,colors.lightgrey),("BOTTOMPADDING",(0,0),(-1,-1),6)]))
    story.append(t);story.append(Spacer(1,8));table.clear()
for line in (root/"REPORT.md").read_text().splitlines():
    if line.startswith("|"):
        cells=[cell.strip() for cell in line.strip("|").split("|")]
        if not all(set(cell)<=set("-:") for cell in cells):table.append(cells)
        continue
    flush()
    if not line.strip() or line.startswith("!["):continue
    if line.startswith("# "):story.append(Paragraph(text(line[2:]),styles["Title"]))
    elif line.startswith("## "):story.append(Spacer(1,8));story.append(Paragraph(text(line[3:]),styles["Heading2"]))
    else:story.append(Paragraph(text(line),styles["BodyText"]));story.append(Spacer(1,5))
flush();story.append(PageBreak());story.append(Paragraph("Recorded verification and cohort preservation",styles["Heading1"]))
for stem in ["review-flow","review-counts"]:
    width,height=Raster.open(root/f"{stem}.png").size
    story.append(Image(str(root/f"{stem}.png"),width=7*inch,height=7*inch*height/width));story.append(Spacer(1,18))
doc.build(story)
print("Rendered review report and figures from retained native outputs.")
