# Eliminating two witness blocks: projection argument

2026-11-15. Written derivation only; formalization pending. PCC-20261115-be28ec
remains unverified. This is standard separability/coprimality preprocessing,
not a new ECDLP mechanism or a solving-degree theorem. [pcc:unverified]

Let K be a field of odd characteristic, c != 0, d=2^t with t>=1, and let
A,C,U be polynomials with U monic of degree m. Let r be a field element.
Assume the retained monic-division chain enforces U | X^d-c, and

    A^2-f C^2 = (X-r)U,    U(r) != 0.

1. X^d-c is separable: its derivative is d X^(d-1), with d nonzero in K,
   and its only possible common root with that derivative is 0, which is not
   a root because c!=0. Every divisor U is therefore squarefree.
2. (X-r)U is squarefree, since U is squarefree and U(r)!=0. If U and C
   had a common root alpha in an algebraic closure, the norm identity would
   force A(alpha)=0. Both A^2 and f C^2 would then be divisible by
   (X-alpha)^2, contradicting squarefreeness of (X-r)U. Thus gcd(U,C)=1.
3. Over a field the usual degree-bounded Bezout coordinates for (U,C) and
   (U,U') consequently exist and are unique: reduce the second coefficient
   modulo monic U to degree <m, then the first has degree <deg(C) or
   <deg(U'), respectively. These satisfy the original bounds even when
   C or U' has degree below its declared maximum. Conversely, discarding
   witnesses preserves every core equation.

Hence projection gives a bijection of geometric points of the explicit-chart
and witness-pruned systems, with the target inverse retained. This argument
claims equality of point sets, not an independently checked equality of ideals,
a scheme isomorphism, a backend provenance certificate or a degree bound.
It does not apply to c=0, characteristic2, an unproved membership chain or
removal of the target-separation inverse. [pcc:unverified]

The retained chain implies divisibility by induction: V_0=X and
V_(j-1)^2=Q_j U+V_j imply V_j = X^(2^j) modulo U; V_t=c closes it.
Quotients and remainders stay bounded and uniquely determined by monic
Euclidean division. Eliminating the Bezout blocks removes 7m-5 over2
variables: h-1+m from (U,C) and 2m-1 from (U,U'), with h=(m+1)/2.
Concretely these are9,16,23 variables for m=3,5,7. The count itself says
nothing about the cost of constructing a basis. [pcc:unverified]

## Sources and prior checks

The prior selected-scheme [reducedness review](../prime_one_shot_20261029/continuation/reviews/torus-and-reducedness.md#8-full-selected-normremainder-scheme-is-reduced-at-fixed-input)
already uses squarefree norm/coprimality and uniquely bounded witness graph
extensions. This note applies that existing reasoning as an encoding reduction;
no novelty is claimed. The candidate retains direct equation, root, denominator
and group-sum checks after extraction.

Code-decoding was also considered and not reintroduced as new: the existing
[code/guessing boundary](../prime_one_shot_20261029/continuation/theory/code-and-guessing-boundaries.md)
records the exact rank-defect formulation and generic root-guessing/list-decoding
limits; [elliptic code experiments](../elliptic_code_list_decoding_20261107/RESULTS.md)
remain scoped and unresolved. Qi Cheng's [primary paper](https://arxiv.org/abs/cs/0507026)
proves randomized subset-sum hardness for particular elliptic AG-code families
with exponential alphabet size. It is not a barrier for the structured PKM
factor bases, nor a proof that every decoding approach fails.

## Formalization map / instance gate

- Structure above: `Polynomial K` over a field, divisibility, separability,
  squarefreeness and `IsCoprime`; algebraic-closure roots when needed.
- Existing laws sought: separability of X^d-c for nonzero c and invertible d;
  divisors of squarefree polynomials; squarefree product criterion; a common
  polynomial factor of A and C square-divides A^2-f C^2; degree-bounded Bezout.
- Object as instance: the retained monic divisor U and its norm N=(X-r)U.
- Assembly probe: pending availability of the pinned Mathlib/crypto-lean
  dependency and exact declaration search. No Lean file or theorem is asserted.
- Missing components: generic shared-law assembly belongs in crypto-lean if
  absent, not copied into this study. Instance endpoint then applies that law.
- Inheritance: field/polynomial algebra from Mathlib; any missing generic norm
  chart law from crypto-lean; only parameter/input discharge in this study.

This map is a formalization attempt inventory, not a passed instance gate.
No `lake build`, axiom audit or independently certified theorem is reported.
The written projection argument and complete-set software controls are the
current evidence; general solver complexity and the original goal remain open.
[pcc:unverified]
