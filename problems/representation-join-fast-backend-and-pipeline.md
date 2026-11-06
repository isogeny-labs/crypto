# Representation join fast backend and production collector integration

- **Status:** open
- **Kind:** implementation gap; access/tool limit for root validation
- **Affected work:** [reference module](../research/representation_join_20261021/README.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-21

## Problem

The native reference cannot establish the paper's asymptotic cost or a complete
index-calculus improvement because it evaluates every candidate pair and is
not wired into the existing production collector, rank, LA and descent path.
The implemented domain is odd quadratic fields with 3<p<=101. General ell,
large fields, prime-field analogues and binary-field arithmetic remain open.

## Evidence and uncertainty

- Observed: the linked study's final-check and sanitizer records qualify
  bounded exact-output joins and public known-answer targets. Initial fixture
  errors and the LeakSanitizer sandbox failure are retained separately.
- Literature: DFKM 2020, HAL hal-02427655, sections 3--5, requires fast
  polynomial multipoint evaluation and carefully balanced scalar sets.
- Missing: symbolic product-polynomial construction, Kedlaya--Umans backend,
  optimized scalar partition construction, production collector binding and
  independent arithmetic replay. Cargo and Conductor are unavailable locally.
- Not established: asymptotic acceleration, practical speedup, ordinary-query
  yield, or any deployed-curve implication. No impossibility is proved.

## What would unblock it

- [ ] Implement the polynomial backend and paper scalar-set construction,
  preserving complete outputs against the exhaustive reference corpus.
- [ ] Bind verified witnesses to the existing native collector with exact
  subgroup validation and multiplicity/sign-preserving row construction.
- [ ] Independently replay arithmetic and run the root required test suites.
- [ ] Freeze and execute the repository's full-cost, matched-target protocol
  on an authorized measurement host before reporting any improvement.

These establish progressively wider implementation and evidence coverage;
passing a toy test alone does not discharge the subsequent performance gate.

## Next discriminating step

Implement a symbolic zero-join product/evaluation backend behind the existing
join API; run the exact p=5,7,11 witness corpus with the current reference as
control, under its 1,000,000 pair/output caps. Stop on any output mismatch.
Equal outputs qualify correctness only. A mismatch requires a corrected
backend and retained failure. An infrastructure failure remains a blocker.
Recheck when the backend or production adapter is available.

## Resolution and history

2026-10-21: reference and callable row adapter added; fast backend and full
pipeline remain unresolved. No root-suite or performance pass claimed.
