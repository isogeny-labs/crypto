#!/usr/bin/env python3
"""Artifact layout only; no algebra, collection, statistics or pricing."""
from pathlib import Path
import hashlib
import json
import shutil
import sys

import fitz
import typst

root=Path(__file__).resolve().parent
out=Path(sys.argv[1]).resolve()
out.mkdir(parents=True,exist_ok=False)
source=out/"source"
(source/"figures").mkdir(parents=True)
for name in ["BUILD_AND_PROOF.typ","render-controls.py","PROTOCOL.md","EXECUTION.md"]:
    shutil.copy2(root/name,source/name)
shutil.copy2(root/"figures/flow.svg",source/"figures/flow.svg")
pdf=root/"BUILD_AND_PROOF.pdf"
assert not pdf.exists()
typst.compile(str(source/"BUILD_AND_PROOF.typ"),output=str(pdf))
pages=[]
with fitz.open(pdf) as doc:
    for i,page in enumerate(doc):
        image=out/f"page-{i+1}.png"
        page.get_pixmap(matrix=fitz.Matrix(1.5,1.5)).save(image)
        pages.append({"page":i+1,"image":str(image),"characters":len(page.get_text())})
diagram=out/"diagram.typ"
diagram.write_text('#set page(width: 1200pt,height:400pt,margin:0pt)\n#image("'+str(source/"figures/flow.svg")+'",width:100%)\n')
typst.compile(str(diagram),output=str(out/"diagram.pdf"),root="/")
with fitz.open(out/"diagram.pdf") as doc:
    pix=doc[0].get_pixmap(matrix=fitz.Matrix(150/72,150/72))
    pix.set_dpi(150,150)
    pix.save(root/"figures/flow.png")
(out/"layout.json").write_text(json.dumps({"scope":"Artifact layout only","pdf_sha256":hashlib.sha256(pdf.read_bytes()).hexdigest(),"pages":pages},indent=2)+'\n')
