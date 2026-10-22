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
The validator removes only wall-clock fields before demanding exact record
equality. `results/targets.jsonl` is a one-row-per-arm-and-target index into
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

The native [price exporter](src/bin/price_sparse.rs) follows the incumbent
PENCIL pilot's operation model. It charges full factor-base construction on
each one-target candidate, all recorded field operations, modular row
updates, and the new variable-order setup. For affine ordering, its native
setup charge also counts the deterministic gcd and stride-selection work
from the frozen order definition. The independent [validator](src/bin/validate_sparse.rs)
recomputes source scalars, checks archived workload and rho controls, compares
non-wall replay fields, and recomputes every published price cell from raw
counters. The validator result is `results/validation.json`.

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
recorded logical work. `solver_stored_nonzero_coefficients` is the maximum
recorded live coefficient count. A `sparse_matrix_cap` row has its candidate
scalar and all candidate cost ratios null, even when its rho control solved.

Each `results/p*-peel-vs-online.csv` contains the 36 paired aggregate cells
for the identical 60 workload IDs on one curve. Its JSON companion retains
the representative candidate/online and candidate/rho ratios, all-cell
range, and the counts of cells below online and below rho. Both the cells
where peel costs less and those where it costs more remain in the grid.
