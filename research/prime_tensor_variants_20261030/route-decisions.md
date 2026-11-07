# Decisions after checking the proposed shortcuts

This is a research record, not a proof that all possible ECDLP routes fail.
Full written arguments and counterexamples are in the reviews. Unformalized
statements remain identified in the proof map.

## Keep: ternary packets as a testable internal variant

The order-three linear action supplies real locality, uniform orbit fibers and
an exact deformation. Unlike a speculative solver-degree assumption, its
central operations can be checked directly and proved structurally. The
non-binary target multiplicity exposed an actual defect in the first proposal;
target-specific modes repair it. A one-level unit checks composition with
interpolation and recovery rather than only checking a symbolic identity.

The payoff is a separately specified internal variant of the existing method.
The same exponent is not an asymptotic improvement or a distinct ECDLP
mechanism. No runtime comparison has been conducted, and there is no basis for
saying the coefficient prime 3 is faster than 2.

## Do not promote: sharper weights alone

At a binary active level write `D=L/2^i` and `A=a+1-i`. Exact geometric-mode
coefficients have a stronger valuation than the crude weight `b+1`. The
review nevertheless constructs many admissible tuples in a band
`b=Theta(D/sqrt(u))`, respecting the exact additive valuation and the listed
degree caps. The current enumeration bound does not improve in exponent class.
Candidate tuples need not have nonzero coefficients, so this is not a lower
bound on a cancellation-aware algorithm. A genuinely better algorithm would
have to exploit more than these weights and caps.

For nonnegative tapers supported below `u`, a maximum-principle calculation
forces the worst valuation loss scale to be at least `Omega(M/u)` under the
review's interpolation requirement. The broader arbitrary-node discussion has
not received the same independent structural/formal check; do not turn it
into a universal taper lower bound.

## Correct: multivariate code occupancy is not a general barrier

For a literal two-axis product, specializing the passive axis leaves a
unit-times-identity block of rank `D/2`. Its channels restore the mode count
lost by naïvely dividing by an extra `D`. This explains why that direct product
does not automatically improve the one-dimensional exponent.

An attempted broader Singleton argument required two extra premises: that real
packet occupancy is `O(D^k)`, and that passive interactions cost a specified
power of `D`. Code distance proves neither premise. In a two-coordinate
repetition code the two accepted orientations repeat across `D^2/2` physical
slots; the three-coordinate version has `D^3/4`. These are counterexamples to
the occupancy inference. They do not themselves supply all the desired
locality properties or an improved contraction. Small-dimensional correlated
packets therefore remain a research question rather than a proved dead end.

## Do not promote: a compact verifier without charging copies or degree

The public scalar predicate is cheap to evaluate and has singleton support,
but a degree-`b` star fails the degree cap at a useful size parameter.
Windowing and checking output chunks honestly costs `O(mb)` Boolean edges,
including the repeated scalar inputs. Its degree is
`O(b/m+b^(3/4))`; balancing this with the existing `S^(9/10)` cap gives
`S=O(b^(20/19))`. The cited upper bound then exceeds a linear exponent
asymptotically. This is a failure of that application, not a runtime lower
bound for ECDLP.

The division-polynomial review also distinguishes global rational-map degree
from agreement on a finite subgroup, and a short arithmetic program from a
low-degree function. It does not rule out an endomorphism or a new compact
certificate with different structure.

The [heavy-predicate follow-up](reviews/heavy-predicates.md) sharpens collision
variance to `P(1-q^-2)+(S2-2P)(q^-1-q^-2)`, with `P` the sum of squared endpoint
multiplicities and `S2` the sum of squared vertex degrees. Common uniform
padding still demands `q<=3M/delta` for each indivisible vertex. For these
windows it forces `a,u=O(log m)`, so even the optimistic unchanged mode bound
has exponent scale `bm/sqrt(log m)` and supplies no sub-exponential guarantee.
A sparse singleton-packet example also retains all mode subsets after exact
valuation charging, yet its terms have a simple symbolic aggregate. It refutes
the unchanged counting shortcut, not a structure-aware algorithm. An exact
public weighted-extension or splitting lemma for the scalar predicate remains
the missing constructible operation.

## Do not promote: low local Fourier degree as low dense width

The literal two-layer transform contains a complete bipartite block of word
interfaces. A weighted balanced cut gives at least a `4/9` fraction of its word
pairs across some contraction-tree cut, hence `Omega(b)` Boolean boundary
indices. Dense materialization is expensive in this representation even
though each local degree is smaller. Symbolic contraction, support pruning,
input conditioning, and representation changes lie outside the argument.
The isolated unconstrained multiplication network can even have a trivial
symbolic count. This is why graph width alone is not an ECDLP lower bound.
