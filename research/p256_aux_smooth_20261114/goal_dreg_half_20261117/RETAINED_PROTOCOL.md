# Exact retained-point membership presentation (native proof development)

The previous full-fibre tower includes nonlifting extension points absent
from the retained dense base. Close that semantic gap before claiming any
matched algebraic improvement. This is not a claim about complete relations.

Use the unchanged deterministic constructor at existing depths2,4,5, fixed
seed and original FB1 codec. Depth5's expected existing identifier is
FB1h8bc0ab2ed2af (21 columns/42 signs); verify identity, do not search seeds
or targets. No relation truth table or unknown discrete log is input.

Build the vanishing ideal of the **known retained points only**, in two
embeddings receiving the same Buchberger–Möller construction:

1. Dense: target affine coordinates(x,y).
2. Structured: (x,y,x_1,...,x_(d-1)), where the extra coordinates are the
   fixed auxiliary chain's rational abscissae for that column. Target signs
   retain the same chain coordinates. No auxiliary ordinate/sign cover.

Use standard graded reverse lexicographic order. Native exact GMP arithmetic
at the actual P256 prime must generate and check the affine basis, all
nontrivial S-pair identities, point vanishing, number of standard monomials,
and independent full-rank evaluation of those monomials on every point.
Homogenize **the degree-compatible basis** with H last; directly check its
homogeneous S-pairs and that all leading terms are H-free. Standard monomials
then give a free Fp[H] basis, so H is regular and the quotient is CM of
dimension1. Its Hilbert numerator's last degree r gives exact ideal-module
regularity r+1. This supplies equality, not merely a leading-ideal bound.

Completeness certificate: N distinct rational points, an N-dimensional
quotient and nonsingular evaluation matrix imply the ideal equals their
radical vanishing ideal, including over the algebraic closure. Extra chain
coordinates have unique rational reconstruction with nonzero denominators.
The affine algebras must be isomorphic; no excluded/nonlifting points remain.

Also emit the abscissa-only controls to separate target-curve/sign costs.
Record setup field operations, terms, variables, equations, Hilbert data and
all construction failures. These are exact correctness units, not runtime
benchmarks or novel algorithms. Buchberger–Möller is prior art; the tower's
squarefree basis is the existing construction. No novelty claim.

Bounds: six signed-point cells and six abscissa-only controls, depths2,4,5;
one native invocation<=120s/2GiB, <=10000 generated monomials/cell,
<=1000000 reduction steps/S-pair, no partial certificate admission or retry.
Reject wrong regularity, altered generator and repeated point controls.
Replay deterministic exports byte-identically in a fresh directory.

The native units qualify a reusable **membership** presentation built from
the already-materialized base. They do not enumerate relations. Preserve
the full m=2–7/depth8 objective and its preregistered **complete generator
homogenization** metric. The selected-point membership ideals are saturated;
do not substitute their degree for complete relation regularity or a measured
F4 solving degree. Full systems may subsequently reuse these memberships
only under a frozen named paired comparison, exact replay and paid setup.
