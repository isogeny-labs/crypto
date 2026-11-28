# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (linear algebra for m = 3 at |F_x| 2048/4096)

## Goal
User (2026-11-15), "Keep working on open leads" after PR #4293: the open leads were linear algebra
(dominant in the m = 3 solve at |F_x| = 4096) and a vectorised multi-threaded CPU sweep.

## Lead / hypothesis
PROTOCOL.md: incremental-gauss-echelon cuts LA row_ops to <= 0.7x incremental-gauss at 4096 with the
same pins, logarithm and S; structured-gauss measured without a prediction.

## Spec
specs/la.json; local ecbench (this container, L0; counts deterministic).

## State
2026-11-15 17:05Z: done. 42/42 verified+replayed; echelon 0.10-0.15x gauss row_ops, identical S; record closed; REPORT.md.

## Runs in flight
None (local run, no pod).

## Stop and success conditions
PROTOCOL.md "Predictions" and "Measurement and reading".

## Next steps
PR; GPU arms with linalg=incremental-gauss-echelon on a pod; validator.
