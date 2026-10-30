# Explicit elliptic quotients of four cubic Hess descent curves

27 October 2026. The [constructor](construct.py), [certificate](certificate.json), [independent four-bit audit](audit.py), and [checked Lean identities](QuotientIdentity.lean) give explicit maps from all four exact genus-four Hess controls to their previously identified elliptic complements. This is a decomposition of a **known Hess family**. It is not an evaluated transfer to any of the residual genus-three quartics and supplies no new rho-beating attack.

## Sufficient family criterion and exact maps

Let `k` have characteristic two and let `H: y²+h(x)y=f(x)` be smooth. Suppose `h(x)=L(x)+h₀`, where `L` is a separable additive polynomial of degree four, and suppose polynomials `B` and `F` satisfy

`f(x)+B(x)²+h(x)B(x)=F(L(x))`, with `deg F=3`.

Then `z=L(x), v=y+B(x)` obey `v²+(z+h₀)v=F(z)`. If this cubic model is nonsingular, it defines an elliptic curve `D/k` and a morphism `H→D`. The morphism has degree four: over the splitting field of `L`, each of its four roots `c` gives the distinct deck transformation `(x,y)↦(x+c, y+B(x+c)+B(x))`. The displayed identity proves it preserves the equation; the transformations fix `z,v` and form the additive kernel of `L`. Conversely `x` satisfies `L(x)=z` and `y=v+B(x)`, so the function-field degree is at most four.

For the following exact data, use `k=GF(2)[a]/(a⁴+a+1)`. A coefficient vector `[c₀,…,c_d]` denotes `Σ c_i x^i`; each coefficient is its four-bit polynomial-basis integer code. The source `h,f` equations are in the [exact Hess ledger](../ghs_q16_4126_hess_g4_20261027/EXACT_MODELS.md). The normalized quotient coordinates are `X=F₃(z+h₀)`, `Y=F₃v`, giving `Y²+XY=X³+a₂X²+a₄X+a₆`.

| Source order / prime | `B` vector | `F` vector | quotient `[a₁,a₂,a₃,a₄,a₆]` | `#D(k)` |
| ---: | :--- | :--- | :--- | ---: |
| 4006 / 2003 | `[5,6,11,3,8,11,14]` | `[14,0,8,13]` | `[1,12,0,3,12]` | 22 |
| 4054 / 2027 | `[14,5,15,11,10,6,4]` | `[10,0,8,7]` | `[1,11,0,5,8]` | 14 |
| 4078 / 2039 | `[6,8,1,12,10,7,3]` | `[8,0,8,9]` | `[1,13,0,2,11]` | 14 |
| 4126 / 2063 | `[8,12,3,0,1,11,7]` | `[8,0,8,14]` | `[1,9,0,1,14]` | 14 |

The `L` vectors are respectively `[0,14,7,0,5]`, `[0,13,0,0,5]`, `[0,1,9,0,5]`, and `[0,1,8,0,5]`. Each has nonzero linear coefficient and four distinct roots over `GF(4096)`. The independently enumerated quotient orders equal `#Jac(H)(k)/#E(K)` in every row. The normalized elliptic models have nonzero discriminant. The quartet therefore realizes the complementary elliptic factors by explicit separable degree-four curve maps.

## Subgroup and attack boundary

The named source subgroups have prime orders `2003, 2027, 2039, 2063`, each coprime to the corresponding quotient order `22` or `14`. Hence the induced pushforward `Jac(H)(k)→D(k)` kills their already evaluated Hess images. The source prime subgroup occupies the three-dimensional complementary abelian factor. The quotient map makes that decomposition explicit, but **does not** identify that factor with any of the plane-quartic Jacobians or provide an evaluable map between them. A characteristic-two correspondence, its action on the named subgroup, and a full attack cost through the quartic remain open.

There is a route-specific obstruction to a simpler bridge. A separable morphism from a smooth genus-four Hess curve `H` to any smooth genus-three curve `C` would have degree at least two, but Riemann--Hurwitz would require `2g(H)-2=6 ≥ 2(2g(C)-2)=8`, which is false. Thus a genus-three bridge cannot be a direct separable curve quotient of `H`; it needs a correspondence or an abelian-variety isogeny. This does not exclude maps in the reverse direction or correspondences through a larger cover.

## Prior work and validation

[Hess's generalized GHS paper, Theorem 15](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf), already gives L-polynomial factorization for these Artin–Schreier descents. [Magma documents the existing source-to-Hess divisor map](https://www.math.uzh.ch/sepp/magma-2.19.8-cr/html/text1383.htm). The explicit quotient equations above were derived and checked in this project; targeted searches for a published characteristic-two genus-four Hess quotient formula found no exact match, which does not establish global novelty. The [recent genus-four `(2,2)` decomposition theorem](https://www.cambridge.org/core/journals/canadian-journal-of-mathematics/article/on-22decomposable-genus-4-jacobians/651E1FC3F6A593EB7B03A0642E81C7F0) expressly assumes characteristic different from two.

[Lombardo--Lorenzo García--Ritzenthaler--Sijsling's Galois-cover method](https://arxiv.org/abs/2003.07774) is a concrete prior route for realizing some Prym factors as Jacobians, including degree-four elliptic-cover cases. Its published development starts in characteristic zero, so using it here requires a new characteristic-two argument and an explicit evaluable correspondence. That is the next discriminating mathematical step for the genus-three quartics; the present quotient identity alone does not supply it.

`construct.py` solves 52 binary coefficient equations for each identity, checks the smooth elliptic models, and retains the coefficient vectors. The independent `audit.py` uses its own GF(16) multiplication and polynomial routines to recheck every identity and enumerate all `16×16` affine point pairs. `QuotientIdentity.lean` checks the four polynomial identities in a four-bit representation and a finite multiplication sanity condition. These checks do not formalize the smooth projective completion, deck-group geometry, or the original Magma source map.

```sh
sage -python research/ghs_q16_hess_elliptic_quotient_20261027/construct.py
python3 research/ghs_q16_hess_elliptic_quotient_20261027/audit.py
lean research/ghs_q16_hess_elliptic_quotient_20261027/QuotientIdentity.lean
```
