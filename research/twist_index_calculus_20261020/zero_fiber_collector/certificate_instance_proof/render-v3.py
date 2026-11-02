#!/usr/bin/env python3
"""Typesetting and page rendering only; no research result generation."""
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
(out / "source/figures").mkdir(parents=True)
for name in ["PROOF-V3.md", "PROOF-V3.typ", "render-v3.py"]:
    shutil.copy2(root / name, out / "source" / name)
shutil.copy2(root / "figures/proof-v3-flow.svg", out / "source/figures/proof-v3-flow.svg")
pdf = root / "PROOF-V3.pdf"
assert not pdf.exists()
typst.compile(str(out / "source/PROOF-V3.typ"), output=str(pdf))
shutil.copy2(pdf, out / "PROOF-V3.pdf")
pages = []
with fitz.open(pdf) as document:
    for number, page in enumerate(document):
        image = out / f"page-{number+1}.png"
        page.get_pixmap(matrix=fitz.Matrix(1.5, 1.5)).save(image)
        pages.append({"page": number+1, "image": str(image), "characters": len(page.get_text())})
(out / "layout.json").write_text(json.dumps({"scope": "Document layout only", "pages": pages, "pdf_sha256": hashlib.sha256(pdf.read_bytes()).hexdigest()}, indent=2)+"\n")
