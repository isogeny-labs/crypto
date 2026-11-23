# Opt-in pair-collision harvest implementation contract

Frozen 2026-11-16 after the exact census and before changing the production
builder, oracle or matrix. This contract covers implementation and correctness
checks only; it does not authorize a local speed comparison or alter the
frozen EKS L2 performance panel.

## Change

Add an explicit `rootfind-negfold-batch-harvest` engine. Preserve the existing
`rootfind-negfold-batch` engine and its record semantics. During the same
batched table build, save every later pair whose sum abscissa matches a first
witness. After the table is built, reconstruct both full sums, distinguish
equal from opposite signs, verify the resulting four-point group equation,
and map it to a homogeneous signed-Frobenius column row modulo `r`. Fail
closed on any mismatched sign or group identity. Add the rows to the ordinary
relation matrix before target trials, RHS 0 and target coefficient 0.

Charge extra full point additions and verification to oracle setup. Charge
row construction and elimination to linear algebra, including seed wall time.
Expose duplicate, nonzero, independent and dependent counts separately from
target-bearing relations. The old engine must remain byte-compatible.

## Checks

On fixed public K13/d5, K19/d7 and K23/d8 bases, require the new builder to
produce the old table's entry and insertion counts and the census's duplicate
counts 27, 99 and 201 and seed ranks 5, 11 and 23. Require each seed row to
be a verified full group equation. A deliberately modified witness must be
rejected. On a fixed public known-answer target for K13/d5, run old and new
engines with the same seed, target source, max trials, factor base and matrix;
check both recovered scalars and full group verification, and record trial,
target-bearing relation and phase counters. If the initial targets or table
keys differ, classify the comparison invalid. Any failed control blocks a
performance interpretation. [pcc:unverified]

No local wall ratio, relation-count difference, or isolated subroutine gain is
an end-to-end speed claim. [pcc:unverified] A quantitative speed claim requires a separate
same-host frozen comparison, strongest available rho control, complete phase
costs, independent validation and a passing `claims/*.json` PCC record.

## Evidence

The exact census is `reports/run-20261116T0711Z/`, source commit
`880d8717cce6b69ac6281192a6456a1c5c8758b3`. The current KB revision
and primary source scope are in `PROTOCOL.md`; duplicate-sum harvesting is
already implemented in the adjacent folded table, so this work adapts it to
the negation-folded batch builder and framework matrix.
