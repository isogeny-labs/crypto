"""Document layout only: copy native table/figure outputs and convert SVG to PDF."""
from pathlib import Path
import shutil
import sys
import fitz

derived, paper = map(Path, sys.argv[1:3])
figures, tables = paper / "figures", paper / "tables"
figures.mkdir(parents=True, exist_ok=True)
tables.mkdir(parents=True, exist_ok=True)
for name in ("cutoff-margins", "evidence-boundary"):
    shutil.copyfile(derived / f"{name}.svg", figures / f"{name}.svg")
    source = fitz.open(figures / f"{name}.svg")
    converted = fitz.open("pdf", source.convert_to_pdf())
    converted.save(figures / f"{name}.pdf")
for name in ("numbers.tex", "field-coverage.tex", "endpoints.tex"):
    shutil.copyfile(derived / name, tables / name)
(tables / "field-table.tex").write_text(
    "\\begin{tabular}{rrrrr}\n\\toprule\n"
    "Field bits & Positive cutoffs & Percent & $m_{691}$ & $m_{5000}$ \\\\\n"
    "\\midrule\n" + (derived / "field-coverage.tex").read_text() +
    "\\bottomrule\n\\end{tabular}\n"
)
(tables / "endpoint-table.tex").write_text(
    "\\begin{tabular}{lcrrrrrr}\n\\toprule\n"
    " & & \\multicolumn{3}{c}{$L=691$} & \\multicolumn{3}{c}{$L=5000$} \\\\\n"
    "\\cmidrule(lr){3-5}\\cmidrule(lr){6-8}\n"
    "Curve & Role & $S$ & $R$ & $S+R$ & $S$ & $R$ & $S+R$ \\\\\n"
    "\\midrule\n" + (derived / "endpoints.tex").read_text() +
    "\\bottomrule\n\\end{tabular}\n"
)
