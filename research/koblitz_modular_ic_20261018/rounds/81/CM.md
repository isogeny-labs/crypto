# R81: explicit candidate F4 quotient maps

The four R80 degree-2 prime quotients now have complete coordinate
maps. Each chooses a generator `g` with `g²+g+1=0`, expresses all 22
ring variables in `{0,1,g,g+1}`, and checks the generated map ideal
equals the corresponding minimal prime by reductions in both
directions. Both frozen serial runs passed the strict native gate,
including all four ideal equalities; the exact 88 coordinate codes are
in `results.json` and the raw logs.

On `y=0`, both maps have `x=g`, `y=p=0` and differ only at auxiliary
coordinates `T(17),T(18),T(19),T(20),T(23)`. On `y=1`, both have
`x=0`, `y=1`, `p=0` and differ at those five plus `T(24)`. Thus the
two candidate factors over each source-model point cannot be
distinguished by `x,y,p`; the proposed splitting is entirely in the
candidate normalP auxiliary coordinates. That is a precise target for
the source-field and local integral-closure check.

R50's candidate remains rejected at its coefficient-descent gate.
These maps certify quotient ideals of that candidate algebra only;
they do not certify branches or ramification in the source function
field. No new Lean theorem is claimed for the native ideal-equality
computations. R78 already carries the audited Lean proofs for the
chart identity and source factorization.
