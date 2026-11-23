# Exact small filtered-closure development controls

Written before these controls, 2026-10-30. This develops a native verifier,
not an n83 production experiment. Preserve the full three-family m2–9 goal.

Use m=3, ell=3, normalized selector weights (1,2,4) in each of three blocks,
one unrestricted intermediate U, curve coefficients a=2,b=3, and primes
101 and103. For each field test targets1,7 and sum(weights)/2. The terminal
witness is g=NF((z6-z7)*S3(U,X2,t)). Compute V4 and V5 directly, with all
input relations and complete degree-bounded closure, not a single initial
Macaulay rowspace. This is a synthetic polynomial-system test; rational curve
decomposition and actual n83 case identities are not asserted.

The monic selector equations z_i^2-w_i*z_i are handled by their exact normal
form. Every reduction has a derivation bounded by the original monomial
degree. Thus the image of V_d can be computed in the selector quotient:
start with both normalized chain generators, reduce by graded pivots, and
multiply every newly independent row of degree below d by every variable,
normalizing after multiplication. Iterate until the queue empties. A graded
echelon basis spans its low-degree subspace by its low-degree pivot rows;
all such rows are processed. This establishes closure under all allowed
multiplications. No U field equation is introduced.

Positive controls: for nonzero delta, g must be absent from V4 and present
in V5. Exceptional targets: record membership without assuming zero pairing
means membership or absence. Negative/algorithm controls: changing the
witness, checking original generators, testing every final low-degree basis
row's variable products, and a system {U^2,U^2-U} that requires a second
closure step. Keep a hard dimension guard and process timeout; a cap is
inconclusive, never a degree result. Fields are fixed small primes; native
u64 arithmetic is bounded to p<=65521.

Retain every monomial column and sparse normalized basis row in JSON so
membership and closure can be independently reconstructed. Summary rows
include exact input coefficients, witness, ranks, processed queue counts,
membership and the degree limit. Run the exact executable again after S3
restoration and compare arithmetic JSON bytes. Development test logs and
all failed attempts remain saved. No n83 LFD or elite-base claim follows.

Primary definition: Caminata–Gorla, https://arxiv.org/html/2112.05579,
Section1's repeated row reduction and bounded multiplication, and its V_d
definition. The proposed full-size bridge remains in
PRIME_FILTERED_FUNCTIONAL_DESIGN.md; these controls can falsify mistakes
in it but do not prove its universal hypotheses.
