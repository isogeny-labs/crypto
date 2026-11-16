#!/usr/bin/env bash
# Regenerate RESULTS.md, results.json, results.svg/png and REPORT.pdf from results/.
set -euo pipefail
cd "$(dirname "$0")"
python3 ../m4gb_e2e_ic_ladder_20261110/summarize.py results RESULTS.md results.json results.svg >/dev/null
pandoc README.md RESULTS.md -s --metadata title="Size rule and inherited-f4 fold, end to end (2026-11-12)" -o REPORT.html
chrome=$(command -v google-chrome || command -v chromium || echo /opt/pw-browsers/chromium)
"$chrome" --headless=new --no-sandbox --disable-gpu --no-pdf-header-footer --print-to-pdf=REPORT.pdf REPORT.html >/dev/null 2>&1
"$chrome" --headless=new --no-sandbox --disable-gpu --window-size=1100,460 --screenshot=results.png results.svg >/dev/null 2>&1
rm -f REPORT.html
ls -la RESULTS.md results.svg results.png REPORT.pdf
