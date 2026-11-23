# Rational elliptic-curve torsion sharpens the xedni height obstruction

Mazur's classification of torsion on elliptic curves over `ℚ` gives a common torsion exponent of **2520** and a cardinality cap of **16**. The checked group-shape lemmas feed both constants into the varying-family xedni obstruction. For a rank cap `D`, point norm bound `U`, minimum nonzero free norm `m>0`, and integer `s>2rU/m`, the coefficient box now uses

`c = 2520(s+1)`, `B_max = 2(16c^D+1)`.

If `(2B_max+1)^r−1<q`, a compatible lift rule for every offset tuple contradicts the checked finite-field count. The curve, its torsion group, free rank, reduction map, and lifted points may vary with the tuple. The theorem applies when each torsion group has one of Mazur's fifteen shapes and the stated uniform norm and rank bounds hold.

## Mathematical proof and source

[Mazur, *Modular curves and the Eisenstein ideal*, Theorem 8](https://www.numdam.org/article/PMIHES_1977__47__33_0.pdf) classifies rational elliptic-curve torsion as `ℤ/nℤ` for `1≤n≤10` or `n=12`, or `ℤ/2ℤ×ℤ/(2v)ℤ` for `1≤v≤4`. The largest group has 16 elements. The LCM of all possible element orders is 2520. A group isomorphism with any listed shape transports the coordinatewise `2520`-annihilation to its elements. The preceding [varying-family theorem](../xedni_varying_family_obstruction_20261120/README.md) then supplies the bounded relation and all-sample contradiction.

The Lean predicate `MazurShape A` asks for the exact classified isomorphism. The Lean module proves the consequences of that isomorphism; it does not formalize Mazur's classification theorem or the identification of a particular selected rational curve's torsion subgroup. The latter and a good-reduction map must be supplied for an elliptic-curve instantiation. Uniform canonical-height upper and lower bounds across the curve-selection law remain an additional arithmetic obligation. The lower-height premise in [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1, is conditional.

## Exact parameter control

`parameter_gate.py` calculates strict integer subgroup-order thresholds with `T=16`, `U=m=1` and the same frozen rank cases as the earlier [cardinality-only control](../xedni_lcm_envelope_20261120/README.md). These are derived bounds, with no lift-selection measurements.

| Sample count `r` | Rank cap `D` | `16!` threshold bits | `lcm(1,…,16)` bits | Mazur exponent 2520 bits |
| ---: | ---: | ---: | ---: | ---: |
| 2 | 1 | 106 | 57 | 40 |
| 3 | 2 | 302 | 153 | 104 |
| 4 | 3 | 595 | 298 | 200 |

For `r=3,D=2`, the 104-bit threshold clears the coefficient-box budget for every 131-bit or 256-bit prime `q` **if** the stated `U=m=1` envelope applies. With `m=1` and integral `U`, the largest `U` clearing the budget for every 131-bit `q` is 27; for every 256-bit `q`, it is 51,420,113. The exact boundary integers and the other rows are in [parameter-gate.json](parameter-gate.json). At `r=4,D=3`, even `U=0` exceeds every 131-bit `q` under this box bound, while the 256-bit budget admits integral `U≤30` for every such `q`.

## Proof map and verification

| Result | Lean declaration | Hypotheses | Status |
| --- | --- | --- | --- |
| Multiples vanish in `ZMod n` | `XedniMazurTorsion.zmod_killed_of_dvd` | `n∣c` | Checked |
| Uniform annihilation for the fifteen shapes | `XedniMazurTorsion.mazur_shape_killed` | `MazurShape A` | Checked |
| Uniform torsion cardinality | `XedniMazurTorsion.mazur_shape_card_le` | Finite abelian `A`, `MazurShape A` | Checked |
| Height-derived varying-family obstruction | `XedniMazurTorsion.no_uniform_varying_height_lifts_mazur` | Prime `q`, per-tuple shape and compatible reductions, shared rank and norm bounds, explicit box budget | Checked |
| Classification for the selected rational curves | — | Mazur's theorem plus actual torsion-group identification | External arithmetic bridge |
| Uniform height and reduction instantiation | — | Actual selected rational points, good reduction, uniform height estimates | Open |

`formal/build.sh` verifies dependency source hashes and compiles the actual module with pinned Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. The saved `#print axioms` audit uses only Lean's standard `propext`, `Classical.choice`, and `Quot.sound`. Generated `.olean` files are excluded; `lean4checker --fresh` is unavailable on this host.
