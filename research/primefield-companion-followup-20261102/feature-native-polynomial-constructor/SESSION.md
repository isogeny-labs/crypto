## 2026-11-05 — Nonlinear coefficient-domain constructor (T-1616)

### What was done (commands, files, features)

Implemented cubic feature construction in F_r[X]/(X^r-X), exact elimination and polynomial recomposition. All 47 optimized overflow-checked tests pass. At r479/q16: 697 columns, observed rank479, 478 output terms, 334342 augmented entries and 56196286 inner elimination updates. The q15 control correctly rejects with observed rank477. All479 inputs and eight public-source integrations pass. Independent check.rs reads the saved packet, expands unreduced polynomials, checks the exact X coefficient vector and all479 values, and rejects a one-coefficient corruption. Eighteen producer and three checker/input hashes bind the runs.

### Decisions made (and why)

Value and coefficient representations are related by an invertible evaluation matrix; neither uses secret scalar labels. This is a native reference and independent certificate, not compression. Derived a proposed Miller recurrence with one Frobenius anchor to build cubic columns in linear field work; it remains unimplemented. PR3469 merged during work, so publish this component on research/primefield-polynomial-constructor-20261105 as a new incremental PR.

### Open threads (unfinished, pick up next time)

Implement and test the characteristic-r recurrence anchor against this dense baseline, then address structured elimination. Dense solve and source/setup/verification costs remain explicit. Original4563-case cohort unchanged. General rank/compactness is not inferred from this fixture.

### Errors (what went wrong, how it was handled)

Four producer compiler warnings retained. Corrupted-packet checker failure is the intended negative control; no producer test failed. Canonical FLINT and linked GitHub source pages could not be fetched; the author-hosted documentation was read with its revision notice and the recurrence was derived explicitly. No cloud job or controller was launched.
