# Report validation

Native trace_cohort_summary.rs aggregates exact saved counters; native
trace_cohort_report.rs renders data/Markdown/SVG/Typst. Typst0.15.1produces
one A4page; Poppler renders it and the separate figure PDF at150dpi.

Inspected the entire color PDF page, final four-color report preview and
16-color standalone chart preview. Both chart segments, all exact table/bar
values, denominator, missing-learning annotation, unsigned status and source
references remain visible and agree with proof-bound-counter-summary.json.
No clipping, overlapping text or missing glyph was observed. Report PNG59708B,
figure PNG17390B; high-color original PNG remains LOCAL. Markdown prose lint
passes. The final Markdown tag fix leaves the rendered Typst source identical.

Original input/replay files and earlier report proof subjects are unchanged.
This is a complete saved denominator and partial solver accounting, not a
new experiment or full-cost comparison. No benchmark/leaderboard promotion,
S3upload, signed validation, fresh yield or rho-parity inference is made.

Root-set wording clarification: re-rendered and inspected the corrected full
one-page PDF/PNG (59970B). It now explicitly says3415coordinate root sets with
3419roots total. Numeric table, graph SVG and proof-bound summary bytes are
unchanged. Initial prose is preserved at2ea1f9f; correction457e58and the named
clarification record preserve the distinction without editing the old claim.
