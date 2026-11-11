#!/usr/bin/env bash
# Rebuild the census binary and regenerate every row in rows/ (schoolbook
# sweeps A, B1, B2 and the Fourier sweep C), then the graph and the PDF.
# Rows are deterministic (seed 20261111); widths from heuristics are exact
# for the order each heuristic chose, so reruns agree to the unit.
set -euo pipefail
cd "$(dirname "$0")/../.."
cargo build --release -p hashed-ecdl-tensor
BIN=target/release/tensor-treewidth
OUT=research/semaev_tensor_treewidth_20261111/rows
$BIN --bits 8,10,12,14,16 --multipliers schoolbook --validate-max-bits 10 --minfill-max-bits 16 --restarts 3 --out $OUT/A-schoolbook-8-16.json > /dev/null
$BIN --bits 20,24,28 --multipliers schoolbook --validate-max-bits 0 --minfill-max-bits 16 --restarts 3 --out $OUT/B1-schoolbook-20-28.json > /dev/null
$BIN --bits 32,36,40 --multipliers schoolbook --validate-max-bits 0 --minfill-max-bits 16 --restarts 3 --out $OUT/B2-schoolbook-32-40.json > /dev/null
$BIN --bits 8,12,16,20,24,28,32,36,40 --multipliers fourier --validate-max-bits 0 --minfill-max-bits 0 --restarts 1 --out $OUT/C-fourier.json > /dev/null
# Layout only (no computation): table, SVG graph, PDF.
python3 research/semaev_tensor_treewidth_20261111/render.py
