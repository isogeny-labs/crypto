# LOCAL33 failure separation, prospective exact diagnostic

The compact field5 attempt terminated exit0 with Singular errors, so it is
rejected; the sequential field83 attempt was not launched. Preserve both the
error and trailing marker. No divisor jets are authorized by that result.

Read the accepted R33 rational family and R32 Jacobian result, and recheck
Stacks29.35.14 https://stacks.math.columbia.edu/tag/01V4: an invertible
Jacobian must belong to a valid local presentation; rank alone is insufficient.
This diagnostic distinguishes derivative failure from redundant-equation
witness failure. Copy the SAME four source generators, small H and 4x3
Jacobian, omitting only the expensive syzygies. Native formal roundtrip checks
every copied object. At both fixed PAD33 points record all four maximal
minors, their unit ideal or failure, and H. Do not require a favorable result.
A generic Groebner calculation records vdim and exact source-to-basis and
basis-to-source lifts, with both identities verified. This is diagnostic,
not a flatness proof or local presentation. 4GiB/1200s per invocation, no
comparative timing, randomization, coefficient search or statistical claim.

Add a Lean algebra bridge for the presentation criterion: in a commutative
ring, a unit coefficient in a relation makes that relation's generator belong
to the ideal of the other terms. Compile/audit with the same pinned Lean,
mathlib, single job,4096MiB,1200seconds. This is a universal implication,
not evidence that the rejected native unit witness exists. No geometry or
native coefficient arrays are imported into Lean.

Diagnostic01 is rejected: Singular does not accept ideal(existingIdeal,existingIdeal). The generic Groebner/lift checks produced vdim32/basis6, but no point result is accepted. Use a typed ideal assignment for the same generator concatenation, preserve01, fresh02.
