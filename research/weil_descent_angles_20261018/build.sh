#!/bin/sh
# Render round 3's documents from their sources (layout only; computes nothing).
# Needs pandoc and the `typst` Python wheel with matplotlib in a scratch
# virtualenv (.claude/skills/research-visuals/SKILL.md):
#   python3 -m venv /tmp/report-venv && /tmp/report-venv/bin/pip install typst matplotlib
set -eu
PY=${PY:-/tmp/report-venv/bin/python}
"$PY" figures/plot.py results figures
rep() { # run-dir figure caption
  mkdir -p "reports/$1"
  "$PY" reports/make_run_report.py "results/$1" "reports/$1/run-report.typ" "../../figures/$2" "$3"
  "$PY" -c "import typst; typst.compile('reports/$1/run-report.typ', output='reports/$1/run-report.pdf', root='.')"
}
rep item4-run-20261018T2049Z item4_traces.svg "Res-shaped genus-4 curves over F_3 by trace: hyperelliptic models (left), non-hyperelliptic classes (right); exact census."
rep item1D-run-20261018T2056Z item1_coverage.svg "Part D: paired residual from Poisson per (characteristic, n), and binary n = 2 coverage per curve; exact."
rep item1D-mult-run-20261018T2106Z item1_multiplicity.svg "Amendment D: point multiplicity histograms, subfield vs null; exact."
rep item2P-run-20261018T2056Z item2_pkm.svg "Part P: z of the paired Poisson residual, tower factor base vs random x-set, per cell."
rep itemOJ-run-20261018T2124Z itemOJ_routes.svg "Graph drawn from the recorded re-run 20261018T2132Z (this run's rho rows are superseded by Amendment G); steps/p and pricing bands."
rep itemF-control-run-20261018T2128Z itemF_control.svg "Amendment F: Arm J trials per p, walk vs independent trials, by cofactor h; each dot one verified run."
rep itemOJ-run-20261018T2132Z itemOJ_routes.svg "Parts O/J: walk steps per p by arm (left) and the IC/rho cost-ratio band over pricing cells (right); 30 verified runs per arm and size."
pandoc REPORT.md -t typst --wrap=none -o /tmp/report_body.typ
{ cat preamble.typ; echo; cat /tmp/report_body.typ; } > REPORT.typ
"$PY" -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf', root='.')"
echo "built REPORT.pdf and reports/*/run-report.pdf"
