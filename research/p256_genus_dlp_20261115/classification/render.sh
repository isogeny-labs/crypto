#!/usr/bin/env bash
set -euo pipefail
study="$(cd "$(dirname "$0")" && pwd)"
assets="$(mktemp -d)"
trap 'rm -rf "$assets"' EXIT
dot -Tsvg "$study/framework.dot" -o "$study/framework.svg"
dot -Tpdf "$study/framework.dot" -o "$assets/framework.pdf"
pandoc "$study/FRAMEWORK.md" --resource-path="$study:$assets" \
  --lua-filter="$study/render.lua" --pdf-engine=pdflatex -o "$study/FRAMEWORK.pdf"
