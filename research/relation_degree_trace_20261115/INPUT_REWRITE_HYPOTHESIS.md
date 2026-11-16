# Lean input-containment check hypothesis

Frozen after diagnostic round 02 and before the code/test below. This is a
development hypothesis, not a new runtime or regularity result `[pcc:unverified]`.

The fixed six-variable native qualifier stops in input-rewrite with polynomial
terms 20, degree 9 and proof terms 55976, exceeding the unchanged 50000-term
proof cap. The coordinate rules and root polynomial already have checked
original-input membership certificates. That phase only asks whether each
original polynomial rewrites to zero; its expanded provenance is discarded.
Round 02's immutable receipt retains the failure `[pcc:unverified]`.

Prediction: run this containment phase on polynomial-only copies of the
already-certified rules, as the existing final Buchberger checker does for
input/S-pair normal forms. Keep all polynomial reductions, rule/root membership
checks, final basis certificates, reducedness and all S-pair checks. Do not
discard provenance when extracting new coordinate or final-basis certificates.
Do not change any degree, term, proof or reduction cap.

Test the fixed six-variable seed-7 system and existing three-/four-variable
cases on the requested fields. Compare full versus polynomial-only rewrite
remainders on a synthetic valid representation and an inconsistent input.
Retain the old immutable qualifier and all failures. A complete six-variable
basis must pass exact original-input membership and the native F4-boundary
checker before it is reported. If another stage exceeds its budget, retain
that failure and diagnose it; do not raise the cap or accept a partial basis.

This tests bookkeeping in a verification phase. It cannot establish a smaller
solving degree, lower regularity, a root-codebase runtime advantage or improved
relation yield. The cold root-F4 runtime hypothesis remains separately gated
and must include every conversion/certificate/check cost `[pcc:unverified]`.
