# Actual source jet decoder handoff

## What was done
Reviewed current shared log and difference13 source/proof/results. KB query,
Tian §4.2–5 primary-source review. Implemented jet-ring CRT translation with
constant checks, actual quotient section jets, Hessian decoder and common
projective differential-scale orientation; froze protocol before execution.

## Decisions made
Retain psi(T) origin shift. Scan fixed ordered multiples and retain exclusions.
Global sign calibrated once by first endpoint scale; all later cases and negatives
must agree. General public-input decomposition remains separate.

## Open threads
Build, commit/lock/source checkpoint, run once and report all outcomes. Then
construct full public-input transport with consistent difference decompositions.

## Errors
None so far; scoped taskT-6233 epoch1. Shared resume untouched.
