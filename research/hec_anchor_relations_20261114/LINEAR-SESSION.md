# Linear-denominator working log, 2026-11-15

## Protocol freeze

### What was done (commands, files, features)
Read shared dated log and primitive study log, verified current dirty state
(only older shared resume edit), reviewed the generic normalization and checked
primary interpolation source. KB receipt and scope are in LINEAR-PROTOCOL.md.
Claimed T-6042for this study before editing; shared log/resume remain untouched.

### Decisions made (and why)
Previous turn was progress: a complete target solver, finite normalization/rank
improvement and replay. Eliminate generic GCD/full norm division in d2 through
the short denominator and exact high-coefficient recurrence, preserving every
tuple/sign/row decision. Benchmark only after this equality gate passes.

### Open threads (unfinished, pick up next time)
Implement/gate the specialization; then run a separately frozen fully cold
same-target comparison. Goal remains mathematical>=10x for genus2+.

### Errors (what went wrong, how it was handled)
No new error. KB lacks a directly relevant retrieved mechanism source; primary
paper was read directly. No inference of novelty follows from search results.

## Exact specialization and complete-solver gate

### What was done (commands, files, features)
Implemented linear.rs: a=b*H-S0^2*A, b=S0*x-S1; Gray-updated anchor probes
classify common factors without generic GCD, and high norm coefficients give
Qwithout full polynomial norm division. All2688curve/sign cases agree exactly
with subset a,b and generic normalized a,b,Q/masks;118common-factor and13
constant-denominator cases are covered, plus forced zero/constant controls.
An explicit solve_linear wrapper preserves all deterministic counts in the
12full generic solver runs, with both raw/projected group gates passing.

### Decisions made (and why)
The specialization changes arithmetic while preserving relation streams and
complete solver work. It is now eligible for a same-target cold screen. It
remains an application of established interpolation, without a novelty claim.

### Open threads (unfinished, pick up next time)
Freeze the cold protocol, implement a separate worker/stage without changing
old ARMS15/cohort/receipts, then measure all cold controls and candidate phases.

### Errors (what went wrong, how it was handled)
No mismatch occurred. These gate clocks are diagnostic, not performance evidence.

## Cold-worker preparation

### What was done (commands, files, features)
Implemented separate --linear-cold stage and rational worker arms while keeping
old ARMS15and its cohort unchanged. All25tests in the heldout example pass,
including exact generic/linear full-solver counts. New native collector pins
source, args and fixture SHA for the six-curve/18target/288cell protocol.

### Decisions made (and why)
Commit scientific source before building the timed binary. Keep collection/
reduction/plot generation native. Shared bench registry/custody tools handle
only generic metadata; they will not launch scientific workers or compute the
study's statistics. Existing progress metadata is separate from the study's
native numerical result path. Scoped metadata taskT-6049was checked/claimed.

### Open threads (unfinished, pick up next time)
Pin source/library/binary, open record, checkpoint, execute capped child stage,
collect/reduce all outputs and evaluate E6/E7. No result is known yet.

### Errors (what went wrong, how it was handled)
No new error. Old frozen raw data and closed records remain unchanged.
