#!/usr/bin/env python3
"""Document layout and plots of literal native values; no research arithmetic."""
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
record = base / "runs/fp-residual-audit-v1/first/summary.json"
data = json.loads(record.read_text())
out = Path(sys.argv[1]).resolve()
out.mkdir(parents=True, exist_ok=False)
figures = root / "figures"
figures.mkdir(exist_ok=True)
phi = [case for case in data["cases"] if case["arm"] == "phi-l0-active"]
double = {(c["p"], c["seed"]): c for c in data["cases"] if c["arm"] == "double-l0-active"}
control = [double[c["p"], c["seed"]] for c in phi]
labels = [f'{c["p"]}/{c["seed"]}' for c in phi]

def save(fig, name):
    svg = figures / f"{name}.svg"
    fig.savefig(svg)
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(figures / f"{name}.png", dpi=150)
    plt.close(fig)

fig, axes = plt.subplots(2, 1, figsize=(10.6, 6.0), sharex=True, layout="constrained")
for cases, label, color, marker in [(phi, "phi", "#3569a8", "o"), (control, "doubling", "#b76b29", "s")]:
    axes[0].plot(labels, [c["counts"]["rejected_finite_fp"] for c in cases], marker=marker, color=color, label=label)
    axes[1].plot(labels, [c["after"]["rank"] for c in cases], marker=marker, color=color, label=label)
axes[0].set_ylabel("Eligible finite-Fp systems (count)")
axes[0].set_title("All 42 zero-fiber arms: native first execution\nSame exact physical sets and per-arm ranks before and after rejection", fontsize=11)
axes[0].legend(ncol=2)
axes[1].set_ylabel("Factor-log rank after rejection")
axes[1].set_xlabel("Prime p / presentation seed; three presentations of scalar 271 are not independent secrets")
axes[1].tick_params(axis="x", rotation=60)
for ax in axes:
    ax.spines[["top", "right"]].set_visible(False)
    ax.grid(axis="y", alpha=0.2)
save(fig, "eligibility-and-rank")

fig, ax = plt.subplots(figsize=(10.6, 4.4), layout="constrained")
for cases, label, color, marker in [(phi, "phi", "#3569a8", "o"), (control, "doubling", "#b76b29", "s")]:
    ax.plot(labels, [c["counts"]["rejected_f4_row_multiplications"] for c in cases], marker=marker, color=color, label=label)
ax.set_ylabel("Historical Fp row-reduction multiplications")
ax.set_xlabel("Prime p / presentation seed")
ax.set_title("Partial historical F4 work in eligible systems\nNo filter deployed; root enumeration, other work and complete cost are unpriced", fontsize=11)
ax.tick_params(axis="x", rotation=60)
ax.ticklabel_format(style="plain", axis="y")
ax.legend(ncol=2)
ax.spines[["top", "right"]].set_visible(False)
save(fig, "eligible-row-work")

source = out / "source"
(source / "figures").mkdir(parents=True)
for name in ["REPORT.md", "REPORT.typ", "render.py"]:
    shutil.copy2(root / name, source / name)
for name in ["eligibility-and-rank", "eligible-row-work"]:
    shutil.copy2(figures / f"{name}.svg", source / "figures" / f"{name}.svg")
pdf = root / "REPORT.pdf"
assert not pdf.exists()
typst.compile(str(source / "REPORT.typ"), output=str(pdf))
shutil.copy2(pdf, out / "REPORT.pdf")
pages = []
with fitz.open(pdf) as document:
    for number, page in enumerate(document):
        path = out / f"page-{number + 1}.png"
        page.get_pixmap(matrix=fitz.Matrix(1.5, 1.5)).save(path)
        pages.append({"page": number + 1, "image": str(path), "characters": len(page.get_text())})
(out / "layout.json").write_text(json.dumps({"scope": "Layout and plotting of recorded native values only", "data_path": str(record), "data_sha256": hashlib.sha256(record.read_bytes()).hexdigest(), "pdf_sha256": hashlib.sha256(pdf.read_bytes()).hexdigest(), "pages": pages}, indent=2) + "\n")
