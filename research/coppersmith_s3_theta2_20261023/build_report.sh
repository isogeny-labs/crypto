#!/bin/sh
# Build a run report PDF from a run directory: <dir>/run.md + recovery.svg → <dir>/report.pdf,
# and the study REPORT.md → REPORT.pdf. Document rendering only (pandoc + Typst wheel in /tmp/report-venv).
set -e
cd "$(dirname "$0")"
PY=/tmp/report-venv/bin/python
for d in "$@"; do
  $PY plot_recovery.py "$d/run.json"
  pandoc "$d/run.md" -f gfm -t typst --wrap=none -o "$d/body.typ"
  sed -i '/^<[^>]*>$/d' "$d/body.typ"   # pandoc's heading labels: Typst rejects non-ASCII label names
  sed -i 's#^\#image("\([^"]*\)")#\#page(flipped: true, margin: 1.2cm)[\#figure(image("\1", width: 100%), caption: [Recovered fraction per cell; see run.md])]#' "$d/body.typ"
  printf '%s\n' '#set page(paper: "a4", margin: 1.8cm, numbering: "1")' '#set text(font: "DejaVu Sans", size: 8.6pt)' '#show raw: set text(font: "DejaVu Sans Mono", size: 7.4pt)' '#show table: set text(size: 7pt)' '#include "body.typ"' > "$d/report.typ"
  (cd "$d" && $PY -c "import typst; typst.compile('report.typ', output='report.pdf')")
done
pandoc REPORT.md -f gfm -t typst --wrap=none -o body.typ
sed -i '/^<[^>]*>$/d' body.typ
sed -i 's#^\#image("\([^"]*\)")#\#page(flipped: true, margin: 1.2cm)[\#figure(image("\1", width: 100%), caption: [Recovered fraction per cell; the run report has the table])]#' body.typ
printf '%s\n' '#set page(paper: "a4", margin: 1.8cm, numbering: "1")' '#set text(font: "DejaVu Sans", size: 8.6pt)' '#show raw: set text(font: "DejaVu Sans Mono", size: 7.4pt)' '#show table: set text(size: 7pt)' '#include "body.typ"' > REPORT.typ
$PY -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
