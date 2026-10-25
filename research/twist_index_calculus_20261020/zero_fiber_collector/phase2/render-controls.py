#!/usr/bin/env python3
"""Artifact layout only: compile the authored report and rasterize for inspection."""
from pathlib import Path
import hashlib
import json
import shutil
import sys

import fitz
import typst

root = Path(__file__).resolve().parent
out = Path(sys.argv[1]).resolve()
out.mkdir(parents=True, exist_ok=False)
source = out / "source"
(source / "figures").mkdir(parents=True)
for name in ["BUILD_AND_CONTROLS.typ", "BUILD_AND_CONTROLS.md", "render-controls.py"]:
    shutil.copy2(root / name, source / name)
shutil.copy2(root / "figures/pipeline.svg", source / "figures/pipeline.svg")
pdf = root / "BUILD_AND_CONTROLS.pdf"
assert not pdf.exists()
typst.compile(str(source / "BUILD_AND_CONTROLS.typ"), output=str(pdf))
with fitz.open(pdf) as document:
    pages = []
    for index, page in enumerate(document):
        destination = out / f"page-{index + 1}.png"
        page.get_pixmap(matrix=fitz.Matrix(1.5, 1.5)).save(destination)
        pages.append({"page": index + 1, "image": str(destination),
                      "characters": len(page.get_text())})
diagram_source = out / "diagram.typ"
diagram_source.write_text(
    '#set page(width: 1200pt, height: 400pt, margin: 0pt)\n'
    '#image("' + str(source / "figures/pipeline.svg") + '", width: 100%)\n'
)
diagram_pdf = out / "diagram.pdf"
typst.compile(str(diagram_source), output=str(diagram_pdf), root="/")
with fitz.open(diagram_pdf) as document:
    diagram = document[0].get_pixmap(matrix=fitz.Matrix(150 / 72, 150 / 72))
    diagram.set_dpi(150, 150)
    diagram.save(root / "figures/pipeline.png")
(out / "layout.json").write_text(json.dumps({
    "scope": "PDF/PNG layout only; no research result computed here",
    "pdf": str(pdf), "pdf_sha256": hashlib.sha256(pdf.read_bytes()).hexdigest(),
    "pages": pages,
}, indent=2) + "\n")
