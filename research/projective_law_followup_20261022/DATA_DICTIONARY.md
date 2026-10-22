# Standardized records

All raw target records are JSON Lines, one target per row, under
`results/projective-<bits>.jsonl` and `results/state-<bits>.jsonl`. Each arm
has seven files of 180 rows, grouped by 16, 20, 24, 28, 32, 36, and 40
source-order bits. The schema ids are `projective-law-collector-record/v1` and
`projective-state-collector-record/v1` respectively. The same serialized
fields appear in both arms.

| Field | Meaning |
| --- | --- |
| `workload_id`, `workload_sha256`, `bits`, `p`, `target_index` | Frozen source instance and target identity. |
| `algorithm_seed`, `cap`, `planted` | Frozen seed, candidate cap, and public known-answer scalar. |
| `recovered`, `verified`, `exhausted` | Candidate scalar, recomputation on source `E(Fp²)`, and cap status. |
| `counts` | Full candidate cold arithmetic counters: `m`, `s`, `i`, `field_adds`, `bit_ops`, `ec_adds`, `jac_adds`, `sqrt_calls`. |
| `source_add` | One instrumented source elliptic addition in the same process; denominator for GAE arithmetic. |
| `native` | Logical and diagnostic counters. Keys ending `_uncharged` receive the native logical price; named operation counts without that suffix are already charged or diagnostic. |
| `native.weighted_normalized_outputs` | Calls to affine normalization inside the weighted law or at split boundaries. |
| `native.weighted_normalization_inversions` | Field inversions inside those normalizations, already included in `counts.i`. |
| `phases` | Construction, transfer, initialization, relation/elimination, and source-verification counts. |
| `rho` | Full same-process matched negation-rho record, independently source verified. |

`results/targets-<arm>.jsonl` is the normalized reviewer table with the raw
record path and representative per-target candidate, external, and rho GAE.
`results/price-grid-<arm>.csv` has one row per arm/slice/price cell: 36 cells
for each bit size and the pooled `all` slice. `S_per_M` is 0, 0.5, or 1;
`I_per_M` is 1, 10, 100, or 1000; `native_GAE_per_logical` is 0, 0.1, or 1.
`candidate_GAE`, `external_GAE`, and `rho_GAE` sum complete cold per-target
costs. Ratios divide these summed costs, not an average of per-target ratios.
`results/summary-<arm>.json` carries representative values and min/max ratios
across the 36 cells for each slice, plus source replay counts. These min/max
ranges are pricing sensitivity, not confidence intervals.

`results/price-grid-state-vs-wrapper.csv` joins the two candidate grids by
slice and all three price coordinates. It records the full cold costs for each
arm and `state_over_wrapper`; values below one favor retained weighted state.
There are 36 rows per bit slice and 36 for the pooled slice. The separate
native validator recomputes both inputs from raw target rows before writing
this direct grid.

`results/cold-phase-breakdown.csv` has one row per `study_arm`, `slice`,
`method` (`candidate`, `external`, `rho`), and `phase`. It includes all
arithmetic and logical-operation counts and representative-price GAE. Field
additions and bit operations are priced in their originating phase. Native
logical counters that cannot be attributed to a phase occupy
`native_logical_unallocated`; this is an accounting row, not additional work.
The phase GAE rows add to the full cold GAE for every arm, method, and slice.

`results/arithmetic-branches.jsonl` has one row per destination model. Every
`*_checked` count denotes an independent Cantor equality check. An
`*_unchecked` count is an input where that branch returned no result. The
`weighted_exception_2plus2_unchecked` count is the number of sampled inputs
where the isolated common helper rejected its prerequisites; the complete
dispatcher handled and Cantor-checked those inputs separately, as recorded
by `weighted_all_2plus2_checked`. The complete-dispatcher unchecked fields
are `weighted_1plus2_unchecked`, `weighted_all_2plus2_unchecked`, and
`weighted_targeted_add22_unchecked`.
The
`n6_w_power_regression` field in the `p=739` row records exact affine and
weighted input coordinates, the transcribed and homogeneous exponents, both
output divisors, and the independently computed Cantor output. A standalone
copy is in `results/n6-w-power-regression.json` for direct review.
