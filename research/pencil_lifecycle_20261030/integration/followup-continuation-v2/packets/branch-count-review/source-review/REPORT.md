# Branch counter events in the signed edge dispatcher

Producer T-1352; archive T-1355. The instrumentation follows constraint, anchor and same/distinct-root branches at source 9276a40901. It counts cycles, negative_cycles and zero_coefficient separately while retaining the previously modeled state and return value.

Result. Eleven named declarations compile. Removing the instrumentation recovers the existing constraint, anchor, same-root, union and complete dispatcher outputs. Each edge increases each branch counter by at most one, and negative_cycles cannot increase without cycles. Together with the earlier find-visit count, the counter update theorem gives exact integer values for all four UInt64 counters under explicit per-counter no-overflow bounds.

Graph. The plotted values are exact controls at modulus seven, one fixed input per case. Both return None. A new anchor increments (cycles, negative_cycles, zero_coefficient) by (1,1,0); a redundant constraint increments them by (1,0,1). These are kernel-checked branch values, not sampled performance data. No uncertainty estimate applies. The graph data binds each case to its theorem.

Checks. Eight controls cover new anchors, redundant constraints, their identical return but different zero count, the increment before an inconsistent assertion, an answer without a zero increment, an existing anchor with zero residual, and distinct-root unanchored/anchored cases. Nineteen named axiom readbacks contain only propext and Quot.sound. Three malformed inventories are rejected. A false equality between the two None cases' zero increments is rejected by Lean.

Scope. Counter projection and bounded deltas are structural properties of the hand-written model. Correct represented answers still use the earlier dispatcher's prime, canonical-word, forest and true-edge assumptions. The inconsistent marker records the increment before the source assertion; it is not a successful Rust return. Exact UInt64 updates require initial+delta < 2^64 for each counter. Rust compiler/profile behavior outside those bounds, allocation, stack limits and partial failures are not established. Independent source-fidelity review remains pending.

Evidence. Every module check used one thread, 4096 MiB and 120 seconds. Imported compiled dependencies were reused and hash-bound; no independent binary/kernel recheck occurred. Pre/post hashes cannot exclude transient concurrent edits. The first helper-bound proof and first anchored-size control failed; both are retained alongside successful attempts and the deliberate false theorem. No scientific execution, timing measurement, cohort change or budget renewal occurred.

Next construction. Compose the dispatcher and counters with component-owner/root-cardinality preservation and native storage representation. The geometric truth of incoming relations and full cohort/independent-review requirements remain tracked separately.
