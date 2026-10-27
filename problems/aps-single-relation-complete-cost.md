# APS single-relation complete-cost validation

**Status:** open implementation/accounting gap, 2026-10-26.

The [bounded native study](../research/aps_extensions_20261026/README.md)
checks the paper's relation path and several extension mechanisms on tiny
public fixtures. Its F4 counter does not include complete factor-base/label
construction, symbolic polynomial construction, root decoding, sign search,
deduplication or final verification. `prime_fast` and the new expression layer
do not yet supply a complete common operation tally for this route. The JSON
fixture runner is not an ecbench/unified experiment export or independent
transcript replayer.

Consequently the current receipts establish bounded correctness only. They
cannot decide whether a partition, unordered-divisor presentation, sparse
orbit, covering family or hybrid solver reduces complete expected cost.
This is not a mathematical obstruction to an improvement.

The [blocked backlog card](../backlog/XP-20261026-aps-complete-cost-panel/README.md)
preserves the requested controlled comparison. To unblock it:

1. Instrument all phases and retries, implement independent transcript replay,
   and integrate native ecbench/export output. Verify that an intentionally
   expensive setup changes total cost, and that invalid relations fail closed.
2. Choose the exact curve/target/base populations, controls, effect threshold,
   sample size, seeds, runtime envelope and external solver comparison. Freeze
   a valid `experiment.protocol/v1` and its analysis on a merged code revision.
3. Retain the toy instrument's caps until a separately reviewed backend/cap
   extension supports the intended ladder. The current implementation does
   not cover m=5 or the full 11–22-bit paper ladder.
4. Supply the S3 upload credential through the environment, complete the
   resume record, push and pass the checkpoint gate before launch.
5. Collect every planned outcome, compare complete cost with the frozen IC
   baseline and strongest matched rho, independently replay and recompute
   headline numbers, and retain null/negative outcomes.

Related existing gap:
[`rootfinding-complete-cost-calibration.md`](rootfinding-complete-cost-calibration.md).
That work informs the accounting contract but is not a completed APS result.
