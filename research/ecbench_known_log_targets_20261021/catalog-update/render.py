"""Document layout and diagram rendering only; no research calculation."""
from pathlib import Path
import re
import subprocess
import fitz
import typst

root = Path(__file__).resolve().parent
for name in ("typed_correspondence", "typed_transfer_context"):
    subprocess.run(["dot", "-Tsvg", str(root / f"{name}.dot"), "-o", str(root / f"{name}.svg")], check=True)
    subprocess.run(["dot", "-Tpng", "-Gdpi=150", str(root / f"{name}.dot"), "-o", str(root / f"{name}.png")], check=True)
pdf = root / "CATALOG.pdf"
typst.compile(str(root / "CATALOG.typ"), output=str(pdf), root=str(root))
subject = re.search(r'^subject: "(.*)"$', (root / "README.md").read_text(), re.MULTILINE).group(1)
with fitz.open(pdf) as doc:
    metadata = doc.metadata
    metadata.update(title="Cover catalog update for benchmark fixtures", subject=subject,
                    creationDate="D:20261021000000Z", modDate="D:20261021000000Z")
    doc.set_metadata(metadata)
    doc.saveIncr()
