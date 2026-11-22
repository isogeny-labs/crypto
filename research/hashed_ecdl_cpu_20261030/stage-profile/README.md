# Complete CPU stage diagnostic mode

Round21 identifies oracle calls and reduction/other as substantial intervals,
but reduction/other is solve time minus the outer oracle/base timers. It includes
target generation, accepted-relation checks, matrix work, and internal replay.
It cannot be interpreted as elimination time alone.

The complete example now accepts an optional final --profile-stages argument.
It uses the existing solve_with_oracle_profile entry point and writes
stages-ARM-CASE-ROUND.csv beside the ordinary complete certificate. It exposes
solve, bases, targets, oracle, linear, internal replay, external replay and
complete milliseconds. Targets includes fixed-pair setup and relation checking.
Linear includes matrix construction, dependency and extraction. Stage clocks
do not cover every statement in solve and must not be presented as an exhaustive
partition or summed to replace the separately recorded complete interval.

Extra clocks perturb execution. Both output rows are labeled L0-profile; the
frozen round21 analyzer rejects this label. The ordinary invocation retains
the existing L0 label, timer boundaries and solver entry point. No frozen
round21 source, binary, protocol or result was changed. The independent second
replay, corrupted-scalar rejection and full certificate output remain mandatory.
The next measured experiment needs a new frozen protocol and bounded admission.

## Evidence and prior work

Source review: src/cryptanalysis/hashed_ecdl.rs ReductionStats and
solve_reduction; examples/complete.rs solve-minus-oracle accounting; round21
results/summary.csv and the existing projective-replay notes. This exposes
existing instrumentation rather than proposing a new arithmetic method.
The existing optional_profile_preserves_queries_certificates_and_stats test
checks identical query order and certificate content for the two solver paths.

## Validation

Standalone release library and complete-example tests pass: 46 library tests,
including profile parity and archived full certificate equality. Strict
complete-example Clippy and formatting are checked in the adjacent logs.
Only the small standalone CPU crate was built, not the root crypto/CUDA project.
These are software checks, not performance measurements.

## Handoff

T-1272 adds diagnostic visibility before selecting the next larger change.
No cloud job or new timing sample was launched. Round21's one attempt remains
consumed. The current PR's integration harness policy failure is separate:
the missing optimization.rs file is not in this branch HEAD; upstream repair
68a78284d742 must be reconciled against the integration base, not applied as
an invented new local source module. Initial rustfmt check required wrapping
the extended usage string; it was corrected before final validation.
