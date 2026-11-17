# Reduce the extra-step question to a character compatibility problem

Status: algebraic reduction and a concrete proof obligation; the universal
extra-step assertion is neither proved nor disproved here.

Work on a specified fine-moduli stratum where the twelve source kernels
and every eligible prepared degree-three neighbor are consistently labelled.
Let u_1,...,u_N be their nonzero obstruction representatives. It is essential
to define these as rational functions with certified divisors and descent;
independently chosen signs or a fitted relation on numerical values do not
provide this definition.

Write epsilon_i=0 for a square and 1 for a nonsquare at a finite-field
specialization. Every exact relation

    product_i u_i^e_i = c * v^2

implies sum_i e_i*epsilon_i = chi_bit(c) in F_2, away from zeros/poles.
The all-obstructed star would have epsilon=(1,...,1). A single certified
relation with sum_i e_i != chi_bit(c) excludes that star. More generally,
assemble the certified relations as A*epsilon=b and test whether
A*ones=b. This is the exact acceptance criterion for a parity-based proof.

Over an algebraically closed constant field, Kummer theory says that the
geometric Galois group of adjoining all sqrt(u_i) is the orthogonal
complement of the square-relation space. The all-minus automorphism exists
if and only if every square relation has even weight. Thus:

* An odd-weight square-product identity would forbid an all-negative star
  on the stratum where it is valid.
* If every square relation has even weight, parity relations alone cannot
  forbid that geometric sign assignment. This does NOT produce an eligible
  finite-field curve, a prime group order, or a certified counterexample.

Arithmetic Frobenius permutations, constants, ordinary/non-subfield gates,
zeros/poles, and prime order remain separate constraints. In particular,
geometric independence plus Chebotarev cannot simply be asserted to
produce prime-order elliptic-curve groups.

## What the native computation now supplies

obstruction.cpp reconstructs the Heisenberg actions, trace-projector
tensors and chi18 representatives in a new FLINT field implementation.
It reproduces the complete saved 26-node component and the p=157 bad
prepared center. Every flagged zero center has all its eligible neighbors
evaluated. This is native replay of finite evidence, not a symbolic identity.

For both saved bad centers, the product of all candidate characters on
eligible neighbors is -1. There are respectively 48 and 24 neighbor
candidates, so an all-negative assignment would instead have product +1.
This suggests a testable **conditional norm-product hypothesis**: on the
all-obstructed-center locus, that neighbor product is always nonsquare.
There are only two distinct centers behind this observation. It is not an
identity or a universal criterion and has no novelty claim.

## The next discriminating calculation

An actual symbolic step is now available: [THETA_ACTION.md](THETA_ACTION.md)
gives an explicit rational three-torsion translation lift, with exact integer
polynomial certificates for the translation law, order three and discriminant.
All 232 nonzero actions on the 29 saved fixtures match the old sampled
construction. This eliminates a generic-point interpolation dependency in the
prospective modular-function computation. It does not supply its missing
neighbor-label or obstruction-divisor calculation.

Construct the labelled obstruction functions on the level-nine moduli
cover, compute their divisors modulo two (including boundary components),
and retain the arithmetic square classes of all constant factors. Prove
the norm-product hypothesis, refute it by an eligible specialized star, or
identify a different inconsistent character relation. A numerical relation
without this divisor calculation is insufficient.

Prepared-neighbor eligibility varies with the E[9] Frobenius stratum.
An identity established in the scalar-nine stratum alone does not cover
the p=157 bad center, which has only four rational ninth-division x-roots.
The label transport and divisor calculation are not implemented. This is
the named remaining blocker, rather than a request for more random samples.
