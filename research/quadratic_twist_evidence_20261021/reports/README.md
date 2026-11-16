# Evidence import report

Latest actual-system experiment: [S3 useful yield, degrees and source-DLP transport](s3-yield-20261024/README.md),
with124 comparisons /1,904 target systems, a finite F43 rank/closure example,
Lean scalar/degree certificates, equal homogeneous regularity and an exact
main-curve pullback control. Its four-page PDF includes all nonzero target
profiles; no speedup or uniform twist advantage is inferred.

Latest proof continuation: [relation yield, row rank and conditional degree cost](yield-20261024/README.md),
with actual twist transport, exact whole-group yield ratio, a kernel-checked F5
control and an inspected four-page report. It supplies conditional mathematical
results, not a new measured IC/rho point.

Latest continuation: [positive twist-assisted pairing transfer](pairing-20261024T0442Z/README.md),
with an independently replayed 4,616-point census and a kernel-checked F43
121-pair Miller table. The four-page inspected PDF, recorded-count graph and
editable transfer diagram are versioned in that dated directory. Earlier
dated reports and the original import report below remain frozen.

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
