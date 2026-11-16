#!/usr/bin/env bash
# Rebuild the census binary and regenerate every row in rows/ exactly as the
# recorded runs were made (sweeps A, B1, B2, B3, E, C, C2), then the tables,
# graphs and PDF. Rows are deterministic (seed 20261111); each heuristic width
# is exact for the order it chose, so reruns agree to the unit. Wall time on
# the 4-core container that produced them: about 2 h per schoolbook sweep,
# about 2.5 h for the Fourier sweeps.
set -euo pipefail
cd "$(dirname "$0")/../.."
cargo build --release -p hashed-ecdl-tensor
BIN=target/release/tensor-treewidth
OUT=research/semaev_tensor_treewidth_20261111/rows
ALL=grouplaw-hash,grouplaw-smallx,semaev-s3-smallx,chained-s3-smallx
$BIN --bits 8,10,12,14,16 --relations $ALL --multipliers schoolbook --validate-max-bits 10 --minfill-max-bits 16 --restarts 3 --out $OUT/A-schoolbook-8-16.json > /dev/null
$BIN --bits 20,24,28     --relations $ALL --multipliers schoolbook --validate-max-bits 0  --minfill-max-bits 16 --restarts 3 --out $OUT/B1-schoolbook-20-28.json > /dev/null
$BIN --bits 32,36        --relations $ALL --multipliers schoolbook --validate-max-bits 0  --minfill-max-bits 16 --restarts 3 --out $OUT/B2-schoolbook-32-36.json > /dev/null
$BIN --bits 40           --relations $ALL --multipliers schoolbook --validate-max-bits 0  --minfill-max-bits 16 --restarts 1 --out $OUT/B3-schoolbook-40.json > /dev/null
$BIN --bits 20,24 --relations semaev-s3-smallx,grouplaw-smallx --multipliers schoolbook --validate-max-bits 0 --minfill-max-bits 24 --restarts 1 --out $OUT/E-eager-20-24.json > /dev/null
$BIN --bits 8,12,16,20,24 --relations $ALL --multipliers fourier --validate-max-bits 0 --minfill-max-bits 0 --restarts 1 --out $OUT/C-fourier-8-24.json > /dev/null
$BIN --bits 28,36 --relations semaev-s3-smallx --multipliers fourier --validate-max-bits 0 --minfill-max-bits 0 --restarts 1 --out $OUT/C2-fourier-28-36.json > /dev/null
# Layout only (no computation): tables, SVG/PNG graphs, Typst tables; then the PDF.
python3 research/semaev_tensor_treewidth_20261111/render.py
# PDF: python3 -m venv /tmp/report-venv && /tmp/report-venv/bin/pip install typst
/tmp/report-venv/bin/python -c "import typst; typst.compile('research/semaev_tensor_treewidth_20261111/REPORT.typ', output='research/semaev_tensor_treewidth_20261111/REPORT.pdf')"
