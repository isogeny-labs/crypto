#!/usr/bin/env bash
# Regenerate the run-1 (paired per call) and, when present, run-2 (end to end)
# tables, graphs and REPORT.pdf.
set -euo pipefail
cd "$(dirname "$0")"
python3 summarize_ab.py results RESULTS_AB.md results_ab.json results_ab.svg >/dev/null
parts="README.md RESULTS_AB.md"
if [ -d results_e2e ] && ls results_e2e/*.json >/dev/null 2>&1; then
  python3 ../m4gb_e2e_ic_ladder_20261110/summarize.py results_e2e RESULTS_E2E.md results_e2e.json results_e2e.svg >/dev/null
  parts="$parts RESULTS_E2E.md"
fi
pandoc $parts -s --metadata title="inherited-f4-next and fes-f2 end to end (2026-11-12)" -o REPORT.html
chrome=$(command -v google-chrome || command -v chromium || echo /opt/pw-browsers/chromium)
"$chrome" --headless=new --no-sandbox --disable-gpu --no-pdf-header-footer --print-to-pdf=REPORT.pdf REPORT.html >/dev/null 2>&1
for s in results_ab results_e2e; do
  [ -f $s.svg ] && "$chrome" --headless=new --no-sandbox --disable-gpu --window-size=1100,460 --screenshot=$s.png $s.svg >/dev/null 2>&1
done
rm -f REPORT.html
ls -la REPORT.pdf results_ab.svg results_ab.png
