#!/bin/bash
# Thin orchestration: render the report PDFs and the PNG copies of the SVG graphs.
# Usage: render.sh REPORT.md OUT.pdf [figdir]   or   render.sh png FIG.svg OUT.png
set -euo pipefail
CH=/opt/pw-browsers/chromium-1194/chrome-linux/chrome
if [ "$1" = "png" ]; then
  svg=$(readlink -f "$2"); out=$(readlink -f "$3")
  w=$(sed -n "s/.*viewBox='0 0 \([0-9.]*\) \([0-9.]*\)'.*/\1/p" "$svg" | head -1)
  h=$(sed -n "s/.*viewBox='0 0 \([0-9.]*\) \([0-9.]*\)'.*/\2/p" "$svg" | head -1)
  tmp=$(mktemp -d); cp "$svg" "$tmp/f.svg"
  printf '<!doctype html><html><body style="margin:0"><img src="f.svg" style="display:block"></body></html>' > "$tmp/t.html"
  $CH --headless=new --no-sandbox --hide-scrollbars --screenshot="$tmp/raw.png" \
     --window-size=$(printf '%.0f,%.0f' "$w" "$(echo "$h+200" | bc)") --force-device-scale-factor=1.5625 "file://$tmp/t.html" >/dev/null 2>&1
  convert "$tmp/raw.png" -crop "$(printf '%.0fx%.0f' "$(echo "$w*1.5625" | bc)" "$(echo "$h*1.5625" | bc)")+0+0" +repage "$out"
  rm -rf "$tmp"; exit 0
fi
md=$(readlink -f "$1"); pdf=$(readlink -f "$2"); dir=$(dirname "$md")
html="$dir/.render-$$.html"
pandoc "$md" -f gfm -t html5 -s --metadata pagetitle="$(basename "$md" .md)" --resource-path="$dir" \
  -c /dev/null -o "$html" -V maxwidth=60em \
  --css="$(dirname "$(readlink -f "$0")")/report.css" 2>/dev/null || pandoc "$md" -f gfm -t html5 -s --metadata pagetitle="$(basename "$md" .md)" -o "$html"
$CH --headless=new --no-sandbox --no-pdf-header-footer --print-to-pdf="$pdf" "file://$html" >/dev/null 2>&1
rm -f "$html"
