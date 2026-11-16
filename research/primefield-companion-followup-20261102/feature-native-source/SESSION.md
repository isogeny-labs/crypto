# Native source integration — T-1486

## What was done (commands, files, features)
Read historical shared-projective-grid/evaluator.py and normalized-partial-block/run.py; reviewed FLINT nmod_poly documentation. Joined checked geometry, phase, value division and polynomial backends. Public-point and joint-window APIs include calibration, partial blocks, singleton tails and identity handling.

## Decisions made (and why)
Keep homogeneous pairs until final quotient. Keep original product/remainder tree. Phase currently computed per finite point and value-only engine separately initialized; outer seed propagation is not silently replaced as a performance claim. Original 4563-case cohort preserved.

## Open threads (unfinished, pick up next time)
Run fixed integration checks; connect outer seed propagation and decoder, then complete counted native runner and authorized EKS admission.

## Errors (what went wrong, how it was handled)
No new cloud work. One guessed component path absent; used actual native-window and decoder-binding paths.

## Checked integration

### What was done (commands, files, features)
One native build and filtered invocation exit0, three integration methods pass. Three saved raw records and three identity-crossing windows agree. Six source/dependency hashes and five original inputs pass.

### Decisions made (and why)
Source commit completed after tests due shared-filesystem latency; receipt binds frozen pretest hashes and records the ordering. No repeat commit or test was launched.

### Open threads (unfinished, pick up next time)
Outer seed propagation, retained prepared strides, native decoder integration and complete costs. Add numerical no-partial/no-singleton coverage.

### Errors (what went wrong, how it was handled)
No build/test failure; one unused diagnostic-field warning retained. No cloud job or original cohort change.

Daily-log scope readback: T-1487 holds docs/session-logs/2026-11-03.md; no edit attempted. One-page PDF rendered and inspected.
