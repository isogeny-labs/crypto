# Cross-lead review schema

`results/standard-cost-grid.csv` is a column mapping of the cited
factor-base, projective-law, paired pencil, Markowitz-priority pencil, and same-process Kummer price grids. Each priced row is one arm, one
size slice (or `all`), and one of 36 declared price cells. It copies the
source cost and ratio strings; the native Rust converter independently checks both quotient
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
`baseline_GAE` and `candidate_over_baseline` are populated only for pencil
rows. The peel baseline is the matched online identity collector; the
Markowitz baseline is the matched peel collector. The peel and Markowitz rows
copy their source paired-grid ratios, while online rows have ratio one. Empty
`external_GAE` for pencil does not mean that the online baseline was unmeasured.
The ten online/peel pencil pairs comprise nine 60-target curves and one
28-bit single target. A separate p=739 Markowitz/peel pair adds 36 rows for
the Markowitz candidate only; its `baseline_GAE` is the source-verified peel
arm already represented in the grid. The 32-bit cap failures have no
aggregate price rows because they did not solve; four null-candidate-cost records
across online, peel, and Markowitz remain in `standard-targets.csv`.

The Kummer differential collector has 288 full-cold price rows (eight slices,
36 cells each) from the independently checked same-process rho run. They use
`arm=differential_uncached` and `status=priced_source_verified_same_process`.
The separate cached-inverse collector and archived separate-process rho
comparison remain in its source study, outside this matched cross-lead grid.
Seven Kummer coordinate/recovery rows use `status=coordinate_and_recover_gate_no_dlp_collector` and
empty price and cost fields. They are one per 180-target size slice, rather
than 36 fabricated price cells. Their `verified` field counts source labels
at the *target* level; the coordinate case counts are in the source JSONL.
An empty numeric field means unmeasured and must not be read as zero.

`results/summary.json` records the representative `S/M=0.5`, `I/M=10`,
logical `GAE=0.1` rows, the requested-lead status, and source links. The
SVG is layout from those representative rows. Per-target records and detailed
counter definitions remain in each source study's `DATA_DICTIONARY.md`.

`results/standard-targets.csv` maps all currently integrated 12,850
target-arm observations to a common key `(bits,p,target_index,workload_id)`:
6,300 factor-base rows, 2,520 projective-law rows, 1,510 pencil rows,
1,260 Kummer differential-collector rows, and 1,260 Kummer coordinate rows. The builder checks that the present rows for
each of 1,260 distinct targets share one workload ID. `source_verified` means the public
source scalar or label passed its source check; for a capped pencil run the
source-verified rho scalar certifies the target while candidate DLP is false.
`candidate_dlp_verified` and
`rho_verified` mean a complete DLP scalar was recovered and checked; they are
empty for Kummer's coordinate-only row. The differential collector has both
source-verified candidate and same-process rho DLP scalars. Target-level costs use the same
representative price as the report. A capped pencil row keeps its measured
same-process `rho_GAE` while `candidate_GAE` and its ratio are empty.
`coordinate_nonzero_matches`, `coordinate_zero_outputs`, and
`coordinate_failures` are the seven Kummer labels on that target, empty for
other leads. `signed_recover_verified` and `recover_method` join the separate
source-checked signed-Recover gate; `recover_record` links its raw file.
`source_record` links the coordinate raw file or the exact source price grid.
This index keeps zero-coordinate
cases and the three exceptional Recover methods visible without treating them
as failed DLP solves.
`failure`, `solver_mode`, and `order` preserve the sparse-pencil outcome and
solver variant. Four p=186,379 rows have `status=incomplete_cap_unpriced`,
empty candidate cost/ratio fields, a measured rho cost, and source-verified
rho. All other pencil rows have
source-verified candidate and rho scalars. The additional 61 Markowitz rows
come from `research/pencil_markowitz_20261022/results/targets.jsonl`; its
independent native validator, replay certificate and validation gate are in
that source study. A missing pencil row for any other
target means that variant was not run; it is not a zero-cost result.
