# C1 checkpoint 9: interim review packet

Observations only. The computation is still running. This packet supplies no
complete candidate/reference pair, algorithm ranking, timing qualification or
full-cohort completion claim.

- `review-packet.pdf`: coverage graph followed by the unaltered native report.
- `report.pdf` / `report.tex`: native report and editable source.
- `observation-coverage.svg` / `coverage-counts.json`: editable coverage graph
  and exact source-linked counts; graph bars show coverage, not performance.
- `cost-grid-exact.csv`: all 36 price cells with explicit null ratios.
- `report.json`: native render/provenance record.
- `independent-report-review.json`: independent five-page inspection and
  data/presentation checks, not independent scientific replay.

The raw snapshot and standardized export are in
`research/pencil_large_prime_20261029/runs/p186379-gate-20261030-c1/checkpoint-g9-observations/`.
Raw files were committed and pushed at `799fff06e` before report rendering.
Only selected non-memory session files are published. The full checkpoint
archive remains private.

## Reading the data

Raw `records.jsonl` has one retained execution per line, keyed by session ID,
record ID and sequence. `warmup` distinguishes calibration runs from measured
rounds. The original producer outcome is retained: three rows say verified and
one pencil warmup says timeout/watchdog expired. Producer verification is not
independent replay. No missing slot is synthesized as a failed or zero-cost row.

The canonical `export.json` has four observed execution rows. Its `flow` counts
that observed scope; its `backend_record.expected_execution_slots` retains all
18 planned slots and `missing_slots` names the 14 absent slots. The report uses
18, not 4, as the planned denominator. Three warmups and 15 measured slots were
planned; only one measured rho row is present. No measured pencil row is present.

The CSV columns are `cell`, three price coordinates (`square_twice`, `inversion`,
`native_tenths`), expected/admitted/missing-or-ineligible pair counts,
`complete`, and exact method/rho ratios with and without calibration. The
coordinate definitions are those of the unchanged frozen native pricing model:
S2 is twice the square/multiply price and N10 is ten times the native event price.
The CSV string `null` means unavailable, never zero or infinity. All 36 cells
have zero admitted pairs out of five expected and null ratios.

Resource allocation, recorded rusage and algorithm cost are separate quantities.
The ongoing run's total active time and total spending remain unknown. Original
resource metadata bound by the native analysis is retained byte-for-byte; the
later support observation is a separate sidecar. Version 4 governs scientific
data; version 5 governs prospective controller supervision only.

## Reproduction

The hash-bound native analysis/export/report requests and command receipts are
under the raw snapshot's `native-analysis/` directory. The native renderer
produced `report.pdf`, `report.tex`, `coverage.svg`, `report.json` and the exact
CSV without running the scientific solver. Canonical checking intentionally
retains five incompleteness findings; it is not a qualified gate result.

The review packet adds the SVG cover without changing any native report page:

```sh
rsvg-convert -f pdf observation-coverage.svg -o /tmp/pencil-c1-g9-coverage.pdf
pdfunite /tmp/pencil-c1-g9-coverage.pdf report.pdf review-packet.pdf
```

The PDFs are content-addressed artifacts listed in `.artifacts.json`; use
`python3 tools/artifacts/artifacts.py pull <this-directory>` from the repository
root to hydrate them. SVG, CSV, JSON and LaTeX sources remain directly reviewable.
Both PDFs and the coverage graph were visually inspected; the four native pages
render identically before and after concatenation.
