#!/usr/bin/env bash
set -euo pipefail
study="$(cd "$(dirname "$0")" && pwd)"
assets="$(mktemp -d)"
trap 'rm -rf "$assets"' EXIT
dot -Tsvg "$study/walk.dot" -o "$study/walk.svg"
dot -Tpdf "$study/walk.dot" -o "$assets/walk.pdf"
pandoc "$study/REPORT.md" --resource-path="$study:$assets" \
  --lua-filter="$study/render.lua" --pdf-engine=pdflatex -o "$study/REPORT.pdf"
