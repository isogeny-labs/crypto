# A published odd-characteristic quartic GHS cascade missing from the local family index

15 November 2026. This is a prior-work audit, not a new attack claim. The primary source is Arita, Matsuo, Nagao and Shimura, [*A Weil Descent Attack against Elliptic Curve Cryptosystems over Quartic Extension Fields*](https://eprint.iacr.org/2004/240.pdf), dated 14 September 2004, later published in *IEICE Transactions on Fundamentals* 89-A(5), 1246–1254 (2006). The [local 21-entry weak-family catalog](../weak_curve_catalog_20261010/REPORT.md) has generic Weil-descent and odd cubic/quadratic entries but no dedicated Scholten-form quartic cascade entry. This is an index coverage gap, **not** a new family in the literature. [pcc:unverified]

## Family and construction

Let `k=F_q` have odd characteristic and `k ⊂ k₂ ⊂ k₄` be the quadratic tower. A curve over `k₄` in Scholten form has

`E: y²=a x³+b x²+b^(q²)x+a^(q²)`, with `a,b∈k₄`.

The paper states a necessary and sufficient *model* condition: an elliptic curve `y²=f(x)` over `k₄` can be transformed to this form if the cubic `f` splits completely over `k₄`, or if `f` is irreducible over `k₄` and `j(E)∉k₂`. The statement is about existence of this model, not a count of source isogeny classes or an attack-cost guarantee. [pcc:unverified]

For `c∈k₄\k₂`, the paper constructs a genus-two curve `H/k₂` with

`Y²=a(X−c)^6+b(X−c)^4(X−c^(q²))²+b^(q²)(X−c)²(X−c^(q²))^4+a^(q²)(X−c^(q²))^6`

and a degree-two map to `E`, with `x=((X−c)/(X−c^(q²)))²` and `y=Y/(X−c^(q²))³`. The first subgroup transfer is the pullback followed by `k₄/k₂` trace. Lemma 1 proves that a source point of order at least `2q²+2` has nonzero image. In particular, for a selected prime `r>2q²+1`, this map is injective on the order-`r` subgroup. The final implication uses only that a nonzero homomorphism from a cyclic prime-order group is injective. [pcc:unverified]

The paper then takes a GHS section of the Weil restriction of `H/k₂` to `k`, obtaining a genus-nine curve `D/k` under its stated squarefreeness and conjugate-factor assumptions. This is a **two-step** route `E(k₄) → Jac(H)(k₂) → Jac(D)(k)`. Proposition 11 gives an additional condition for nonzero final image: the source prime order must be sufficiently large relative to the cover-map degree, and `r²` must not divide `#Jac(D)(k₄)`. The paper verifies scalar compatibility for its exact `q=71` example and gives a larger 160-bit-order example. Its index-calculus costs `O(q^(9/5))` and variants are asymptotic estimates, not measured full-workload times against matched rho in this audit. [pcc:unverified]

## Rigorous bounds and limits

The first map has an explicit subgroup threshold: `r>2q²+1` is sufficient for its image to be nonzero. For the second map, genus nine follows from the paper's construction hypotheses, and its subgroup condition must be checked per exact curve. A selected curve is therefore **not** certified vulnerable merely by the Scholten-form recognition test. Full source-to-target recovery and a same-subgroup total-cost comparison remain separate gates. [pcc:unverified]

This family is absent as a dedicated entry in the local catalog, but it is published prior art. The next novelty test is to find a cover or subgroup-preserving correspondence outside Arita's Scholten cascade, the binary Hess/two-pole and classical selectors, and the Momose–Chao odd cubic families. A proposed route fails if the selected prime-order image vanishes, if its geometry is a disguised instance of one of those constructions, or if complete measured work does not beat matched rho. [pcc:unverified]
