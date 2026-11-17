#!/usr/bin/env python3
"""Render a descriptive producer-audit diagram and PDF from retained records."""
from pathlib import Path
import json
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib.pagesizes import A4
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle
from reportlab.graphics.shapes import Drawing, Rect, String, Line
from reportlab.graphics import renderSVG

p = Path(__file__).resolve().parent
audit = json.loads((p / "audit.json").read_text())
custody = json.loads((p / "custody.json").read_text())
styles = getSampleStyleSheet()
styles["Normal"].fontSize = 9
styles["Normal"].leading = 12
diagram = Drawing(480, 125)
for x, title, rows, fill in [
    (0, "Source custody", ["6 shared files; original source", "52 original report paths", "14 pinned archive hashes"], "#e7f0f7"),
    (165, "Native controls", ["3 CLI tests in each consumer", "33 older suite library tests", "Tensor Clippy + locked graph"], "#eaf3eb"),
    (330, "Claim standing", ["Original bodies unchanged", "PCC standing: unverified", "No signed execution verdict"], "#f7efe1"),
]:
    diagram.add(Rect(x, 20, 150, 100, rx=6, ry=6, fillColor=colors.HexColor(fill), strokeColor=colors.HexColor("#61717f")))
    diagram.add(String(x + 9, 101, title, fontName="Helvetica-Bold", fontSize=11))
    for i, text in enumerate(rows):
        diagram.add(String(x + 9, 78 - i * 17, text, fontName="Helvetica", fontSize=8))
diagram.add(String(0, 4, "Producer software/custody observations; independent execution evidence remains absent.", fontName="Helvetica", fontSize=8))
renderSVG.drawToFile(diagram, str(p / "audit.svg"))

story = [Paragraph("Structured IC: post-merge audit", styles["Title"]),
         Paragraph("5 November 2026 | PCC-20261105-e660a7 | Producer statement: unverified", styles["Normal"]),
         Spacer(1, 12), diagram, Spacer(1, 10)]
for text in [
    "The public known-answer controls report 3 passing CLI tests in each of the crypto, cryptanalysis bindings and older suite entry points, plus 33 passing older suite library tests. Scoped strict tensor Clippy and the repaired genus-2 tool's locked dependency graph report passing. [pcc:unverified]",
    "Read-only comparisons report that six shared files match both current copies and the published source; 26 original report paths in each implementation remain unchanged. All 14 receipt hashes match the reachable b7955c3d13834c076b7b345c44b4912bfdb03d4b archive. The archived journal is checked at that exact commit; its live copy has subsequent entries. [pcc:unverified]",
    "Both original claim bodies remain unchanged and their standing is unverified with no signed verdicts. Ledger prefix/chain controls report passing against the merged commits. This report is not an independent signed execution verdict. [pcc:unverified]",
]:
    story.extend([Paragraph(text, styles["Normal"]), Spacer(1, 8)])
rows = [["Repository", "Pinned main revision"]] + [[repo, rev] for repo, rev in audit["revisions"].items()]
table = Table(rows, colWidths=[115, 365])
table.setStyle(TableStyle([("FONTNAME", (0, 0), (-1, 0), "Helvetica-Bold"), ("FONTSIZE", (0, 0), (-1, -1), 8), ("VALIGN", (0, 0), (-1, -1), "TOP"), ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#eef2f5")), ("BOTTOMPADDING", (0, 0), (-1, -1), 7)]))
story.extend([table, Spacer(1, 12), Paragraph("CI observations and preserved limits", styles["Heading2"])])
for text in [
    "Historical release transfer and autoresearcher record failures are retained; the pinned current-main release and record jobs report success. Crypto still has whole-repository Clippy, ephemeral-CLI and curve-naming failures at source paths unchanged by this integration. The report preserves these baseline failures; no gate or unrelated benchmark data was changed. [pcc:unverified]",
    "Raw commands, exit codes, original failure logs and digests: checks/*.json and checks/*.txt.gz. Source comparisons: custody.json. API snapshots: ci-snapshot.json.gz. Source/reproduction notes: README.md. Host and toolchain: environment.json. No performance/isolation comparison is asserted.",
    "Primary context: docs/structured-prime-ic.md, the original structured consumer reports, KN-OPEN-08ba7e and docs/SABOTAGE_RESISTANCE.md. The full constrained-summation-message requirement remains open. [pcc:unverified]",
]:
    story.extend([Paragraph(text, styles["Normal"]), Spacer(1, 8)])
SimpleDocTemplate(str(p / "report.pdf"), pagesize=A4, rightMargin=42, leftMargin=42, topMargin=36, bottomMargin=32).build(story)
