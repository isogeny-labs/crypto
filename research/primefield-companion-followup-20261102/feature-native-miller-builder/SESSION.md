## 2026-11-05 — Miller/Frobenius column builder (T-1623)

### What was done (commands, files, features)

Implemented the short polynomial-power recurrence, characteristic-r Frobenius anchor, zero-shift factorization and shared inverse table. All49 optimized overflow-checked native tests pass. Every one of1037 columns matches the dense reference across seven fields, including571 singular anchors; the r479 packet is byte-identical. Hash-verified independent checker replay passes polynomial and all479 input identities. Twenty source/data locks verify before and after build. Elimination/back-substitution/recomposition code is byte-identical.

### Decisions made (and why)

Preserve the dense reference and all prior records. Column building now has derived linear field work in r for fixed cubic degree, after shared inverse setup; no elapsed-time gain is inferred from test logs. The q15 OutsideSpan control remains. Extend the open PR3473 with implementation, derivation and inspected report.

### Open threads (unfinished, pick up next time)

Investigate whether the column recurrences support structured elimination or a reusable factor, retaining exact inconsistency controls. Dense O(r^2 m) solve, source costs and original4563-case cohort remain separate. Current-head CI and review remain publication gates.

### Errors (what went wrong, how it was handled)

No native test failed. Four compiler warnings are preserved. The previous independent checker's corruption control was not rerun; a fresh positive replay on the new packet was performed with the binary hash checked. No cloud submission occurred.
