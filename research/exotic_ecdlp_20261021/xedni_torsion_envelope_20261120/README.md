# A torsion-size envelope supplies the shared xedni annihilator

The varying-family obstruction now needs only a common bound `|A(t)| ≤ T` on the finite torsion groups. It constructs a single annihilator

`c = T! (s+1)`,

where the integer `s` is chosen above `2rU/m`. Thus the chosen lift group may still vary with every sampled tuple, while the relation coefficients have one explicit bound. This removes the separately supplied common-annihilator hypothesis from the preceding [varying-family theorem](../xedni_varying_family_obstruction_20261120/README.md).

## Mathematical proof

For each finite abelian group `A(t)`, Lagrange's theorem gives `|A(t)| • a = 0` for every `a`. Since `1 ≤ |A(t)| ≤ T`, its cardinality divides `T!`; hence `T! • a = 0`. Multiplying by `s+1` preserves annihilation. The choice `s > 2rU/m` and `m>0` yields `2rU < c m` because `s ≤ c`. Apply the checked varying-family height theorem with this `c`.

The resulting complete coefficient-box bound is

`B_max = 2 (T c^D + 1)`.

If `(2 B_max + 1)^r − 1 < q`, a compatible lift rule for every offset tuple leads to a contradiction. This factorial choice is explicit and conservative; a known smaller common torsion exponent would improve the numerical bound.

The theorem assumes prime subgroup order `q`, `d(t)<r`, `d(t)≤D`, `|A(t)|≤T`, norm laws for every selected free lattice, point norm at most `U`, and minimum nonzero free norm at least `m>0`. The reduction maps and lift points may differ with `t`; compatibility is stated for each tuple. The rational elliptic-curve bridge still requires a point-group decomposition, good-reduction maps, and uniform canonical-height estimates for the actual selected curve family. The lower-height premise in [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1, is conditional and remains an explicit hypothesis here.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| One factorial annihilates every group under a cardinality cap | `XedniTorsionEnvelope.factorial_kills_bounded_group` | Finite abelian `A`, `|A|≤T` | Checked |
| Varying-family height obstruction from torsion and norm envelopes | `XedniTorsionEnvelope.no_uniform_varying_height_lifts_factorial` | Prime `q`; per-tuple group, rank, reduction, lifts and norms; shared `D,T,U,m`; `s>2rU/m`; explicit coefficient-box budget | Checked |
| Rational elliptic-curve instantiation | — | Actual selected point groups, good reduction and uniform height estimates | Open |

Run `formal/build.sh` from any directory. It checks the source hashes of the earlier proof modules, then builds every imported module and `formal/TorsionEnvelope.lean`. The toolchain is Lean `v4.35.0-rc4` with Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. The retained `formal/build-output.txt` includes the `#print axioms` audit: both new theorems use only `propext`, `Classical.choice`, and `Quot.sound`. Generated `.olean` files are excluded. `lean4checker --fresh` is unavailable on this host.
