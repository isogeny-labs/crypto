# R77 quadratic infinity norm identities, frozen before native execution

Transform the checked R76 quadratic source equation using `t=1/z`,
`U=t^2 V`. In `F2[t,U]/(U^2+U+t+t^3)`, set `B=1+(1+t)U` and
`B'=1+(1+t)(U+1)`. Check the derivative in `U` is one; the two
specialized `t=0` roots are distinct; and the full polynomial
identities `B*B'=t^5` and `(B+t^3)(B'+t^3)=t^3(1+t)^3` hold modulo
the actual quadratic equation. Check that replacing `t^5` by `t^4`
is rejected. The formulas `R=t^-3 B`, `b=t^-7 B(B+t^3)`,
`c=t^-4(B+t^3)` then give local orders at the two etale infinity
places. The interpretation uses ordinary DVR valuation and the
degree-16 tower equality separately; the native program proves the
polynomial identities.

This is one exact deterministic local Singular run, one Singular/FLINT
thread, seed 77, timeout 1200 s and 4 GiB virtual memory. Source,
protocol and driver bytes must match frozen Git, and donor hashes must
match. Accept only exit 0, exact marker, zero whitespace-question-mark
native errors, and all assertions. Retain failed attempts and freeze
repairs before a fresh ID. No comparison or statistical claim applies.
