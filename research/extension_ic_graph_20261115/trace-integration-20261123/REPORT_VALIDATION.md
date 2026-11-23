# Visual and evidence validation

Native Rust generators: trace_integration_report.rs and unit_deferred_run_report.rs.
Typst0.15.1renders the PDFs; Poppler renders every page at150dpi. Final report
preview PNGs use four-color palettes (57139/57244bytes), preserving both chart
segments. Standalone diagram/count/wall PNGs use16colors and remain below64KiB.
Original high-color renders stay LOCAL under /tmp/ic-trace-report-review and
/tmp/ic-run51-report-review. No scientific calculation uses Python.

Inspected:

- Trace report, one full A4page, full-color render and final palette preview:
  all text/table/figure bounds,34root compact counts and3415/3419full controls
  agree with the retained JSON; source and unsigned status remain visible.
- Run51report, one full A4page, full-color render and final palette preview:
  all ten table/bar values agree with run-observation.json; original137and
  separate replay0remain prominent; N1fixed order/nonadmitted wall/no interval
  are visible. Its diagram PDF source bytes were unchanged after prose lint fix.
- Flow, coordinate-count and Run51wall standalone one-page PDF/150dpiPNG
  renderings: labels, legend, values, bounds and captions checked.

Rejected presentations retained LOCAL:

- Both original automated Callgrind reports, two pages each: all four pages
  inspected; long Rust-symbol tables overflow on page1. Do not publish those
  PDFs as final readable reports.
- Standalone copies of original profiler SVGs also clip long symbol labels.
  Keep them LOCAL with original hashes. Public manifests/hotspot JSON preserve
  their data; the concise native report presents their actual scope and totals.
- Bilevel report previews erased the orange linear-stage bars. Replaced with
  inspected four-color previews; rejected files remain LOCAL. Numeric sources
  and original PDF/SVG bytes were not altered by palette changes.

Run51and trace REPORT.md pass PCC prose lint with unsigned evidence labelled.
No headline benchmark/leaderboard promotion: total-cost accounting, repeated
isolated Runpod trials, fresh/growing targets and signed admission remain open.
Full-library syntax and inherited benchmark-record gate failures are retained
in validation; source publication does not assert those gates pass.
