# PDF and figure inspection

Inspected rendered page images for:

- Study REPORT.pdf: both pages, graph, numerical table and limits.
- Primary run REPORT.pdf / identical RUN_REPORT.pdf: all three pages;
  replaced an overlong source path in report layout to avoid clipping.
- Replay REPORT.pdf: all three pages, matching plot and case table.
- Larger-run REPORT.pdf / identical RUN_REPORT.pdf: both final pages;
  condensed duplicate provenance text to keep the complete report in two pages.

Figures have visible axes, units, arm labels, run captions and scope limits.
The plots use the native summary's already-computed values. They do not
compute new metrics or confidence intervals. PNG copies are 150 dpi.
Benchmark PDF/graph copies are checked against their source hashes.
