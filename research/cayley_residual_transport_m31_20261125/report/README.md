# Residual-transport publication report

This report publishes the frozen `cayley-residual-transport-m31-v1` component
and its November 25, 2026 native development evidence. It adds no experiment,
timing result, natural-yield observation, complete solve or certificate.
The algebra is derived prose; the evidence remains producer evidence /
PCC-unverified, without an IDC1h record, Lean/kernel proof or signed verdict.

## Files

- `REPORT.pdf`: three-page publication report.
- `build_report.py`: editable layout, exact diagram definitions and citations.
- `residual_transport.svg`: supplied-prefix structure and both allowed-set constraints.
- `evidence_outcomes.svg`: the two recorded empty-fiber counts, with each
  stratum's case count. These deterministic development strata are not
  natural relation-yield measurements or statistical replicates.

The SVGs and PDF use the same ReportLab vector drawing definitions. Python
only lays out recorded values and the retained mathematical statements. It
does not generate research inputs, execute a solver or recompute a scientific
result. The build reads the existing `evidence/receipt.json` and checks its
study and retained executable identity before rendering.

## Build

From the study directory after these files are placed under `report/`:

```bash
python3 report/build_report.py
```

From another location:

```bash
python3 build_report.py --study-root /path/to/cayley_residual_transport_m31_20261125
```

Dependencies are Python 3, ReportLab and the DejaVu Sans / Sans Mono fonts.
Use `--font-dir` if those font files are not under
`/usr/share/fonts/truetype/dejavu`. The PDF embeds its fonts.
Poppler (`pdftoppm`) is used only for visual QA after rendering.

## Evidence links

The report links to the publication branch
`codex/cayley-residual-transport-m31-20261126`, at
`research/cayley_residual_transport_m31_20261125/`, for the mathematics,
receipt, frozen protocol and component README. The original arithmetic is
pinned to `crypto@eb55e93110676c91213d91d6c8d00489311a4bd8` and the earlier
x-Cayley component is identified separately by merged PR #5232.
No absolute workspace paths appear in the public report.

The diagram and cost model distinguish a second allowed-set constraint from
a second residual degree of freedom. Input-list lengths and distinct set
sizes are separated so duplicate-heavy supplied lists do not disappear from
preprocessing cost. Signed identity summands are explicitly represented by
`null` in the retained records, while incompatible labels, nonsplit fibers
and invalid inputs remain separate outcomes.

## Publication QA

Completed checks for this publication:

- Rendered all three pages with Poppler at 150 dpi and visually inspected
  every page after layout corrections. Text, tables, formulas, headers,
  footers and page numbering are legible and unclipped.
- Rendered both standalone SVGs with Inkscape and inspected the images.
  Their figure captions identify the frozen study; SVG descriptions link
  the derivation and exact receipt.
- Confirmed three PDF pages, searchable text, embedded DejaVu text fonts,
  13 source-link annotations, required evidence counts/status wording and
  no workspace absolute paths or missing-glyph replacement characters.
- Confirmed that this script reads the retained receipt without executing
  research code or modifying frozen inputs.

Layout checks used `pypdf`, `pdftoppm`, `pdffonts`, Inkscape and visual
inspection. These are document QA, not a new scientific validation claim.
Do not place the report inside any sealed evidence directory or overwrite
the frozen source, protocol, receipt or validation-run files. The optional
`tmp/pdfs/` images are local QA intermediates and need not be committed.

## Canonical graphs

The publication review found no applicable canonical-graph update under
the repository's section 7d exclusion: this change adds no priced IC/rho
row, reference/curve/factor-base identity, ecbench or tournament result,
candidate identity, or measured performance change. The index-calculus
scoreboard, progress timeline, current-state/leaderboard pages and data,
browser data, and performance-gains comparisons/rendered summaries were
checked at their applicable contract/prose scope and left unchanged. The
full path list is recorded in publication validation. Full browser
generation and coverage were not run.
