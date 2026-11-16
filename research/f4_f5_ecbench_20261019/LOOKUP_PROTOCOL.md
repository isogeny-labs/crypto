# Column-index dispatch refinement, before candidate profiles

The first quadratic-tail source0acfc55b7f is frozen in frozen-iteration1,
with its benchmark and ecbench executable hashes in iteration1.sha256.
Its complete five-pair profiles place18% of n23 planted F4 instructions
inside F4ColumnIndex::get, whose representation branch executes per term.

Select that representation once outside the product loop and specialize
the existing direct parity packer for standard hash, mask hash and dense
indices. Preserve all row/column/rank/word-XOR counters, root order, exact
support rejection and regular-builder fallback. New tests force all three
representations and high-variable masks; no degree/cap/workload changes.

Before native measurements, compare the frozen first-tail binary against
the refined binary on n23 planted seed230, both solve-f4/f5, five interleaved
Callgrind pairs with identical quadratic-tail policy and previous GF2 flags.
Require complete verified roots and identical deterministic work counters;
retain the refinement only if median Ir falls by more than2% on both arms.
Profiles are instruction-only evidence, never a native-time qualification.
Per-call300second /whole1500second bounds remain. Keep every failure.

If retained, rerun the full native and ecbench panels on fresh dedicated
hosts with this refined source. The already launched first-tail jobs remain
intermediate evidence and are not relabelled as measurements of the
refinement. Their paired ratios are never multiplied with this experiment.
