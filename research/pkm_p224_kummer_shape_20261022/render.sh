#!/bin/bash
# Renders figures/*.svg to 150 dpi PNG and README.md to report.pdf with
# headless Chromium (no LaTeX on the host). From anywhere:
#   research/pkm_p224_kummer_shape_20261022/render.sh
set -eu
here=$(cd "$(dirname "$0")" && pwd)
chrome=${CHROME:-/opt/pw-browsers/chromium-1194/chrome-linux/chrome}
for svg in "$here"/figures/*.svg; do
  png=${svg%.svg}.png
  "$chrome" --headless=new --no-sandbox --hide-scrollbars --force-device-scale-factor=1.5625 \
    --window-size=860,607 --screenshot="$png" "file://$svg" > /dev/null 2>&1
done
tmp=$(mktemp -d)
cat > "$tmp/style.html" <<'CSS'
<style>
body { font-family: -apple-system, "Segoe UI", Helvetica, Arial, sans-serif; font-size: 10.5pt; line-height: 1.38; max-width: 190mm; margin: 0 auto; color: #111; }
table { border-collapse: collapse; font-size: 8pt; margin: 8px 0; }
th, td { border: 1px solid #bbb; padding: 2px 5px; vertical-align: top; }
th { background: #f0f0f0; }
code { font-size: 8.5pt; }
pre { font-size: 8pt; background: #f6f6f6; padding: 6px; overflow-x: auto; }
img { max-width: 100%; }
h1 { font-size: 17pt; } h2 { font-size: 13pt; } h3 { font-size: 11pt; }
</style>
CSS
pandoc "$here/README.md" --from gfm --to html5 --standalone --metadata title="PKM Kummer tower at P-224's 2-adic shape" \
  --include-in-header="$tmp/style.html" --resource-path="$here" -o "$tmp/report.html"
sed -i "s#src=\"figures/#src=\"file://$here/figures/#g" "$tmp/report.html"
"$chrome" --headless=new --no-sandbox --no-pdf-header-footer --print-to-pdf="$here/report.pdf" "file://$tmp/report.html" > /dev/null 2>&1
rm -rf "$tmp"
ls -la "$here/report.pdf"
