#!/usr/bin/env bash
set -euo pipefail
study=research/factor_base_heldout_replay_20261025
python3 "$study/render_report.py"
convert -background white -density 160 "$study/ratio-by-size.svg" "$study/ratio-by-size.png"
(cd "$study" && pdflatex -interaction=nonstopmode -halt-on-error REPORT.tex > REPORT.build.log)
pdfinfo "$study/REPORT.pdf" | rg '^(Pages|Page size|File size):'
