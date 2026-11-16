# Oriented cross-radius recovery qualification

The optional adapter retains relative oriented-mode information across radii.
It normalizes the observed plus mode as n_b=b*chi(b)*fplus(b)=m*k*F1(b^-mY),
using the character power already computed for the existing constraints.
A nonzero-product anchor supplies n_a*F1_b-n_b*F1_a=0. Explicit zero-mode
constraints apply separately. The powered path reuses sections; the streaming
path exposes factors from its existing paired pass. PCC-20261108-704e15 is
unsigned producer evidence. [pcc:unverified]

Validation03 passes23552 exact comparisons against the direct-sign oracle
and23552 predecessor support-inclusion checks. In4024 checked states the
support is a strict subset of the predecessor; all4024 are also strict
subsets of the separate zero-only oracle and occur after the first radius.
These are state comparisons across scalars, signs, capacities and rounds,
not4024 distinct targets or counts of removed candidates.1824 checked states
have a delayed anchor. There are11616 true-image retentions,11616 wrong-sign
exclusions and320 zero exclusions. [pcc:unverified]

All3600 public target/capacity cases recover correctly, including228 odd-L
cases. Among3372 order-281 predecessor comparisons,440 routes and400 call
totals differ. The new path performs2220 subsequent updates in these fixtures
and respects298 single-call guards. No direction or timing ranking is inferred
from the changed route/call counts. [pcc:unverified]

Fixtures preserve the balanced-lift configurations, both initial signs,
capacities0/16/96/1024 and radii2/1/3/2. The oracle adds zero-mode conditions
and correlations from the first product-nonzero target observation onward.
It does not pretend to encode the entire oriented transcript. Enumeration
remains confined to correctness controls; runtime consumes public points.

Validation01 failed to compile because OrientedBranch collided with an
inherited type. The snapshot and errors are retained; RadiusAnchorBranch
fixes the collision. Validation02 passed with the original b^(h-1)
normalization; its sources and receipt are preserved. Validation03 reuses
chi(b) and has byte-identical focused and inherited outputs. The fresh
build/focused/inherited checks pass within90/60/60-second bounds.
146 copied files and162 compiler dependencies are hash-bound. [pcc:unverified]

Additional anchor storage/copies, scalar normalization, conjugate construction,
remainders and canonical intersections remain charged. There is no extra
coefficient pass or character query for the same processed radius, but
changed pruning can change the complete recovery route and total calls.
Initial phase-domain expansion is unchanged. Independent signed review is
pending. Next apply the relative-mode information before dense phase expansion
or retain it in an implicit multi-radius constraint representation.
