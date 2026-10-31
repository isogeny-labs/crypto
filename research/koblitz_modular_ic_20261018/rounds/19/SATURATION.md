# Coupled elliptic elimination at the actual boundary

Q29 profiles the four frozen Q28 models on support[95,97,123]. Nodes95
and97 are degree9 pure-x source polynomials, already the maximum in
their univariate cones. There is no higher-degree pure-x comparison to
use. That failed trigger is retained in q29-source-profiles.json.

The actual source DAG gives U=Z95=P(x), V=Z123=y*S(x), with deg P=9
and deg S=8. Gate123 is exactly Mul(1,122). The ordinary input equation
is y^2+xy=x^3+a*x^2+1, with a extracted from its actual cubic gates.
Hence

    V^2+x*S(x)*V+S(x)^2*(x^3+a*x^2+1)=0.

This norm polynomial has degree19 in x. In the rank9 algebra with
P(x)=U, take its multiplication determinant. Each graph produces a
191-term polynomial R(U,V). Exact substitution U=P(x),V=y*S(x),
followed by elliptic reduction, gives zero. After normalization the
only total-degree19 term is U^19. Homogenization therefore gives

    R_h(h,U,V)=U^19+h*T(h,U,V).

At h=0 every geometric point of the reduced projective closure must have
U=0. All four Q28 models have U=1, so they are excluded under the actual
source-cone/normalization bridge. The relation lies in the h-saturation:
after inverting h every source gate equals its recorded affine function,
where the exact norm identity vanishes. No explicit minimal h-power
membership certificate or equality of whole saturated ideals is claimed.

This argument uses no coefficient-Frobenius valuation rule. Its selected
cones contain only Add, Mul, constants and the actual elliptic input
relation. The full F360 polynomial has leading coefficient1. The exported
bit0 scalar polynomial rejects the retained F2-valued model; the full
monic relation gives the geometric radical consequence over extensions.

## Q30 through Q32

Retaining every Q13--Q17 constraint, all five source cube and square
repairs, and the new node95-zero consequence gives four Q30 models on
[120,122], each in71 visits. These two nodes are a degree8 source
polynomial and its constant translate. Compare every distinct actual
pure-x input0 polynomial of degree1 through8 with P(x) at node95.
For a lower polynomial A(x), the determinant of P(x)-V in the algebra
A(x)=U has only U^9 at total degree9. Exact univariate substitution
checks every relation independently of the determinant algorithm.

Q31 computes66,66,67,67 distinct resultants, covering70,70,71,71 actual
source-node aliases. Their recorded homogeneous polynomials and source
cones prove the corresponding radical boundary zeros. They exclude all
four Q30 models. Q32 retains all those constraints and finds four new
complete checked models with identical support[123,124,126,127], in9
visits each without a cap. These models remain unclassified. Node123 is
now a highest-pole y*S component, so the failed lower-degree comparison
must not be repeated as though it applies to it.

## Proof and remaining work

Lean proves the universal elliptic norm identity, determinant vanishing
from an actual nonzero evaluation-kernel vector, unit-chart cancellation,
and the reduced monic boundary implication. It also checks actual Q28 and
Q30 source-node bindings, scalar boundary separators, every Q31 leading
term list and complete Q30/Q32 finite constraints. The connection from the
exported native multiplication matrices to that evaluation vector, the
F360 implementation, full polynomial substitutions and scheme saturation
are not formalized. They are not replaced by assumed geometric axioms.

Next discriminating test: construct a complete simultaneous source arc
for Q32, including every inverse witness, Kummer root and coefficient
Frobenius component; or find another source-bound polynomial rejecting it.
A leading-pole estimate or partial arc does not establish membership in
the closure. Full quotient saturation, proper boundary gluing and closed
polarized embeddings remain open.
