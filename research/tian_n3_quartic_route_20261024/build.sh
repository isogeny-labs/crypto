#!/bin/sh
# Render the figures and REPORT.pdf from the recorded runs (layout only; computes nothing).
# Needs pandoc and a virtualenv with the `typst` wheel and matplotlib (python3 -m venv /tmp/report-venv &&
# /tmp/report-venv/bin/pip install typst matplotlib), as .claude/skills/research-visuals/SKILL.md describes.
set -eu
PY=${PY:-/tmp/report-venv/bin/python}
A=$(ls -d results/a2-summarize-run-* | head -1)
B=$(ls -d results/b-ic-vs-rho-run-* | tail -1)
ARAW=$(ls -d results/a-census-run-* | head -1)
mkdir -p /tmp/claude-0/work/figin
cp "$ARAW"/census_q*.csv /tmp/claude-0/work/figin/ 2>/dev/null || true
cp "$A"/summary_detail.csv /tmp/claude-0/work/figin/
"$PY" figures/plot.py /tmp/claude-0/work/figin "$B" figures
pandoc REPORT.md -t typst --wrap=none -o /tmp/claude-0/work/report_body.typ
{ cat preamble.typ; echo; cat /tmp/claude-0/work/report_body.typ; } > REPORT.typ
"$PY" -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf', root='.')"
echo "built REPORT.pdf"
