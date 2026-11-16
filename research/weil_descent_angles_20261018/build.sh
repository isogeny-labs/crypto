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
# round 4 (Addendum H)
"$PY" figures/plot4.py results figures
rep H1-hsweep-run-20261018T2240Z h1_cofactor_law.svg "H1: Arm J trials per p against the cofactor h, with rho and the M-H1 model; 30 cells x 4 instances x 10 targets."
rep H2-glue-run-20261018T2240Z h2_glue.svg "H2: general E~E^sigma gluing, h = 1 and 4: group operations per p and the pricing band."
rep H4-memsweep-run-20261018T2209Z h4_memory_time.svg "H4: trials against stored relations as the distinguished fraction theta falls."
rep H5-aut-census-run-20261018T2227Z h7_subgroup.svg "H5 has no graph of its own (an exact census table); shown: the H7 graph of the same round."
rep H6-large-run-20261018T2210Z h1_cofactor_law.svg "H6 values are in this report's summary; graph shown: H1 of the same round."
rep H7-subgroup-run-20261018T2209Z h7_subgroup.svg "H7: coverage of the l-subgroup, subfield over null, mean and 1.96 s.e. per cell."
pandoc REPORT_ROUND4.md -t typst --wrap=none -o /tmp/report4_body.typ
{ cat preamble.typ; echo; cat /tmp/report4_body.typ; } > REPORT_ROUND4.typ
"$PY" -c "import typst; typst.compile('REPORT_ROUND4.typ', output='REPORT_ROUND4.pdf', root='.')"
echo "built REPORT_ROUND4.pdf"
