# Opt-in batched standard-subspace lifts: correctness screen

Status: frozen before candidate implementation and tests. This is a source-level
correctness screen, not a timing experiment or a change to the frozen K13
F4/F5/F6 comparison. The baseline is
`build_standard_subspace_factor_base`, which lifts each abscissa separately.
The candidate uses the existing `Gf2::batch_inv` to invert the ordered packed
abscissae together, then applies the same Artin--Schreier lift and lowering.

## Fixed checks

- On Koblitz E0 degrees 5, 7, 13, 17 and 23, compare the candidate and
  baseline factor bases for polynomial-basis subspace dimensions 2, 3, 5, 6
  and 8 respectively. Compare ordered subspace elements, ordered points,
  signed orbit indices and all metadata that affects relation columns.
- Exhaust all packed abscissae at degrees 5 and 7. At degrees 13, 17 and 23,
  use the deterministic stream already in the repository's lift tests and
  compare the old and new lift paths with the point-lift cache disabled and
  enabled. Include zero, a nonlifting abscissa and a two-point abscissa.
- Reject invalid dimensions with the same errors. Any changed point, order,
  index, metadata or error is a failed candidate. A test failure stops the
  implementation; retain it in the report.

Run the focused release library tests and formatting check from a committed
candidate source. Preserve the command, source commit, stdout, stderr and exit
status. A full release library suite is required before publishing the PR;
record unrelated failures without silently reclassifying them as success.

## Accounting and decision

The candidate is opt-in and is not selected by ecbench. The current
`bitop/v1` factor-base record prices per-abscissa inversion; turning on a
batched path without updating that record would make the work ledger false.
Passing correctness checks establishes only equivalence of the factor-base
object on the fixed cases. The later integration must update actual inversion
and multiplication counts, retain exact factor-base identity, and run the
frozen same-target L2 and A/A comparison before any performance advantage is
claimed. No wall samples or speed ratios are collected in this screen.

Sources checked: `papers/rootfinding-decomposition/rootfinding_decomposition.tex`
(pair-table construction and complete-cost boundary),
`papers/ecdlp-index-calculus-program/sections/rootfinding.tex` (prior IC
engineering), `src/cryptanalysis/semaev_decomp.rs` (existing zero-tolerant
four-lane `Gf2::batch_inv`), and
`src/cryptanalysis/koblitz_index_calculus.rs` (current lift ordering). Shared
KB revision `9141e386254340a9bdba3036f2bb706a`, refreshed 2026-11-14,
predates the current source. Its rootfinding sources are
`source:17a1b67993e4b781bbf1198fd2598c2f` and
`source:e9db8cc2be204429e8fb1daa12a3324c`.
[pcc:unverified]
