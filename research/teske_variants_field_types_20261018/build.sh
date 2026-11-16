#!/bin/sh
# Render the study's documents from their sources (layout only; computes nothing).
# Needs: pandoc (Markdown -> Typst body) and the `typst` Python wheel with
# matplotlib in a scratch virtualenv, as .claude/skills/research-visuals/SKILL.md
# describes:  python3 -m venv /tmp/report-venv && /tmp/report-venv/bin/pip install typst matplotlib
set -eu
PY=${PY:-/tmp/report-venv/bin/python}
RUN_AB=results/run-20261018T0630Z
RUN_C=results/run-20261018T0845Z
"$PY" figures/plot.py "$RUN_AB" figures "$RUN_C"
for r in run-20261018T0624Z run-20261018T0630Z run-20261018T0845Z; do
  mkdir -p "reports/$r"
  "$PY" reports/make_run_report.py "results/$r" "reports/$r/run-report.typ"
  "$PY" -c "import typst; typst.compile('reports/$r/run-report.typ', output='reports/$r/run-report.pdf', root='.')"
done
pandoc REPORT.md -t typst -o /tmp/report_body.typ
{ cat preamble.typ; echo; cat /tmp/report_body.typ; } > REPORT.typ
"$PY" -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf', root='.')"
echo "built REPORT.pdf and reports/*/run-report.pdf"
