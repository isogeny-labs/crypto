# An explicit uniform coefficient constant for xedni lift relations

The checked theorem `exists_uniform_bounded_relation` constructs the coefficient limit in the conditional rational-lift argument. For a rank-`d` free group plus finite torsion `A`, `r>d` lifted points, a pointwise norm upper bound `U`, and a positive minimum free-vector norm `m`, it chooses an integer `c` that annihilates `A` and exceeds the ratio `2rU/m`. It then sets

`K = |A| c^d + 1`, `N = cK`.

The [point-height radius theorem](../xedni_height_radius_20261120/README.md) bounds every coefficient-box sum by `rKU`; `2rKU<Nm` follows from the choice of `c`. The [explicit quotient theorem](../xedni_concrete_relation_20261120/README.md) then yields a nonzero relation with every integer coefficient bounded by `2K`. No radius, quotient modulus, or strict cardinality inequality is supplied to the final theorem.

## Construction and exact assumptions

Choose a natural `s` with `2rU/m < s`, available because `m>0` and the natural numbers are unbounded in the reals. Set `c=|A|(s+1)`. The finite group cardinality annihilates every element of `A`, so `c` does too. The formula for `K` gives `|A|c^d<K`; the formula for `N` gives `N≤cK` as equality. Multiplying `2rU<cm` by positive `K` gives the kernel-gap inequality. The checked result is uniform over the selected points whenever `U`, `m`, `d`, `r`, and `|A|` have the declared uniform bounds.

The theorem uses a free-coordinate seminorm with zero, subadditivity, integer-scalar upper bound, and exact natural-scalar scaling; every nonzero free vector has norm at least `m`. These are mathematical inputs to connect to the canonical height of rational elliptic-curve points. The discriminant and Lang-type height premises in [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1, are a proposed source of a uniform `U/m` ratio. An exact model of xedni's curve-selection distribution is still needed for the probability statement in Theorem 4.1.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Explicit radius and modulus from an annihilating `c` | `XedniUniformConstant.bounded_relation_with_constant` | `d<r`; `c>0`; `c` annihilates `A`; norm and pointwise bounds; `2rU<cm` | Checked |
| Existence of an annihilating `c` and bounded relation | `XedniUniformConstant.exists_uniform_bounded_relation` | `d<r`; `m>0`; finite abelian `A`; free seminorm laws and pointwise bound `U≥0` | Checked |
| Canonical-height and Mordell–Weil instantiation | — | Rational-curve group equivalence; uniform upper and minimum positive heights for selected lifts | Open |
| Curve-selection probability | — | Exact adaptive sampling law | Open |

`formal/build.sh` verifies SHA-256 hashes of the four checked source dependencies before compiling them and `formal/UniformConstant.lean`. The toolchain and Mathlib revision are pinned to Lean `v4.35.0-rc4` and Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9`. The build output retains `#print axioms` audits for all declarations; the two new theorems use only standard Lean foundations (`propext`, `Classical.choice`, `Quot.sound`). Generated `.olean` files are excluded. `lean4checker --fresh` is unavailable on this host.
