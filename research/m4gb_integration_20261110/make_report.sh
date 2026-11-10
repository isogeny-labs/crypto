#!/usr/bin/env bash
# Regenerate RESULTS.md, results.svg, results.json and REPORT.pdf from results.tsv (+ logs/).
set -euo pipefail
cd "$(dirname "$0")"
python3 ab_report.py results.tsv RESULTS.md results.svg results.json logs results_rerun.tsv >/dev/null
pandoc README.md RESULTS.md -s --metadata title="M4GB integration and kernel speed-up (2026-11-10)" -o REPORT.html
chrome=$(command -v google-chrome || command -v chromium || echo /opt/pw-browsers/chromium)
"$chrome" --headless=new --no-sandbox --disable-gpu --no-pdf-header-footer --print-to-pdf=REPORT.pdf REPORT.html >/dev/null 2>&1
"$chrome" --headless=new --no-sandbox --disable-gpu --window-size=1100,420 --screenshot=results.png results.svg >/dev/null 2>&1
rm -f REPORT.html
ls -la RESULTS.md results.svg results.png REPORT.pdf
