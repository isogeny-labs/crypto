# Pencil sparse-order review data

The [protocol](PROTOCOL.md) fixes the 21-curve, 1,260-target parent cohort and
the identity, reverse, and affine variable orders. The complete per-target
record, including failed solves, is the raw `results/p*-*.jsonl` file: the
first line is `pencil-cohort-setup/v1`, and every later line is one
`pencil-cohort-target/v1` with the original `workload_id`, workload SHA-256,
target index, algorithm seed, planted scalar, recovered candidate and rho
scalars, verification flags, failure code, factor-base size, pivot rank,
verified relations, modular row updates, variable-order operations, full
candidate field/native counts, phases, and same-process rho counts.

`results/replay/` contains independent archive-input reruns where available.
The validator removes wall-clock fields and, for old online records that
precede peel support, fills only three absent metadata defaults:
`solver_mode=online`, `solver_attempts=0`, and `solver_index_ops=0`. It then
demands field-level equality for every other non-wall field, including all
numeric operation counters and scalar checks. This is field equivalence,
not byte identity. `results/targets.jsonl` is a one-row-per-arm-and-target
index into
the raw files. It repeats verification and failure status, row updates, and
the representative cold-cost ratio. An absent or unsuccessful solve has a
`null` representative value; it is never assigned a price ratio.

`results/price-grid.csv` has one row per arm, public target and pricing cell:
`S_per_M` in {0, 0.5, 1}, `I_per_M` in {1, 10, 100, 1000}, and
`native_GAE_per_logical` in {0, 0.1, 1}. These 36 cells preserve the complete
cold candidate GAE, same-process source-negation-rho GAE and candidate/rho
ratio for every verified pair. `results/summary.json` aggregates each file's
target count, failure count, row updates, variable-order work, representative
price and full price-band extrema. No significance threshold is encoded.
`results/status.json` counts distinct source-verified workload IDs across
the new study arms, rather than counting repeated solver arms as new
targets. It lists every failed target row and keeps the uncompleted portion
of the original 1,260-target cohort explicit.

The native [price exporter](src/bin/price_sparse.rs) follows the incumbent
PENCIL pilot's operation model. It charges full factor-base construction on
each one-target candidate, all recorded field operations, modular row
updates, and the new variable-order setup. For affine ordering, its native
setup charge also counts the deterministic gcd and stride-selection work
from the frozen order definition. The independent [validator](src/bin/validate_sparse.rs)
recomputes source scalars, checks archived workload and rho controls, compares
normalized non-wall replay fields, and recomputes every published price cell
from raw
counters. The validator result is `results/validation.json`, with a separate
`results/replay-certificate.json` indexing each raw/replay pair. A running
or missing replay gives `partial_replays_pending`; only complete matching
non-wall replays yield `pass` and `verified` respectively. The validator's
`headline` list records each paired representative ratio as published and
as independently recomputed from raw operation counters.

Elapsed wall time is a level-0 diagnostic: multiple independent targets ran
concurrently, and the host also ran other repository tests. The graph and
PDF only draw values already present in the native price summary. They do
not add statistical or asymptotic claims.

The `solver_mode` field distinguishes `online` from `peel`. The peel mode
retains each exact verified relation, repeatedly constructs its incidence
index after a completed walk, removes degree-one rows, then eliminates the
remaining core by minimum live variable degree. `solver_index_ops` records
the charged logical index and row-copy operations across all attempts; the
index price is varied over the same three native-logical prices as the other
recorded logical work. An index operation is a high-level map, set, heap or
row-copy event; the counter does not separately count tree comparisons,
allocation, or cache traffic. The 0/0.1/1 GAE band is therefore part of
the result, rather than a hardware-time claim.
`solver_stored_nonzero_coefficients` is the maximum recorded live
coefficient count. A `sparse_matrix_cap` row has its candidate
scalar and all candidate cost ratios null, even when its rho control solved.

Each `results/p*-peel-vs-online.csv` contains the 36 paired aggregate cells
for the identical 60 workload IDs on one curve. Its JSON companion retains
the representative candidate/online and candidate/rho ratios, all-cell
range, and the counts of cells below online and below rho. Both the cells
where peel costs less and those where it costs more remain in the grid.

The optional `matrix_coefficient_cap` field records the configured maximum
number of live nonzero coefficients for later runs. Earlier runs without
that field used 10,000,000. The later 25,000,000-cap diagnostic is a
separate resource extension, with the executable revision and external
wall/memory caps in its run record. It is never pooled into a 60-target
curve summary unless all 60 distinct targets actually complete.
