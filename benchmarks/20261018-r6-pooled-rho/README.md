# R6.2 pooled rho and path-cost results

This directory contains browsing copies of the measured report and its graphs. The source of truth, inputs, run records, and plotting code are in [research/isogeny_fibers_20261010/round6/r6_2_pooled_rho/](../../research/isogeny_fibers_20261010/round6/r6_2_pooled_rho/).

The batch graph reports counted group-operation equivalents divided by sqrt(r) for 20 trials at each target count L. The path graph reports bidirectional BFS neighbor enumerations for small toy components. Neither graph reports elapsed-time performance; the pooled arms are descriptive diagnostics, with no equivalence or speedup claim.

The SVG and PNG files were regenerated from the committed plot.py, batch.json, and paths.json. The PDF is copied from the corresponding run report.

## Correction (paths v2, run-20261025T0156Z)

The path-finding graph above (`paths_enumerations_vs_component.*`, from the
frozen v1 `paths.json`) came from a search that expanded outgoing edges on both
sides; for degree 5 and 7 with p not = +-1 mod l such a route is not executable.
It is kept as the record of what was first committed and is superseded by
`paths_v2_enumerations_vs_component.*` (certified, forward-validated routes;
counts include certification and validation work) and
`paths_v2_legacy_not_executable.*`; the PDF is `report_paths_v2.pdf`, copied from
[`research/isogeny_fibers_20261010/round6/r6_2_pooled_rho/reports/run-20261025T0156Z/`](../../research/isogeny_fibers_20261010/round6/r6_2_pooled_rho/reports/run-20261025T0156Z/).
The batch graph and `report.pdf` (batch half) are unchanged. Counts only; no
elapsed-time claim.
