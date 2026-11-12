# Frozen input view: anchor-eight run

`reports/run-20261024-anchor-eight/ARTIFACTS.sha256` was frozen at commit
`d0597e0ea3df9ce4003d27f182c06d9e19ee5c4b` (2026-10-24). It pins two files
whose current bytes on `main` have since changed:

- `.github/workflows/p192-geometry-collector-lean.yml` at `b350582e…`: later
  repository-wide CI changes (runner routing, artifact pulls) rewrote the
  workflow. The replay `p192_anchor_eight_20261024` in
  `.github/scripts/frozen-workflow-pins.json` restores it from `d0597e0ea3`
  with its per-replay `revision`.
- `research/p192_extension_cover_20261012/REPORT.md` at `0444d2bf…`: merge
  `097e2b2361` (2026-10-26) added four lines after the line-census paragraph
  linking the summary figure (`type_i_line_census.pdf`/`.svg` and
  `render_line_census_figure.py`). No result, number or claim changed. The
  input view `.github/scripts/p192-anchor-eight-report-view-v1.json` restores
  the pinned bytes from `d0597e0ea3`, after matching the digest against the
  original manifest and the manifest's own SHA-256.

Scope: `sha256sum -c` in that isolated view checks the frozen evidence, the
frozen producer and verifier scripts, the Lean source and the logs exactly as
the run recorded them. The REPORT.md it checks is the frozen one, not the
current one. The current REPORT.md is unchanged by this view and is not
checked by it. Neither the manifest nor any evidence file was edited.
