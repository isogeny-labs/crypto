# Selected moments of a terminal phase-quotient product

## Exact weighted pairing

Let R=F_r[Y]/(Y^L-epsilon), with the same certified split/separable phase as
the predecessor. For canonical representatives A=sum a_iY^i and B=sum b_iY^i,
0<=i<L, the j-th coefficient C_j of AB in R is

    C_j = sum_(i=0)^(L-1) a_i b_((j-i) mod L) epsilon^[i>j].

A product exponent is either j or j+L. This accounts for every wrap exactly
once. For sparse operands, iterate the smaller support and look up the one
required partner. Two selected coefficients need at most2 min(supp(A),supp(B))
probes. For L1 the two needed coefficient positions coincide, so evaluate once.
No output coefficient vector is allocated by this pairing kernel.

The desired moments are Tr(AB)=L C_0 and Tr(YAB)=L epsilon C_(L-1).
For L1, both use C_0 and the second moment equals epsilon times the first.
The code validates canonical nonzero map entries, preflights a separate probe
cap, and charges each matched coefficient product against the existing pair
budget. Missing partners still consume a lookup probe. Wrapped scalings and
trace scalings are additional arithmetic, not hidden in the monomial-pair count.

## Integration and exact scope

Evaluate circuit-node representatives and predicate factors as before. If
there are active constraints, multiply the earlier predicates into prefix A;
let B be the final keep factor1-E^(r-1). Replace only the final multiplication
A*B with the selected pairing. A zero prefix returns zero moments. With no
active constraints use the moments of1. The phase and predicate identities
from the predecessor make these two quantities the survivor count d and sum
of images. Because0<=d<=L<r, its field representative is the exact integer d.
If d1, the sum is the unique image. Apply the rational scalar chart and verify
the public point. If d>1, retain an unresolved result with the known degree;
this path does not reconstruct the whole candidate polynomial.

This removes the terminal product vector and Newton reconstruction from the
singleton path. It does NOT remove circuit-node values, predicate powers or
prefix products. A prefix may already be dense or even equal the final
indicator when later constraints are redundant. Therefore the claim is only
that the final product is evaluated through two selected coefficients, not
that every intermediate avoids the complete support indicator.

The old Caps.output degree limit is irrelevant to this two-scalar path: no
vector of length d is built. Terms, retained node slots, monomial pairs and
the new probe cap remain enforced. Matrix/polynomial tests and recovery
comparisons use deterministic fixtures; counters are not total-work or timing
measurements. Container validation, tree lookups, copies, allocation, trace
scalings, source setup/calls, point shifts and verification remain charged.

## Rank boundary for a universal feature compression

The bilinear form C_0(A,B) has matrix H with H_(0,0)=1 and
H_(i,L-i)=epsilon for1<=i<L, all other entries zero. H is an invertible
weighted permutation matrix, hence has rank L over F_r. The trace pairing
is L times H and also has rank L, since r does not divide L.

Suppose a universal exact formula for arbitrary dense A,B has the form
sum_(t=1)^K alpha_t(A) beta_t(B), where each alpha_t and beta_t is a scalar
linear functional of its own operand. Its matrix is a sum of K rank-one
matrices, so K>=L. Thus a constant number of separate linear features cannot
suffice universally even for the count moment. This is a bilinear-scheme
bound; it does not bound all nonlinear algorithms, structured circuit inputs,
preprocessing, or approximate methods. Sparse supplied inputs can still be
handled in support-proportional work, as the kernel controls demonstrate.

Bostan-Lecerf-Schost, Tellegen's Principle into Practice (ISSAC2003), sections3-4,
https://specfun.inria.fr/bostan/publications/BoLeSc03.pdf, develops transposed
linear polynomial operations. Their explicit distinction that nonlinear
precomputation remains unchanged informs this scope. Our cyclic pairing and
rank argument are supplied above; no novelty or generic whole-circuit speedup
claim. KB unavailable (no MCP/wrapper executable). Repository source is
feature-support-projector at7873133a9f9b.

## Next construction

Push selected functionals into the particular predicate-power and section
circuits, accounting for the operands or dual states that must be constructed.
A large-L sparse-operand check qualifies only the pairing kernel. The actual
large transcript still refuses its term cap before any terminal probe, so
that upstream representation remains the material dependency. A transposition
principle alone does not make the nonlinear circuit precomputation disappear.
