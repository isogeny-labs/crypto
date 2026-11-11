# Branch-controls successor machine review

## Outcome

The sealed successor completed its three bounded phases in 287.195459435 worker
wall seconds. Four Lean modules exited zero, the deliberate wrong-branch module
exited one with the expected `False` goal unsolved, and all five child process
groups were reaped. The exact 13 requested declaration readbacks were present;
each reported only `propext`, `Classical.choice`, and `Quot.sound`.

This independently checks the native receipts and frozen source identity. It
does not rerun Lean, and it does not promote the theorem beyond its explicit
hypotheses.

## Observation

- Frozen source SHA-256:
  `d20691095b0e6f932a5f34f9107e09c56629b57da85ea94dea35c007d87f3a40`.
- Released coordinator record SHA-256:
  `bcbe1b0b3a1da832b45e3d1fed9dabfc6c8d8fe2afdc8c60b4be9fb1c054bdfe`.
- Published source revision:
  `d6365d5265bb24a3ffffa1560a13a66d04c1741a`.
- Calls and exits: `BranchRootMultiplicity=0`,
  `BranchCoefficientMultiplicity=0`, `IndependentControls=0`, `Readback=0`,
  and `ExpectedFailure=1` as declared.
- Pre-custody: 16,736 files and 3,111,892,318 bytes read and hashed.
- Post-custody: the same 16,736-entry map was byte-for-byte equal, including
  the retained imports and runtime maps.
- Pre/compiler/post phase wall times were 55.443576350, 185.603433031, and
  46.039223451 seconds. Every phase receipt says passed, no wait timeout, and
  terminal owner confirmed.
- Each systemd command retained 100% CPU quota, 8 GiB memory, zero swap,
  128 tasks, and control-group kill mode.

## The checked theorem scope

The principal theorem states that for a field, a simple branch root, a
separable quintic, nonzero coefficient determinant, two support divisibilities,
and vanishing of the support product at the branch, the two source support
polynomials and residual are nonzero and their three root multiplicities sum to
one. The rational control establishes those hypotheses for its fixed source
instance. The two negative controls show that the equal-support coefficient
choice has zero determinant and that the selected wrong branch is not a root.

The machine evidence establishes elaboration of these exact frozen statements
and exact named axiom readback. It is not an independent derivation of their
connection to other collector claims, and no separate binary kernel rechecker
was used.

## Custody boundary

The compiler-phase receipt itself reports zero dependency files checked. That
is not presented as compiler-phase custody. Custody instead comes from the
separate complete pre and post phases, which each read all 16,736 declared files
and produced equal maps. Those maps are observations of pinned shared caches;
they do not exclude a transient concurrent change that occurs and is restored
between observations.

## Accounting

The conservative charge is 600 worker seconds. Together with the preceding
600-second attempt, this yields 1,200 charged seconds and 128,400 remaining
under the 129,600-second campaign ceiling. The bound pre-launch ledger still
labels this attempt reserved; updating that campaign record is a coordinator
operation, not part of this review.

## Files

- `review.json`: standardized findings and boundaries.
- `review.csv`: one row per compiler call, named axiom readback, and custody
  summary.
- `flow.svg`: recorded phase wall-time diagram.
- `audit.py`: deterministic, solver-free receipt audit.
- `report.pdf`: rendered and visually inspected report.
