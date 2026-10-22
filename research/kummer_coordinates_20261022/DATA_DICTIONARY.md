# Fast-Kummer coordinate gate data dictionary

`results/generic-models.jsonl` has one row per prime model. Counts
`map_checks`, `xDBL_pass`, and `xADD_pass` refer only to four generic
degree-two doubles and six generic differential additions per model.
`torsion_actions` gives one 4-by-4 Fp projective matrix for each of five
branch-point classes, derived from checked sample pairs and tested on the
cohort. `exceptional_cases` preserves 56 cases/model, with `kind`, degrees,
`status`, exact coordinate witnesses and the independent Jacobian expected
class. `status` is `pass`, `zero_output`, `projection_inapplicable`, or `fail`.

`results/target-gate-{16,20,24,28,32,36,40}.jsonl` has 180 rows each,
one public frozen target per row. Every row binds `workload_id`, source
archive hash, size, prime, target index, algorithm seed, planted public
scalar, source scalar check and seven labeled pairs. Each case records its
two labels, source and affine-map checks, oracle signed-lift count, and
`coordinate_witness`: projective inputs, difference, expected sum, observed
output and Mumford expected class. Field coordinates are integers in
`[0,p)`; projective vectors are four integers, compared up to nonzero scale.
`null` observed output records a genuine zero vector from the published
projective xADD expression. It is not counted as a correct xADD output.

`results/coordinate-exceptions.jsonl` is the 175-case extraction of all
168 model and seven target non-pass rows, with the same exact witnesses.
`results/preliminary-shortcut-failures.jsonl` freezes the earlier attempt
to use a single signed-coordinate permutation. That attempt left 1,008
model cases inapplicable; the later general projective linear actions
reduced the final inapplicable count to 168. The earlier rows are diagnostic
history, not part of the final coordinate totals.

`results/summary.json` aggregates the final gate; `candidate_cost`,
`rho_cost` and `candidate_to_rho_ratio` are null because no fast signed DLP
collector was run. The earlier Kummer reference gate's 288 null price
rows remain in `../kummer_walk_20261022/results/price-grid.csv`.
