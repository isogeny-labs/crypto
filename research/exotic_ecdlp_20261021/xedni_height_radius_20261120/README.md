# Coefficient-box height radius for xedni lifts

The new theorem derives the box-sum radius used by the [concrete xedni bounded-relation proof](../xedni_concrete_relation_20261120/README.md) from bounds on the individual lifted points. For a seminorm `ν` with `ν(0)=0`, subadditivity, and `ν(aP)≤|a|ν(P)`, if each of `r` points has norm at most `U`, then every integer combination with `|aᵢ|≤K` has norm at most `rKU`.

The integrated theorem `bounded_relation_from_point_heights` substitutes this radius into the explicit free-plus-torsion result. Its geometric hypotheses are now pointwise upper norms and a lower norm `m` for nonzero free vectors. Together with the rank, torsion, and scaling assumptions, the numerical gap is `2rKU<Nm`. The resulting integer relation has a nonzero coefficient and all coefficients bounded by `2K`.

## Mathematical proof

Induct over the finite set of summands. The seminorm triangle inequality gives `ν(ΣaᵢPᵢ)≤Σν(aᵢPᵢ)`. Integer homogeneity bounds each term by `|aᵢ|ν(Pᵢ)≤KU`, so the sum is at most `rKU`. Applying the [exact quotient theorem](../xedni_uniform_quotient_20261120/README.md) through the [explicit quotient model](../xedni_free_torsion_quotient_20261120/README.md) yields the bounded relation.

For rational elliptic-curve lifts, the remaining bridge is to connect the free-coordinate seminorm to the square root of the canonical height and prove the pointwise upper bound and minimum positive height uniformly for the selected family. The height lower bound in [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1, is conditional. Its curve-selection law also needs a separate exact model before applying the paper's probability conclusion.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Individual norms imply coefficient-box radius `rKU` | `XedniHeightRadius.coefficient_box_sum_radius` | Additive group, zero and triangle laws, integer-scalar norm inequality, pointwise bound `U≥0` | Checked |
| Pointwise height bounds imply a relation | `XedniHeightRadius.bounded_relation_from_point_heights` | Explicit `ℤ^d×A`, rank and torsion conditions, free seminorm laws, pointwise upper bound, positive free minimum, `2rKU<Nm` | Checked |
| Rational-curve canonical-height instantiation | — | Mordell–Weil decomposition and uniform height inputs for selected lifts | Open |

The three predecessor Lean modules are copied exactly and source-pinned in `formal/`; `formal/HeightRadius.lean` is compiled after them by `formal/build.sh`. The local project pins Lean `v4.35.0-rc4` and Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9`. Generated `.olean` files are excluded; compiler output and `#print axioms` results are retained in `formal/build-output.txt`. The two new declarations use only standard Lean foundations (`propext`, `Classical.choice`, `Quot.sound`). `lean4checker --fresh` is unavailable on this host.
