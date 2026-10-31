#!/bin/sh
# Render the study's documents from their sources (layout only; computes nothing).
# Needs: pandoc (Markdown -> Typst body) and the `typst` Python wheel with
# matplotlib in a scratch virtualenv, as .claude/skills/research-visuals/SKILL.md
# describes:  python3 -m venv /tmp/report-venv && /tmp/report-venv/bin/pip install typst matplotlib
set -eu
PY=${PY:-/tmp/report-venv/bin/python}
RUN=results/run-20261018T1605Z
"$PY" figures/plot.py "$RUN" figures
"$PY" figures/diagram.py figures
mkdir -p "reports/run-20261018T1605Z"
"$PY" reports/make_run_report.py "$RUN" "reports/run-20261018T1605Z/run-report.typ"
"$PY" -c "import typst; typst.compile('reports/run-20261018T1605Z/run-report.typ', output='reports/run-20261018T1605Z/run-report.pdf', root='.')"
pandoc REPORT.md -t typst -o /tmp/report_body.typ
{ cat preamble.typ; echo; cat /tmp/report_body.typ; } > REPORT.typ
"$PY" -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf', root='.')"
echo "built REPORT.pdf and reports/run-20261018T1605Z/run-report.pdf"
