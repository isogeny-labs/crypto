# Native source integration — T-1486

## What was done (commands, files, features)
Read historical shared-projective-grid/evaluator.py and normalized-partial-block/run.py; reviewed FLINT nmod_poly documentation. Joined checked geometry, phase, value division and polynomial backends. Public-point and joint-window APIs include calibration, partial blocks, singleton tails and identity handling.

## Decisions made (and why)
Keep homogeneous pairs until final quotient. Keep original product/remainder tree. Phase currently computed per finite point and value-only engine separately initialized; outer seed propagation is not silently replaced as a performance claim. Original 4563-case cohort preserved.

## Open threads (unfinished, pick up next time)
Run fixed integration checks; connect outer seed propagation and decoder, then complete counted native runner and authorized EKS admission.

## Errors (what went wrong, how it was handled)
No new cloud work. One guessed component path absent; used actual native-window and decoder-binding paths.
