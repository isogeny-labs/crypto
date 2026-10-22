# Cross-lead review schema

`results/standard-cost-grid.csv` is a lossless column mapping of the cited
factor-base and projective-law price grids. Each priced row is one arm, one
size slice (or `all`), and one of 36 declared price cells. It copies the
source cost and ratio strings; `build.py` independently checks both quotient
identities to relative tolerance `1e-9`. It does not recompute raw operation
counts or source scalar products. The source study validators perform those
checks. `source_grid` is the repository-relative source for each row.

`lead` and `arm` identify the experiment; `slice` is the order bit size or
`all`; `targets` and `verified` are counts within that slice.
`candidate_failures` and `rho_failures` retain source failure counts.
`S_per_M`, `I_per_M`, and `native_GAE_per_logical` identify the price cell.
`candidate_GAE`, `external_GAE`, and `rho_GAE` are full summed cold costs in
source addition equivalents. The two `candidate_over_*` columns divide
candidate by the named comparator for that same row.

Seven Kummer coordinate rows use `status=identity_only_no_dlp_collector` and
empty price and cost fields. They are one per 180-target size slice, rather
than 36 fabricated price cells. Their `verified` field counts source labels
at the *target* level; the coordinate case counts are in the source JSONL.
An empty numeric field means unmeasured and must not be read as zero.

`results/summary.json` records the representative `S/M=0.5`, `I/M=10`,
logical `GAE=0.1` rows, the requested-lead status, and source links. The
SVG is layout from those representative rows. Per-target records and detailed
counter definitions remain in each source study's `DATA_DICTIONARY.md`.
