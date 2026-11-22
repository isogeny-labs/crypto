# Independent bounded S4 recurrence review

Reviewed external patch SHA256 `ceb83dc7a7c3e57b113106aadc9b5b3b3c0243e4ae9927855acc7a25c952f454`, base/draft source, review notes, and retained-fixture arithmetic. No source-level blocker found. No worktree edits, patch application, build, Rust tests, CLI or cloud execution.

## Algebra and word bounds

The original table has reduced[i][j] = z^(i+j) modulo the original modulus. Therefore reduced[n-1][1] is precisely z^n modulo that modulus. Starting at the original n-bit coefficient f, each shift/mask and conditional XOR by this tail computes multiplication by z in the same polynomial basis. By induction row[i] equals the old dense sum of f_j*reduced[i][j]. This is a quotient identity and does not introduce an irreducibility assumption.

Public build validates n in 23..31 or 47..61 and every selector/target coefficient below 2^n. Table values and subsequent field operations preserve that invariant. Consequently coefficient>>(n-1) is zero or one, its wrapping negation is zero or u64::MAX, and all shifted values fit u64 (highest intermediate bit at most 61). The last stored row is z^(n-1)f; the final unused recurrence update is also in bounds. Empty right support creates no rows, while the unchanged pair/support limits bound allocation and arithmetic sizes. Canonical support ordering, cancellations, Boolean mask unions and final coordinate order are untouched.

## Actual removed work and reservations

This removes the actual repeated dense scans for each multiplier row; it is not a tariff-only change. The pre-existing coefficient 32 multiplies a new fixed eight-step row allowance rather than n scans of up to n coefficient bits. Eight modeled steps correspond to store, carry shift, carry negation, data shift, mask, tail mask, XOR and loop/control; this remains a modeled convention, not an instruction counter. Existing fixed setup, pair, sorting and output terms remain.

Gate::admit still occurs before any output/row allocation or mutation. Work and byte limit failures have no new reservation, observer failure retains the complete admitted reservation, and new per-row deadline checks retain already admitted work. Byte formulas, global ceilings, support/pair caps, expression order and observer stage positions are unchanged. Parent-storage tariff remains 32. No budget reset or renewed deadline is introduced.

Independently recomputed all 17 saved-fixture Product quotes from the listed supports: old total 5,394,624, proposed total 1,152,928, difference 4,241,696 modeled work. This verifies the conditional arithmetic for identical supports, not a new execution result or downstream completion.

## Meaningful controls and remaining validation

New row controls compare against the removed dense algorithm at all 24 requested degrees, all basis coefficients plus zero/dense/top-bit/repeated cases, and dense/high-tail quotient edges. Complete FieldSum comparison checks coefficient/support/order; exact and one-less work/byte gates, expired deadline and observer failure test reservation behavior. All 17 Product observer stops check exact admitted event prefix.

The final revision also expands BOTH existing canonical full-equation comparison against sym_semaev_s4 and independent scalar/resultant comparison to all 24 requested degrees, with three target choices and all eight selector assignments. This resolves the initial representative-degree coverage gap. These Rust controls remain uncompiled/unrun in this review. Current-source component and actual worker/both-twist controls must be executed by the integrating owner before runtime validation is claimed.
