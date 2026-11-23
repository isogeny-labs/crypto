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

## Checked construction and publication

Four native methods pass once;60 filtered. Ten source hashes pass. Protocol draft validates with zero findings after system-jsonschema and enum corrections; failure logs retained. No comparison execution or cloud job. Shared daily log is held by T1522, so this local handoff is authoritative for this component. Next matched instrumentation/runner and protocol freeze/admission.
