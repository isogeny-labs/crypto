#!/usr/bin/env bash
# Render a round's README.md (with its SVG graphs) to REPORT.pdf.
# Document rendering only (pandoc + headless Chromium); no research computation.
#   scripts/render_pdf.sh research/curve25519_autolab_20261104/round-001
set -euo pipefail
dir="$1"
chrome="${CHROME:-$(ls -d /opt/pw-browsers/chromium*/chrome-linux*/chrome 2>/dev/null | head -1 || true)}"
[ -n "$chrome" ] || chrome="$(command -v chromium || command -v chromium-browser || command -v google-chrome)"
abs="$(cd "$dir" && pwd)"
(cd "$abs" && pandoc README.md --standalone --embed-resources --resource-path=".:.." --metadata pagetitle="$(head -1 "$abs/README.md" | sed 's/^# //')" \
  -o REPORT.html --css <(printf 'body{font-family:Helvetica,Arial,sans-serif;max-width:1100px;margin:auto;font-size:12px}table{border-collapse:collapse}td,th{border:1px solid #999;padding:2px 6px}img{max-width:100%%}'))
"$chrome" --headless --no-sandbox --disable-gpu --print-to-pdf="$dir/REPORT.pdf" --no-pdf-header-footer "file://$abs/REPORT.html" 2>/dev/null
rm -f "$dir/REPORT.html"
echo "wrote $dir/REPORT.pdf"
