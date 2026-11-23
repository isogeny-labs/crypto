"""Render recorded model-drift JSON; never computes cryptographic results."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.platypus import SimpleDocTemplate, Paragraph, Image, Spacer
from xml.sax.saxutils import escape

root = Path(__file__).resolve().parent
data = json.loads((root / "transfer-model-drift.json").read_text())
models = [data["legacy_10g2_model_current_geometry"], data["current_measured_cantor_model"]]
values = [m["routes"][m["best_route"]]["cost"]["ratio_log2"] for m in models]
fig, ax = plt.subplots(figsize=(6, 2.8))
ax.bar(["Legacy 10 g²", "Current measured Cantor"], values, color=["#306888", "#A35C38"])
ax.axhline(0, color="black", linewidth=.8)
ax.set(ylabel="Formula log₂(rho/route), bits", title="c2pnb304w1 · unchanged genus-8 / GF(2⁷⁶) route")
for i, value in enumerate(values):
    ax.annotate(f"{value:+.3f}", (i, value), xytext=(0, 5 if value >= 0 else -14), textcoords="offset points", ha="center")
ax.set_ylim(-.45, 2.5)
fig.tight_layout()
for ext in ["svg", "png", "pdf"]:
    fig.savefig(root / f"model-drift.{ext}", dpi=150)
styles = getSampleStyleSheet()
styles["BodyText"].fontSize = 9
styles["BodyText"].leading = 12
story = [Paragraph("Repairing generated-schema and model-test drift", styles["Title"])]
for para in (root / "REPORT.md").read_text().split("\n\n"):
    if para.startswith("!["):
        story.append(Image(str(root / "model-drift.png"), width=495, height=231))
    elif para.strip():
        story.extend([Paragraph(escape(para.replace("`", "")).replace("\n", "<br/>"), styles["BodyText"]), Spacer(1, 8)])
SimpleDocTemplate(str(root / "REPORT.pdf"), pagesize=A4, leftMargin=42, rightMargin=42).build(story)
