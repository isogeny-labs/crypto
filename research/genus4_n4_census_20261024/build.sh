#!/bin/sh
# Render the study's documents from their sources (layout only; computes nothing).
# python3 -m venv /tmp/report-venv && /tmp/report-venv/bin/pip install typst matplotlib
set -eu
PY=${PY:-/tmp/report-venv/bin/python}
"$PY" scripts/plot.py results figures
rep() { # run-dir figure caption
  mkdir -p "reports/$1"
  "$PY" scripts/make_run_report.py "results/$1" "reports/$1/run-report.typ" "../../figures/$2" "$3"
  "$PY" -c "import typst; typst.compile('reports/$1/run-report.typ', output='reports/$1/run-report.pdf', root='.')"
}
for d in results/*; do
  n=$(basename "$d")
  case "$n" in
    ctrlH*) f=fig_controls.svg; c="Parity of Res-shaped traces: hyperelliptic (Theorem 5, no odd t) and non-hyperelliptic." ;;
    tian*|tiantheory*) f=fig_tian.svg; c="Tian kernel existence per prime-order trace and l (n = 4)." ;;
    *) f=fig_mass_by_t.svg; c="Mass of hits per prime-order trace with the Haar-density fit." ;;
  esac
  rep "$n" "$f" "$c"
done
pandoc REPORT.md -f markdown-auto_identifiers -t typst --wrap=none -o /tmp/g4n4_body.typ
{ cat preamble.typ; echo; cat /tmp/g4n4_body.typ; } > REPORT.typ
"$PY" -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf', root='.')"
echo "built REPORT.pdf and reports/*/run-report.pdf"
