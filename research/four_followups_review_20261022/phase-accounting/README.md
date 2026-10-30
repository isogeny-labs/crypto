# Phase accounting appendix

This is a reporting-only normalization of the already verified, frozen public
known-answer records. It does not execute a collector, solver, nullspace, or
recovery step. The cost denominator and the 36-cell price band are the same as
the source study's matched rho comparison.

The native generator reads `results/standard-targets.csv`, joins each row to
its `source_record` by `(p,target_index)`, and then reads the Kummer cold model
and target setup records where needed. It preserves every source phase name in
`phase` and declares a broad `phase_class`: `setup`, `transfer`, `walk`,
`relation_processing`, `recovery`, `verification`, or `source_unallocated`.
The class is for grouping only; `phase` and `source_record` retain the exact
counter origin.

- Factor-base and projective-law arrays provide setup, transfer, walk,
  relation, and source-verification counters. Their native logical counters
  are carried in `native_logical_unallocated`.
- Pencil object phases provide field and base/scalar arithmetic counters. The
  residual against its priced top-level counts and solver counters is
  `unallocated_setup_and_solver`. Its rho phase array omits source
  verification; the checked residual and native logical counters are
  `rho_unallocated_verification_and_logical`.
- Kummer cold setup is split into the raw root, theta, torsion, and transfer
  counter objects. `walk_ops`, `recover_ops`, and `field_counts` remain
  distinct; the collector's graph/attempt counters are unallocated. Kummer's
  same-process rho has only one total counter object and is labeled
  `rho_total_unphased`.
- The 1,260 coordinate-only Kummer gate rows and four pencil cap rows have
  explicit null candidate costs. They are absent from the phase graph.

The generator checks 23,176 representative candidate/rho method totals
against the source-linked standardized target records and 6,120 aggregate
method cells against the standardized grid. A failed check stops generation.
`phase-targets.csv` contains target-level representative rows; `phase-grid.csv`
contains sums and fractions across all 36 cells and each source slice. The
graph shows one representative price cell and retains the source phase names
in the CSV for unrestricted inspection. The [PDF](REPORT.pdf) is the compact
review copy. This appendix does not fill the 717 unresolved pencil candidate
targets or establish a full 1,260-target pencil ratio.

Rebuild from the repository root:

```sh
cargo run --release --locked --offline \
  --manifest-path research/four_followups_review_20261022/Cargo.toml \
  --bin normalize_phases
python3 research/four_followups_review_20261022/phase-accounting/render.py
uvx --from cairosvg==2.8.2 cairosvg -d 150 -s 1.5625 \
  research/four_followups_review_20261022/phase-accounting/phase-fractions.svg \
  -o research/four_followups_review_20261022/phase-accounting/phase-fractions.png
SOURCE_DATE_EPOCH=1792627200 tectonic \
  research/four_followups_review_20261022/phase-accounting/REPORT.tex \
  --outdir research/four_followups_review_20261022/phase-accounting
python3 tools/artifacts/artifacts.py verify \
  research/four_followups_review_20261022/phase-accounting --remote
(cd research/four_followups_review_20261022/phase-accounting && sha256sum -c SHA256SUMS)
```

The two CSVs use the repository artifact store; `.artifacts.json` supplies
their immutable hashes and sizes. `SHA256SUMS` covers the remaining local
report files and the manifest.
