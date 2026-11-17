# A genus lower bound across cubic Weil-restriction transfer constructions

27 October 2026. This finite classification applies to the complete 8,190 normalized ordinary curves in the [frozen GF(4096) census](../ghs_cubic_hess_closure_q16_20261027/certificate.json). It extends the previous **construction-specific** Hess genus census with a bound for *any algebraic correspondence* from the cubic Weil restriction. It is not a newly vulnerable curve family or a claim that any new transfer beats rho.

## Criterion

Let `k=GF(16)`, `K=GF(4096)`, and `E/K` be one of the source curves. Write `N=#E(K)` and `t=4097-N`. For `A=Res_(K/k)(E)`, the Frobenius polynomial is

`F_E(T)=T^6-t*T^3+4096`.

If `F_E` is irreducible over `Q`, then `A` is `k`-simple. A nonzero homomorphism of abelian varieties `A -> Jac(C)` has finite kernel, so its image has dimension three. **Every such target curve has genus at least three.** This applies to GHS norm/conorm maps, covers, and isogeny correspondences that induce a nonzero algebraic homomorphism. It does not constrain an arbitrary algorithmic homomorphism between the finite groups of rational points. A geometrically nonzero map may also annihilate a particular input subgroup, which must be checked separately.

Independently, if a target class has prime order `r`, Weil's point-count bound gives `r <= #Jac(C)(k) <= (sqrt(16)+1)^(2g)=25^g`. Thus `g >= ceil(log(r)/log(25))`; in particular `r>625` requires `g>=3` even without simplicity. This is a necessary bound on a surviving target subgroup, not an attack-cost bound.

## Exact finite census

The [Sage certificate generator](census.py) factors all 128 distinct integer polynomials and records a prime below 100 at which each of the 120 simple-class polynomials remains irreducible. Since each polynomial is monic, irreducibility after reduction proves irreducibility over `Q`. The [independent SymPy audit](audit.py) repeats every integer factorization and every modular irreducibility check. [Lean](SimpleCounts.lean) checks the arithmetic partitions and the consequence `r>625` plus `r<=25^g` implies `g>=3`; the modular arithmetic and abelian-variety theorem are external to Lean.

| Class | Distinct orders | Exact source models |
| :--- | ---: | ---: |
| Irreducible `F_E`, hence simple `A` | 120 | 7,584 |
| Reducible `F_E` | 8 | 606 |
| Simple and outside certified genus-three Hess isogeny closure | 46 | 2,691 |
| Reducible and outside that closure | 3 | 184 |
| Simple, outside closure, catalog genus-three Jacobian class present | 44 | 2,643 |
| Simple, outside closure, no catalog genus-three Jacobian class | 2 | 48 |

The eight reducible orders are `3980, 3982, 4050, 4090, 4104, 4144, 4212, 4214`. Their polynomial factor-degree profile is `(2,4)`; all other listed orders have irreducible degree-six polynomial. The four exact high-prime quartic-source orders `4006, 4054, 4078, 4126` are simple. Their modular irreducibility witnesses are respectively primes `5, 11, 5, 5`. For those four, the previously verified plane quartic gives a genus-three Jacobian in the same isogeny class, so the lower bound is **attained as an abstract isogeny-class dimension**. An evaluable source-to-quartic map with verified action on the named subgroup remains missing.

In the residual simple, catalog-positive class, the 2,643 equations form a **structural search family**: they have a genus-three carrier at the level of an isogeny class and no algebraic carrier of smaller genus. This is an exact classification within this finite census, not a new attack construction or a statement that all 2,643 are vulnerable. Of all 8,190 source models, 1,986 have largest prime factor greater than 625; 984 of those are outside the Hess genus-three closure. Those are also subject to the point-count lower bound.

## The elliptic complement in the four complete Hess controls

The [four evaluated direct Hess maps](../ghs_q16_4126_hess_g4_20261027/EXACT_MODELS.md) are nonzero on their named prime-order generators. Since their source restrictions are `k`-simple, each image is a three-dimensional abelian subvariety of the genus-four Hess Jacobian. Poincare complete reducibility therefore gives a `k`-isogeny `Jac(H) ~ Res(E) × B` for an elliptic curve `B/k`. The exact recorded group orders determine `#B(k)` and its trace:

| Source order | `#Jac(H)(k)` | `#B(k)=#Jac(H)(k)/#E(K)` | `Tr(B)=17-#B(k)` |
| ---: | ---: | ---: | ---: |
| 4006 | 88132 | 22 | -5 |
| 4054 | 56756 | 14 | 3 |
| 4078 | 57092 | 14 | 3 |
| 4126 | 57764 | 14 | 3 |

This decomposition is an algebraic consequence of the verified maps and source simplicity. It supplies no explicit projection from the Hess Jacobian to a genus-three quartic Jacobian. In particular, the recent [genus-four `(2,2)` decomposition theorem](https://www.cambridge.org/core/journals/canadian-journal-of-mathematics/article/on-22decomposable-genus-4-jacobians/651E1FC3F6A593EB7B03A0642E81C7F0) assumes characteristic different from two and cannot be applied directly here; a characteristic-two correspondence and its action on the named subgroup remain the next proof and implementation obligations.

## Prior work and novelty boundary

[Tate's isogeny theorem](https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf) supplies the standard Frobenius/isogeny criterion. [Hess's generalized GHS construction](https://doi.org/10.1112/S146115700000108X), the [published characteristic-two field-family analysis](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf), and [Magma's documented norm/conorm map](https://www.math.uzh.ch/sepp/magma-2.19.8-cr/html/text1383.htm) predate this census. The new contribution here is the **exact local cross-classification** of all 8,190 equations against simplicity, the existing Hess closure, and the catalog carrier check. The general dimension theorem and both known transfer constructions are prior work. The four candidate isogeny classes and their per-candidate publication checks are in the [quartic search ledger](../ghs_q16_residual_quartic_search_20261027/prior_work.json).

For the four exact source groups, [complete direct Hess genus-four transfers](../ghs_q16_4126_hess_g4_20261027/REPORT.md) preserve their named subgroups and recover the supplied targets, but cost 512–698 times the same-source rho control in the recorded backend. That measured result does not transfer to the genus-three quartics. A new vulnerability finding still requires an evaluable source-to-genus-three map, nonzero action on the input subgroup, complete target-dependent relation/decomposition work, and a lower total cost than matched rho.

## Reproduction

```sh
sage -python research/ghs_q16_simple_restriction_bound_20261027/census.py
python3 research/ghs_q16_simple_restriction_bound_20261027/audit.py
lean research/ghs_q16_simple_restriction_bound_20261027/SimpleCounts.lean
```
