#!/usr/bin/env bash
set -euo pipefail
packet_dir="$(cd "$(dirname "$0")" && pwd)"
build_dir="$(mktemp -d /dev/shm/f17-semantic-report.XXXXXX)"
trap 'rm -rf "$build_dir"' EXIT
pdflatex -interaction=nonstopmode -halt-on-error -output-directory "$build_dir" \
  "$packet_dir/REPORT.tex" > "$build_dir/pdflatex.stdout"
TMPDIR=/dev/shm gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite \
  -dCompatibilityLevel=1.4 -dPDFSETTINGS=/ebook \
  -sOutputFile="$build_dir/optimized.pdf" "$build_dir/REPORT.pdf"
cp "$build_dir/optimized.pdf" "$packet_dir/REPORT.pdf"
pdfinfo "$packet_dir/REPORT.pdf" | rg 'Pages:|File size:'
