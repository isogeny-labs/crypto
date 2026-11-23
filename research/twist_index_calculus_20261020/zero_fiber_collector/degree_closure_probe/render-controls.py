#!/usr/bin/env python3
"""Document and figure layout only; all research values come from Rust receipts."""
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
for name in ["BUILD_AND_CONTROLS.typ", "render-controls.py", "PROTOCOL.md", "BUILD_AND_CONTROLS.md"]:
    shutil.copy2(root / name, source / name)
shutil.copy2(root / "figures/flow.svg", source / "figures/flow.svg")
pdf = root / "BUILD_AND_CONTROLS.pdf"
assert not pdf.exists()
typst.compile(str(source / "BUILD_AND_CONTROLS.typ"), output=str(pdf))
pages = []
with fitz.open(pdf) as document:
    for number, page in enumerate(document):
        image = out / f"page-{number + 1}.png"
        page.get_pixmap(matrix=fitz.Matrix(1.5, 1.5)).save(image)
        pages.append({"page": number + 1, "image": str(image), "characters": len(page.get_text())})
diagram = out / "diagram.typ"
diagram.write_text('#set page(width: 1200pt,height:390pt,margin:0pt)\n#image("' + str(source / "figures/flow.svg") + '",width:100%)\n')
typst.compile(str(diagram), output=str(out / "diagram.pdf"), root="/")
with fitz.open(out / "diagram.pdf") as document:
    pixmap = document[0].get_pixmap(matrix=fitz.Matrix(150 / 72, 150 / 72))
    pixmap.set_dpi(150, 150)
    pixmap.save(root / "figures/flow.png")
(out / "layout.json").write_text(json.dumps({"scope": "Artifact layout only", "pdf_sha256": hashlib.sha256(pdf.read_bytes()).hexdigest(), "pages": pages}, indent=2) + '\n')
