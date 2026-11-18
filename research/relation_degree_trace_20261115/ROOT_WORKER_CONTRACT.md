# Actual-root worker: development contract

This extends the integration with a native executable linked to the actual
root library. It is preparation for H2 in INTEGRATION_HYPOTHESIS.md, not a
timing campaign. Scientific timing remains subject to the checkpoint and
isolation gates; no new launch, spending or performance result is supplied.

The existing study worker's baseline is a different solver. The new
`native-root-worker` reads one bounded System JSON from stdin and calls either
`f4_fp::f4` or `f4_fp::certified::try_quotient_basis` on the same admitted
equations. It never calls the study reference oracle, generates witnesses,
adds oracle rows or silently falls back. F4 must finish without timeout,
dropped over-bound pairs or early stopping before it can return a complete
status. The documented small-prime F4 scope is enforced explicitly; the
53-bit field remains unsupported by that control.

The returned basis is normalized and deterministically ordered in both arms.
The certified arm retains all API-internal original-input and native boundary
checks; their time is charged. Its extra provenance obligations are not
subtracted to match the F4 control. Both arms still need identical-output
comparison and mature-solver checking outside this worker before any runtime
conclusion. A complete status is a producer algorithm status, not a signed
independent validation verdict.

Common clocks cover bounded input read/JSON parsing, primality and shape
admission, conversion, actual-root construction/internal checking and output
canonicalization. Phase clocks are diagnostics. The charged runtime protocol
must additionally include process launch, serialization/output consumption,
matched basis comparison and external checking in a common scope; a worker's
single-call wall field cannot substitute for that comparison.

The regular F4 report's highest processed pair degree and highest productive
degree are engine observations. Only a complete F4 run receives an observed
solving-degree value; incomplete runs keep it null and retain raw diagnostics.
The certified arm does not report an F4/Macaulay solving degree. Neither route
infers degree of regularity from a final basis.

Development qualification uses only existing fixed seed-7 fixtures. Predict
that both worker arms emit the same full basis for the 31-bit three-variable
case; the F4 arm explicitly rejects the 53-bit control; and a capped F4 call
stays incomplete with null observed solving degree. Freeze source and use a
separate immutable copied test executable with distinct build/pre/post
manifests. Test elapsed times do not establish a runtime ratio.
