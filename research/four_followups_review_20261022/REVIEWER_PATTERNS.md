# Reviewer-pattern appendix

This appendix reorganizes the frozen four-follow-up evidence for review. It
does not run or alter a collector, solver, replay, matrix, nullspace, or
recovery routine. The native reporting binary reads only
`results/standard-targets.csv` and `results/standard-cost-grid.csv`.

The generated material is in [`reviewer-patterns/`](reviewer-patterns/):

- `REPORT.pdf` and `README.md` give the neutral narrative and accounting
  boundaries;
- `paired-deltas.csv` retains every standardized observation and explicit nulls;
- `ecdf.csv` expands every cost arm to the same 1,260 workload keys, and the
  graph renders the conditional and bound curves as right-continuous steps;
- `descriptive-tails.csv` reports empirical nearest-rank quantiles;
- `cluster-bootstrap.csv` contains deterministic descriptive model resampling;
- `price-cell-heatmap.csv` contains four panels of 36 price cells;
- `break-even.csv` gives integer coefficients for all 85 complete grid groups.

The coordinate-only Kummer arm remains present in `paired-deltas.csv` with a
specific null reason. It is absent from every cost distribution, heatmap, and
bootstrap calculation because it has no DLP candidate cost.

The original appendix generator did not include phase fractions because its
standardized inputs contain totals only. The later
[phase-accounting appendix](phase-accounting/README.md) joins the frozen rows
to their raw source counters, checks 23,176 representative method totals and
6,120 aggregate price cells, and publishes source-named phases, a cross-study
class, explicit nulls, a graph and a PDF. The original `summary.json` retains
its generation-time status; the phase appendix is the current phase evidence.

Regenerate from the repository root:

```sh
cargo run --release --locked --offline \
  --manifest-path research/four_followups_review_20261022/Cargo.toml \
  --bin reviewer_patterns
uvx --from cairosvg==2.8.2 cairosvg -d 150 -s 1.5625 \
  research/four_followups_review_20261022/reviewer-patterns/ecdf.svg \
  -o research/four_followups_review_20261022/reviewer-patterns/ecdf.png
uvx --from cairosvg==2.8.2 cairosvg -d 150 -s 1.5625 \
  research/four_followups_review_20261022/reviewer-patterns/price-cell-heatmap.svg \
  -o research/four_followups_review_20261022/reviewer-patterns/price-cell-heatmap.png
SOURCE_DATE_EPOCH=1792627200 tectonic \
  research/four_followups_review_20261022/reviewer-patterns/REPORT.tex \
  --outdir research/four_followups_review_20261022/reviewer-patterns
python3 tools/artifacts/artifacts.py verify \
  research/four_followups_review_20261022/reviewer-patterns --remote
(cd research/four_followups_review_20261022/reviewer-patterns && \
  sha256sum -c SHA256SUMS)
```

The large generated CSVs follow the repository artifact-store policy. Pull or
verify the directory with `tools/artifacts/artifacts.py` before comparing a
fresh checkout byte-for-byte.
