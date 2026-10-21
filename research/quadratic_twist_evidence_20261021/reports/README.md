# Evidence import report

[Report PDF](report.pdf) · [Report source](report.tex) ·
[Historical count graph](coverage.svg) · [Native-audit scope diagram](audit-scope.svg)

The figures render recorded data only. Their editable source is `render.py`,
which performs presentation work and never runs a research implementation.
`coverage-data.json` records the plotted source fields. PNGs use 150 dpi.

The result report builds on the historical run and both retained native audit
attempts. The final native run and the separate positive/negative controls are
linked from the study README and retain their own transcripts. The source for
the earlier native auditor was preserved and checked against its recorded hash.

The canonical cost pages considered are `docs/index-calculus-scoreboard.html`,
`docs/ic-leaderboard.html`, `docs/ic/progress-timeline.json`, and
`docs/browser/data.json`. They are unchanged because this is a historical
correctness-evidence import and finite-certificate audit, with no admissible
performance measurement, IC/rho ratio, new registered curve, or leaderboard row.

Build the presentation with:

```sh
cd research/quadratic_twist_evidence_20261021/reports
python3 render.py
pdflatex -interaction=nonstopmode -halt-on-error report.tex
```

Python is used here only for rendering existing records, as permitted by the
research-visuals skill. It is not used by the native audit or CI execution path.
