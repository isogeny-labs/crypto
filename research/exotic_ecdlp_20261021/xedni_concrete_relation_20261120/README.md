# A concrete bounded-relation theorem for free-plus-torsion xedni lifts

The integrated Lean theorem `bounded_relation_of_free_torsion_rank` turns the exact finite quotient and a rank gap into a bounded relation for points in `(Fin d → ℤ) × A`, where `A` is any finite abelian group. It composes the [quotient collision theorem](../xedni_uniform_quotient_20261120/README.md) and the [explicit free-plus-torsion quotient](../xedni_free_torsion_quotient_20261120/README.md) without assuming a quotient map, its cardinality, or its kernel law as input.

## Mathematical statement

Fix `r>d`, coefficient radius `K`, modulus `N>0`, and an integer `c` with `N≤cK` and `|A|c^d<K`. Let `L₁,…,Lᵣ` be points of `ℤ^d×A`. Give the free coordinates a real-valued norm `ν`. Assume its triangle law on differences and `ν(Nu)=Nν(u)`. Assume every nonzero free vector has norm at least `m`, and `N` annihilates `A`. Finally, suppose every coefficient-box sum `Σ aᵢLᵢ`, `|aᵢ|≤K`, has free norm at most `S`, with `2S<Nm`.

The theorem produces a nonzero integer vector `b` with `|bᵢ|≤2K` and `Σ bᵢLᵢ=0`. The rank inequalities give `|A|N^d<(2K+1)^r`. The explicit quotient reduces the free coordinates modulo `N` and preserves `A`; its kernel is `N`-divisible. Since `N` kills `A`, a nonzero `N`-multiple has nonzero free coordinate and norm at least `Nm`. Two box sums with the same quotient image must therefore be the same point, and the larger coefficient box forces a collision.

This supplies the full finite-group and rank-count part of the conditional bounded-relation step in [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1. To apply it to each rational elliptic-curve lift, the remaining mathematical bridge is an explicit Mordell–Weil decomposition and a canonical-height norm satisfying the stated uniform lower and upper bounds. The theorem makes those inputs visible; it does not encode a curve-selection probability law.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Bounded relation on the explicit product group | `XedniConcreteRelation.bounded_relation_of_free_torsion_rank` | Finite abelian `A`; `d<r`; `N≤cK`; `|A|c^d<K`; free norm triangle and scaling laws; box-sum radius; `N` kills `A`; positive free norm minimum; `2S<Nm` | Checked |
| Rational elliptic-curve instantiation | — | Mordell–Weil decomposition and canonical-height bounds for the selected family | Open |
| Curve-selection probability | — | Exact sampling and selection law | Open |

`formal/UniformQuotient.lean` and `formal/FreeTorsionQuotient.lean` are exact pinned source copies of the preceding proof packets, compiled before `formal/ConcreteRelation.lean`. `formal/build.sh` records the reproducible command using Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. Generated `.olean` files stay outside the committed packet; the compiler output and `#print axioms` audit are retained in `formal/build-output.txt`. All twelve audited declarations use only standard Lean foundations (`propext`, `Classical.choice`, `Quot.sound`); `lean4checker --fresh` is unavailable on this host.
