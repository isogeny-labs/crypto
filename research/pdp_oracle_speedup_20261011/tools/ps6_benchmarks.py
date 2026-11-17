#!/usr/bin/env python3
"""Copy PS-6's visual record into benchmarks/20261031-pdp-ps6-charge/ and write its copies.json (file copying only).

  ps6_benchmarks.py REPO_ROOT

The study directory stays the source of truth; this copies the report PDFs, every graph as SVG and its 150 dpi PNG
(rendered by tools/plot_ps6.py from the same code as the SVG) and records the source path, SHA-256 and size of each.
"""
import hashlib
import json
import os
import shutil
import sys

root = sys.argv[1].rstrip("/") + "/"
study = "research/pdp_oracle_speedup_20261011/"
dest = root + "benchmarks/20261031-pdp-ps6-charge/"
files = [
    (study + "ps6_charge/report/report.pdf", "report.pdf"),
    (study + "reports/run-20261018T2327Z/report.pdf", "run-20261018T2327Z-report.pdf"),
    (study + "reports/run-20261031T1925Z/report.pdf", "run-20261031T1925Z-report.pdf"),
]
pngs = []
for g in ("s_over_floor", "band_order", "exponents", "wall", "memory"):
    files.append((study + f"ps6_charge/report/{g}.svg", f"{g}.svg"))
    files.append((study + f"ps6_charge/report/{g}.png", f"{g}.png"))
    pngs.append({"source": f"{g}.svg", "copy": f"{g}.png", "dpi": 150})
os.makedirs(dest, exist_ok=True)
rows = []
for src, name in files:
    shutil.copyfile(root + src, dest + name)
    data = open(dest + name, "rb").read()
    rows.append({"source": src, "copy": name, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)})
json.dump(
    {
        "schema": "benchmark-browsing-copies/v1",
        "measurement_date": "2026-10-31",
        "copied_during_integration": "2026-10-31",
        "study": study + "ps6_charge",
        "runs": ["run-20261018T2327Z", "run-20261031T1925Z"],
        "files": [r for r in rows if not r["copy"].endswith(".png")],
        "pngs": pngs,
    },
    open(dest + "copies.json", "w"),
    indent=2,
)
open(dest + "copies.json", "a").write("\n")
open(dest + ".gitattributes", "w").write(
    "# Preserve copied scientific artifact bytes.\n*.pdf binary\n*.png binary\n*.svg -whitespace\n"
)
print("copied", len(rows), "files")
