# Adaptive xedni lift obstruction from uniform height bounds

The integrated theorem `no_uniform_adaptive_height_lifts` joins the exact height-derived coefficient limit with the previously checked count of relations among lifts selected after the random offsets are seen. It proves a contradiction if **every** offset tuple has a compatible selected lift in the same rank-`d<r` free-plus-finite group, all selected points obey the same height bound, and the complete bounded-relation coefficient box is smaller than the subgroup order.

The selector is an arbitrary function of the sampled offset tuple. The argument does not require it to choose curves or lifts independently of the offsets. The shared height bound supplies one coefficient box fixed before sampling; reduction carries every selected lifted relation into that box's reduced-point event, whose exact finite-field count controls all choices at once.

## Mathematical statement and proof

Let the lifted group be `ℤ^d×A` with finite abelian `A`, and let `r>d`. For each offset tuple `t∈(ZMod q)^r`, a selection rule returns `r` lifted points. Suppose one additive reduction map sends each selected point to the prescribed sampled reduced point `sᵢw−tᵢ`. Give the free coordinates a seminorm with the zero, triangle, and integer-scaling laws, a positive minimum `m` on nonzero free vectors, and a uniform upper bound `U` on every selected point. Choose a positive `c` that annihilates `A` and satisfies `2rU<cm`. Set `K=|A|c^d+1` and `B=2K`.

The [uniform-constant theorem](../xedni_uniform_constant_20261120/README.md) gives a nonzero relation bounded by `B` for each selected lift tuple. The existing dependent-lift count then bounds the number of offset tuples with a selected relation by `((2B+1)^r−1)/q` of the full sample space when `B<q`. If `(2B+1)^r−1<q`, this fraction is strictly below one, contradicting the assumed compatible lift for every tuple. The subgroup-order inequality implies `B<q`; the Lean proof checks that implication.

This replaces the bounded-coordinate lattice representation premise in the earlier rank-gap obstruction with norm and height-ratio premises. Applying it to a particular rational elliptic-curve lift family still requires a good-reduction homomorphism, a common group model or a proved transport among its selected groups, and the stated uniform canonical-height bounds. The conditional height assumptions in [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1, are a possible route to those bounds.

`parameter_gate.py` evaluates the exact inequalities for illustrative integer norm ratios. With `r=2`, `d=1`, `U=m=1`, and torsion order `1`, the smallest tested annihilating `c` is `5`, giving `K=6`, `B=12`, and a strict box budget for integer `q≥625`. At torsion order `16`, the same ratio gives `c=16`, `K=257`, `B=514`, and integer `q≥1,058,841`; `r=3`, `d=2` gives integer `q≥4,402,074,271,869`. An actual subgroup order must also be prime. These are derived sensitivity controls, recorded in `parameter-gate.json`, rather than measured elliptic-curve height ratios.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Adaptive all-sample height obstruction | `XedniAdaptiveHeightObstruction.no_uniform_adaptive_height_lifts` | Prime `q`; fixed `A,d,r,c,m,U`, reduction map; compatibility for every `t`; uniform point bound; norm laws; `d<r`; `c` annihilates `A`; `2rU<cm`; `(4K+1)^r−1<q` | Checked |
| Rational-curve instantiation | — | Good reduction, group transport across selected curves, uniform canonical-height bounds | Open |

`formal/XedniObstruction.lean`, `formal/HeightPigeonhole.lean`, and `formal/XedniDependentLift.lean` are pinned copies of the earlier adaptive-selection formalization. `formal/build.sh` checks SHA-256 digests of those sources and the five quotient/height dependencies before building the new module. The toolchain and Mathlib revision are pinned to Lean `v4.35.0-rc4` and Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9`; the complete compiler output and `#print axioms` audit are retained in `formal/build-output.txt`. The new theorem uses only standard Lean foundations (`propext`, `Classical.choice`, `Quot.sound`); `lean4checker --fresh` is unavailable on this host.
