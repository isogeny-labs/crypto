#!/usr/bin/env bash
set -euo pipefail
study="$(cd "$(dirname "$0")" && pwd)"
assets="$(mktemp -d)"
trap 'rm -rf "$assets"' EXIT
dot -Tsvg "$study/routes.dot" -o "$study/routes.svg"
dot -Tpdf "$study/routes.dot" -o "$assets/routes.pdf"
pandoc "$study/REPORT.md" --resource-path="$study:$assets" \
  --lua-filter="$study/render.lua" --pdf-engine=pdflatex -o "$study/REPORT.pdf"
