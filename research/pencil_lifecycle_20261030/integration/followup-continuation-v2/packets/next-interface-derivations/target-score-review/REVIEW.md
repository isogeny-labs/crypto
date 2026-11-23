# Independent target-score source review

Publication: `125c631e4005785e344167d8f699b4bf23415fe6`. Reviewed after publication with a source-first plan; reviewer has prior familiarity with SignedGraph and its formal components. This is independent checking, not a claim of blind review.

The five producer fixture groups replayed exactly: normalized JSON is identical to the published result. Five additional reviewer fixture groups passed, including three deliberately inconsistent constraints caught at the actual native assertion in `three_leads.rs:1602`. Panic messages in reviewer stderr are expected evidence; the reviewer executable exited zero only after checking that each assertion occurred.

The copied full native `three_leads.rs` and the full `three_leads_followup.rs` insertion-policy source match the published commit byte-for-byte. SHA256 custody and unchanged-input checks are retained.

## Findings

1. A zero-score edge can establish an anchor that makes a later edge recover a scalar. The producer's paired fixed fixture demonstrates this local behavior; the otherwise unanchored fixture returns no scalar.
2. The score omits constant `a`. Score zero therefore cannot distinguish a consistent zero scalar equation from a contradictory constant equation. Independent fixtures cover an opposite-sign same-root edge and positive/negative equal-sign anchored edges. Consistent constants return None; changed constants trigger the actual inconsistent-relation assertion. Score is not a consistency checker.
3. Independent equal-sign anchored and opposite-sign anchored fixtures with nonzero residual target coefficient return scalar6 over F7. The validity of recovered scalar as a target relation remains conditional on consistent input relations.
4. In `three_leads_followup.rs:347–356`, rank mode computes `(score > 0, score)` priority and then calls `graph.edge` unconditionally for the accepted unseen split edge. The current policy does not filter insertion by score. Priority subsequently affects the chosen walk state. No filtering repair follows from these controls.
5. The first producer score call records6 scalar additions and2 find visits, with0 scalar multiplications and0 scalar inversions. These are instrumentation deltas for that call only; constructor, insertion and subsequent work are excluded. They are not a comparative measurement.

## Scope and checks

Two fresh offline locked Cargo builds exited0. The original fixture binary and independent fixture binary exited0. Original output matches exactly. Three reviewer panic hooks report the same native assertion and are caught intentionally. No failed compilation or failed expected fixture occurred in this packet. Full source and fixture hashes were rechecked after execution. Five means fixture groups, not assertion count.

This packet contains fixed native controls and source inspection, not a universal Lean theorem. Conditions exercised are prime modulus7, canonical residues, signs±1 and specified small graph states. It does not establish arbitrary-graph termination, overflow bounds for all sizes, global rank optimality or comparative performance. No scientific cohort, curve generation, timing or cloud operation was performed. Repository files were not edited. Existing prospective formal or scientific review gates are unchanged.
