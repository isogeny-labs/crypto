# Monic remainder specialization — T-1519

## What was done
Read original polynomial engine, native accounting evidence and FLINT nmod_poly
reference. Implemented unit-series initialization, linear-leaf Horner evaluation
and low-degree remainder reconstruction using existing Karatsuba/Newton machinery.
General divrem remains available. Two routed source/recovery copies differ only
in dependency paths; all earlier files remain immutable.

## Decisions
Separate bounded software correctness checks from the matched cost panel. The
experiment-protocol skill requires freezing before comparative data. Its checker
is absent locally and was found in canonical crypto-autoresearcher; do not claim
comparison validation or execution before all protocol/runner/admission gates.

## Open threads
Run fixed correctness checks, then finish and freeze counted comparison protocol,
matched instrumentation and runner. EKS admission must be reconciled before a
measurement submission. Preserve all original curves and arms.

## Errors
Protocol tooling and controlled-experiment skill absent in current checkout;
canonical protocol tool/schema/example retrieved read-only into external cache.
No native run yet, no cloud job, no original evidence mutation.
