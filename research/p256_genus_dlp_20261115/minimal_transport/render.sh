#!/usr/bin/env bash
set -euo pipefail
study="$(cd "$(dirname "$0")" && pwd)"
assets="$(mktemp -d)"
trap 'rm -rf "$assets"' EXIT
dot -Tsvg "$study/transport.dot" -o "$study/transport.svg"
dot -Tpdf "$study/transport.dot" -o "$assets/transport.pdf"
pandoc "$study/REPORT.md" --resource-path="$study:$assets" \
  --lua-filter="$study/render.lua" --pdf-engine=pdflatex -o "$study/REPORT.pdf"
