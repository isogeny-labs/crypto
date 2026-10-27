#!/usr/bin/env python3
"""Plot recorded native rows and render the report; no research computation."""
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
record = base / "runs/degree-closure-probe-panel-v1/analysis/summary.json"
data = json.loads(record.read_text())
rows = data["rows"]
raw = [r for r in rows if r["mode"] == "raw16"]
other = {(r["arm"], r["original_system_index"]): r for r in rows if r["mode"] == "closure8"}
closure = [other[r["arm"], r["original_system_index"]] for r in raw]
labels = [f'{"phi" if r["arm"] == "phi-l0-active" else "double"} / {r["original_system_index"]}' for r in raw]
out = Path(sys.argv[1]).resolve()
out.mkdir(parents=True, exist_ok=False)
(root / "figures").mkdir(exist_ok=True)
fig, axes = plt.subplots(2, 1, figsize=(10.6, 6.2), sharex=True, layout="constrained")
for cases, label, color, marker in [(raw, "raw16", "#3569a8", "o"), (closure, "closure8", "#177d67", "s")]:
    axes[0].plot(labels, [r["search_work"]["modular_multiplications"] for r in cases], color=color, marker=marker, label=label)
    axes[1].plot(labels, [r["certificate_check_work"]["modular_multiplications"] for r in cases], color=color, marker=marker, label=label)
    for r, x in zip(cases, labels):
        if not r["certified"]:
            axes[0].annotate("none found", (x, r["search_work"]["modular_multiplications"]), xytext=(0, 9), textcoords="offset points", ha="center", fontsize=8, color=color)
axes[0].set_yscale("log")
axes[0].set_ylim(top=300000000)
axes[0].set_ylabel("Search Fp37 multiplications (log)")
axes[0].set_title("Complete fixed 12-configuration panel: first / audit / A/A / parity pass\np37 seed0, six stored systems; different bounded search methods", fontsize=11)
axes[0].legend(ncol=2)
axes[1].set_ylabel("Sparse-check Fp37 multiplications")
axes[1].set_xlabel("Arm / original system index; check count 0 means no certificate was produced")
for ax in axes:
    ax.spines[["top", "right"]].set_visible(False)
    ax.grid(axis="y", alpha=0.2)
svg = root / "figures/full-panel.svg"
fig.savefig(svg)
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(root / "figures/full-panel.png", dpi=150)
plt.close(fig)
source = out / "source"
(source / "figures").mkdir(parents=True)
for name in ["REPORT.md", "REPORT.typ", "render-results.py"]:
    shutil.copy2(root / name, source / name)
shutil.copy2(svg, source / "figures/full-panel.svg")
pdf = root / "REPORT.pdf"
assert not pdf.exists()
typst.compile(str(source / "REPORT.typ"), output=str(pdf))
shutil.copy2(pdf, out / "REPORT.pdf")
pages = []
with fitz.open(pdf) as document:
    for number, page in enumerate(document):
        image = out / f"page-{number+1}.png"
        page.get_pixmap(matrix=fitz.Matrix(1.5, 1.5)).save(image)
        pages.append({"page":number+1,"image":str(image),"characters":len(page.get_text())})
(out / "layout.json").write_text(json.dumps({"scope":"Layout and literal native values only","data_path":str(record),"data_sha256":hashlib.sha256(record.read_bytes()).hexdigest(),"pdf_sha256":hashlib.sha256(pdf.read_bytes()).hexdigest(),"pages":pages},indent=2)+"\n")
