#!/usr/bin/env bash
# REPORT.md -> REPORT.html (pandoc) -> REPORT.pdf (headless Chrome). Layout only.
set -eu
cd "$(dirname "$0")"
google-chrome --headless --no-sandbox --disable-gpu --hide-scrollbars --window-size=780,480 --screenshot=figures/gap_to_rho.png "file://$PWD/figures/gap_to_rho.svg" 2>/dev/null
cat > /tmp/bdsl_style.html <<'CSS'
<style>
body{font-family:"DejaVu Sans",sans-serif;font-size:10.5pt;max-width:none;margin:0 1.2cm;line-height:1.35}
table{border-collapse:collapse;font-size:8.5pt;margin:0.5em 0}
th,td{border:1px solid #bbb;padding:2px 5px}
code,pre{font-size:8.5pt}
pre{background:#f6f6f6;padding:6px;white-space:pre-wrap}
img{max-width:100%}
h1{font-size:16pt} h2{font-size:13pt} h3{font-size:11pt}
</style>
CSS
sed 's#figures/gap_to_rho.svg#figures/gap_to_rho.png#' REPORT.md | pandoc -f gfm -t html5 -s --metadata pagetitle="Arm D-S: sparse LA" -H /tmp/bdsl_style.html -o REPORT.html
google-chrome --headless --no-sandbox --disable-gpu --no-pdf-header-footer --print-to-pdf=REPORT.pdf "file://$PWD/REPORT.html" 2>/dev/null
rm -f REPORT.html
ls -la REPORT.pdf
