# Exact quartic support and an independent affine oracle

This is a correctness component, not an original-system solver or speed claim.
For K=GF(2^n), the fourth-power Frobenius is bijective. For c in K let
a=c^(2^((-2) mod n)); then H=X^5+cX=X(X+a)^4. If c=0 its only root is
zero with multiplicity five. If c is nonzero its roots are zero (simple) and
a (multiplicity four). Every ordered source tuple supported in a declared
F2-subspace V is therefore the one all-zero tuple, or the five distinct
permutations of (0,a,a,a,a), exactly when a belongs to V. Equivalently c lies
in the fourth-power image of V, obtained by applying fourth power to every
declared basis column. A Gaussian span test retains arbitrary non-prefix
bases; no scalar field labels are substituted for polynomial arithmetic.

The small independent oracle constructs field multiplication by coefficient
convolution and polynomial reduction rather than bit-shift multiplication.
It enumerates every unordered root multiset of size five in each domain,
multiplies its five monic linear factors, selects precisely quartic-stratum
polynomials, then restores all distinct permutations. This is complete
because multiplication is symmetric and every ordered tuple has exactly one
sorted multiset. All c in GF4/GF8/GF16 are compared for every declared prefix
and two non-prefix basis cases. The largest multiset sweep is C(20,5)=15504,
not 16^5 ordered tuples. Full source outputs are retained in raw evidence.

At degree83 with the original modulus X83+X45+X2+X+1, the low18 basis has
fourth-power image columns at exponents 0,4,...,68, without reduction.
Their rank is exactly18. The degree<=68 ambient span has dimension69 and
therefore codimension51; the full83-bit ambient space has codimension65.
These dimensions concern ONLY this coefficient stratum, never the global
73-parameter norm family. Sixty-four supplied coefficients include zero,
all18 basis images, c=alpha (unsupported), and44 fixed SHA-selected image
combinations. Inverse Frobenius and bit-span membership independently agree.
The full field setup proof is reused rather than reverified. No domain of
2^18 elements is materialized, and no actual norm cell is admitted here.

The synthetic affine oracle maps k<=8 binary parameters to fifteen bits
(five GF8 coefficient coordinates). It requires e1=e2=e3=e5=0 and e4 in the
two-dimensional image of V=span(1,alpha). The image's orthogonal equations
are obtained by exhaustive search of seven nonzero functionals. A row solver
carries original-row combination bits, so every inconsistent result has an
independently checked 0=1 certificate. On consistent results its particular
vector and complete nullspace generate a set compared with direct enumeration
of all2^k parameter values against the original coefficient map and allowed
quartic image. Three additional fixed packed coefficient maps guarantee
nonempty intersections, a free image coordinate and a coefficient-map kernel.
Explicit underdetermined, zero-variable and contradictory
fixtures cover both branches. Parameter counts are not root counts: maps may
have kernels. No synthetic affine map is claimed to be an actual norm cell.

All loops are statically bounded. The200000 predicate counter is not an inner
arithmetic-operation counter. Each predicate checks the80-second work limit.
Raw complete outputs are serialized/exclusively written before completion is
accepted. A late84-second report is demoted and the final print requires88
seconds; the coordinator separately supervises the entire worker at90 seconds,
4GiB inherited per-process address space and one core. Cap/failure/late means
inconclusive, not an empty root set. No global root exact count is returned.

The formula uses classical Frobenius and linear algebra. A future algorithm
must admit the actual affine norm map, retain coefficient-map kernels and
recover every original curve/auxiliary/exceptional branch before promotion.
