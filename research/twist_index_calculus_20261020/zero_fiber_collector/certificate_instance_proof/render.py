#!/usr/bin/env python3
"""Plot literal native receipts and lay out the report; no research computation."""
from pathlib import Path
import hashlib
import json
import shutil
import sys

import fitz
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

root = Path(__file__).resolve().parent
base = root.parent
out = Path(sys.argv[1]).resolve()
out.mkdir(parents=True, exist_ok=False)
(root / "figures").mkdir(exist_ok=True)
source = out / "source"
(source / "figures").mkdir(parents=True)
raw_path = base / "runs/degree-closure-probe-panel-v1/cases/p37-seed0-phi-l0-active-sys0-raw16/first/record.json"
closure_path = base / "runs/degree-closure-probe-panel-v1/cases/p37-seed0-phi-l0-active-sys0-closure8/first/record.json"
reference_path = base / "runs/panel-v2/cases/p37-seed0/phi-l0-active/first/record.json"
raw = json.loads(raw_path.read_text())
closure = json.loads(closure_path.read_text())
reference = json.loads(reference_path.read_text())
values = [raw["search_work"]["modular_multiplications"], raw["certificate_check_work"]["modular_multiplications"],
          closure["search_work"]["modular_multiplications"], closure["certificate_check_work"]["modular_multiplications"],
          reference["systems"][0]["f4_row_reduction_fp_muls"]]
labels = ["raw16\nsearch", "raw16\nsparse check", "closure8\nsearch", "closure8\nsparse check", "old F4\nrows only"]
fig, ax = plt.subplots(figsize=(10.6, 4.4), layout="constrained")
bars = ax.bar(labels, values, color=["#3569a8", "#81a7d2", "#177d67", "#88c6b5", "#a4a8ac"])
bars[-1].set_hatch("//")
ax.set_yscale("log")
ax.set_ylabel("Recorded Fp37 modular multiplications (log scale)")
ax.set_title("p37 seed0 phi system 0: literal native counts\nFirst/audit/A/A pass for both certificate methods; one fixed system", fontsize=11)
ax.bar_label(bars, labels=[f"{v:,}" for v in values], padding=4, fontsize=9)
ax.set_ylim(top=100000000)
ax.spines[["top", "right"]].set_visible(False)
fig.supxlabel("F4 bar is an incomplete reference for finding all roots. No complete-cost ranking or admitted timing.", fontsize=9)
fig.savefig(root / "figures/first-pair.svg")
svg_path = root / "figures/first-pair.svg"
svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")
fig.savefig(root / "figures/first-pair.png", dpi=150)
plt.close(fig)
bindings = []
for name, path in [("raw16", raw_path), ("closure8", closure_path), ("original_reference", reference_path)]:
    bindings.append({"name": name, "path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
for name in ["REPORT.typ", "REPORT.md", "render.py"]:
    shutil.copy2(root / name, source / name)
shutil.copy2(root / "figures/first-pair.svg", source / "figures/first-pair.svg")
pdf = root / "REPORT.pdf"
assert not pdf.exists()
typst.compile(str(source / "REPORT.typ"), output=str(pdf))
shutil.copy2(pdf, out / "REPORT.pdf")
pages = []
with fitz.open(pdf) as document:
    for number, page in enumerate(document):
        target = out / f"page-{number + 1}.png"
        page.get_pixmap(matrix=fitz.Matrix(1.5, 1.5)).save(target)
        pages.append({"page": number + 1, "image": str(target), "characters": len(page.get_text())})
(out / "layout.json").write_text(json.dumps({"scope": "Document layout and literal native values only", "pdf_sha256": hashlib.sha256(pdf.read_bytes()).hexdigest(), "pages": pages, "data_bindings": bindings}, indent=2) + "\n")
