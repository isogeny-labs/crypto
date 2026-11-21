# The optimal cardinality-only torsion annihilator sharpens the xedni bound

For finite lift torsion groups with only a common cardinality cap `|A(t)| ≤ T`, the universal annihilator can be set to `L(T) = lcm(1,…,T)`. This divides every positive integer that is a multiple of all admissible cardinalities. Since every positive `n≤T` occurs as the order of a cyclic group, `L(T)` is the smallest annihilator that follows from the cardinality cap alone. The checked varying-family height obstruction now uses `c=L(T)(s+1)` for an integer `s>2rU/m`, instead of the larger `T!(s+1)` construction in the preceding [torsion-envelope packet](../xedni_torsion_envelope_20261120/README.md).

## Mathematical proof

Induction on `T` proves that every `1≤n≤T` divides the recursive LCM `L(T)`, and that `L(T)` divides any integer divisible by all those `n`. For a finite abelian `A(t)`, its cardinality divides `L(T)`, and its cardinality annihilates every element, so `L(T)` annihilates the group. The norm inequality `2rU<cm` follows from `s>2rU/m`, `m>0`, and `s≤c`. Applying the checked varying-family theorem yields a contradiction for a compatible all-tuple lift rule when

`(2 B_max + 1)^r − 1 < q`, where `B_max = 2(T c^D + 1)`.

The theorem retains prime `q`, per-tuple free rank `d(t)<r` and `d(t)≤D`, torsion cardinality `|A(t)|≤T`, point free norm at most `U`, minimum nonzero free norm at least `m>0`, the stated norm laws, and compatibility of each selected reduction map. The selected lift groups, maps, and points may vary with each offset tuple.

## Exact parameter control

`parameter_gate.py` computes integer thresholds without sampling or timing. Its frozen cases use `U=m=1`; `T` is a specified cardinality envelope, not an assertion about a particular elliptic-curve family. The listed bit length is that of the smallest **integer** `q` meeting the strict coefficient-box budget; a prime subgroup order must additionally satisfy the theorem's prime hypothesis.

| `r` | rank cap `D` | `T` | `T!` threshold bits | `L(T)` threshold bits |
| ---: | ---: | ---: | ---: | ---: |
| 2 | 1 | 16 | 106 | 57 |
| 3 | 2 | 16 | 302 | 153 |
| 4 | 3 | 16 | 595 | 298 |
| 3 | 2 | 8 | 125 | 92 |
| 3 | 2 | 4 | 58 | 52 |

For `r=3,D=2,T=16,U=m=1`, the factorial threshold exceeds every 256-bit subgroup order, while the LCM threshold is below every 256-bit subgroup order. This comparison concerns the algebraic budget only. The [machine-readable result](parameter-gate.json) retains the exact integers, annihilators, coefficients, and 256-bit classification.

## Proof map and verification

| Result | Lean declaration | Hypotheses | Status |
| --- | --- | --- | --- |
| Positivity of `L(T)` | `XedniLcmEnvelope.universalExponent_pos` | Natural `T` | Checked |
| Divisibility by every admissible cardinality | `XedniLcmEnvelope.dvd_universalExponent` | `0<n≤T` | Checked |
| Minimality in the divisibility order | `XedniLcmEnvelope.universalExponent_dvd_of_all` | `n∣M` for every `0<n≤T` | Checked |
| Annihilation of a bounded finite abelian group | `XedniLcmEnvelope.universalExponent_kills_bounded_group` | Finite abelian `A`, `|A|≤T` | Checked |
| Varying-family height obstruction with `L(T)` | `XedniLcmEnvelope.no_uniform_varying_height_lifts_lcm` | Prime `q`; per-tuple group, rank, reduction, lifts and norms; shared `D,T,U,m`; `s>2rU/m`; explicit coefficient-box budget | Checked |
| Rational elliptic-curve instantiation | — | Actual selected point groups, good reduction, and uniform height estimates | Open |

The next arithmetic step is to bound the actual torsion exponents and canonical heights for a specified family of rational lifts, then insert those bounds into the same exact gate. The lower-height premise in [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1, remains explicit.

`formal/build.sh` verifies dependency SHA-256 hashes and builds the actual theorem module. Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9` are pinned. `formal/build-output.txt` retains the `#print axioms` audit; the new declarations use only Lean's standard `propext`, `Classical.choice`, and `Quot.sound`. Generated `.olean` files are excluded. `lean4checker --fresh` is unavailable on this host.
