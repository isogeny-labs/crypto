# Canonical Koblitz component to outer outcome gate

The test-only `verify_outer_outcome` consumes already authenticated canonical
components. It checks outcome labels after the component verifier has joined
the original equations, signed arithmetic, native boundaries and witness
publication. Original execution admission is still required at runtime dispatch;
this function neither creates an execution allowance nor resets a deadline.

A completed label requires the reported completed group phase, independent
replay of all signed tuples, and either a published verified witness or an
exhaustive negative finding on that finite domain. A verified status additionally
requires the corresponding explicit positive or negative label. Partial status
does not claim solver completion. Parent persistence errors prevent completion.

The actual supervisor's timeout, trace-budget and failed-exit transformations
are exercised on both engines' positive, negative and capped native fixtures.
These outer outcomes must have `solver_completed=false` and cannot promote
positive, negative, certificate, domain or native-completion flags. The verified
component object can still establish a retained witness independently. The
optional `retained_pdp_witness_verified` field is checked against that evidence,
including on censored outcomes; it cannot create a witness on a negative run.

Declared claim fields require booleans. Null, numeric and string substitutes
are rejected. Exact fall degrees, hardware instruction counts, complete native
derivations and full execution claims remain unsupported and are rejected.

The combined canonical fixtures exercise this gate before their rehashed
artifact-mutation controls. This is a software validation boundary, not central
replay registration or runtime activation. Parent artifact storage accounting,
original-bound runtime admission, central verifier wiring and CLI coverage remain
required before generation through this adapter.
