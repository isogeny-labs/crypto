# R79 bounded candidate-special-fiber probe, frozen before data

The R50 finite-`u` normalP output from `normal-relative-finite50-01`
is a **rejected candidate**, not an accepted normalization. Its
22-variable, 400-relation `normalization50.sing` is retained with an
exact hash. Load it, require characteristic two and the literal
source map `(x,y,p)`. First substitute `p=0` into all 400 relations;
then compute the zero-dimensional standard bases of the `y=0` and
`y=1` candidate quotient fibers, each with `p=0`. Record their exact
`F2` dimensions and verify the R78 specialized source polynomial
reduces to zero in both. Require their dimensions to sum to 32.
These checks are **candidate-fiber diagnostics** only. They do not
verify the global normalP producer, `norTest`, reducedness, equality
with the source field, or normal branch ramification.

One deterministic local Singular run, one Singular/FLINT thread,
seed 79, 1200-second timeout and 4 GiB virtual memory. Copy and
verify committed source/protocol/driver bytes and all donor hashes.
Accept only exit 0, exact marker, zero native whitespace-question-mark
errors, and all assertions. Preserve timeouts/failures; freeze repairs
before a fresh attempt ID. No comparative or statistical claim.

## Version 2 constructor syntax repair, before attempt 02

Attempt `candidate-u0-79-01` emitted Singular parser errors at the
nested `ideal(specialized79,p,y)` constructor before computing either
quotient dimension. Exit code zero and a printed marker do not override
the strict error-line gate; retain the rejected raw log. Declare the
two base ideals explicitly and pass each to `std`. No input relation,
quotient target, resource cap or acceptance condition changes.
