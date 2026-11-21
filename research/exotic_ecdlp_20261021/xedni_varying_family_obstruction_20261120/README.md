# Xedni obstruction with a different selected lift group for each sample

The checked varying-family count removes the common lift-group and common reduction-map premise from the earlier adaptive xedni theorem. For each sampled offset tuple `t`, the selector may choose a different additive group `H(t)`, a different reduction homomorphism `ρ(t)`, and different lifted points. If every admitted tuple has a nonzero lifted relation bounded by one fixed integer `B`, the number of admitted tuples is at most `((2B+1)^r−1)/q` of the full uniform offset space, provided `B<q`.

The height corollary specializes each `H(t)` to `ℤ^(d(t))×A(t)` with a different finite abelian torsion group `A(t)` and free rank `d(t)`. It assumes shared bounds `d(t)≤D`, `|A(t)|≤T`, `d(t)<r`, upper point norm `U`, and minimum nonzero free norm `m`. A common positive integer `c` annihilates every `A(t)` and satisfies `2rU<cm`. The complete coefficient box uses

`K_max = T c^D + 1`, `B_max = 2 K_max`.

For each tuple, the [uniform-constant theorem](../xedni_uniform_constant_20261120/README.md) produces a relation bounded by `2(|A(t)|c^(d(t))+1)≤B_max`. The varying-family count then contradicts compatibility for **every** tuple when `(2B_max+1)^r−1<q`. The selector may inspect `t` before choosing its group, reduction map, and lifts.

## Mathematical proof and scope

The counting proof transports each lifted relation through that tuple's own homomorphism `ρ(t)`. Compatibility turns it into a reduced relation on the sampled point tuple `sᵢw−tᵢ`. All coefficient vectors lie in the same finite box because `B` is uniform, so the existing exact finite-field union count applies. The per-tuple groups and maps disappear from the counted event. The height corollary supplies the fixed box from rank, torsion, and norm envelopes before applying this argument.

For rational elliptic-curve lifts, the remaining arithmetic bridge is to instantiate each selected `H(t)` with its actual rational-point group, prove its good-reduction map, and establish the uniform canonical-height upper and lower bounds. The height lower bound used in [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), Lemma 4.1, is conditional; the Lean theorem keeps that input explicit.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Exact count for admitted varying-group lifts | `XedniVaryingFamily.varying_lift_sample_count` | Prime `q`, per-tuple additive group and reduction homomorphism, compatibility and a common bounded relation for each admitted tuple, `B<q` | Checked |
| All-sample contradiction with varying groups | `XedniVaryingFamily.no_uniform_varying_lift_rule` | Preceding hypotheses for all tuples; `(2B+1)^r−1<q` | Checked |
| Height-derived all-sample contradiction with varying rank and torsion | `XedniVaryingFamily.no_uniform_varying_height_lifts` | `d(t)<r`, `d(t)≤D`, `|A(t)|≤T`, shared `c,U,m`, per-tuple norm laws and good-reduction compatibility; `(2B_max+1)^r−1<q` | Checked |
| Rational elliptic-curve instantiation | — | Point-group decomposition, good reduction, uniform height bounds for the selected curve family | Open |

`formal/build.sh` verifies SHA-256 hashes of the checked counting and height dependency modules before compiling them and `formal/VaryingFamily.lean`. Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9` are pinned. The build output and `#print axioms` audit are retained; all three new declarations use only standard Lean foundations (`propext`, `Classical.choice`, `Quot.sound`). Generated `.olean` files are excluded; `lean4checker --fresh` is unavailable on this host.
